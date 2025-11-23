# Démarrage de l'Application
app-title = Éditeur de Texte Interactif IOCraft
app-startup = 🎉 Démarrage de l'Éditeur de Texte Interactif IOCraft...
app-thanks = 👋 Merci d'avoir utilisé l'Éditeur de Texte IOCraft !

# Opérations sur les Fichiers
file-new = 📝 Création d'un nouveau fichier...
file-opening = 📂 Ouverture du fichier : { $filename }
file-loaded = Chargé : { $filename }
file-saved = Sauvegardé : { $filename }
file-error-loading = Erreur lors du chargement de { $filename } : { $error }
file-error-saving = Erreur lors de la sauvegarde de { $filename } : { $error }
file-not-found = ⚠️ Fichier '{ $filename }' non trouvé.
file-create-new = 🆕 Voulez-vous créer un nouveau fichier ? (o/n)
file-creation-canceled = Création de fichier annulée
file-new-ready = Nouveau fichier '{ $filename }' prêt à éditer
file-no-input = Aucune entrée fournie
file-no-file-specified = Aucun fichier spécifié
file-opened-success = ✅ Ouvert : { $filename }
file-new-created = 🆕 Nouveau fichier '{ $filename }' créé - prêt à éditer !
file-unsaved-changes = Le fichier contient des modifications non sauvegardées ! Appuyez à nouveau sur Ctrl+Q pour quitter sans sauvegarder.
file-save-before-new = Sauvegardez le fichier actuel avant d'en créer un nouveau (Ctrl+S)
file-save-before-open = Sauvegardez le fichier actuel avant d'en ouvrir un nouveau (Ctrl+S)

# Contenu de Bienvenue
welcome-title = Bienvenue dans l'Éditeur de Texte Amélioré IOCraft !
welcome-features = Fonctionnalités :
welcome-feature-highlighting = ✨ Beaux numéros de ligne et coloration syntaxique
welcome-feature-cursor = 🎯 Curseur moderne et indicateurs visuels
welcome-feature-mouse = 🖱️ Support complet de la souris (clic, glisser, sélectionner)
welcome-feature-dialogs = 📁 Boîtes de dialogue IOCraft et navigateur de fichiers
welcome-feature-shortcuts = ⌨️ Raccourcis clavier style Sublime Text
welcome-feature-linting = 🔍 Détection de problèmes de code en temps réel
welcome-shortcuts = Raccourcis Clavier :
welcome-shortcuts-file = 📄 Fichier : Ctrl+O (ouvrir), Ctrl+S (sauvegarder), Ctrl+N (nouveau)
welcome-shortcuts-edit = ✂️ Édition : Ctrl+D (dupliquer ligne), Ctrl+K (supprimer ligne)
welcome-shortcuts-navigate = 🔍 Navigation : Ctrl+Home/End (document), Home/End (ligne)
welcome-shortcuts-view = 🎨 Affichage : Ctrl+H (basculer coloration), Ctrl+E (basculer vérification)
welcome-shortcuts-quit = 🚪 Quitter : Ctrl+Q
welcome-start-editing = Commencez à éditer ici...

# Messages d'État
status-file-saved = Fichier sauvegardé avec succès !
status-syntax-enabled = Coloration syntaxique activée
status-syntax-disabled = Coloration syntaxique désactivée
status-linting-enabled = Vérification de code activée
status-linting-disabled = Vérification de code désactivée
status-new-file-created = Nouveau fichier créé
status-document-start = Début du document
status-document-end = Fin du document
status-goto-line-soon = Aller à la ligne : (fonctionnalité bientôt disponible)
status-line-duplicated = Ligne dupliquée
status-line-deleted = Ligne supprimée
status-line-cleared = Ligne effacée
status-cursor-moved = Curseur déplacé à la ligne { $row }, colonne { $col }
status-word-selected = Mot sélectionné (double-clic)
status-text-selected = { $count } caractères sélectionnés
status-selecting = Sélection du texte...
status-scrolled-up = Défilement vers le haut
status-scrolled-down = Défilement vers le bas

# Menu Contextuel
context-menu-copy-cut-paste = Menu contextuel : Copier/Couper/Coller disponible à la ligne { $row }, colonne { $col }
context-menu-paste = Menu contextuel : Coller disponible à la ligne { $row }, colonne { $col }

# Système de Dialogue
dialog-open-file = 📂 Ouvrir Fichier - Navigateur de Fichiers IOCraft
dialog-navigate-select = 🎯 Naviguez et sélectionnez un fichier à ouvrir :
dialog-quick-actions = 🚀 Actions Rapides :
dialog-action-type-filename = 📝 [1] Taper nom de fichier ci-dessous
dialog-action-browse-recent = 📁 [2] Parcourir fichiers récents
dialog-action-create-new = 🆕 [3] Créer nouveau fichier
dialog-action-cancel = ❌ [ESC] Annuler et retourner à l'éditeur
dialog-enter-filename = 📝 Entrez nom de fichier ou action [1-3] :
dialog-filename-prompt = 📝 Nom de fichier :
dialog-new-filename = 🆕 Nouveau nom de fichier :
dialog-recent-files = 📁 Fichiers Récents :
dialog-select-file = 📝 Sélectionnez fichier (ou tapez nom) :
dialog-create-choice = 📝 Votre choix :
dialog-open-canceled = Ouverture annulée
dialog-error-reading-directory = Erreur de lecture du répertoire

# Système d'Aide
help-status-message = Ctrl+S : Sauvegarder | Ctrl+O : Ouvrir | Ctrl+N : Nouveau | Ctrl+Q : Quitter | Ctrl+H : Basculer coloration | Ctrl+E : Basculer vérification | Souris : Clic pour déplacer curseur

# Éléments d'Interface
ui-no-file = [Aucun fichier]
ui-plain-text = Texte Brut
ui-line-count = { $count ->
    [one] { $count } ligne
   *[other] { $count } lignes
}
ui-modified-indicator = ✅
ui-line-prefix = Ln
ui-column-prefix = Col
ui-chars-selected = { $count } caractères sélectionnés

# Paramètres de Langue
lang-switch-success = Langue changée vers { $language }
lang-switch-error = Échec du changement de langue : { $error }
lang-not-supported = La langue '{ $language }' n'est pas supportée
lang-current = Langue actuelle : { $language }
lang-available = Langues disponibles : { $languages }

# Messages d'Erreur
error-invalid-locale = Locale invalide : { $locale }
error-locale-not-supported = Locale '{ $locale }' non supportée
error-file-operation = Erreur d'opération fichier : { $error }
error-general = Erreur : { $message }

# Invite de Sortie
exit-prompt = Appuyez sur Ctrl+C pour quitter si nécessaire
