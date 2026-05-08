import { BrowserRouter, Routes, Route, Link } from "react-router-dom";
import { Welcome } from "./routes/Welcome";
import { Settings } from "./routes/Settings";

export function App() {
  return (
    <BrowserRouter>
      <nav>
        <Link to="/">Home</Link> | <Link to="/settings">Settings</Link>
      </nav>
      <Routes>
        <Route path="/" element={<Welcome />} />
        <Route path="/settings" element={<Settings />} />
      </Routes>
    </BrowserRouter>
  );
}
