import { useMemo, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

interface ReconstructResponse {
  recovered_word: string;
  recovered_mnemonic: string;
  address_index: number;
  elapsed_ms: number;
}

type Kind = "eth" | "btc" | "sol";

const TEST_VECTORS: Record<
  Kind,
  { template: string; target: string; label: string }
> = {
  eth: {
    label: "Ethereum",
    template:
      "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon ?",
    target: "0x9858effd232b4033e47d90003d41ec34ecaeda94",
  },
  btc: {
    label: "Bitcoin",
    template:
      "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon ?",
    target: "bc1qcr8te4kr609gcawutmrza0j4xv80jy8z306fyu",
  },
  sol: {
    label: "Solana",
    template:
      "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon ?",
    target: "HAgk14JpMQLgt6rVgv7cBQFJWFto5Dqxi472uT3DKpqk",
  },
};

interface Validation {
  ok: boolean;
  reasons: string[];
  hints: string[];
  tokenCount: number;
  wildcardCount: number;
}

function validateTemplate(t: string): Validation {
  const tokens = t.trim().split(/\s+/).filter(Boolean);
  const wildcardCount = tokens.filter((tk) => tk === "?").length;
  const reasons: string[] = [];
  const hints: string[] = [];

  if (tokens.length === 0) {
    reasons.push("Template is empty.");
    hints.push(
      "Type your 12 BIP-39 words separated by spaces, with `?` for the missing one."
    );
  } else if (tokens.length !== 12) {
    reasons.push(
      `You typed ${tokens.length} words; BIP-39 mnemonics are exactly 12.`
    );
  }

  if (tokens.length > 0 && wildcardCount === 0) {
    reasons.push("No `?` wildcard in your template.");
    hints.push("Replace the missing word with `?`. Example: ...abandon ?");
  }

  if (wildcardCount > 2) {
    reasons.push(
      `${wildcardCount} ? wildcards is too many — Phoenix supports 1 or 2 missing words.`
    );
  }

  return {
    ok: reasons.length === 0,
    reasons,
    hints,
    tokenCount: tokens.length,
    wildcardCount,
  };
}

function validateTarget(t: string, k: Kind): { ok: boolean; reason?: string } {
  const v = t.trim();
  if (!v) return { ok: false, reason: "Target address is empty." };
  if (k === "eth") {
    if (!v.toLowerCase().startsWith("0x") || v.length !== 42) {
      return {
        ok: false,
        reason: "Ethereum addresses are 0x… followed by 40 hex characters (42 total).",
      };
    }
  } else if (k === "btc") {
    if (!(v.startsWith("bc1") || v.startsWith("1") || v.startsWith("3"))) {
      return {
        ok: false,
        reason:
          "Bitcoin addresses start with bc1 (segwit), 1 (legacy), or 3 (script). Phoenix v0.8 supports bc1q (native segwit).",
      };
    }
  } else if (k === "sol") {
    if (!/^[1-9A-HJ-NP-Za-km-z]{32,44}$/.test(v)) {
      return {
        ok: false,
        reason: "Solana addresses are 32–44 base58 characters (no 0, O, I, l).",
      };
    }
  }
  return { ok: true };
}

export function Reconstruct() {
  const [template, setTemplate] = useState("");
  const [target, setTarget] = useState("");
  const [kind, setKind] = useState<Kind>("eth");
  const [passphrase, setPassphrase] = useState("");
  const [indexRange, setIndexRange] = useState(5);
  const [busy, setBusy] = useState(false);
  const [result, setResult] = useState<ReconstructResponse | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [showAdvanced, setShowAdvanced] = useState(false);

  const v = useMemo(() => validateTemplate(template), [template]);
  const tv = useMemo(() => validateTarget(target, kind), [target, kind]);
  const canSubmit = !busy && v.ok && tv.ok;

  const fillTestVector = () => {
    const vec = TEST_VECTORS[kind];
    setTemplate(vec.template);
    setTarget(vec.target);
    setPassphrase("");
    setError(null);
    setResult(null);
  };

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
    <main style={{ maxWidth: 760, margin: "0 auto", padding: "2.5rem 2rem" }}>
      <h1 style={{ marginBottom: "0.5rem" }}>Reconstruct missing word</h1>
      <p style={{ color: "#666", marginTop: 0 }}>
        Remember 10 or 11 of your 12 BIP-39 words plus your wallet address?
        Phoenix brute-forces the missing word in milliseconds. Use{" "}
        <code>?</code> for each unknown position.
      </p>

      <div
        style={{
          marginTop: "1rem",
          padding: "0.8rem 1rem",
          background: "#fff7f3",
          border: "1px solid #ffd6c2",
          borderRadius: 8,
          fontSize: "0.9rem",
        }}
      >
        <strong>Try a test vector:</strong> the public BIP-39 zero-vector for{" "}
        <strong>{TEST_VECTORS[kind].label}</strong>. No real funds — just to
        see Phoenix work end-to-end.{" "}
        <button
          type="button"
          onClick={fillTestVector}
          style={{
            marginLeft: "0.4rem",
            background: "transparent",
            border: "1px solid #ff6b35",
            color: "#ff6b35",
            borderRadius: 4,
            padding: "0.2rem 0.6rem",
            cursor: "pointer",
          }}
        >
          Fill the form
        </button>
      </div>

      <label style={{ display: "block", marginTop: "1.5rem", fontWeight: 600 }}>
        12-word template{" "}
        <span
          style={{
            fontWeight: 400,
            fontSize: "0.85rem",
            color: v.tokenCount === 12 && v.wildcardCount > 0 ? "#2c8a4f" : "#888",
          }}
        >
          ({v.tokenCount}/12 words · {v.wildcardCount} ? wildcard)
        </span>
        <textarea
          value={template}
          onChange={(e) => setTemplate(e.target.value)}
          rows={3}
          spellCheck={false}
          style={{
            width: "100%",
            padding: "0.6rem",
            marginTop: "0.4rem",
            fontFamily: "monospace",
            fontSize: "0.95rem",
            border: `1px solid ${
              template.length === 0 ? "#ddd" : v.ok ? "#2c8a4f" : "#d97706"
            }`,
            borderRadius: 6,
          }}
          placeholder="abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon ?"
        />
      </label>
      {!v.ok && template.length > 0 && (
        <div
          style={{
            marginTop: "0.5rem",
            padding: "0.5rem 0.8rem",
            background: "#fff8e1",
            border: "1px solid #ffd97a",
            borderRadius: 6,
            fontSize: "0.85rem",
          }}
        >
          {v.reasons.map((r, i) => (
            <div key={i}>⚠ {r}</div>
          ))}
          {v.hints.map((h, i) => (
            <div key={`h${i}`} style={{ color: "#666", marginTop: "0.2rem" }}>
              {h}
            </div>
          ))}
        </div>
      )}

      <div style={{ display: "flex", gap: "1rem", marginTop: "1.5rem", alignItems: "flex-end" }}>
        <label style={{ flex: 1, fontWeight: 600 }}>
          Wallet kind
          <select
            value={kind}
            onChange={(e) => setKind(e.target.value as Kind)}
            style={{
              display: "block",
              width: "100%",
              padding: "0.5rem",
              marginTop: "0.4rem",
              borderRadius: 6,
              border: "1px solid #ddd",
            }}
          >
            <option value="eth">
              Ethereum (also Base, Optimism, Arbitrum, Polygon, BNB, Monad)
            </option>
            <option value="btc">Bitcoin (native segwit, bc1q…)</option>
            <option value="sol">Solana (Phantom, Solflare, Backpack)</option>
          </select>
        </label>
      </div>

      <label style={{ display: "block", marginTop: "1.5rem", fontWeight: 600 }}>
        Target address (your wallet&apos;s public address)
        <input
          value={target}
          onChange={(e) => setTarget(e.target.value)}
          spellCheck={false}
          style={{
            width: "100%",
            padding: "0.6rem",
            marginTop: "0.4rem",
            fontFamily: "monospace",
            fontSize: "0.95rem",
            border: `1px solid ${
              target.length === 0 ? "#ddd" : tv.ok ? "#2c8a4f" : "#d97706"
            }`,
            borderRadius: 6,
          }}
          placeholder={
            kind === "eth"
              ? "0x9858effd232b4033e47d90003d41ec34ecaeda94"
              : kind === "btc"
              ? "bc1qcr8te4kr609gcawutmrza0j4xv80jy8z306fyu"
              : "HAgk14JpMQLgt6rVgv7cBQFJWFto5Dqxi472uT3DKpqk"
          }
        />
      </label>
      {!tv.ok && target.length > 0 && tv.reason && (
        <div
          style={{
            marginTop: "0.5rem",
            padding: "0.5rem 0.8rem",
            background: "#fff8e1",
            border: "1px solid #ffd97a",
            borderRadius: 6,
            fontSize: "0.85rem",
          }}
        >
          ⚠ {tv.reason}
        </div>
      )}

      <div style={{ marginTop: "1.5rem" }}>
        <button
          type="button"
          onClick={() => setShowAdvanced(!showAdvanced)}
          style={{
            background: "transparent",
            border: "none",
            color: "#666",
            cursor: "pointer",
            fontSize: "0.85rem",
            padding: 0,
          }}
        >
          {showAdvanced ? "▾" : "▸"} Advanced options (optional)
        </button>

        {showAdvanced && (
          <div
            style={{
              marginTop: "1rem",
              padding: "1rem",
              background: "#fafafa",
              borderRadius: 6,
              border: "1px solid #eee",
            }}
          >
            <label style={{ display: "block", fontWeight: 600 }}>
              BIP-39 passphrase{" "}
              <span style={{ fontWeight: 400, color: "#888", fontSize: "0.85rem" }}>
                (the optional &quot;25th word&quot; — leave empty if you did not
                use one)
              </span>
              <input
                type="password"
                value={passphrase}
                onChange={(e) => setPassphrase(e.target.value)}
                spellCheck={false}
                style={{
                  width: "100%",
                  padding: "0.5rem",
                  marginTop: "0.4rem",
                  borderRadius: 6,
                  border: "1px solid #ddd",
                }}
                placeholder="(empty if you did not use one)"
              />
            </label>

            <label
              style={{
                display: "block",
                marginTop: "1rem",
                fontWeight: 600,
              }}
            >
              Address index range{" "}
              <span
                style={{
                  fontWeight: 400,
                  color: "#888",
                  fontSize: "0.85rem",
                }}
              >
                (BTC/ETH only — how many derivation indexes to scan per
                candidate; default 5 covers most wallets)
              </span>
              <input
                type="number"
                value={indexRange}
                onChange={(e) => setIndexRange(parseInt(e.target.value, 10) || 1)}
                min={1}
                max={50}
                style={{
                  width: "6rem",
                  display: "block",
                  padding: "0.4rem",
                  marginTop: "0.4rem",
                  borderRadius: 6,
                  border: "1px solid #ddd",
                }}
              />
            </label>
          </div>
        )}
      </div>

      <button
        onClick={submit}
        disabled={!canSubmit}
        style={{
          padding: "0.85rem 1.6rem",
          background: canSubmit ? "#ff6b35" : "#ccc",
          color: "white",
          border: "none",
          borderRadius: 6,
          cursor: canSubmit ? "pointer" : "not-allowed",
          marginTop: "1.5rem",
          fontSize: "1rem",
          fontWeight: 600,
        }}
      >
        {busy ? "Searching…" : "Reconstruct"}
      </button>

      {error && (
        <div
          style={{
            marginTop: "1.5rem",
            padding: "1rem 1.2rem",
            background: "#fff5f5",
            border: "1px solid #f8c1c1",
            borderRadius: 8,
          }}
        >
          <strong style={{ color: "#c0392b" }}>Could not reconstruct.</strong>
          <p style={{ margin: "0.5rem 0 0", color: "#444", fontSize: "0.92rem" }}>
            {error}
          </p>
        </div>
      )}

      {result && (
        <div
          style={{
            marginTop: "1.5rem",
            padding: "1.4rem",
            background: "#f0fdf4",
            border: "2px solid #86efac",
            borderRadius: 10,
          }}
        >
          <h3 style={{ margin: "0 0 0.8rem", color: "#166534" }}>
            ✓ Recovered in {result.elapsed_ms} ms
          </h3>
          <p style={{ margin: "0.4rem 0" }}>
            <strong>Missing word:</strong>{" "}
            <code style={{ background: "#fff", padding: "0.15rem 0.4rem", borderRadius: 3 }}>
              {result.recovered_word}
            </code>
          </p>
          <p style={{ margin: "0.4rem 0" }}>
            <strong>Full mnemonic:</strong>
          </p>
          <code
            style={{
              display: "block",
              marginTop: "0.3rem",
              padding: "0.6rem",
              background: "#fff",
              border: "1px solid #d4d4d8",
              borderRadius: 6,
              wordBreak: "break-all",
              fontSize: "0.9rem",
            }}
          >
            {result.recovered_mnemonic}
          </code>
          <p
            style={{
              marginTop: "1rem",
              padding: "0.6rem 0.8rem",
              background: "#fffbeb",
              border: "1px solid #fde68a",
              borderRadius: 6,
              fontSize: "0.85rem",
              color: "#78350f",
            }}
          >
            ⚠ Save this mnemonic somewhere safe (paper, hardware wallet) before
            closing this window. Phoenix does not persist it.
          </p>
        </div>
      )}
    </main>
  );
}
