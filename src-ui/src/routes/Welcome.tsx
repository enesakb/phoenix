import { useEffect, useState } from "react";
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
    <main>
      <h1>Phoenix</h1>
      <p>Open-source forensic recovery assistant.</p>
      {error && <p style={{ color: "red" }}>IPC error: {error}</p>}
      {info && (
        <p>
          {info.name} v{info.version}
        </p>
      )}
    </main>
  );
}
