use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AppInfo {
    pub name: String,
    pub version: String,
}

#[tauri::command]
pub fn app_info() -> AppInfo {
    AppInfo {
        name: "Phoenix".to_string(),
        version: phoenix_core::version().to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn app_info_returns_name_and_version() {
        let info = app_info();
        assert_eq!(info.name, "Phoenix");
        assert_eq!(info.version, "0.1.0");
    }
}
