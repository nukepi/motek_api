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
  String get copyright => '© 2025 Motek UI';

  @override
  String get noDescription => 'Brak opisu';

  @override
  String error(Object message) {
    return 'Błąd: $message';
  }

  @override
  String get tryAgain => 'Spróbuj ponownie';

  @override
  String created(String date) {
    return 'Utworzono: $date';
  }

  @override
  String get cancel => 'Anuluj';

  @override
  String get save => 'Zapisz';

  @override
  String get delete => 'Usuń';

  @override
  String get create => 'Utwórz';

  @override
  String get edit => 'Edytuj notatkę';

  @override
  String get undoAction => 'Cofnij';

  @override
  String get confirmDelete => 'Potwierdź usunięcie';

  @override
  String get navigationLabels => '--- Etykiety nawigacji ---';

  @override
  String get home => 'Strona główna';

  @override
  String get settings => 'Ustawienia';

  @override
  String get notes => 'Notatki';

  @override
  String get notebooks => 'Notatniki';

  @override
  String get login => 'Zaloguj się';

  @override
  String get authenticationLabels => '--- Etykiety uwierzytelniania ---';

  @override
  String get loggedInAs => 'Zalogowano jako';

  @override
  String get confirmLogout => 'Czy na pewno chcesz się wylogować?';

  @override
  String get logoutSuccess => 'Wylogowano pomyślnie';

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
  String get logout => 'Wyloguj się';

  @override
  String get settingsLabels => '--- Etykiety ustawień ---';

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
  String get aboutApp => 'O aplikacji';

  @override
  String get license => 'Licencja';

  @override
  String get logsLabels => '--- Etykiety logów ---';

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
  String get register => 'Rejestracja';

  @override
  String get confirmPassword => 'Potwierdź hasło';

  @override
  String get passwordMismatch => 'Hasła nie są identyczne';

  @override
  String get passwordTooShort => 'Hasło musi mieć co najmniej 6 znaków';

  @override
  String get registrationSuccess => 'Rejestracja udana! Możesz się teraz zalogować.';

  @override
  String get invalidEmail => 'Proszę podać poprawny adres email';

  @override
  String get welcomeMessage => 'Witaj w aplikacji Motek UI!';

  @override
  String get testLogs => 'Testuj logi';

  @override
  String get testLogSaved => 'Test log zapisany w konsoli';

  @override
  String get loginRequired => 'Musisz być zalogowany, aby przeglądać notatki';

  @override
  String get settingsSaved => 'Ustawienia zapisane';

  @override
  String get logFilePath => 'Ścieżka do pliku logów:';

  @override
  String get logLevel => 'Poziom logowania:';

  @override
  String get loggingSettings => 'Ustawienia logowania';

  @override
  String get notLoggedIn => 'Niezalogowany';

  @override
  String get notesLabels => '--- Etykiety notatek ---';

  @override
  String get saveNote => 'Zapisz notatkę';

  @override
  String get editingNote => 'Edycja notatki';

  @override
  String get formatText => 'Formatuj tekst';

  @override
  String get boldText => 'Pogrubienie';

  @override
  String get italicText => 'Kursywa';

  @override
  String get underlineText => 'Podkreślenie';

  @override
  String get bulletList => 'Lista punktowana';

  @override
  String get numberList => 'Lista numerowana';

  @override
  String get insertImage => 'Wstaw obraz';

  @override
  String get noNotes => 'Brak notatek. Utwórz swoją pierwszą notatkę!';

  @override
  String get noTitle => 'Notatka bez tytułu';

  @override
  String get noContent => 'Brak treści';

  @override
  String get dateUnknown => 'Data nieznana';

  @override
  String get newNote => 'Nowa notatka';

  @override
  String get editNote => 'Edytuj notatkę';

  @override
  String get title => 'Tytuł';

  @override
  String get content => 'Treść';

  @override
  String confirmDeleteNote(String title) {
    return 'Czy na pewno chcesz usunąć notatkę \"$title\"?';
  }

  @override
  String noteDeleted(String title) {
    return 'Notatka została pomyślnie usunięta';
  }

  @override
  String get deleteError => 'Nie udało się usunąć notatki';

  @override
  String loadNotesError(Object error) {
    return 'Nie udało się załadować notatek: $error';
  }

  @override
  String get notebooksLabels => '--- Etykiety notatników ---';

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
  String loadNotebooksError(String message) {
    return 'Nie udało się załadować notatników: $message';
  }

  @override
  String notebookDeleted(String name) {
    return 'Usunięto notatnik: $name';
  }

  @override
  String get untitledNote => 'Notatka bez tytułu';

  @override
  String get retry => 'Spróbuj ponownie';

  @override
  String get deleteNote => 'Usuń notatkę';

  @override
  String get deleteNoteConfirmTitle => 'Usuń notatkę';

  @override
  String deleteNoteConfirmMessage(Object title) {
    return 'Czy na pewno chcesz usunąć notatkę \"$title\"?';
  }

  @override
  String get deleteNoteFailed => 'Nie udało się usunąć notatki';

  @override
  String get sessionExpired => 'Twoja sesja wygasła. Zaloguj się ponownie.';

  @override
  String noteCount(num count) {
    String _temp0 = intl.Intl.pluralLogic(
      count,
      locale: localeName,
      other: '# notatek',
      many: '# notatek',
      few: '# notatki',
      one: '# notatka',
    );
    return '$_temp0';
  }

  @override
  String get noTags => 'Brak tagów';

  @override
  String get unsavedChanges => 'Masz niezapisane zmiany.';

  @override
  String get discardChanges => 'Czy chcesz porzucić zmiany?';

  @override
  String get discard => 'Porzuć';

  @override
  String get noteTitle => 'Tytuł notatki';

  @override
  String get startTyping => 'Zacznij pisać...';

  @override
  String get notebook => 'Notatnik';

  @override
  String get noNotebook => 'Brak notatnika';
}
