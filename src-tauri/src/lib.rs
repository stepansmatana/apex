use std::fs;
use tauri::Manager;

#[tauri::command]
fn save_data(app: tauri::AppHandle, data: String) -> Result<(), String> {
    let dir = app.path().app_local_data_dir().map_err(|e| e.to_string())?;
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    fs::write(dir.join("data.json"), data).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn load_data(app: tauri::AppHandle) -> Result<Option<String>, String> {
    let path = app.path().app_local_data_dir()
        .map_err(|e| e.to_string())?
        .join("data.json");
    if path.exists() {
        Ok(Some(fs::read_to_string(path).map_err(|e| e.to_string())?))
    } else {
        Ok(None)
    }
}

#[tauri::command]
fn get_data_path(app: tauri::AppHandle) -> Result<String, String> {
    let path = app.path().app_local_data_dir()
        .map_err(|e| e.to_string())?
        .join("data.json");
    Ok(path.to_string_lossy().to_string())
}

#[tauri::command]
fn export_backup(app: tauri::AppHandle, data: String, filename: String) -> Result<String, String> {
    let desktop = app.path().desktop_dir().map_err(|e| e.to_string())?;
    let path = desktop.join(&filename);
    fs::write(&path, &data).map_err(|e| e.to_string())?;
    Ok(path.to_string_lossy().to_string())
}

#[tauri::command]
fn open_data_folder(app: tauri::AppHandle) -> Result<(), String> {
    let dir = app.path().app_local_data_dir().map_err(|e| e.to_string())?;
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    #[cfg(target_os = "windows")]
    std::process::Command::new("explorer")
        .arg(dir.to_string_lossy().as_ref())
        .spawn()
        .map_err(|e| e.to_string())?;
    #[cfg(target_os = "macos")]
    std::process::Command::new("open")
        .arg(dir.to_string_lossy().as_ref())
        .spawn()
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            save_data,
            load_data,
            get_data_path,
            open_data_folder,
            export_backup
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
