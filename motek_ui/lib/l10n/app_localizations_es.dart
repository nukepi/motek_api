// ignore: unused_import
import 'package:intl/intl.dart' as intl;
import 'app_localizations.dart';

// ignore_for_file: type=lint

/// The translations for Spanish Castilian (`es`).
class AppLocalizationsEs extends AppLocalizations {
  AppLocalizationsEs([String locale = 'es']) : super(locale);

  @override
  String get appTitle => 'Motek UI';

  @override
  String version(String version) {
    return 'Versión $version';
  }

  @override
  String get copyright => '© 2025 Motek UI';

  @override
  String get noDescription => 'Sin descripción';

  @override
  String error(Object message) {
    return 'Error: $message';
  }

  @override
  String get tryAgain => 'Intentar de nuevo';

  @override
  String created(String date) {
    return 'Creado: $date';
  }

  @override
  String get cancel => 'Cancelar';

  @override
  String get save => 'Guardar';

  @override
  String get delete => 'Eliminar';

  @override
  String get create => 'Crear';

  @override
  String get edit => 'Editar nota';

  @override
  String get undoAction => 'Deshacer';

  @override
  String get confirmDelete => 'Confirmar eliminación';

  @override
  String get navigationLabels => '--- Etiquetas de navegación ---';

  @override
  String get home => 'Inicio';

  @override
  String get settings => 'Configuración';

  @override
  String get notes => 'Notas';

  @override
  String get notebooks => 'Cuadernos';

  @override
  String get login => 'Iniciar sesión';

  @override
  String get authenticationLabels => '--- Etiquetas de autenticación ---';

  @override
  String get loggedInAs => 'Conectado como';

  @override
  String get confirmLogout => '¿Seguro que quieres cerrar sesión?';

  @override
  String get logoutSuccess => 'Sesión cerrada con éxito';

  @override
  String get email => 'Correo electrónico';

  @override
  String get password => 'Contraseña';

  @override
  String get pleaseEnterEmail => 'Por favor, introduce tu correo electrónico';

  @override
  String get pleaseEnterPassword => 'Por favor, introduce tu contraseña';

  @override
  String get loginButton => 'Iniciar sesión';

  @override
  String get registerPrompt => '¿No tienes cuenta? Regístrate';

  @override
  String get loggingIn => 'Iniciando sesión...';

  @override
  String get logout => 'Cerrar sesión';

  @override
  String get settingsLabels => '--- Etiquetas de configuración ---';

  @override
  String get appearance => 'Apariencia';

  @override
  String get darkMode => 'Modo oscuro';

  @override
  String get darkModeDescription => 'Cambiar la apariencia de la aplicación a oscuro';

  @override
  String get fontSize => 'Tamaño de fuente';

  @override
  String get notifications => 'Notificaciones';

  @override
  String get notificationsEnable => 'Activar o desactivar notificaciones';

  @override
  String get language => 'Idioma de la aplicación';

  @override
  String get account => 'Cuenta';

  @override
  String get editProfile => 'Editar perfil';

  @override
  String get changePassword => 'Cambiar contraseña';

  @override
  String get aboutApp => 'Acerca de la aplicación';

  @override
  String get license => 'Licencia';

  @override
  String get logsLabels => '--- Etiquetas de registros ---';

  @override
  String get logs => 'Registros';

  @override
  String get refresh => 'Actualizar';

  @override
  String get copyToClipboard => 'Copiar al portapapeles';

  @override
  String get logsCopied => 'Registros copiados al portapapeles';

  @override
  String get generateTestLogs => 'Generar registros de prueba';

  @override
  String get register => 'Registrarse';

  @override
  String get confirmPassword => 'Confirmar contraseña';

  @override
  String get passwordMismatch => 'Las contraseñas no coinciden';

  @override
  String get passwordTooShort => 'La contraseña debe tener al menos 6 caracteres';

  @override
  String get registrationSuccess => '¡Registro exitoso! Ahora puedes iniciar sesión.';

  @override
  String get invalidEmail => 'Por favor, introduce una dirección de correo válida';

  @override
  String get welcomeMessage => '¡Bienvenido a Motek UI!';

  @override
  String get testLogs => 'Registros de prueba';

  @override
  String get testLogSaved => 'Registro de prueba guardado en la consola';

  @override
  String get loginRequired => 'Necesitas iniciar sesión para ver las notas';

  @override
  String get settingsSaved => 'Configuración guardada';

  @override
  String get logFilePath => 'Ruta del archivo de registro:';

  @override
  String get logLevel => 'Nivel de registro:';

  @override
  String get loggingSettings => 'Configuración de registro';

  @override
  String get notLoggedIn => 'No conectado';

  @override
  String get notesLabels => '--- Etiquetas de notas ---';

  @override
  String get saveNote => 'Guardar nota';

  @override
  String get editingNote => 'Editando nota';

  @override
  String get formatText => 'Formatear texto';

  @override
  String get boldText => 'Negrita';

  @override
  String get italicText => 'Cursiva';

  @override
  String get underlineText => 'Subrayado';

  @override
  String get bulletList => 'Lista de viñetas';

  @override
  String get numberList => 'Lista numerada';

  @override
  String get insertImage => 'Insertar imagen';

  @override
  String get noNotes => 'Aún no hay notas. ¡Crea tu primera nota!';

  @override
  String get noTitle => 'Nota sin título';

  @override
  String get noContent => 'Sin contenido';

  @override
  String get dateUnknown => 'Fecha desconocida';

  @override
  String get newNote => 'Nueva nota';

  @override
  String get editNote => 'Editar nota';

  @override
  String get title => 'Título';

  @override
  String get content => 'Contenido';

  @override
  String confirmDeleteNote(String title) {
    return '¿Seguro que quieres eliminar la nota \"$title\"?';
  }

  @override
  String noteDeleted(String title) {
    return 'Nota eliminada correctamente';
  }

  @override
  String get deleteError => 'No se pudo eliminar la nota';

  @override
  String loadNotesError(Object error) {
    return 'No se pudieron cargar las notas: $error';
  }

  @override
  String get notebooksLabels => '--- Etiquetas de cuadernos ---';

  @override
  String get noNotebooks => 'Sin cuadernos';

  @override
  String get newNotebook => 'Nuevo cuaderno';

  @override
  String get notebookName => 'Nombre del cuaderno';

  @override
  String confirmDeleteNotebook(String name) {
    return '¿Seguro que quieres eliminar el cuaderno \"$name\"?';
  }

  @override
  String get notebookDeleteError => 'No se pudo eliminar el cuaderno';

  @override
  String get editNotebook => 'Editar cuaderno';

  @override
  String notebookOpened(String name) {
    return 'Cuaderno abierto: $name';
  }

  @override
  String loadNotebooksError(String message) {
    return 'No se pudieron cargar los cuadernos: $message';
  }

  @override
  String notebookDeleted(String name) {
    return 'Cuaderno eliminado: $name';
  }

  @override
  String get untitledNote => 'Notatka bez tytułu';

  @override
  String get retry => 'Spróbuj ponownie';

  @override
  String get deleteNote => 'Eliminar nota';

  @override
  String get deleteNoteConfirmTitle => 'Eliminar nota';

  @override
  String deleteNoteConfirmMessage(Object title) {
    return '¿Seguro que quieres eliminar la nota \"$title\"?';
  }

  @override
  String get deleteNoteFailed => 'No se pudo eliminar la nota';

  @override
  String get sessionExpired => 'Tu sesión ha expirado. Por favor, inicia sesión de nuevo.';

  @override
  String noteCount(num count) {
    String _temp0 = intl.Intl.pluralLogic(
      count,
      locale: localeName,
      other: '# notas',
      one: '# nota',
    );
    return '$_temp0';
  }

  @override
  String get noTags => 'No se encontraron etiquetas';

  @override
  String get unsavedChanges => 'Tienes cambios sin guardar.';

  @override
  String get discardChanges => '¿Quieres descartar tus cambios?';

  @override
  String get discard => 'Descartar';

  @override
  String get noteTitle => 'Título de la nota';

  @override
  String get startTyping => 'Empieza a escribir...';

  @override
  String get notebook => 'Cuaderno';

  @override
  String get noNotebook => 'Sin cuaderno';
}
