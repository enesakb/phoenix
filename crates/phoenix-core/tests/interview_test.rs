use phoenix_core::interview::{
    candidate::{self, CandidateKind},
    interviewer::{build_extraction_prompt, parse_extraction},
    questions::{QuestionBank, QuestionCategory},
    session::{InterviewSession, SessionStore},
    state::{MemoryNode, MemoryNodeKind, MemoryState},
};

#[test]
fn embedded_question_bank_loads_with_50_plus() {
    let bank = QuestionBank::from_embedded().unwrap();
    assert!(
        bank.len() >= 30,
        "expected ≥30 questions, got {}",
        bank.len()
    );
    let ordered = bank.ordered();
    assert!(matches!(ordered[0].category, QuestionCategory::FreeRecall));
}

#[test]
fn question_bank_grouped_by_category() {
    let bank = QuestionBank::from_embedded().unwrap();
    let free_recall = bank.by_category(QuestionCategory::FreeRecall);
    assert!(!free_recall.is_empty());
    let wallet = bank.by_category(QuestionCategory::WalletSpecific);
    assert!(wallet.len() >= 5);
}

#[test]
fn memory_state_filters_by_kind_and_confidence() {
    let mut state = MemoryState::default();
    state.add(MemoryNode::new(
        MemoryNodeKind::SeedFragment,
        "abandon",
        0.9,
        vec!["q1".into()],
    ));
    state.add(MemoryNode::new(
        MemoryNodeKind::Fact,
        "wallet created in 2021",
        0.8,
        vec!["q2".into()],
    ));
    state.add(MemoryNode::new(
        MemoryNodeKind::PassphraseFragment,
        "skywalker",
        0.5,
        vec!["q3".into()],
    ));

    assert_eq!(state.nodes_of(MemoryNodeKind::SeedFragment).len(), 1);
    assert_eq!(state.nodes_of(MemoryNodeKind::Fact).len(), 1);
    assert_eq!(state.high_confidence(0.7).len(), 2);
}

#[test]
fn candidate_extraction_orders_by_score() {
    let mut state = MemoryState::default();
    state.add(MemoryNode::new(
        MemoryNodeKind::PasswordPattern,
        "BasePass99!",
        0.6,
        vec!["q1".into()],
    ));
    state.add(MemoryNode::new(
        MemoryNodeKind::SeedFragment,
        "abandon",
        0.9,
        vec!["q2".into()],
    ));
    state.add(MemoryNode::new(
        MemoryNodeKind::Fact,
        "created 2021",
        1.0,
        vec!["q3".into()],
    ));

    let candidates = candidate::extract(&state);
    assert_eq!(candidates.len(), 2, "facts are not candidates");
    assert!(candidates[0].score >= candidates[1].score);
    assert_eq!(candidates[0].kind, CandidateKind::SeedPhraseFragment);
}

#[test]
fn extraction_prompt_contains_question_and_answer() {
    let bank = QuestionBank::from_embedded().unwrap();
    let q = bank.ordered()[0];
    let prompt = build_extraction_prompt(q, "I created the wallet in 2021");
    assert!(prompt.contains(&q.text));
    assert!(prompt.contains("2021"));
}

#[test]
fn parse_extraction_handles_clean_json() {
    let raw = r#"{"nodes":[{"kind":"fact","content":"created 2021","confidence":0.9}]}"#;
    let parsed = parse_extraction(raw).unwrap();
    assert_eq!(parsed.nodes.len(), 1);
    assert_eq!(parsed.nodes[0].content, "created 2021");
}

#[test]
fn parse_extraction_strips_code_fence() {
    let raw =
        "```json\n{\"nodes\":[{\"kind\":\"fact\",\"content\":\"x\",\"confidence\":0.5}]}\n```";
    let parsed = parse_extraction(raw).unwrap();
    assert_eq!(parsed.nodes.len(), 1);
}

#[test]
fn parse_extraction_rejects_garbage() {
    assert!(parse_extraction("hello world").is_err());
}

#[test]
fn session_round_trip_via_disk() {
    let dir = tempfile::tempdir().unwrap();
    let store = SessionStore::new(dir.path()).unwrap();

    let mut session = InterviewSession::new();
    session.record_answer("q1", "I created in 2021");
    session.memory.add(MemoryNode::new(
        MemoryNodeKind::Fact,
        "year:2021",
        0.9,
        vec!["q1".into()],
    ));
    session.complete();

    store.save(&session).unwrap();
    let loaded = store.load(&session.id).unwrap();

    assert_eq!(loaded.id, session.id);
    assert_eq!(loaded.answers.len(), 1);
    assert_eq!(loaded.answers[0].content, "I created in 2021");
    assert_eq!(loaded.memory.nodes.len(), 1);
    assert!(loaded.completed_at.is_some());
}

#[test]
fn session_store_lists_sessions() {
    let dir = tempfile::tempdir().unwrap();
    let store = SessionStore::new(dir.path()).unwrap();

    let s1 = InterviewSession::new();
    let s2 = InterviewSession::new();
    store.save(&s1).unwrap();
    store.save(&s2).unwrap();

    let mut ids = store.list().unwrap();
    ids.sort();
    let mut expected = vec![s1.id, s2.id];
    expected.sort();
    assert_eq!(ids, expected);
}
