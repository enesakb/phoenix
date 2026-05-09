import { useState } from "react";
import { useParams } from "react-router-dom";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";

interface ImportResult {
  nodes_added: number;
  total_candidates: number;
}

export function Import() {
  const { sessionId } = useParams<{ sessionId: string }>();
  const [busy, setBusy] = useState(false);
  const [history, setHistory] = useState<{ file: string; result: ImportResult }[]>([]);
  const [error, setError] = useState<string | null>(null);

  const pickAndImport = async () => {
    if (!sessionId) return;
    setError(null);
    try {
      const selected = await open({
        multiple: false,
        title: "Pick a file to import (CSV export, Chrome History copy, or text file)",
      });
      if (!selected) return;
      const filePath = selected as string;
      setBusy(true);
      const result = await invoke<ImportResult>("import_file", {
        sessionId,
        filePath,
      });
      setHistory((h) => [{ file: filePath, result }, ...h]);
    } catch (e: unknown) {
      setError(e instanceof Error ? e.message : String(e));
    } finally {
      setBusy(false);
    }
  };

  if (!sessionId) {
    return (
      <main style={{ maxWidth: 720, margin: "0 auto", padding: "2rem" }}>
        <p>No session selected. Start an interview first.</p>
      </main>
    );
  }

  return (
    <main style={{ maxWidth: 720, margin: "0 auto", padding: "2rem" }}>
      <h1>Import forensic files</h1>
      <p>
        Add password-manager exports, Chrome <code>History</code> copies, or any
        text file. Phoenix scans for BIP-39 word sequences, password patterns,
        and crypto-related browser visits.
      </p>
      <p style={{ opacity: 0.7 }}>
        Session <code>{sessionId}</code>
      </p>

      <button
        onClick={pickAndImport}
        disabled={busy}
        style={{
          padding: "0.75rem 1.5rem",
          background: "#ff6b35",
          color: "white",
          border: "none",
          borderRadius: 6,
          cursor: busy ? "wait" : "pointer",
          marginTop: "1rem",
        }}
      >
        {busy ? "Scanning…" : "Pick a file to import"}
      </button>

      {error && (
        <p style={{ color: "red", marginTop: "1rem" }}>Error: {error}</p>
      )}

      {history.length > 0 && (
        <section style={{ marginTop: "2rem" }}>
          <h2>Import history</h2>
          <ul>
            {history.map((h, i) => (
              <li key={i}>
                <code>{h.file}</code>:{" "}
                <strong>{h.result.nodes_added}</strong> new memory nodes ·{" "}
                {h.result.total_candidates} total candidates
              </li>
            ))}
          </ul>
        </section>
      )}
    </main>
  );
}
