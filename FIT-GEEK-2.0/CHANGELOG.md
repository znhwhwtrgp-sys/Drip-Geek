# Änderungen – FIT-GEEK 2.0

- Ausgangsprojekt unverändert unter `originals/Drip-Geek-original/` gesichert.
- Produktbranding auf FIT-GEEK 2.0 in Fenstertitel, Seitenüberschriften, PWA-Manifest und Backup-Dateinamen angepasst.
- Persistenzfehler werden nicht mehr still verschluckt: IndexedDB-Transaktion wartet auf Abschluss/Abbruch; localStorage-Schreiben wird nachgelesen; fehlgeschlagene Writes werden als Fehler weitergegeben und der letzte bestätigte Zustand wird wiederhergestellt.
- Erfolgsfeedback in vier zuvor unmittelbar nach Start des asynchronen Speicherns ausgelösten Aktionen auf die erfolgreiche Persistenz verschoben.
- JSON-Import validiert Größe, JSON-Grundstruktur, Listentypen und Datensatzformen vor dem Austausch und stellt den vorherigen Zustand bei fehlgeschlagener Speicherung wieder her.
- Tauri-2-Desktop-Konfiguration, Rust-Startpunkt und lokale SQLite-Initialisierung mit Schema-Versionierung hinzugefügt.
- README, Architektur-/Risikoübersicht und Node-Regressionstests hinzugefügt.
- Keine externen KI- oder Cloud-Abhängigkeiten ergänzt.
