// ignore: unused_import
import 'package:intl/intl.dart' as intl;
import 'app_localizations.dart';

// ignore_for_file: type=lint

/// The translations for English (`en`).
class AppLocalizationsEn extends AppLocalizations {
  AppLocalizationsEn([String locale = 'en']) : super(locale);

  @override
  String get appTitle => 'Motek UI';

  @override
  String version(String version) {
    return 'Version $version';
  }

  @override
  String get copyright => '© 2025 Motek UI';

  @override
  String get noDescription => 'No description';

  @override
  String error(Object message) {
    return 'Error: $message';
  }

  @override
  String get tryAgain => 'Try Again';

  @override
  String created(String date) {
    return 'Created: $date';
  }

  @override
  String get cancel => 'Cancel';

  @override
  String get save => 'Save';

  @override
  String get delete => 'Delete';

  @override
  String get create => 'Create';

  @override
  String get edit => 'Edit Note';

  @override
  String get undoAction => 'Undo';

  @override
  String get confirmDelete => 'Confirm Delete';

  @override
  String get navigationLabels => '--- Navigation Labels ---';

  @override
  String get home => 'Home';

  @override
  String get settings => 'Settings';

  @override
  String get notes => 'Notes';

  @override
  String get notebooks => 'Notebooks';

  @override
  String get login => 'Login';

  @override
  String get authenticationLabels => '--- Authentication Labels ---';

  @override
  String get loggedInAs => 'Logged in as';

  @override
  String get confirmLogout => 'Are you sure you want to log out?';

  @override
  String get logoutSuccess => 'Successfully logged out';

  @override
  String get email => 'Email';

  @override
  String get password => 'Password';

  @override
  String get pleaseEnterEmail => 'Please enter your email';

  @override
  String get pleaseEnterPassword => 'Please enter your password';

  @override
  String get loginButton => 'Log in';

  @override
  String get registerPrompt => 'Don\'t have an account? Register';

  @override
  String get loggingIn => 'Logging in...';

  @override
  String get logout => 'Log out';

  @override
  String get settingsLabels => '--- Settings Labels ---';

  @override
  String get appearance => 'Appearance';

  @override
  String get darkMode => 'Dark Mode';

  @override
  String get darkModeDescription => 'Change app appearance to dark';

  @override
  String get fontSize => 'Font Size';

  @override
  String get notifications => 'Notifications';

  @override
  String get notificationsEnable => 'Enable or disable notifications';

  @override
  String get language => 'App Language';

  @override
  String get account => 'Account';

  @override
  String get editProfile => 'Edit Profile';

  @override
  String get changePassword => 'Change Password';

  @override
  String get aboutApp => 'About App';

  @override
  String get license => 'License';

  @override
  String get logsLabels => '--- Logs Labels ---';

  @override
  String get logs => 'Logs';

  @override
  String get refresh => 'Refresh';

  @override
  String get copyToClipboard => 'Copy to clipboard';

  @override
  String get logsCopied => 'Logs copied to clipboard';

  @override
  String get generateTestLogs => 'Generate test logs';

  @override
  String get register => 'Register';

  @override
  String get confirmPassword => 'Confirm password';

  @override
  String get passwordMismatch => 'Passwords do not match';

  @override
  String get passwordTooShort => 'Password must be at least 6 characters long';

  @override
  String get registrationSuccess => 'Registration successful! You can now log in.';

  @override
  String get invalidEmail => 'Please enter a valid email address';

  @override
  String get welcomeMessage => 'Welcome to Motek UI!';

  @override
  String get testLogs => 'Test logs';

  @override
  String get testLogSaved => 'Test log saved to console';

  @override
  String get loginRequired => 'You need to be logged in to view notes';

  @override
  String get settingsSaved => 'Settings saved';

  @override
  String get logFilePath => 'Log file path:';

  @override
  String get logLevel => 'Log level:';

  @override
  String get loggingSettings => 'Logging settings';

  @override
  String get notLoggedIn => 'Not logged in';

  @override
  String get notesLabels => '--- Notes Labels ---';

  @override
  String get saveNote => 'Save Note';

  @override
  String get editingNote => 'Editing Note';

  @override
  String get formatText => 'Format Text';

  @override
  String get boldText => 'Bold';

  @override
  String get italicText => 'Italic';

  @override
  String get underlineText => 'Underline';

  @override
  String get bulletList => 'Bullet List';

  @override
  String get numberList => 'Numbered List';

  @override
  String get insertImage => 'Insert Image';

  @override
  String get noNotes => 'No notes yet. Create your first note!';

  @override
  String get noTitle => 'Untitled Note';

  @override
  String get noContent => 'No content';

  @override
  String get dateUnknown => 'Date unknown';

  @override
  String get newNote => 'New Note';

  @override
  String get editNote => 'Edit Note';

  @override
  String get title => 'Title';

  @override
  String get content => 'Content';

  @override
  String confirmDeleteNote(String title) {
    return 'Are you sure you want to delete note \"$title\"?';
  }

  @override
  String noteDeleted(String title) {
    return 'Note deleted successfully';
  }

  @override
  String get deleteError => 'Failed to delete note';

  @override
  String loadNotesError(Object error) {
    return 'Failed to load notes: $error';
  }

  @override
  String get notebooksLabels => '--- Notebooks Labels ---';

  @override
  String get noNotebooks => 'No notebooks';

  @override
  String get newNotebook => 'New Notebook';

  @override
  String get notebookName => 'Notebook name';

  @override
  String confirmDeleteNotebook(String name) {
    return 'Are you sure you want to delete notebook \"$name\"?';
  }

  @override
  String get notebookDeleteError => 'Failed to delete notebook';

  @override
  String get editNotebook => 'Edit Notebook';

  @override
  String notebookOpened(String name) {
    return 'Opened notebook: $name';
  }

  @override
  String loadNotebooksError(String message) {
    return 'Failed to load notebooks: $message';
  }

  @override
  String notebookDeleted(String name) {
    return 'Notebook deleted: $name';
  }

  @override
  String get untitledNote => 'Notatka bez tytułu';

  @override
  String get retry => 'Spróbuj ponownie';

  @override
  String get deleteNote => 'Delete note';

  @override
  String get deleteNoteConfirmTitle => 'Delete Note';

  @override
  String deleteNoteConfirmMessage(Object title) {
    return 'Are you sure you want to delete note \"$title\"?';
  }

  @override
  String get deleteNoteFailed => 'Failed to delete note';

  @override
  String get sessionExpired => 'Your session has expired. Please log in again.';

  @override
  String noteCount(num count) {
    String _temp0 = intl.Intl.pluralLogic(
      count,
      locale: localeName,
      other: '# notes',
      one: '# note',
    );
    return '$_temp0';
  }

  @override
  String get noTags => 'No tags found';

  @override
  String get unsavedChanges => 'You have unsaved changes.';

  @override
  String get discardChanges => 'Do you want to discard your changes?';

  @override
  String get discard => 'Discard';

  @override
  String get noteTitle => 'Note title';

  @override
  String get startTyping => 'Start typing...';

  @override
  String get notebook => 'Notebook';

  @override
  String get noNotebook => 'No notebook';
}
