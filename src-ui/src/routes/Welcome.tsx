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
    <main style={{ maxWidth: 720, margin: "0 auto", padding: "2rem" }}>
      <h1>Phoenix</h1>
      <p>Open-source forensic recovery assistant.</p>
      {error && <p style={{ color: "red" }}>IPC error: {error}</p>}
      {info && (
        <p style={{ opacity: 0.7 }}>
          {info.name} v{info.version}
        </p>
      )}

      <hr style={{ margin: "2rem 0" }} />

      <h2>Start a recovery</h2>
      <p>
        We will ask 30+ questions about your lost wallet. Answer freely; even
        fragments help. The interview never leaves this device.
      </p>
      <Link
        to="/interview"
        style={{
          display: "inline-block",
          padding: "0.75rem 1.5rem",
          background: "#ff6b35",
          color: "white",
          borderRadius: "6px",
          textDecoration: "none",
          marginTop: "1rem",
        }}
      >
        Begin cognitive interview →
      </Link>
    </main>
  );
}
