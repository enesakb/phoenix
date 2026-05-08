import { jsx as _jsx, jsxs as _jsxs } from "react/jsx-runtime";
import { BrowserRouter, Routes, Route, Link } from "react-router-dom";
import { Welcome } from "./routes/Welcome";
import { Settings } from "./routes/Settings";
export function App() {
    return (_jsxs(BrowserRouter, { children: [_jsxs("nav", { children: [_jsx(Link, { to: "/", children: "Home" }), " | ", _jsx(Link, { to: "/settings", children: "Settings" })] }), _jsxs(Routes, { children: [_jsx(Route, { path: "/", element: _jsx(Welcome, {}) }), _jsx(Route, { path: "/settings", element: _jsx(Settings, {}) })] })] }));
}
