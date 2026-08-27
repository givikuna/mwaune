pub mod book;
pub mod commands;
pub mod config;
pub mod extract;
pub mod metadata;
pub mod notes;
pub mod search;
pub mod types;

use commands::AppState;
use tauri::Builder;

/*
pub fn run() {
    let config = config::Config::load().unwrap_or_default();
    let metadata =
        metadata::MetadataStore::load(&config.data_dir).unwrap_or_default();

    let state = AppState {
        config: std::sync::Mutex::new(config),
        metadata: std::sync::Mutex::new(metadata),
    };

    Builder::default()
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            commands::get_books,
            commands::add_book,
            commands::update_book,
            commands::delete_book,
            commands::add_note,
            commands::update_progress,
            commands::get_config,
            commands::update_config,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
*/
