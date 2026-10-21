# QA-Status

Stand der Prüfung: 2026-10-21

## Ausgeführt

- `npm test`: 4/4 Tests erfolgreich (Inline-JavaScript-Syntax, Funktions-/Branding-Präsenz, Persistenzfehlerpfade als statische Regression, JSON-Konfiguration und Capability-Datei).
- Inline-JavaScript zusätzlich mit `node --check` geprüft: erfolgreich.
- Tauri-Konfiguration und Capability-JSON werden von den Node-Tests geparst.
- ZIP-Archiv mit dem geänderten Projekt und einer unveränderten Kopie der gelieferten Quellen erstellt.

## Nicht ausgeführt

- Kein macOS-Build und kein Start der Tauri-WebView: Arbeitsumgebung ist Linux, und `cargo`/`rustc` sind nicht installiert.
- Keine Browser-E2E-Tests mit echtem IndexedDB-/localStorage-Speicher.
- Kein Restore-Test mit echten Foto-Backups.
- Keine Tests gegen absichtlich simulierte Speicherfehler im Browser.
- Die SQLite-Schicht wurde nicht kompiliert und ist noch nicht mit der UI verbunden.

Die vier automatisierten Tests sind statische Regressionstests und ersetzen keine End-to-End- oder macOS-Buildtests.


## Follow-up integration work

- Added Tauri `storage_load` / `storage_save` commands backed by SQLite `app_state`.
- Frontend now selects native SQLite persistence when the Tauri global API is available; browser mode retains IndexedDB/localStorage.
- A previous-state JSON snapshot (including embedded photos) is written before each native state overwrite. Failure to write that snapshot prevents the save.
- Added regression coverage for the native storage wiring.
- Still not verified: Rust compilation, Tauri/macOS build, runtime GUI behavior, native data migration, and restoration from an actual generated backup. Browser-profile data is not auto-imported; use JSON export/import.
