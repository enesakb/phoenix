import { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";
import {
  AnswerResult,
  Question,
  answerQuestion,
  completeInterview,
  listQuestions,
  startInterview,
} from "../lib/interview";

export function Interview() {
  const navigate = useNavigate();
  const [questions, setQuestions] = useState<Question[]>([]);
  const [sessionId, setSessionId] = useState<string | null>(null);
  const [index, setIndex] = useState(0);
  const [answer, setAnswer] = useState("");
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [lastResult, setLastResult] = useState<AnswerResult | null>(null);

  useEffect(() => {
    (async () => {
      try {
        const [qs, sid] = await Promise.all([listQuestions(), startInterview()]);
        setQuestions(qs);
        setSessionId(sid);
      } catch (e: unknown) {
        setError(e instanceof Error ? e.message : String(e));
      }
    })();
  }, []);

  const submit = async () => {
    if (!sessionId || !questions[index]) return;
    setBusy(true);
    setError(null);
    try {
      const res = await answerQuestion(sessionId, questions[index].id, answer);
      setLastResult(res);
      setAnswer("");
      const next = index + 1;
      if (next >= questions.length) {
        await completeInterview(sessionId);
        navigate(`/candidates/${sessionId}`);
      } else {
        setIndex(next);
      }
    } catch (e: unknown) {
      setError(e instanceof Error ? e.message : String(e));
    } finally {
      setBusy(false);
    }
  };

  const skip = async () => {
    if (!sessionId) return;
    if (index + 1 >= questions.length) {
      await completeInterview(sessionId);
      navigate(`/candidates/${sessionId}`);
    } else {
      setIndex(index + 1);
      setAnswer("");
    }
  };

  if (error) return <p style={{ color: "red" }}>Error: {error}</p>;
  if (!sessionId || questions.length === 0) return <p>Loading…</p>;

  const q = questions[index];
  const progress = `${index + 1} / ${questions.length}`;

  return (
    <main style={{ maxWidth: 720, margin: "0 auto", padding: "2rem" }}>
      <header style={{ marginBottom: "1.5rem" }}>
        <p style={{ opacity: 0.7, fontSize: "0.9rem" }}>
          {q.category.replace(/_/g, " ")} · {progress}
        </p>
        <h2 style={{ marginTop: "0.5rem" }}>{q.text}</h2>
      </header>

      <textarea
        value={answer}
        onChange={(e) => setAnswer(e.target.value)}
        rows={10}
        style={{ width: "100%", padding: "0.75rem", fontSize: "1rem" }}
        placeholder="Type freely. Even fragments help."
        disabled={busy}
      />

      <div style={{ marginTop: "1rem", display: "flex", gap: "0.75rem" }}>
        <button
          onClick={submit}
          disabled={busy || answer.trim().length === 0}
          style={{ padding: "0.6rem 1.2rem" }}
        >
          {busy ? "Extracting…" : "Submit & next"}
        </button>
        <button
          onClick={skip}
          disabled={busy}
          style={{ padding: "0.6rem 1.2rem" }}
        >
          Skip
        </button>
      </div>

      {lastResult && (
        <p style={{ marginTop: "1.5rem", opacity: 0.8 }}>
          Last answer: <strong>{lastResult.extracted_nodes}</strong> new memory nodes ·{" "}
          <strong>{lastResult.total_candidates}</strong> candidates total.
        </p>
      )}
    </main>
  );
}
