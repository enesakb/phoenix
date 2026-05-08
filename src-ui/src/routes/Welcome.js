import { jsx as _jsx, jsxs as _jsxs } from "react/jsx-runtime";
import { useEffect, useState } from "react";
import { fetchAppInfo } from "../lib/ipc";
export function Welcome() {
    const [info, setInfo] = useState(null);
    const [error, setError] = useState(null);
    useEffect(() => {
        fetchAppInfo()
            .then(setInfo)
            .catch((e) => setError(e.message));
    }, []);
    return (_jsxs("main", { children: [_jsx("h1", { children: "Phoenix" }), _jsx("p", { children: "Open-source forensic recovery assistant." }), error && _jsxs("p", { style: { color: "red" }, children: ["IPC error: ", error] }), info && (_jsxs("p", { children: [info.name, " v", info.version] }))] }));
}
