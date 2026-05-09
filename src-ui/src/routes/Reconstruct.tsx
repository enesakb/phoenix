import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";

interface ReconstructResponse {
  recovered_word: string;
  recovered_mnemonic: string;
  address_index: number;
  elapsed_ms: number;
}

export function Reconstruct() {
  const [template, setTemplate] = useState("");
  const [target, setTarget] = useState("");
  const [kind, setKind] = useState<"eth" | "btc">("eth");
  const [passphrase, setPassphrase] = useState("");
  const [indexRange, setIndexRange] = useState(5);
  const [busy, setBusy] = useState(false);
  const [result, setResult] = useState<ReconstructResponse | null>(null);
  const [error, setError] = useState<string | null>(null);

  const submit = async () => {
    setBusy(true);
    setResult(null);
    setError(null);
    try {
      const res = await invoke<ReconstructResponse>("reconstruct", {
        req: {
          template,
          target,
          kind,
          passphrase,
          index_range: indexRange,
        },
      });
      setResult(res);
    } catch (e: unknown) {
      setError(e instanceof Error ? e.message : String(e));
    } finally {
      setBusy(false);
    }
  };

  return (
    <main style={{ maxWidth: 720, margin: "0 auto", padding: "2rem" }}>
      <h1>Reconstruct missing word</h1>
      <p>
        If you remember 11 of 12 BIP-39 words and have a target address,
        Phoenix brute-forces the missing word in seconds. Use <code>?</code> for
        the unknown position.
      </p>

      <label style={{ display: "block", marginTop: "1rem" }}>
        12-word template
        <textarea
          value={template}
          onChange={(e) => setTemplate(e.target.value)}
          rows={3}
          style={{ width: "100%", padding: "0.5rem", marginTop: "0.25rem" }}
          placeholder="abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon ?"
        />
      </label>

      <label style={{ display: "block", marginTop: "1rem" }}>
        Target address
        <input
          value={target}
          onChange={(e) => setTarget(e.target.value)}
          style={{ width: "100%", padding: "0.5rem", marginTop: "0.25rem" }}
          placeholder="0x9858effd232b4033e47d90003d41ec34ecaeda94"
        />
      </label>

      <div style={{ display: "flex", gap: "1rem", marginTop: "1rem" }}>
        <label>
          Kind
          <select
            value={kind}
            onChange={(e) => setKind(e.target.value as "eth" | "btc")}
            style={{ marginLeft: "0.5rem" }}
          >
            <option value="eth">Ethereum (m/44&apos;/60&apos;/0&apos;/0/i)</option>
            <option value="btc">BTC native segwit (m/84&apos;/0&apos;/0&apos;/0/i)</option>
          </select>
        </label>

        <label>
          Index range
          <input
            type="number"
            value={indexRange}
            onChange={(e) => setIndexRange(parseInt(e.target.value, 10) || 1)}
            min={1}
            max={50}
            style={{ width: "5rem", marginLeft: "0.5rem", padding: "0.25rem" }}
          />
        </label>
      </div>

      <label style={{ display: "block", marginTop: "1rem" }}>
        BIP-39 passphrase (optional)
        <input
          value={passphrase}
          onChange={(e) => setPassphrase(e.target.value)}
          style={{ width: "100%", padding: "0.5rem", marginTop: "0.25rem" }}
          placeholder="(empty if you did not use one)"
        />
      </label>

      <button
        onClick={submit}
        disabled={busy || !template.includes("?") || !target}
        style={{
          padding: "0.75rem 1.5rem",
          background: "#ff6b35",
          color: "white",
          border: "none",
          borderRadius: 6,
          cursor: busy ? "wait" : "pointer",
          marginTop: "1.5rem",
        }}
      >
        {busy ? "Searching…" : "Reconstruct"}
      </button>

      {error && (
        <div
          style={{
            marginTop: "1.5rem",
            padding: "1rem",
            background: "#ffe5e5",
            borderRadius: 6,
          }}
        >
          <strong>Not found.</strong> {error}
        </div>
      )}

      {result && (
        <div
          style={{
            marginTop: "1.5rem",
            padding: "1rem",
            background: "#e0ffe5",
            borderRadius: 6,
          }}
        >
          <h3>✓ Recovered</h3>
          <p>
            <strong>Word:</strong> <code>{result.recovered_word}</code>
          </p>
          <p>
            <strong>Mnemonic:</strong> <code>{result.recovered_mnemonic}</code>
          </p>
          <p>
            <strong>Address index:</strong> {result.address_index}
          </p>
          <p>
            <strong>Elapsed:</strong> {result.elapsed_ms} ms
          </p>
        </div>
      )}
    </main>
  );
}
