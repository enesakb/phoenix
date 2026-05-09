//! Layer 1 — Cognitive Excavation Engine.
//!
//! Adaptive interview that extracts memory hints from a user about a lost
//! wallet. Combines a Fisher–Geiselman-style structured question bank with an
//! LLM interviewer that adapts follow-ups based on prior answers, then
//! consolidates the answers into a ranked candidate list for Layer 3 (inference)
//! to crack against.

pub mod candidate;
pub mod debate;
pub mod interviewer;
pub mod questions;
pub mod session;
pub mod state;

pub use candidate::{Candidate, CandidateKind};
pub use questions::{Question, QuestionBank, QuestionCategory};
pub use session::{InterviewSession, SessionStore};
pub use state::{MemoryNode, MemoryNodeKind, MemoryState};
