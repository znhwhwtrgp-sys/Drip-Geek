# Architektur und Risiken

## Ausgangslage

Das Originalarchiv enthält `index.html`, `manifest.webmanifest`, `sw.js`, drei PNG-Assets, `README.md` und `GITHUB-INSTALLATION.md`. Es gibt keine package.json, keine automatisierten Tests und keine Lizenzdatei. Die Oberfläche ist eine eigenständige Single-Page-App mit eingebettetem CSS und JavaScript.

## Persistenz heute

`index.html` öffnet IndexedDB `drip-geek-standalone-v1` mit einem Object Store `app` und Schlüssel `state`. Fällt IndexedDB vollständig aus, verwendet sie localStorage `drip-geek-state-v1`. Fotos werden als JPEG-Data-URL gespeichert. Das ist bequem, führt bei großen Bildbeständen aber zu Speicherlimits.

Der ursprüngliche `persist()`-Fehler wurde abgefangen und nicht weitergegeben. Dadurch konnten nachgelagerte Aktionen weiterlaufen und Erfolg anzeigen, obwohl die Speicherung fehlgeschlagen war. Die überarbeitete Funktion wartet auf den Abschluss/Abbruch der IndexedDB-Transaktion, prüft localStorage nach dem Schreiben, gibt Fehler weiter und stellt den letzten bestätigten Snapshot wieder her.

## Desktop-Grundlage

Tauri 2 Konfiguration und Rust-Einstiegspunkt wurden ergänzt. Beim Start wird eine lokale SQLite-Datei mit `schema_migrations` und Tabellen für garments, outfits, events, trips und wishlist initialisiert. Ein `storage_status`-Command liefert Schema-Version und Datenordner zurück. Die Oberfläche ruft diesen Command noch nicht auf; die aktive Persistenz bleibt deshalb IndexedDB/localStorage. SQLite ist ein Scaffold, keine abgeschlossene Migration.

## Risiken / Prioritäten

1. SQLite als einzige persistente Quelle anbinden, mit expliziten Commands und Transaktionen.
2. Fotos als Dateien unter Application Support speichern, mit stabilen relativen IDs und atomarer Datei-/DB-Aktualisierung.
3. Versionierte Backup-Datei mit Fotos, Hashes und Manifest implementieren; Restore erst nach vollständiger Validierung und in temporärer Datenbank.
4. Erfolgsfeedback für jede mutierende Aktion nur nach bestätigter Speicherung auslösen; alle Mutation-Handler mit Fehlerbehandlung ausstatten.
5. E2E-Tests mit isoliertem Testprofil sowie Fehler-Injektion ergänzen.
6. Analyzer nur mit expliziter Nutzerfreigabe für Online-KI; Bildanalyse-Ergebnisse als Schätzungen kennzeichnen.

## iPhone und Sync

Für spätere Wiederverwendung sind stabile IDs und exportierbare JSON-Objekte vorhanden bzw. bereits im Modell angelegt. Vor Sync braucht es ein versioniertes, dokumentiertes Schema, `createdAt`/`updatedAt`, Tombstones für Löschungen, Konfliktregeln und eine explizite Verschlüsselungs-/Transportentscheidung. Noch nicht implementiert.


## Native macOS persistence (current implementation)

In the Tauri runtime, the frontend calls `storage_load` and `storage_save` through Tauri's core invoke API. The authoritative application state (`items`, `outfits`, `events`, `trips`, `wishlist`) is stored as JSON in the SQLite `app_state` row and committed in a SQLite transaction. This is an integration step: the older per-entity tables remain schema placeholders and are not the active source of truth yet.

Before each overwrite, the previous confirmed state is written to a timestamped JSON safety snapshot in the app data `FIT-GEEK/backups` directory. The snapshot includes garment photos because the existing model stores them as image data URLs. A snapshot write failure aborts the database update. User-triggered JSON export remains available, and its format is compatible with the import validator.

The existing web-browser runtime continues to use IndexedDB/localStorage. This does not automatically migrate browser-profile data into the Tauri app's separate WebView profile. Export a JSON backup from the old browser version and import it into the desktop app to migrate data.
