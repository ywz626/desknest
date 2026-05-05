pub mod commands;
pub mod models;
pub mod services;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            commands::scan::scan_desktop_items,
            commands::icon::extract_file_icon,
            commands::open::open_item,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
