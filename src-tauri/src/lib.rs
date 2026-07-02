use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use tauri::Manager;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ImportEntry {
    pub name: String,
    pub content: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ExportNote {
    pub name: String,
    pub content: String,
}

fn fn_walk_dir(dir: &Path, entries: &mut Vec<ImportEntry>) -> Result<(), String> {
    for entry in std::fs::read_dir(dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if path.is_dir() {
            fn_walk_dir(&path, entries)?;
        } else if let Some(ext) = path.extension() {
            if ext == "md" || ext == "txt" {
                let name = path
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string();
                let content = std::fs::read_to_string(&path).map_err(|e| e.to_string())?;
                entries.push(ImportEntry { name, content });
            }
        }
    }
    Ok(())
}

#[tauri::command]
fn fn_read_file(path: String) -> Result<String, String> {
    std::fs::read_to_string(&path).map_err(|e| e.to_string())
}

#[tauri::command]
fn fn_write_file(path: String, content: String) -> Result<(), String> {
    std::fs::write(&path, &content).map_err(|e| e.to_string())
}

#[tauri::command]
fn fn_import_folder(path: String) -> Result<Vec<ImportEntry>, String> {
    let mut entries = Vec::new();
    fn_walk_dir(Path::new(&path), &mut entries)?;
    Ok(entries)
}

#[tauri::command]
fn fn_export_zip(notes: Vec<ExportNote>, save_path: String) -> Result<(), String> {
    let file = std::fs::File::create(&save_path).map_err(|e| e.to_string())?;
    let mut zip = zip::ZipWriter::new(file);
    let options: zip::write::FileOptions<'_, ()> =
        zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Deflated);

    for note in &notes {
        let filename = if note.name.ends_with(".md") || note.name.ends_with(".txt") {
            note.name.clone()
        } else {
            format!("{}.md", note.name)
        };
        zip.start_file(&filename, options).map_err(|e| e.to_string())?;
        zip.write_all(note.content.as_bytes())
            .map_err(|e| e.to_string())?;
    }

    zip.finish().map_err(|e| e.to_string())?;
    Ok(())
}

fn fn_get_config_path(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    let config_dir = app.path().app_config_dir().map_err(|e| e.to_string())?;
    std::fs::create_dir_all(&config_dir).map_err(|e| e.to_string())?;
    Ok(config_dir.join("md-editor.ini"))
}

#[tauri::command]
fn fn_read_config(app: tauri::AppHandle) -> Result<String, String> {
    let config_path = fn_get_config_path(&app)?;
    if !config_path.exists() {
        return Ok("[General]\nStartupType=viewer\nLanguage=english\n".to_string());
    }
    std::fs::read_to_string(&config_path).map_err(|e| e.to_string())
}

#[tauri::command]
fn fn_get_config_dir(app: tauri::AppHandle) -> Result<String, String> {
    let config_dir = app.path().app_config_dir().map_err(|e| e.to_string())?;
    Ok(config_dir.to_string_lossy().to_string())
}

#[tauri::command]
fn fn_write_config(app: tauri::AppHandle, content: String) -> Result<(), String> {
    let config_path = fn_get_config_path(&app)?;
    std::fs::write(&config_path, &content).map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            fn_read_file,
            fn_write_file,
            fn_import_folder,
            fn_export_zip,
            fn_read_config,
            fn_write_config,
            fn_get_config_dir,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
