import { invoke } from "@tauri-apps/api/core";
export async function fetchAppInfo() {
    return invoke("app_info");
}
