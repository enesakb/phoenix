import { useEffect, useState } from "react";
import { Link } from "react-router-dom";
import { fetchAppInfo, AppInfo } from "../lib/ipc";

export function Welcome() {
  const [info, setInfo] = useState<AppInfo | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    fetchAppInfo()
      .then(setInfo)
      .catch((e: Error) => setError(e.message));
  }, []);

  return (
    <main style={{ maxWidth: 760, margin: "0 auto", padding: "2.5rem 2rem" }}>
      <h1 style={{ marginBottom: "0.4rem" }}>Phoenix</h1>
      <p style={{ color: "#666", margin: 0 }}>
        Open-source forensic recovery for crypto wallets.
      </p>
      {error && (
        <p style={{ color: "red", marginTop: "0.5rem" }}>IPC error: {error}</p>
      )}
      {info && (
        <p style={{ opacity: 0.5, fontSize: "0.85rem", marginTop: "0.4rem" }}>
          {info.name} v{info.version}
        </p>
      )}

      <hr style={{ margin: "2rem 0", border: 0, borderTop: "1px solid #eee" }} />

      <h2 style={{ marginBottom: "0.5rem" }}>What do you remember?</h2>
      <p style={{ color: "#666", marginTop: 0 }}>
        Pick the path that matches your situation. Both run locally on this
        machine — your seed never leaves.
      </p>

      <div style={{ display: "grid", gap: "1rem", marginTop: "1.5rem" }}>
        <Link
          to="/reconstruct"
          style={{
            display: "block",
            padding: "1.4rem 1.5rem",
            background: "#fff7f3",
            border: "2px solid #ff6b35",
            borderRadius: "10px",
            textDecoration: "none",
            color: "inherit",
          }}
        >
          <div
            style={{
              fontSize: "0.7rem",
              color: "#ff6b35",
              textTransform: "uppercase",
              letterSpacing: "0.08em",
              fontWeight: 700,
            }}
          >
            Recommended · sub-second
          </div>
          <div
            style={{
              fontWeight: 700,
              fontSize: "1.1rem",
              marginTop: "0.3rem",
            }}
          >
            I know most of my seed →
          </div>
          <div style={{ color: "#666", fontSize: "0.9rem", marginTop: "0.4rem" }}>
            You remember 10 or 11 of your 12 BIP-39 words plus your wallet
            address. Phoenix brute-forces the missing position(s) in
            milliseconds. <strong>Skip the interview.</strong>
          </div>
        </Link>

        <Link
          to="/interview"
          style={{
            display: "block",
            padding: "1.4rem 1.5rem",
            background: "#fafafa",
            border: "1px solid #ddd",
            borderRadius: "10px",
            textDecoration: "none",
            color: "inherit",
          }}
        >
          <div
            style={{
              fontSize: "0.7rem",
              color: "#666",
              textTransform: "uppercase",
              letterSpacing: "0.08em",
              fontWeight: 700,
            }}
          >
            For partial-info cases · 30–90 minutes
          </div>
          <div
            style={{
              fontWeight: 700,
              fontSize: "1.1rem",
              marginTop: "0.3rem",
            }}
          >
            I remember almost nothing — guide me →
          </div>
          <div style={{ color: "#666", fontSize: "0.9rem", marginTop: "0.4rem" }}>
            Cognitive interview with a local LLM (Ollama). 50 Fisher–Geiselman
            questions help you surface fragments — patterns, devices, dates —
            that lead back to the seed.
          </div>
        </Link>

        <Link
          to="/import/new"
          style={{
            display: "block",
            padding: "1.4rem 1.5rem",
            background: "#fafafa",
            border: "1px solid #ddd",
            borderRadius: "10px",
            textDecoration: "none",
            color: "inherit",
          }}
        >
          <div
            style={{
              fontSize: "0.7rem",
              color: "#666",
              textTransform: "uppercase",
              letterSpacing: "0.08em",
              fontWeight: 700,
            }}
          >
            Forensic scan · seconds–minutes
          </div>
          <div
            style={{
              fontWeight: 700,
              fontSize: "1.1rem",
              marginTop: "0.3rem",
            }}
          >
            I have an old backup or password manager export →
          </div>
          <div style={{ color: "#666", fontSize: "0.9rem", marginTop: "0.4rem" }}>
            Drop in a Bitwarden CSV, Chrome History copy, mbox archive, or any
            text file. Phoenix scans for BIP-39 sequences, password patterns,
            and crypto-related artifacts.
          </div>
        </Link>
      </div>

      <p
        style={{
          marginTop: "2rem",
          color: "#888",
          fontSize: "0.85rem",
          lineHeight: 1.55,
        }}
      >
        ⚠️ Phoenix never asks for your seed phrase to be sent anywhere. The
        recovery happens on this machine, in this process. You can verify by
        running <code>tcpdump</code> while Phoenix is active — no outbound
        traffic.
      </p>
    </main>
  );
}
