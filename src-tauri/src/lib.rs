pub mod book;
pub mod config;
pub mod extract;
pub mod metadata;
pub mod notes;
pub mod types;

/*
pub mod metadata;
pub mod book;
pub mod notes;
pub mod extract;
pub mod search;
pub mod commands;
pub mod calm;
pub mod protocol;
*/

/*
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
*/
