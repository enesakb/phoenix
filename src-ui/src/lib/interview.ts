import { invoke } from "@tauri-apps/api/core";

export type QuestionCategory =
  | "free_recall"
  | "context_reinstatement"
  | "reverse_order"
  | "change_perspective"
  | "wallet_specific"
  | "pattern_identification"
  | "physical_artifacts"
  | "digital_artifacts";

export interface Question {
  id: string;
  category: QuestionCategory;
  text: string;
  follow_up_hints: string[];
}

export type CandidateKind =
  | "seed_word"
  | "seed_phrase_fragment"
  | "passphrase"
  | "password_candidate"
  | "derivation_path"
  | "wallet_kind_hint";

export interface Candidate {
  kind: CandidateKind;
  value: string;
  score: number;
  supporting_node_ids: string[];
}

export type MemoryNodeKind =
  | "fact"
  | "password_pattern"
  | "passphrase_fragment"
  | "seed_fragment"
  | "contextual_lead"
  | "artifact_pointer";

export interface MemoryNode {
  id: string;
  kind: MemoryNodeKind;
  content: string;
  confidence: number;
  source_question_ids: string[];
  created_at: string;
}

export interface MemoryState {
  nodes: MemoryNode[];
}

export interface AnswerResult {
  extracted_nodes: number;
  total_candidates: number;
}

export const listQuestions = (): Promise<Question[]> => invoke("list_questions");

export const startInterview = (): Promise<string> => invoke("start_interview");

export const answerQuestion = (
  sessionId: string,
  questionId: string,
  content: string
): Promise<AnswerResult> =>
  invoke("answer_question", { sessionId, questionId, content });

export const getCandidates = (sessionId: string): Promise<Candidate[]> =>
  invoke("get_candidates", { sessionId });

export const getMemory = (sessionId: string): Promise<MemoryState> =>
  invoke("get_memory", { sessionId });

export const completeInterview = (sessionId: string): Promise<void> =>
  invoke("complete_interview", { sessionId });
