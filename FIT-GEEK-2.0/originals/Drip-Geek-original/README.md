# Drip Geek — unabhängige App

Eigenständige, dependency-freie Web-App. Die Implementierung wurde neu erstellt und verwendet keine Dateien oder Laufzeit-Abhängigkeiten der vorherigen App.

## Start

- `index.html` direkt im Browser öffnen (Kernfunktionen und lokale Speicherung).
- Für PWA-/Offline-Service-Worker-Funktionen die Dateien über einen lokalen oder statischen HTTP-Server aufrufen. GitHub Pages funktioniert ohne Build-Schritt.
- Lade `index.html`, `manifest.webmanifest` und `sw.js` ins Root-Verzeichnis des Repositorys.
- GitHub: **Settings → Pages → Deploy from a branch**, Branch `main`, Folder `/ (root)`.

## Funktionen

- Home-Dashboard und Kennzahlen
- Digitaler Kleiderschrank: CRUD, Foto, Kategorie, Farbe, Saison, Marke, Größe, Zustand, Preis, Notizen, Favoriten
- Suche, Filter und Sortierung
- Outfit-Studio: Teile aus dem eigenen Schrank auswählen, speichern, bearbeiten, duplizieren, löschen und planen
- Style-Vorschläge aus vorhandenen Teilen
- Monatskalender und Termine mit Outfit-Zuordnung
- Reisen und interaktive Packlisten
- Wunschliste
- Statistiken
- JSON-Export und -Import
- Lokale Persistenz über IndexedDB (Fallback: localStorage)
- Responsive Dark UI mit fester mobiler Bottom-Navigation

## Datenschutz

Daten bleiben lokal im Browser. Es gibt keinen Server, keine Anmeldung und keine automatische Synchronisierung. Browserdaten regelmäßig exportieren. Beim Import werden die aktuellen Daten nach Bestätigung ersetzt.

## Hinweise

- Diese Version benötigt keine npm-Abhängigkeiten und keinen Build-Schritt.
- Der Service Worker kann nur über HTTP(S), nicht bei `file://`, aktiv sein.
- Browser-Speicher ist nicht unbegrenzt; Fotos werden vor dem Speichern auf max. 1000 px Kantenlänge verkleinert.
