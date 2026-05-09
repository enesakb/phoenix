import { useEffect, useState } from "react";
import { useParams } from "react-router-dom";
import { Candidate, MemoryState, getCandidates, getMemory } from "../lib/interview";

export function Candidates() {
  const { sessionId } = useParams<{ sessionId: string }>();
  const [candidates, setCandidates] = useState<Candidate[]>([]);
  const [memory, setMemory] = useState<MemoryState | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (!sessionId) return;
    (async () => {
      try {
        const [c, m] = await Promise.all([
          getCandidates(sessionId),
          getMemory(sessionId),
        ]);
        setCandidates(c);
        setMemory(m);
      } catch (e: unknown) {
        setError(e instanceof Error ? e.message : String(e));
      }
    })();
  }, [sessionId]);

  if (error) return <p style={{ color: "red" }}>Error: {error}</p>;
  if (!memory) return <p>Loading…</p>;

  return (
    <main style={{ maxWidth: 900, margin: "0 auto", padding: "2rem" }}>
      <h1>Recovery candidates</h1>
      <p style={{ opacity: 0.7 }}>
        Session <code>{sessionId}</code> — {memory.nodes.length} memory nodes,{" "}
        {candidates.length} candidates.
      </p>

      <h2 style={{ marginTop: "2rem" }}>Ranked candidates</h2>
      {candidates.length === 0 ? (
        <p>No candidates extracted yet.</p>
      ) : (
        <table style={{ width: "100%", borderCollapse: "collapse" }}>
          <thead>
            <tr>
              <th align="left">Score</th>
              <th align="left">Kind</th>
              <th align="left">Value</th>
            </tr>
          </thead>
          <tbody>
            {candidates.map((c, i) => (
              <tr key={i} style={{ borderTop: "1px solid #eee" }}>
                <td>{c.score.toFixed(2)}</td>
                <td>{c.kind.replace(/_/g, " ")}</td>
                <td>
                  <code>{c.value}</code>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      )}

      <h2 style={{ marginTop: "2rem" }}>All memory nodes</h2>
      <ul>
        {memory.nodes.map((n) => (
          <li key={n.id}>
            <strong>{n.kind.replace(/_/g, " ")}</strong> ({n.confidence.toFixed(2)}):{" "}
            <code>{n.content}</code>
          </li>
        ))}
      </ul>
    </main>
  );
}
