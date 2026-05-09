use std::path::PathBuf;
use std::sync::Mutex;

use phoenix_core::crypto::{
    address::AddressKind,
    hashcat::{build_command, AttackMode, HashcatMode},
    reconstruct::{brute_force_passphrase, reconstruct_missing_word, reconstruct_multi},
};
use phoenix_core::forensic::{
    Bip39TextExtractor, BitwardenCsvExtractor, ChromeHistoryExtractor, ExtractorRegistry,
    MboxExtractor,
};
use phoenix_core::interview::{
    candidate::{self, Candidate},
    interviewer::Interviewer,
    questions::{Question, QuestionBank},
    session::{InterviewSession, SessionStore},
    state::MemoryState,
};
use phoenix_core::llm::OllamaClient;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AppInfo {
    pub name: String,
    pub version: String,
}

#[tauri::command]
pub fn app_info() -> AppInfo {
    AppInfo {
        name: "Phoenix".to_string(),
        version: phoenix_core::version().to_string(),
    }
}

pub struct InterviewState {
    pub store: SessionStore,
    pub bank: QuestionBank,
    pub registry: ExtractorRegistry,
    pub current: Mutex<Option<InterviewSession>>,
    pub llm_endpoint: String,
    pub llm_model: String,
}

impl InterviewState {
    pub fn new() -> Self {
        let mut data_dir = dirs::data_local_dir().unwrap_or_else(std::env::temp_dir);
        data_dir.push("phoenix");
        data_dir.push("sessions");
        let store = SessionStore::new(&data_dir).expect("init session store");
        let bank = QuestionBank::from_embedded().expect("embedded question bank");
        let registry = ExtractorRegistry::new()
            .register(BitwardenCsvExtractor)
            .register(ChromeHistoryExtractor)
            .register(MboxExtractor)
            .register(Bip39TextExtractor);
        Self {
            store,
            bank,
            registry,
            current: Mutex::new(None),
            llm_endpoint: "http://localhost:11434".to_string(),
            llm_model: "qwen3:14b".to_string(),
        }
    }
}

impl Default for InterviewState {
    fn default() -> Self {
        Self::new()
    }
}

#[tauri::command]
pub fn list_questions(
    state: tauri::State<'_, InterviewState>,
) -> Result<Vec<Question>, String> {
    Ok(state.bank.ordered().into_iter().cloned().collect())
}

#[tauri::command]
pub fn start_interview(
    state: tauri::State<'_, InterviewState>,
) -> Result<String, String> {
    let session = InterviewSession::new();
    let id = session.id.clone();
    state
        .store
        .save(&session)
        .map_err(|e| e.to_string())?;
    *state.current.lock().unwrap() = Some(session);
    Ok(id)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnswerResult {
    pub extracted_nodes: usize,
    pub total_candidates: usize,
}

#[tauri::command]
pub async fn answer_question(
    state: tauri::State<'_, InterviewState>,
    session_id: String,
    question_id: String,
    content: String,
) -> Result<AnswerResult, String> {
    let mut session: InterviewSession = state
        .store
        .load(&session_id)
        .map_err(|e| e.to_string())?;
    let question = state
        .bank
        .get(&question_id)
        .ok_or_else(|| format!("unknown question: {question_id}"))?
        .clone();

    session.record_answer(&question.id, &content);

    let llm = OllamaClient::new(&state.llm_endpoint, &state.llm_model);
    let interviewer = Interviewer::new(&llm);
    let new_nodes = interviewer
        .extract(&question, &content)
        .await
        .unwrap_or_else(|err| {
            tracing::warn!(?err, "LLM extraction failed; recording answer without nodes");
            Vec::new()
        });
    let extracted = new_nodes.len();
    for node in new_nodes {
        session.memory.add(node);
    }
    session.candidates = candidate::extract(&session.memory);

    state
        .store
        .save(&session)
        .map_err(|e| e.to_string())?;
    let total_candidates = session.candidates.len();
    *state.current.lock().unwrap() = Some(session);

    Ok(AnswerResult {
        extracted_nodes: extracted,
        total_candidates,
    })
}

#[tauri::command]
pub fn get_candidates(
    state: tauri::State<'_, InterviewState>,
    session_id: String,
) -> Result<Vec<Candidate>, String> {
    let session = state
        .store
        .load(&session_id)
        .map_err(|e| e.to_string())?;
    Ok(session.candidates)
}

#[tauri::command]
pub fn get_memory(
    state: tauri::State<'_, InterviewState>,
    session_id: String,
) -> Result<MemoryState, String> {
    let session = state
        .store
        .load(&session_id)
        .map_err(|e| e.to_string())?;
    Ok(session.memory)
}

#[tauri::command]
pub fn complete_interview(
    state: tauri::State<'_, InterviewState>,
    session_id: String,
) -> Result<(), String> {
    let mut session = state
        .store
        .load(&session_id)
        .map_err(|e| e.to_string())?;
    session.complete();
    state
        .store
        .save(&session)
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImportResult {
    pub nodes_added: usize,
    pub total_candidates: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReconstructRequest {
    pub template: String,
    pub target: String,
    pub kind: String,
    pub passphrase: String,
    pub index_range: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReconstructResponse {
    pub recovered_word: String,
    pub recovered_mnemonic: String,
    pub address_index: u32,
    pub elapsed_ms: u128,
}

#[tauri::command]
pub fn reconstruct(req: ReconstructRequest) -> Result<ReconstructResponse, String> {
    let kind = parse_kind(&req.kind)?;
    let started = std::time::Instant::now();
    let result = reconstruct_missing_word(
        &req.template,
        &req.target,
        kind,
        &req.passphrase,
        req.index_range,
    )
    .map_err(|e| e.to_string())?;
    Ok(ReconstructResponse {
        recovered_word: result.recovered_word,
        recovered_mnemonic: result.recovered_mnemonic,
        address_index: result.address_index,
        elapsed_ms: started.elapsed().as_millis(),
    })
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MultiReconstructResponse {
    pub recovered_words: Vec<String>,
    pub recovered_mnemonic: String,
    pub address_index: u32,
    pub elapsed_ms: u128,
}

#[tauri::command]
pub fn reconstruct_multi_words(req: ReconstructRequest) -> Result<MultiReconstructResponse, String> {
    let kind = parse_kind(&req.kind)?;
    let started = std::time::Instant::now();
    let r = reconstruct_multi(&req.template, &req.target, kind, &req.passphrase, req.index_range)
        .map_err(|e| e.to_string())?;
    Ok(MultiReconstructResponse {
        recovered_words: r.recovered_words,
        recovered_mnemonic: r.recovered_mnemonic,
        address_index: r.address_index,
        elapsed_ms: started.elapsed().as_millis(),
    })
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PassphraseBruteRequest {
    pub mnemonic: String,
    pub target: String,
    pub kind: String,
    pub candidates: Vec<String>,
    pub index_range: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PassphraseBruteResponse {
    pub passphrase: String,
    pub address_index: u32,
    pub elapsed_ms: u128,
}

#[tauri::command]
pub fn brute_passphrase(req: PassphraseBruteRequest) -> Result<PassphraseBruteResponse, String> {
    let kind = parse_kind(&req.kind)?;
    let started = std::time::Instant::now();
    let r = brute_force_passphrase(&req.mnemonic, &req.target, kind, &req.candidates, req.index_range)
        .map_err(|e| e.to_string())?;
    Ok(PassphraseBruteResponse {
        passphrase: r.passphrase,
        address_index: r.address_index,
        elapsed_ms: started.elapsed().as_millis(),
    })
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HashcatBuildRequest {
    pub hash_file: String,
    pub mode: u32,
    pub attack: u32,
    pub wordlist_or_mask: String,
}

#[tauri::command]
pub fn build_hashcat(req: HashcatBuildRequest) -> Result<String, String> {
    let mode = match req.mode {
        11300 => HashcatMode::BitcoinWalletDat,
        12700 => HashcatMode::BlockchainV1,
        22500 => HashcatMode::MultiBitKey,
        21700 => HashcatMode::Electrum1,
        21800 => HashcatMode::Electrum2,
        15700 => HashcatMode::MewV2,
        27800 => HashcatMode::Bip38,
        other => return Err(format!("unsupported hashcat mode: {other}")),
    };
    let attack = match req.attack {
        0 => AttackMode::Straight,
        1 => AttackMode::Combination,
        3 => AttackMode::Mask,
        other => return Err(format!("unsupported attack mode: {other}")),
    };
    let path = std::path::Path::new(&req.hash_file);
    Ok(build_command(path, mode, attack, &req.wordlist_or_mask))
}

fn parse_kind(s: &str) -> Result<AddressKind, String> {
    match s {
        "eth" => Ok(AddressKind::Eth),
        "btc" => Ok(AddressKind::BtcSegwit),
        other => Err(format!("unknown kind: {other}")),
    }
}

#[tauri::command]
pub fn import_file(
    state: tauri::State<'_, InterviewState>,
    session_id: String,
    file_path: String,
) -> Result<ImportResult, String> {
    let mut session: InterviewSession = state
        .store
        .load(&session_id)
        .map_err(|e| e.to_string())?;

    let path = PathBuf::from(&file_path);
    if !path.exists() {
        return Err(format!("file not found: {file_path}"));
    }

    let new_nodes = state
        .registry
        .dispatch(&path)
        .map_err(|e| e.to_string())?;
    let added = new_nodes.len();
    for node in new_nodes {
        session.memory.add(node);
    }
    session.candidates = candidate::extract(&session.memory);

    state
        .store
        .save(&session)
        .map_err(|e| e.to_string())?;
    let total_candidates = session.candidates.len();

    Ok(ImportResult {
        nodes_added: added,
        total_candidates,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn app_info_returns_name_and_version() {
        let info = app_info();
        assert_eq!(info.name, "Phoenix");
        assert_eq!(info.version, "0.1.0");
    }

    #[test]
    fn interview_state_loads_embedded_bank() {
        let state = InterviewState::new();
        assert!(state.bank.len() >= 30);
    }
}
