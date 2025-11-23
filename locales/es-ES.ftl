# Inicio de la Aplicación
app-title = Editor de Texto Interactivo IOCraft
app-startup = 🎉 Iniciando Editor de Texto Interactivo IOCraft...
app-thanks = 👋 ¡Gracias por usar el Editor de Texto IOCraft!

# Operaciones de Archivo
file-new = 📝 Creando nuevo archivo...
file-opening = 📂 Abriendo archivo: { $filename }
file-loaded = Cargado: { $filename }
file-saved = Guardado: { $filename }
file-error-loading = Error cargando { $filename }: { $error }
file-error-saving = Error guardando { $filename }: { $error }
file-not-found = ⚠️ Archivo '{ $filename }' no encontrado.
file-create-new = 🆕 ¿Deseas crear un nuevo archivo? (s/n)
file-creation-canceled = Creación de archivo cancelada
file-new-ready = Nuevo archivo '{ $filename }' listo para editar
file-no-input = No se proporcionó entrada
file-no-file-specified = No se especificó archivo
file-opened-success = ✅ Abierto: { $filename }
file-new-created = 🆕 Nuevo archivo '{ $filename }' creado - ¡listo para editar!
file-unsaved-changes = ¡El archivo tiene cambios sin guardar! Presiona Ctrl+Q de nuevo para salir sin guardar.
file-save-before-new = Guarda el archivo actual antes de crear uno nuevo (Ctrl+S)
file-save-before-open = Guarda el archivo actual antes de abrir uno nuevo (Ctrl+S)

# Contenido de Bienvenida
welcome-title = ¡Bienvenido al Editor de Texto Mejorado IOCraft!
welcome-features = Características:
welcome-feature-highlighting = ✨ Hermosos números de línea y resaltado de sintaxis
welcome-feature-cursor = 🎯 Cursor moderno e indicadores visuales
welcome-feature-mouse = 🖱️ Soporte completo para ratón (clic, arrastrar, seleccionar)
welcome-feature-dialogs = 📁 Diálogos de archivo IOCraft y navegador
welcome-feature-shortcuts = ⌨️ Atajos de teclado estilo Sublime Text
welcome-feature-linting = 🔍 Detección de problemas de código en tiempo real
welcome-shortcuts = Atajos de Teclado:
welcome-shortcuts-file = 📄 Archivo: Ctrl+O (abrir), Ctrl+S (guardar), Ctrl+N (nuevo)
welcome-shortcuts-edit = ✂️ Editar: Ctrl+D (duplicar línea), Ctrl+K (eliminar línea)
welcome-shortcuts-navigate = 🔍 Navegar: Ctrl+Home/End (documento), Home/End (línea)
welcome-shortcuts-view = 🎨 Vista: Ctrl+H (alternar resaltado), Ctrl+E (alternar análisis)
welcome-shortcuts-quit = 🚪 Salir: Ctrl+Q
welcome-start-editing = Comienza a editar aquí...

# Mensajes de Estado
status-file-saved = ¡Archivo guardado exitosamente!
status-syntax-enabled = Resaltado de sintaxis habilitado
status-syntax-disabled = Resaltado de sintaxis deshabilitado
status-linting-enabled = Análisis de código habilitado
status-linting-disabled = Análisis de código deshabilitado
status-new-file-created = Nuevo archivo creado
status-document-start = Inicio del documento
status-document-end = Fin del documento
status-goto-line-soon = Ir a línea: (función próximamente)
status-line-duplicated = Línea duplicada
status-line-deleted = Línea eliminada
status-line-cleared = Línea limpiada
status-cursor-moved = Cursor movido a fila { $row }, columna { $col }
status-word-selected = Palabra seleccionada (doble clic)
status-text-selected = Seleccionados { $count } caracteres
status-selecting = Seleccionando texto...
status-scrolled-up = Desplazado hacia arriba
status-scrolled-down = Desplazado hacia abajo

# Menú Contextual
context-menu-copy-cut-paste = Menú contextual: Copiar/Cortar/Pegar disponible en fila { $row }, columna { $col }
context-menu-paste = Menú contextual: Pegar disponible en fila { $row }, columna { $col }

# Sistema de Diálogo
dialog-open-file = 📂 Abrir Archivo - Explorador de Archivos IOCraft
dialog-navigate-select = 🎯 Navega y selecciona un archivo para abrir:
dialog-quick-actions = 🚀 Acciones Rápidas:
dialog-action-type-filename = 📝 [1] Escribir nombre de archivo abajo
dialog-action-browse-recent = 📁 [2] Explorar archivos recientes
dialog-action-create-new = 🆕 [3] Crear nuevo archivo
dialog-action-cancel = ❌ [ESC] Cancelar y volver al editor
dialog-enter-filename = 📝 Ingresa nombre de archivo o acción [1-3]:
dialog-filename-prompt = 📝 Nombre de archivo:
dialog-new-filename = 🆕 Nuevo nombre de archivo:
dialog-recent-files = 📁 Archivos Recientes:
dialog-select-file = 📝 Selecciona archivo (o escribe nombre):
dialog-create-choice = 📝 Tu elección:
dialog-open-canceled = Apertura cancelada
dialog-error-reading-directory = Error leyendo directorio

# Sistema de Ayuda
help-status-message = Ctrl+S: Guardar | Ctrl+O: Abrir | Ctrl+N: Nuevo | Ctrl+Q: Salir | Ctrl+H: Alternar resaltado | Ctrl+E: Alternar análisis | Ratón: Clic para mover cursor

# Elementos de UI
ui-no-file = [Sin archivo]
ui-plain-text = Texto Plano
ui-line-count = { $count ->
    [one] { $count } línea
   *[other] { $count } líneas
}
ui-modified-indicator = ✅
ui-line-prefix = Ln
ui-column-prefix = Col
ui-chars-selected = { $count } caracteres seleccionados

# Configuración de Idioma
lang-switch-success = Idioma cambiado a { $language }
lang-switch-error = Error al cambiar idioma: { $error }
lang-not-supported = Idioma '{ $language }' no es compatible
lang-current = Idioma actual: { $language }
lang-available = Idiomas disponibles: { $languages }

# Mensajes de Error
error-invalid-locale = Configuración regional inválida: { $locale }
error-locale-not-supported = Configuración regional '{ $locale }' no compatible
error-file-operation = Error de operación de archivo: { $error }
error-general = Error: { $message }

# Prompt de Salida
exit-prompt = Presiona Ctrl+C para salir si es necesario
