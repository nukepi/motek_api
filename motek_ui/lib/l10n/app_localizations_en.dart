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
  String get logout => 'Log out';

  @override
  String get aboutApp => 'About App';

  @override
  String get license => 'License';

  @override
  String get copyright => '© 2025 Motek UI';

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
  String get logs => 'Logi';

  @override
  String get refresh => 'Odśwież';

  @override
  String get copyToClipboard => 'Kopiuj do schowka';

  @override
  String get logsCopied => 'Logi skopiowane do schowka';

  @override
  String get generateTestLogs => 'Generuj testowe logi';

  @override
  String get noNotes => 'No notes';

  @override
  String get noTitle => 'Untitled Note';

  @override
  String get noContent => 'No content';

  @override
  String get dateUnknown => 'Date unknown';

  @override
  String get newNote => 'New Note';

  @override
  String get title => 'Title';

  @override
  String get content => 'Content';

  @override
  String get cancel => 'Cancel';

  @override
  String get create => 'Create';

  @override
  String get edit => 'Edit Note';

  @override
  String get save => 'Save';

  @override
  String get confirmDelete => 'Confirm Delete';

  @override
  String confirmDeleteNote(String title) {
    return 'Are you sure you want to delete note \"$title\"?';
  }

  @override
  String get delete => 'Delete';

  @override
  String noteDeleted(String title) {
    return 'Deleted note: $title';
  }

  @override
  String get undoAction => 'Undo';

  @override
  String get deleteError => 'Failed to delete note';

  @override
  String error(String message) {
    return 'Error: $message';
  }

  @override
  String get tryAgain => 'Try again';

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
  String created(String date) {
    return 'Created: $date';
  }

  @override
  String loadNotesError(String message) {
    return 'Failed to load notes: $message';
  }

  @override
  String loadNotebooksError(String message) {
    return 'Failed to load notebooks: $message';
  }
}
