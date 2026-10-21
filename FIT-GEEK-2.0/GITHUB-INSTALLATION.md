# GitHub Pages Installation

1. Neues Repository anlegen oder vorhandenes Repository öffnen.
2. `index.html`, `manifest.webmanifest`, `sw.js`, `README.md` und diese Datei in das Repository-Root hochladen.
3. Repository → **Settings → Pages**.
4. Unter **Build and deployment** die Quelle **Deploy from a branch** wählen.
5. Branch `main` und Ordner `/ (root)` wählen, dann speichern.
6. Nach Abschluss der Veröffentlichung die von GitHub angezeigte Pages-URL öffnen.

Es wird kein GitHub Actions Build und kein `npm install` benötigt.

Wenn eine ältere Version angezeigt wird: Hard Reload (Strg+F5) ausführen. Wenn früher ein Service Worker verwendet wurde, einmal die Website-Daten im Browser löschen oder unter DevTools → Application → Service Workers den alten Worker abmelden.
