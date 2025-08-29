use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::Manager;
use tauri_plugin_dialog::DialogExt;
use tokio::process::Command;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AppConfig {
    pub gemini_cli_path: Option<String>,
    pub gemini_cli_args: Vec<String>,
}

// Cargar la configuración desde config.json
fn load_config(app_handle: &tauri::AppHandle) -> Result<AppConfig, String> {
    let app_dir: PathBuf = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?;
    let config_path = app_dir.join("config.json");

    if config_path.exists() {
        let config_str = fs::read_to_string(&config_path).map_err(|e| e.to_string())?;
        serde_json::from_str(&config_str).map_err(|e| e.to_string())
    } else {
        Ok(AppConfig::default())
    }
}

// Guardar la configuración en config.json
fn save_config(app_handle: &tauri::AppHandle, config: &AppConfig) -> Result<(), String> {
    let app_dir: PathBuf = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?;
    let config_path = app_dir.join("config.json");

    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }

    let config_str = serde_json::to_string_pretty(config).map_err(|e| e.to_string())?;
    fs::write(&config_path, config_str).map_err(|e| e.to_string())?;
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
    pub id: u64,
    pub name: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub id: u64,
    pub session_id: u64,
    pub text: String,
    pub from_user: String,
    pub timestamp: String,
}

// Inicializar la base de datos SQLite
fn initialize_db(app_handle: &tauri::AppHandle) -> Result<Connection, String> {
    let app_dir: PathBuf = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?;
    let db_path = app_dir.join("gemini_ui.db");

    // Crear directorios padres si no existen
    if let Some(parent) = db_path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }

    let conn = Connection::open(&db_path).map_err(|e| e.to_string())?;

    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS sessions (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            created_at TEXT DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT DEFAULT CURRENT_TIMESTAMP
        );
        CREATE TABLE IF NOT EXISTS messages (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            session_id INTEGER NOT NULL,
            text TEXT NOT NULL,
            from_user TEXT NOT NULL,
            timestamp TEXT DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (session_id) REFERENCES sessions(id) ON DELETE CASCADE
        );",
    )
    .map_err(|e| e.to_string())?;

    Ok(conn)
}

// Comando básico de ejemplo
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn create_session(name: String, app_handle: tauri::AppHandle) -> Result<u64, String> {
    let conn = initialize_db(&app_handle)?;
    conn.execute("INSERT INTO sessions (name) VALUES (?1)", &[&name])
        .map_err(|e| e.to_string())?;
    Ok(conn.last_insert_rowid() as u64)
}

#[tauri::command]
fn get_sessions(app_handle: tauri::AppHandle) -> Result<Vec<Session>, String> {
    let conn = initialize_db(&app_handle)?;
    let mut stmt = conn
        .prepare("SELECT id, name, created_at, updated_at FROM sessions")
        .map_err(|e| e.to_string())?;
    let sessions_iter = stmt
        .query_map([], |row| {
            Ok(Session {
                id: row.get(0)?,
                name: row.get(1)?,
                created_at: row.get(2)?,
                updated_at: row.get(3)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut sessions = Vec::new();
    for session in sessions_iter {
        sessions.push(session.map_err(|e| e.to_string())?);
    }
    Ok(sessions)
}

// --- Detectar Gemini CLI ---
fn find_gemini_cli_executable() -> Option<String> {
    #[cfg(target_os = "windows")]
    {
        // Usar PowerShell Get-Command
        let output = std::process::Command::new("powershell")
            .arg("-NoProfile")
            .arg("-Command")
            .arg("Get-Command gemini | Select-Object -ExpandProperty Source")
            .output();

        if let Ok(output) = output {
            if output.status.success() {
                let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if !path.is_empty() {
                    return Some(path);
                }
            }
        }

        // Fallback manual
        let program_files = std::env::var_os("ProgramFiles").unwrap_or_default();
        let program_files_x86 = std::env::var_os("ProgramFiles(x86)").unwrap_or_default();
        let app_data_local = std::env::var_os("LOCALAPPDATA").unwrap_or_default();

        let paths = [
            PathBuf::from(program_files).join("Google").join("Gemini").join("gemini.exe"),
            PathBuf::from(program_files_x86).join("Google").join("Gemini").join("gemini.exe"),
            PathBuf::from(app_data_local).join("Google").join("Gemini").join("gemini.exe"),
        ];

        for path in paths {
            if path.exists() {
                return Some(path.to_string_lossy().into_owned());
            }
        }
    }

    #[cfg(target_os = "macos")]
    {
        let home = std::env::var_os("HOME").unwrap_or_default();
        let paths = [
            PathBuf::from("/usr/local/bin/gemini"),
            PathBuf::from("/usr/bin/gemini"),
            PathBuf::from(home).join(".local").join("bin").join("gemini"),
        ];

        for path in paths {
            if path.exists() {
                return Some(path.to_string_lossy().into_owned());
            }
        }
    }

    #[cfg(target_os = "linux")]
    {
        let home = std::env::var_os("HOME").unwrap_or_default();
        let paths = [
            PathBuf::from("/usr/local/bin/gemini"),
            PathBuf::from("/usr/bin/gemini"),
            PathBuf::from(home).join(".local").join("bin").join("gemini"),
        ];

        for path in paths {
            if path.exists() {
                return Some(path.to_string_lossy().into_owned());
            }
        }
    }

    None
}

// Ejecuta el CLI de Gemini de forma asíncrona
#[tauri::command]
async fn run_gemini_command(
    prompt: String,
    app_handle: tauri::AppHandle,
) -> Result<String, String> {
    let config_mutex = app_handle.state::<Mutex<AppConfig>>();
    let (gemini_cli_path, command_args) = {
        let config = config_mutex.lock().expect("Failed to lock config");
        let gemini_cli_path = config.gemini_cli_path.clone().unwrap_or_else(|| {
            find_gemini_cli_executable().unwrap_or_else(|| "gemini".to_string())
        });
        let mut command_args = config.gemini_cli_args.clone();
        command_args.push("--prompt".to_string());
        command_args.push(prompt.clone());
        (gemini_cli_path, command_args)
    };

    #[cfg(target_os = "windows")]
    let output = {
        if gemini_cli_path.ends_with(".ps1") {
            // Ejecutar con PowerShell
            Command::new("powershell")
                .arg("-NoProfile")
                .arg("-ExecutionPolicy")
                .arg("Bypass")
                .arg("-File")
                .arg(&gemini_cli_path)
                .args(&command_args)
                .output()
                .await
        } else if gemini_cli_path.ends_with(".bat") || gemini_cli_path.ends_with(".cmd") {
            // Ejecutar con CMD
            Command::new("cmd")
                .arg("/C")
                .arg(&gemini_cli_path)
                .args(&command_args)
                .output()
                .await
        } else {
            // Ejecutable nativo .exe
            Command::new(&gemini_cli_path)
                .args(&command_args)
                .output()
                .await
        }
    };

    #[cfg(not(target_os = "windows"))]
    let output = Command::new(&gemini_cli_path)
        .args(&command_args)
        .output()
        .await;

    let output = output.map_err(|e| {
        eprintln!("Error executing command: {}", e);
        e.to_string()
    })?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    eprintln!("Command executed: {} {:?}", gemini_cli_path, command_args);
    eprintln!("Command status: {}", output.status);
    eprintln!("Command stdout: {}", stdout);
    eprintln!("Command stderr: {}", stderr);

    if output.status.success() {
        Ok(stdout)
    } else {
        // Attempt to parse stderr as JSON to check for specific errors
        if let Ok(json_err) = serde_json::from_str::<serde_json::Value>(&stderr) {
            if let Some(error_obj) = json_err.get(0).and_then(|v| v.get("error")) {
                if error_obj.get("code") == Some(&serde_json::Value::from(429)) &&
                   error_obj.get("message").and_then(|m| m.as_str()).map_or(false, |m| m.contains("Quota exceeded")) {
                    return Err("GEMINI_QUOTA_EXCEEDED".to_string());
                }
            }
        }
        Err(stderr) // Return original stderr if not a quota error or not parsable
    }
}

#[tauri::command]
fn get_messages(session_id: u64, app_handle: tauri::AppHandle) -> Result<Vec<Message>, String> {
    let conn = initialize_db(&app_handle)?;
    let mut stmt = conn
        .prepare(
            "SELECT id, session_id, text, from_user, timestamp FROM messages WHERE session_id = ?1",
        )
        .map_err(|e| e.to_string())?;
    let messages_iter = stmt
        .query_map([session_id], |row| {
            Ok(Message {
                id: row.get(0)?,
                session_id: row.get(1)?,
                text: row.get(2)?,
                from_user: row.get(3)?,
                timestamp: row.get(4)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut messages = Vec::new();
    for message in messages_iter {
        messages.push(message.map_err(|e| e.to_string())?);
    }
    Ok(messages)
}

#[tauri::command]
fn save_message(
    session_id: u64,
    text: String,
    from_user: String,
    app_handle: tauri::AppHandle,
) -> Result<(), String> {
    let conn = initialize_db(&app_handle)?;
    conn.execute(
        "INSERT INTO messages (session_id, text, from_user) VALUES (?1, ?2, ?3)",
        rusqlite::params![session_id, text, from_user],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
async fn export_session(
    session_id: u64,
    format: String,
    app_handle: tauri::AppHandle,
) -> Result<String, String> {
    let conn = initialize_db(&app_handle)?;
    let mut stmt = conn
        .prepare(
            "SELECT text, from_user FROM messages WHERE session_id = ?1 ORDER BY timestamp ASC",
        )
        .map_err(|e| e.to_string())?;
    let messages_iter = stmt
        .query_map([session_id], |row| {
            Ok(Message {
                id: 0,
                session_id,
                text: row.get(0)?,
                from_user: row.get(1)?,
                timestamp: String::new(),
            })
        })
        .map_err(|e| e.to_string())?;

    let mut content = String::new();
    let file_extension = match format.as_str() {
        "markdown" => {
            for message in messages_iter {
                let msg = message.map_err(|e| e.to_string())?;
                if msg.from_user == "user" {
                    content.push_str(&format!("**User:** {}\n\n", msg.text));
                } else {
                    content.push_str(&format!("**Gemini:** {}\n\n", msg.text));
                }
            }
            "md"
        }
        "txt" => {
            for message in messages_iter {
                let msg = message.map_err(|e| e.to_string())?;
                content.push_str(&format!("{}: {}\n", msg.from_user, msg.text));
            }
            "txt"
        }
        "json" => {
            let messages: Vec<Message> = messages_iter.filter_map(|m| m.ok()).collect();
            content = serde_json::to_string_pretty(&messages).map_err(|e| e.to_string())?;
            "json"
        }
        _ => return Err("Unsupported format".to_string()),
    };

    let default_file_name = format!("session_{}.{}", session_id, file_extension);

    let path = app_handle
        .dialog()
        .file()
        .set_file_name(default_file_name)
        .blocking_save_file();

    if let Some(path_buf) = path {
        let p = path_buf.to_string();
        std::fs::write(&p, content).map_err(|e| e.to_string())?;
        Ok(p)
    } else {
        Err("File save cancelled".to_string())
    }
}

#[tauri::command]
fn delete_session(session_id: u64, app_handle: tauri::AppHandle) -> Result<(), String> {
    let conn = initialize_db(&app_handle)?;
    conn.execute("DELETE FROM sessions WHERE id = ?1", &[&session_id])
        .map_err(|e| e.to_string())?; 
    Ok(())
}

#[tauri::command]
fn get_config(app_handle: tauri::AppHandle) -> Result<AppConfig, String> {
    load_config(&app_handle)
}

#[tauri::command]
fn set_config(config: AppConfig, app_handle: tauri::AppHandle) -> Result<(), String> {
    save_config(&app_handle, &config)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(Mutex::new(AppConfig::default()))
        .invoke_handler(tauri::generate_handler![
            greet,
            run_gemini_command,
            create_session,
            get_sessions,
            get_messages,
            save_message,
            get_config,
            set_config,
            export_session,
            delete_session
        ])
        .plugin(tauri_plugin_dialog::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
