use std::path::PathBuf;
use std::sync::Mutex;

use phoenix_core::forensic::{
    Bip39TextExtractor, BitwardenCsvExtractor, ChromeHistoryExtractor, ExtractorRegistry,
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
