import { BrowserRouter, Routes, Route, Link } from "react-router-dom";
import { Welcome } from "./routes/Welcome";
import { Settings } from "./routes/Settings";
import { Interview } from "./routes/Interview";
import { Candidates } from "./routes/Candidates";
import { Import } from "./routes/Import";

export function App() {
  return (
    <BrowserRouter>
      <nav style={{ padding: "1rem", borderBottom: "1px solid #eee" }}>
        <Link to="/">Home</Link>
        {" · "}
        <Link to="/interview">Interview</Link>
        {" · "}
        <Link to="/settings">Settings</Link>
      </nav>
      <Routes>
        <Route path="/" element={<Welcome />} />
        <Route path="/interview" element={<Interview />} />
        <Route path="/import/:sessionId" element={<Import />} />
        <Route path="/candidates/:sessionId" element={<Candidates />} />
        <Route path="/settings" element={<Settings />} />
      </Routes>
    </BrowserRouter>
  );
}
