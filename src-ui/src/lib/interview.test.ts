import { describe, it, expect, vi, beforeEach } from "vitest";
import {
  answerQuestion,
  completeInterview,
  getCandidates,
  getMemory,
  listQuestions,
  startInterview,
} from "./interview";

const invokeMock = vi.fn();

vi.mock("@tauri-apps/api/core", () => ({
  invoke: (cmd: string, args?: Record<string, unknown>) => invokeMock(cmd, args),
}));

beforeEach(() => {
  invokeMock.mockReset();
});

describe("interview IPC", () => {
  it("listQuestions calls list_questions", async () => {
    invokeMock.mockResolvedValueOnce([
      { id: "fr-01", category: "free_recall", text: "Tell me", follow_up_hints: [] },
    ]);
    const result = await listQuestions();
    expect(invokeMock).toHaveBeenCalledWith("list_questions", undefined);
    expect(result).toHaveLength(1);
  });

  it("startInterview returns session id", async () => {
    invokeMock.mockResolvedValueOnce("abc-123");
    const id = await startInterview();
    expect(id).toBe("abc-123");
  });

  it("answerQuestion forwards camelCase keys to invoke", async () => {
    invokeMock.mockResolvedValueOnce({ extracted_nodes: 2, total_candidates: 5 });
    const result = await answerQuestion("sess", "fr-01", "I created in 2021");
    expect(invokeMock).toHaveBeenCalledWith("answer_question", {
      sessionId: "sess",
      questionId: "fr-01",
      content: "I created in 2021",
    });
    expect(result.extracted_nodes).toBe(2);
  });

  it("getCandidates calls get_candidates with session id", async () => {
    invokeMock.mockResolvedValueOnce([]);
    await getCandidates("sess");
    expect(invokeMock).toHaveBeenCalledWith("get_candidates", { sessionId: "sess" });
  });

  it("getMemory calls get_memory with session id", async () => {
    invokeMock.mockResolvedValueOnce({ nodes: [] });
    await getMemory("sess");
    expect(invokeMock).toHaveBeenCalledWith("get_memory", { sessionId: "sess" });
  });

  it("completeInterview calls complete_interview", async () => {
    invokeMock.mockResolvedValueOnce(undefined);
    await completeInterview("sess");
    expect(invokeMock).toHaveBeenCalledWith("complete_interview", { sessionId: "sess" });
  });
});
