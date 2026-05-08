import { describe, it, expect, vi } from "vitest";
import { fetchAppInfo } from "./ipc";

vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn(async (cmd: string) => {
    if (cmd === "app_info") {
      return { name: "Phoenix", version: "0.1.0" };
    }
    throw new Error(`unknown command: ${cmd}`);
  }),
}));

describe("fetchAppInfo", () => {
  it("returns name and version from the IPC bridge", async () => {
    const info = await fetchAppInfo();
    expect(info.name).toBe("Phoenix");
    expect(info.version).toBe("0.1.0");
  });
});
