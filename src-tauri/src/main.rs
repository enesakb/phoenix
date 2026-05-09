#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;

fn main() {
    tauri::Builder::default()
        .manage(commands::InterviewState::new())
        .invoke_handler(tauri::generate_handler![
            commands::app_info,
            commands::list_questions,
            commands::start_interview,
            commands::answer_question,
            commands::get_candidates,
            commands::get_memory,
            commands::complete_interview,
        ])
        .run(tauri::generate_context!())
        .expect("error while running Phoenix");
}
