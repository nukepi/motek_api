// ignore: unused_import
import 'package:intl/intl.dart' as intl;
import 'app_localizations.dart';

// ignore_for_file: type=lint

/// The translations for Polish (`pl`).
class AppLocalizationsPl extends AppLocalizations {
  AppLocalizationsPl([String locale = 'pl']) : super(locale);

  @override
  String get appTitle => 'Motek UI';

  @override
  String version(String version) {
    return 'Wersja $version';
  }

  @override
  String get home => 'Strona główna';

  @override
  String get settings => 'Ustawienia';

  @override
  String get notes => 'Notatki';

  @override
  String get notebooks => 'Notatniki';

  @override
  String get login => 'Logowanie';

  @override
  String get appearance => 'Wygląd';

  @override
  String get darkMode => 'Tryb ciemny';

  @override
  String get darkModeDescription => 'Zmień wygląd aplikacji na ciemny';

  @override
  String get fontSize => 'Rozmiar czcionki';

  @override
  String get notifications => 'Powiadomienia';

  @override
  String get notificationsEnable => 'Włącz lub wyłącz powiadomienia';

  @override
  String get language => 'Język aplikacji';

  @override
  String get account => 'Konto';

  @override
  String get editProfile => 'Edytuj profil';

  @override
  String get changePassword => 'Zmień hasło';

  @override
  String get logout => 'Wyloguj się';

  @override
  String get aboutApp => 'O aplikacji';

  @override
  String get license => 'Licencja';

  @override
  String get copyright => '© 2025 Motek UI';

  @override
  String get email => 'Email';

  @override
  String get password => 'Hasło';

  @override
  String get pleaseEnterEmail => 'Proszę podać email';

  @override
  String get pleaseEnterPassword => 'Proszę podać hasło';

  @override
  String get loginButton => 'Zaloguj się';

  @override
  String get registerPrompt => 'Nie masz konta? Zarejestruj się';

  @override
  String get loggingIn => 'Logowanie...';

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
  String get noNotes => 'Brak notatek';

  @override
  String get noTitle => 'Notatka bez tytułu';

  @override
  String get noContent => 'Brak treści';

  @override
  String get dateUnknown => 'Data nieznana';

  @override
  String get newNote => 'Nowa notatka';

  @override
  String get title => 'Tytuł';

  @override
  String get content => 'Treść';

  @override
  String get cancel => 'Anuluj';

  @override
  String get create => 'Utwórz';

  @override
  String get edit => 'Edytuj notatkę';

  @override
  String get save => 'Zapisz';

  @override
  String get confirmDelete => 'Potwierdź usunięcie';

  @override
  String confirmDeleteNote(String title) {
    return 'Czy na pewno chcesz usunąć notatkę \"$title\"?';
  }

  @override
  String get delete => 'Usuń';

  @override
  String noteDeleted(String title) {
    return 'Usunięto notatkę: $title';
  }

  @override
  String get undoAction => 'Cofnij';

  @override
  String get deleteError => 'Nie udało się usunąć notatki';

  @override
  String error(String message) {
    return 'Błąd: $message';
  }

  @override
  String get tryAgain => 'Spróbuj ponownie';

  @override
  String get noNotebooks => 'Brak notatników';

  @override
  String get newNotebook => 'Nowy notatnik';

  @override
  String get notebookName => 'Nazwa notatnika';

  @override
  String confirmDeleteNotebook(String name) {
    return 'Czy na pewno chcesz usunąć notatnik \"$name\"?';
  }

  @override
  String get notebookDeleteError => 'Nie udało się usunąć notatnika';

  @override
  String get editNotebook => 'Edytuj notatnik';

  @override
  String notebookOpened(String name) {
    return 'Otwarto notatnik: $name';
  }

  @override
  String created(String date) {
    return 'Utworzono: $date';
  }

  @override
  String loadNotesError(String message) {
    return 'Nie udało się załadować notatek: $message';
  }

  @override
  String loadNotebooksError(String message) {
    return 'Nie udało się załadować notatników: $message';
  }
}
