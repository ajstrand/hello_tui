# Anwendungsstart
app-title = IOCraft Interaktiver Texteditor
app-startup = 🎉 Starte IOCraft Interaktiven Texteditor...
app-thanks = 👋 Vielen Dank für die Nutzung des IOCraft Texteditors!

# Dateioperationen
file-new = 📝 Neue Datei wird erstellt...
file-opening = 📂 Öffne Datei: { $filename }
file-loaded = Geladen: { $filename }
file-saved = Gespeichert: { $filename }
file-error-loading = Fehler beim Laden von { $filename }: { $error }
file-error-saving = Fehler beim Speichern von { $filename }: { $error }
file-not-found = ⚠️ Datei '{ $filename }' nicht gefunden.
file-create-new = 🆕 Möchten Sie eine neue Datei erstellen? (j/n)
file-creation-canceled = Dateierstellung abgebrochen
file-new-ready = Neue Datei '{ $filename }' bereit zum Bearbeiten
file-no-input = Keine Eingabe angegeben
file-no-file-specified = Keine Datei angegeben
file-opened-success = ✅ Geöffnet: { $filename }
file-new-created = 🆕 Neue Datei '{ $filename }' erstellt - bereit zum Bearbeiten!
file-unsaved-changes = Datei hat ungespeicherte Änderungen! Drücken Sie Ctrl+Q erneut, um ohne Speichern zu beenden.
file-save-before-new = Aktuelle Datei vor Erstellen einer neuen speichern (Ctrl+S)
file-save-before-open = Aktuelle Datei vor Öffnen einer neuen speichern (Ctrl+S)

# Willkommensinhalt
welcome-title = Willkommen im IOCraft Erweiterten Texteditor!
welcome-features = Funktionen:
welcome-feature-highlighting = ✨ Schöne Zeilennummern und Syntaxhervorhebung
welcome-feature-cursor = 🎯 Moderner Cursor und visuelle Indikatoren
welcome-feature-mouse = 🖱️ Vollständige Mausunterstützung (Klicken, Ziehen, Auswählen)
welcome-feature-dialogs = 📁 IOCraft Dateidialoge und Browser
welcome-feature-shortcuts = ⌨️ Tastenkombinationen im Sublime Text-Stil
welcome-feature-linting = 🔍 Echtzeit-Code-Linting und Problemerkennung
welcome-shortcuts = Tastenkombinationen:
welcome-shortcuts-file = 📄 Datei: Ctrl+O (öffnen), Ctrl+S (speichern), Ctrl+N (neu)
welcome-shortcuts-edit = ✂️ Bearbeiten: Ctrl+D (Zeile duplizieren), Ctrl+K (Zeile löschen)
welcome-shortcuts-navigate = 🔍 Navigation: Ctrl+Home/End (Dokument), Home/End (Zeile)
welcome-shortcuts-view = 🎨 Ansicht: Ctrl+H (Hervorhebung umschalten), Ctrl+E (Linting umschalten)
welcome-shortcuts-quit = 🚪 Beenden: Ctrl+Q
welcome-start-editing = Beginnen Sie hier zu bearbeiten...

# Statusmeldungen
status-file-saved = Datei erfolgreich gespeichert!
status-syntax-enabled = Syntaxhervorhebung aktiviert
status-syntax-disabled = Syntaxhervorhebung deaktiviert
status-linting-enabled = Code-Linting aktiviert
status-linting-disabled = Code-Linting deaktiviert
status-new-file-created = Neue Datei erstellt
status-document-start = Dokumentanfang
status-document-end = Dokumentende
status-goto-line-soon = Gehe zu Zeile: (Funktion kommt bald)
status-line-duplicated = Zeile dupliziert
status-line-deleted = Zeile gelöscht
status-line-cleared = Zeile geleert
status-cursor-moved = Cursor bewegt zu Zeile { $row }, Spalte { $col }
status-word-selected = Wort ausgewählt (Doppelklick)
status-text-selected = { $count } Zeichen ausgewählt
status-selecting = Text wird ausgewählt...
status-scrolled-up = Nach oben gescrollt
status-scrolled-down = Nach unten gescrollt

# Kontextmenü
context-menu-copy-cut-paste = Kontextmenü: Kopieren/Ausschneiden/Einfügen verfügbar bei Zeile { $row }, Spalte { $col }
context-menu-paste = Kontextmenü: Einfügen verfügbar bei Zeile { $row }, Spalte { $col }

# Dialogsystem
dialog-open-file = 📂 Datei Öffnen - IOCraft Dateibrowser
dialog-navigate-select = 🎯 Navigieren Sie und wählen Sie eine zu öffnende Datei:
dialog-quick-actions = 🚀 Schnellaktionen:
dialog-action-type-filename = 📝 [1] Dateiname unten eingeben
dialog-action-browse-recent = 📁 [2] Zuletzt verwendete Dateien durchsuchen
dialog-action-create-new = 🆕 [3] Neue Datei erstellen
dialog-action-cancel = ❌ [ESC] Abbrechen und zum Editor zurückkehren
dialog-enter-filename = 📝 Dateiname oder Aktion eingeben [1-3]:
dialog-filename-prompt = 📝 Dateiname:
dialog-new-filename = 🆕 Neuer Dateiname:
dialog-recent-files = 📁 Zuletzt verwendete Dateien:
dialog-select-file = 📝 Datei auswählen (oder Namen eingeben):
dialog-create-choice = 📝 Ihre Wahl:
dialog-open-canceled = Öffnen abgebrochen
dialog-error-reading-directory = Fehler beim Lesen des Verzeichnisses

# Hilfesystem
help-status-message = Ctrl+S: Speichern | Ctrl+O: Öffnen | Ctrl+N: Neu | Ctrl+Q: Beenden | Ctrl+H: Hervorhebung umschalten | Ctrl+E: Linting umschalten | Maus: Klicken zum Cursor bewegen

# UI-Elemente
ui-no-file = [Keine Datei]
ui-plain-text = Klartext
ui-line-count = { $count ->
    [one] { $count } Zeile
   *[other] { $count } Zeilen
}
ui-modified-indicator = ✅
ui-line-prefix = Zeile
ui-column-prefix = Spalte
ui-chars-selected = { $count } Zeichen ausgewählt

# Spracheinstellungen
lang-switch-success = Sprache gewechselt zu { $language }
lang-switch-error = Sprachwechsel fehlgeschlagen: { $error }
lang-not-supported = Sprache '{ $language }' wird nicht unterstützt
lang-current = Aktuelle Sprache: { $language }
lang-available = Verfügbare Sprachen: { $languages }

# Fehlermeldungen
error-invalid-locale = Ungültige Locale: { $locale }
error-locale-not-supported = Locale '{ $locale }' nicht unterstützt
error-file-operation = Dateioperation Fehler: { $error }
error-general = Fehler: { $message }

# Beenden-Eingabeaufforderung
exit-prompt = Drücken Sie Ctrl+C zum Beenden, falls nötig
