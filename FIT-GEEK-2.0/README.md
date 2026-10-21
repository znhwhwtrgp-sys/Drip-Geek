# FIT-GEEK 2.0

Lokale Garderoben-App auf Basis des bereitgestellten Drip-Geek-Projekts. Die bestehende HTML/CSS/JavaScript-Oberfläche bleibt erhalten; die Oberfläche wurde auf den Produktnamen FIT-GEEK 2.0 umgestellt.

## Status dieser Übergabe

Dies ist ein **Arbeitsstand mit Desktop-Hülle und angebundener SQLite-Zustandsspeicherung**, noch kein fertig validierter Release. Im Tauri-Modus lädt und speichert die Oberfläche den kompletten App-Zustand über native Rust-Befehle in SQLite. Die Browser-Variante nutzt weiterhin IndexedDB (`drip-geek-standalone-v1`) mit localStorage-Fallback (`drip-geek-state-v1`). Bestehende Browserdaten werden nicht automatisch in das separate Tauri-WebView-Profil migriert: exportiere sie in der alten Variante als JSON und importiere dieses Backup anschließend in der Desktop-App.

## Voraussetzungen auf macOS

- Aktuelle unterstützte macOS-Version
- Node.js und npm
- Rust toolchain (`rustup`, `cargo`)
- Xcode Command Line Tools
- Internet nur zum erstmaligen Herunterladen der Entwicklungsabhängigkeiten; die Garderobenfunktionen selbst benötigen kein Internet. Die optionale Wetterfunktion fragt Open-Meteo ab.

Installation der Voraussetzungen erfolgt nach den offiziellen Dokumentationen von Node.js, Rust und Tauri. Keine globalen Administratorrechte sind für die Projektabhängigkeiten erforderlich; Xcode Command Line Tools können je nach Mac eine Systeminstallation benötigen.

## Start

Im Projektverzeichnis:

```sh
npm install
npm run dev
```

Release-Paket (auf macOS ausführen):

```sh
npm run build
```

Erwartete Ausgabe ist eine `.app` und/oder `.dmg` unter `src-tauri/target/release/bundle/`. Dieser Pfad ist eine erwartete Tauri-Ausgabe, nicht die Bestätigung eines bereits erzeugten Builds.

Browser-Entwicklungsvariante ohne Tauri: `index.html` öffnen. Offline-Service-Worker-Funktionen sind nur über HTTP(S) verfügbar. Browserdaten können bei Browserbereinigung verloren gehen; regelmäßig ein JSON-Backup exportieren.

## Daten und Datenschutz

- **Aktive Weboberfläche:** IndexedDB `drip-geek-standalone-v1`, Fallback localStorage `drip-geek-state-v1` im jeweiligen WebView-Profil.
- **Tauri-App:** Betriebssystem-App-Datenordner mit Unterordner `FIT-GEEK/`, darin `fit-geek.sqlite3`, `photos/` und `backups/`. Der aktuelle App-Zustand wird in SQLite in `app_state` gespeichert; zusätzliche Tabellen sind Schema-Grundlage und noch nicht die aktive Datenquelle. Vor dem Überschreiben des Zustands wird eine JSON-Sicherheitskopie des vorherigen Zustands geschrieben. Fotos bleiben aktuell als eingebettete Data-URLs erhalten; eine getrennte Dateispeicherung ist noch offen.
- Keine Anmeldung, kein eigener Server, keine Synchronisation und keine automatische Fotoübertragung. Wetterabfrage ist optional und benötigt Internet.
- Kleidungsfotos werden im vorhandenen Web-Frontend auf maximal 1000 Pixel Kantenlänge verkleinert und als JPEG-Daten-URL gespeichert.

## Backup und Wiederherstellung

1. Öffne **Mehr → Datensicherung → Backup exportieren**.
2. Speichere die heruntergeladene `fit-geek-backup-YYYY-MM-DD.json` an einem sicheren Ort außerhalb des App-Datenordners.
3. Prüfe, dass die Datei vorhanden ist und eine plausible Größe hat; behalte möglichst mehrere Versionen.
4. Zur Wiederherstellung: **Backup importieren**, die JSON-Datei auswählen und die Ersetzungsbestätigung sorgfältig prüfen.
5. Der Import validiert JSON-Grundstruktur, Listentypen, Datensatzformen und Dateigröße vor dem Austausch. Bei Speicherfehlern wird der Import abgebrochen. Vor dem Import sollte trotzdem ein aktuelles Backup erstellt werden.

Die Benutzer-Wiederherstellung erfolgt über den validierten JSON-Import, der auch in Tauri den Zustand atomar in SQLite speichert. Automatische Sicherheitskopien vor nativen Schreibvorgängen liegen im App-Datenordner unter `FIT-GEEK/backups/` und sind JSON-Dateien mit eingebetteten Foto-Daten. Die Wiederherstellung aus einem solchen Snapshot sollte vor produktiver Nutzung noch explizit end-to-end getestet werden.

## Entfernen

Die Browservariante kann durch Entfernen der Projektdateien gelöscht werden; Browserdaten sind separat in den Website-/WebView-Daten zu verwalten. Für eine installierte Desktop-App die `.app` in den Papierkorb verschieben. **Das löscht nicht zwangsläufig Datenordner oder Browserprofil.** Vor dem Löschen privater Daten ein Backup exportieren. Ein geführter Deinstallations-/Datenlöschdialog ist noch offen.

## Funktionsinventar aus dem gelieferten Projekt

- Dashboard und Kennzahlen
- Garderobe: CRUD, Fotos, Kategorie, Farbe, Saison, Marke, Größe, Zustand, Preis, Notizen und Favoriten
- Suche, Filter und Sortierung
- Outfit-Studio: Erstellen/Bearbeiten/Duplizieren/Löschen/Planen
- Style-Vorschläge anhand vorhandener Teile
- Wetterbasierte Outfit-Auswahl (Internet erforderlich)
- Monatskalender und Termine mit Outfit-Zuordnung
- Reisen und Packlisten
- Wunschliste
- Statistiken
- JSON-Export und -Import
- Responsive Dark UI, Offline-Grundfunktion

## Tests und bekannte Lücken

- `npm test` führt die automatisierten statischen Regressionstests aus.
- `node --check` prüft die eingebettete JavaScript-Syntax.
- Ein echter Tauri/macOS-Build wurde in dieser Umgebung nicht ausgeführt (Rust/Cargo und macOS-Buildumgebung nicht vorhanden).
- Noch offen: getrennte Dateiablage für Fotos statt Data-URLs, Retention/Verwaltung automatischer Snapshots, End-to-End-Test von Backup und Wiederherstellung im Tauri-WebView, UI-Anbindung der Analyzer-Vorschläge, native iPhone-App und Synchronisationskonzept.
- `Smart Clothing Analyzer` ist noch nicht integriert. Es gibt keine KI-Funktion, die Kleidungsstücke automatisch analysiert; diese Lücke wird ausdrücklich nicht als erledigt ausgegeben.
- Im gelieferten Archiv war keine Lizenzdatei enthalten. Vor Weiterverteilung ist die Lizenzlage zu klären.
