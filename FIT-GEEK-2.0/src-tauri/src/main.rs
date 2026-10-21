#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use rusqlite::{Connection, OptionalExtension};
use std::{fs, path::PathBuf};
use tauri::{Manager, State};
use std::sync::Mutex;

struct LocalDb(Mutex<Option<Connection>>);
fn data_dir(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    let base = app.path().app_data_dir().map_err(|e| format!("App-Datenordner nicht verfügbar: {e}"))?;
    let dir = base.join("FIT-GEEK");
    fs::create_dir_all(dir.join("photos")).map_err(|e| format!("Datenordner konnte nicht angelegt werden: {e}"))?;
    fs::create_dir_all(dir.join("backups")).map_err(|e| format!("Backup-Ordner konnte nicht angelegt werden: {e}"))?;
    Ok(dir)
}
fn open_db(app: &tauri::AppHandle) -> Result<Connection, String> {
    let path = data_dir(app)?.join("fit-geek.sqlite3");
    let db = Connection::open(path).map_err(|e| format!("SQLite konnte nicht geöffnet werden: {e}"))?;
    db.execute_batch("PRAGMA foreign_keys=ON; PRAGMA journal_mode=WAL; PRAGMA busy_timeout=5000;
      CREATE TABLE IF NOT EXISTS schema_migrations (version INTEGER PRIMARY KEY, applied_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP);
      CREATE TABLE IF NOT EXISTS app_state (id INTEGER PRIMARY KEY CHECK(id=1), data_json TEXT NOT NULL, updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP);
      CREATE TABLE IF NOT EXISTS garments (id TEXT PRIMARY KEY NOT NULL, data_json TEXT NOT NULL, created_at TEXT NOT NULL, updated_at TEXT NOT NULL);
      CREATE TABLE IF NOT EXISTS outfits (id TEXT PRIMARY KEY NOT NULL, data_json TEXT NOT NULL, created_at TEXT NOT NULL, updated_at TEXT NOT NULL);
      CREATE TABLE IF NOT EXISTS events (id TEXT PRIMARY KEY NOT NULL, data_json TEXT NOT NULL, updated_at TEXT NOT NULL);
      CREATE TABLE IF NOT EXISTS trips (id TEXT PRIMARY KEY NOT NULL, data_json TEXT NOT NULL, updated_at TEXT NOT NULL);
      CREATE TABLE IF NOT EXISTS wishlist (id TEXT PRIMARY KEY NOT NULL, data_json TEXT NOT NULL, updated_at TEXT NOT NULL);
      INSERT OR IGNORE INTO schema_migrations(version) VALUES (1);")
      .map_err(|e| format!("SQLite-Schema konnte nicht initialisiert werden: {e}"))?;
    Ok(db)
}
#[tauri::command]
fn storage_load(db: State<'_, LocalDb>) -> Result<Option<serde_json::Value>, String> {
    let guard = db.0.lock().map_err(|_| "Datenbanksperre fehlgeschlagen".to_string())?;
    let conn = guard.as_ref().ok_or_else(|| "Datenbank ist nicht initialisiert".to_string())?;
    let raw: Option<String> = conn.query_row("SELECT data_json FROM app_state WHERE id=1", [], |r| r.get(0)).optional()
        .map_err(|e| format!("Lokale Daten konnten nicht gelesen werden: {e}"))?;
    raw.map(|s| serde_json::from_str(&s).map_err(|e| format!("Gespeicherte Daten sind ungültig: {e}"))).transpose()
}
#[tauri::command]
fn storage_save(app: tauri::AppHandle, db: State<'_, LocalDb>, data: serde_json::Value) -> Result<(), String> {
    for key in ["items", "outfits", "events", "trips", "wishlist"] {
        if !data.get(key).is_some_and(|v| v.is_array()) { return Err(format!("Speichern abgebrochen: '{key}' muss eine Liste sein.")); }
    }
    let serialized = serde_json::to_string(&data).map_err(|e| format!("Daten konnten nicht serialisiert werden: {e}"))?;
    let dir = data_dir(&app)?;
    let guard = db.0.lock().map_err(|_| "Datenbanksperre fehlgeschlagen".to_string())?;
    let conn = guard.as_ref().ok_or_else(|| "Datenbank ist nicht initialisiert".to_string())?;
    // Snapshot des vorherigen bestätigten Zustands vor jeder Änderung; Fotos bleiben als Data-URLs enthalten.
    let previous: Option<String> = conn.query_row("SELECT data_json FROM app_state WHERE id=1", [], |r| r.get(0)).optional()
        .map_err(|e| format!("Vorheriger Zustand konnte nicht gelesen werden: {e}"))?;
    if let Some(old) = previous.as_deref() {
        let backup = dir.join("backups").join(format!("auto-{}.json", chrono_stamp()));
        let mut snapshot: serde_json::Value = serde_json::from_str(old)
            .map_err(|e| format!("Vorheriger Zustand ist kein gültiges JSON; nichts wurde gespeichert: {e}"))?;
        if let Some(obj) = snapshot.as_object_mut() {
            obj.insert("app".into(), serde_json::json!("FIT-GEEK 2.0"));
            obj.insert("version".into(), serde_json::json!(2));
            obj.insert("backupType".into(), serde_json::json!("automatic-before-save"));
            obj.insert("exportedAt".into(), serde_json::json!(format!("unix-ms:{}", chrono_stamp())));
        }
        let backup_json = serde_json::to_vec_pretty(&snapshot)
            .map_err(|e| format!("Sicherungsdatei konnte nicht erstellt werden: {e}"))?;
        fs::write(&backup, backup_json)
            .map_err(|e| format!("Automatisches Sicherheitsbackup fehlgeschlagen; nichts wurde gespeichert: {e}"))?;
    }
    let tx = conn.unchecked_transaction().map_err(|e| format!("Transaktion konnte nicht gestartet werden: {e}"))?;
    tx.execute("INSERT INTO app_state(id,data_json,updated_at) VALUES(1,?1,CURRENT_TIMESTAMP) ON CONFLICT(id) DO UPDATE SET data_json=excluded.data_json, updated_at=CURRENT_TIMESTAMP", [&serialized])
        .map_err(|e| format!("Datenbank konnte nicht aktualisiert werden: {e}"))?;
    tx.commit().map_err(|e| format!("Speichern konnte nicht bestätigt werden: {e}"))
}
fn chrono_stamp() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_millis().to_string()
}
#[tauri::command]
fn storage_status(app: tauri::AppHandle, db: State<'_, LocalDb>) -> Result<serde_json::Value, String> {
    let guard = db.0.lock().map_err(|_| "Datenbanksperre fehlgeschlagen".to_string())?;
    let conn = guard.as_ref().ok_or_else(|| "Datenbank ist nicht initialisiert".to_string())?;
    let version: i64 = conn.query_row("SELECT MAX(version) FROM schema_migrations", [], |r| r.get(0))
        .map_err(|e| format!("Schema-Version konnte nicht gelesen werden: {e}"))?;
    Ok(serde_json::json!({"ok": true, "schemaVersion": version, "dataDirectory": data_dir(&app)?.to_string_lossy()}))
}
fn main() {
    tauri::Builder::default()
      .setup(|app| {
        let conn = open_db(app.handle()).map_err(std::io::Error::other)?;
        app.manage(LocalDb(Mutex::new(Some(conn))));
        Ok(())
      })
      .invoke_handler(tauri::generate_handler![storage_status, storage_load, storage_save])
      .run(tauri::generate_context!())
      .expect("FIT-GEEK 2.0 konnte nicht gestartet werden");
}
