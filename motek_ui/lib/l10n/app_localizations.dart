import 'dart:async';

import 'package:flutter/foundation.dart';
import 'package:flutter/widgets.dart';
import 'package:flutter_localizations/flutter_localizations.dart';
import 'package:intl/intl.dart' as intl;

import 'app_localizations_en.dart';
import 'app_localizations_es.dart';
import 'app_localizations_pl.dart';

// ignore_for_file: type=lint

/// Callers can lookup localized strings with an instance of AppLocalizations
/// returned by `AppLocalizations.of(context)`.
///
/// Applications need to include `AppLocalizations.delegate()` in their app's
/// `localizationDelegates` list, and the locales they support in the app's
/// `supportedLocales` list. For example:
///
/// ```dart
/// import 'l10n/app_localizations.dart';
///
/// return MaterialApp(
///   localizationsDelegates: AppLocalizations.localizationsDelegates,
///   supportedLocales: AppLocalizations.supportedLocales,
///   home: MyApplicationHome(),
/// );
/// ```
///
/// ## Update pubspec.yaml
///
/// Please make sure to update your pubspec.yaml to include the following
/// packages:
///
/// ```yaml
/// dependencies:
///   # Internationalization support.
///   flutter_localizations:
///     sdk: flutter
///   intl: any # Use the pinned version from flutter_localizations
///
///   # Rest of dependencies
/// ```
///
/// ## iOS Applications
///
/// iOS applications define key application metadata, including supported
/// locales, in an Info.plist file that is built into the application bundle.
/// To configure the locales supported by your app, you’ll need to edit this
/// file.
///
/// First, open your project’s ios/Runner.xcworkspace Xcode workspace file.
/// Then, in the Project Navigator, open the Info.plist file under the Runner
/// project’s Runner folder.
///
/// Next, select the Information Property List item, select Add Item from the
/// Editor menu, then select Localizations from the pop-up menu.
///
/// Select and expand the newly-created Localizations item then, for each
/// locale your application supports, add a new item and select the locale
/// you wish to add from the pop-up menu in the Value field. This list should
/// be consistent with the languages listed in the AppLocalizations.supportedLocales
/// property.
abstract class AppLocalizations {
  AppLocalizations(String locale) : localeName = intl.Intl.canonicalizedLocale(locale.toString());

  final String localeName;

  static AppLocalizations? of(BuildContext context) {
    return Localizations.of<AppLocalizations>(context, AppLocalizations);
  }

  static const LocalizationsDelegate<AppLocalizations> delegate = _AppLocalizationsDelegate();

  /// A list of this localizations delegate along with the default localizations
  /// delegates.
  ///
  /// Returns a list of localizations delegates containing this delegate along with
  /// GlobalMaterialLocalizations.delegate, GlobalCupertinoLocalizations.delegate,
  /// and GlobalWidgetsLocalizations.delegate.
  ///
  /// Additional delegates can be added by appending to this list in
  /// MaterialApp. This list does not have to be used at all if a custom list
  /// of delegates is preferred or required.
  static const List<LocalizationsDelegate<dynamic>> localizationsDelegates = <LocalizationsDelegate<dynamic>>[
    delegate,
    GlobalMaterialLocalizations.delegate,
    GlobalCupertinoLocalizations.delegate,
    GlobalWidgetsLocalizations.delegate,
  ];

  /// A list of this localizations delegate's supported locales.
  static const List<Locale> supportedLocales = <Locale>[
    Locale('en'),
    Locale('es'),
    Locale('pl')
  ];

  /// No description provided for @appTitle.
  ///
  /// In pl, this message translates to:
  /// **'Motek UI'**
  String get appTitle;

  /// Wersja aplikacji
  ///
  /// In pl, this message translates to:
  /// **'Wersja {version}'**
  String version(String version);

  /// No description provided for @copyright.
  ///
  /// In pl, this message translates to:
  /// **'© 2025 Motek UI'**
  String get copyright;

  /// No description provided for @noDescription.
  ///
  /// In pl, this message translates to:
  /// **'Brak opisu'**
  String get noDescription;

  /// No description provided for @error.
  ///
  /// In pl, this message translates to:
  /// **'Błąd: {message}'**
  String error(Object message);

  /// No description provided for @tryAgain.
  ///
  /// In pl, this message translates to:
  /// **'Spróbuj ponownie'**
  String get tryAgain;

  /// Data utworzenia
  ///
  /// In pl, this message translates to:
  /// **'Utworzono: {date}'**
  String created(String date);

  /// No description provided for @cancel.
  ///
  /// In pl, this message translates to:
  /// **'Anuluj'**
  String get cancel;

  /// No description provided for @save.
  ///
  /// In pl, this message translates to:
  /// **'Zapisz'**
  String get save;

  /// No description provided for @delete.
  ///
  /// In pl, this message translates to:
  /// **'Usuń'**
  String get delete;

  /// No description provided for @create.
  ///
  /// In pl, this message translates to:
  /// **'Utwórz'**
  String get create;

  /// No description provided for @edit.
  ///
  /// In pl, this message translates to:
  /// **'Edytuj notatkę'**
  String get edit;

  /// No description provided for @undoAction.
  ///
  /// In pl, this message translates to:
  /// **'Cofnij'**
  String get undoAction;

  /// No description provided for @confirmDelete.
  ///
  /// In pl, this message translates to:
  /// **'Potwierdź usunięcie'**
  String get confirmDelete;

  /// No description provided for @navigationLabels.
  ///
  /// In pl, this message translates to:
  /// **'--- Etykiety nawigacji ---'**
  String get navigationLabels;

  /// No description provided for @home.
  ///
  /// In pl, this message translates to:
  /// **'Strona główna'**
  String get home;

  /// No description provided for @settings.
  ///
  /// In pl, this message translates to:
  /// **'Ustawienia'**
  String get settings;

  /// No description provided for @notes.
  ///
  /// In pl, this message translates to:
  /// **'Notatki'**
  String get notes;

  /// No description provided for @notebooks.
  ///
  /// In pl, this message translates to:
  /// **'Notatniki'**
  String get notebooks;

  /// No description provided for @login.
  ///
  /// In pl, this message translates to:
  /// **'Zaloguj się'**
  String get login;

  /// No description provided for @authenticationLabels.
  ///
  /// In pl, this message translates to:
  /// **'--- Etykiety uwierzytelniania ---'**
  String get authenticationLabels;

  /// No description provided for @loggedInAs.
  ///
  /// In pl, this message translates to:
  /// **'Zalogowano jako'**
  String get loggedInAs;

  /// No description provided for @confirmLogout.
  ///
  /// In pl, this message translates to:
  /// **'Czy na pewno chcesz się wylogować?'**
  String get confirmLogout;

  /// No description provided for @logoutSuccess.
  ///
  /// In pl, this message translates to:
  /// **'Wylogowano pomyślnie'**
  String get logoutSuccess;

  /// No description provided for @email.
  ///
  /// In pl, this message translates to:
  /// **'Email'**
  String get email;

  /// No description provided for @password.
  ///
  /// In pl, this message translates to:
  /// **'Hasło'**
  String get password;

  /// No description provided for @pleaseEnterEmail.
  ///
  /// In pl, this message translates to:
  /// **'Proszę podać email'**
  String get pleaseEnterEmail;

  /// No description provided for @pleaseEnterPassword.
  ///
  /// In pl, this message translates to:
  /// **'Proszę podać hasło'**
  String get pleaseEnterPassword;

  /// No description provided for @loginButton.
  ///
  /// In pl, this message translates to:
  /// **'Zaloguj się'**
  String get loginButton;

  /// No description provided for @registerPrompt.
  ///
  /// In pl, this message translates to:
  /// **'Nie masz konta? Zarejestruj się'**
  String get registerPrompt;

  /// No description provided for @loggingIn.
  ///
  /// In pl, this message translates to:
  /// **'Logowanie...'**
  String get loggingIn;

  /// No description provided for @logout.
  ///
  /// In pl, this message translates to:
  /// **'Wyloguj się'**
  String get logout;

  /// No description provided for @settingsLabels.
  ///
  /// In pl, this message translates to:
  /// **'--- Etykiety ustawień ---'**
  String get settingsLabels;

  /// No description provided for @appearance.
  ///
  /// In pl, this message translates to:
  /// **'Wygląd'**
  String get appearance;

  /// No description provided for @darkMode.
  ///
  /// In pl, this message translates to:
  /// **'Tryb ciemny'**
  String get darkMode;

  /// No description provided for @darkModeDescription.
  ///
  /// In pl, this message translates to:
  /// **'Zmień wygląd aplikacji na ciemny'**
  String get darkModeDescription;

  /// No description provided for @fontSize.
  ///
  /// In pl, this message translates to:
  /// **'Rozmiar czcionki'**
  String get fontSize;

  /// No description provided for @notifications.
  ///
  /// In pl, this message translates to:
  /// **'Powiadomienia'**
  String get notifications;

  /// No description provided for @notificationsEnable.
  ///
  /// In pl, this message translates to:
  /// **'Włącz lub wyłącz powiadomienia'**
  String get notificationsEnable;

  /// No description provided for @language.
  ///
  /// In pl, this message translates to:
  /// **'Język aplikacji'**
  String get language;

  /// No description provided for @account.
  ///
  /// In pl, this message translates to:
  /// **'Konto'**
  String get account;

  /// No description provided for @editProfile.
  ///
  /// In pl, this message translates to:
  /// **'Edytuj profil'**
  String get editProfile;

  /// No description provided for @changePassword.
  ///
  /// In pl, this message translates to:
  /// **'Zmień hasło'**
  String get changePassword;

  /// No description provided for @aboutApp.
  ///
  /// In pl, this message translates to:
  /// **'O aplikacji'**
  String get aboutApp;

  /// No description provided for @license.
  ///
  /// In pl, this message translates to:
  /// **'Licencja'**
  String get license;

  /// No description provided for @logsLabels.
  ///
  /// In pl, this message translates to:
  /// **'--- Etykiety logów ---'**
  String get logsLabels;

  /// No description provided for @logs.
  ///
  /// In pl, this message translates to:
  /// **'Logi'**
  String get logs;

  /// No description provided for @refresh.
  ///
  /// In pl, this message translates to:
  /// **'Odśwież'**
  String get refresh;

  /// No description provided for @copyToClipboard.
  ///
  /// In pl, this message translates to:
  /// **'Kopiuj do schowka'**
  String get copyToClipboard;

  /// No description provided for @logsCopied.
  ///
  /// In pl, this message translates to:
  /// **'Logi skopiowane do schowka'**
  String get logsCopied;

  /// No description provided for @generateTestLogs.
  ///
  /// In pl, this message translates to:
  /// **'Generuj testowe logi'**
  String get generateTestLogs;

  /// No description provided for @register.
  ///
  /// In pl, this message translates to:
  /// **'Rejestracja'**
  String get register;

  /// No description provided for @confirmPassword.
  ///
  /// In pl, this message translates to:
  /// **'Potwierdź hasło'**
  String get confirmPassword;

  /// No description provided for @passwordMismatch.
  ///
  /// In pl, this message translates to:
  /// **'Hasła nie są identyczne'**
  String get passwordMismatch;

  /// No description provided for @passwordTooShort.
  ///
  /// In pl, this message translates to:
  /// **'Hasło musi mieć co najmniej 6 znaków'**
  String get passwordTooShort;

  /// No description provided for @registrationSuccess.
  ///
  /// In pl, this message translates to:
  /// **'Rejestracja udana! Możesz się teraz zalogować.'**
  String get registrationSuccess;

  /// No description provided for @invalidEmail.
  ///
  /// In pl, this message translates to:
  /// **'Proszę podać poprawny adres email'**
  String get invalidEmail;

  /// No description provided for @welcomeMessage.
  ///
  /// In pl, this message translates to:
  /// **'Witaj w aplikacji Motek UI!'**
  String get welcomeMessage;

  /// No description provided for @testLogs.
  ///
  /// In pl, this message translates to:
  /// **'Testuj logi'**
  String get testLogs;

  /// No description provided for @testLogSaved.
  ///
  /// In pl, this message translates to:
  /// **'Test log zapisany w konsoli'**
  String get testLogSaved;

  /// No description provided for @loginRequired.
  ///
  /// In pl, this message translates to:
  /// **'Musisz być zalogowany, aby przeglądać notatki'**
  String get loginRequired;

  /// No description provided for @settingsSaved.
  ///
  /// In pl, this message translates to:
  /// **'Ustawienia zapisane'**
  String get settingsSaved;

  /// No description provided for @logFilePath.
  ///
  /// In pl, this message translates to:
  /// **'Ścieżka do pliku logów:'**
  String get logFilePath;

  /// No description provided for @logLevel.
  ///
  /// In pl, this message translates to:
  /// **'Poziom logowania:'**
  String get logLevel;

  /// No description provided for @loggingSettings.
  ///
  /// In pl, this message translates to:
  /// **'Ustawienia logowania'**
  String get loggingSettings;

  /// No description provided for @notLoggedIn.
  ///
  /// In pl, this message translates to:
  /// **'Niezalogowany'**
  String get notLoggedIn;

  /// No description provided for @notesLabels.
  ///
  /// In pl, this message translates to:
  /// **'--- Etykiety notatek ---'**
  String get notesLabels;

  /// No description provided for @saveNote.
  ///
  /// In pl, this message translates to:
  /// **'Zapisz notatkę'**
  String get saveNote;

  /// No description provided for @editingNote.
  ///
  /// In pl, this message translates to:
  /// **'Edycja notatki'**
  String get editingNote;

  /// No description provided for @formatText.
  ///
  /// In pl, this message translates to:
  /// **'Formatuj tekst'**
  String get formatText;

  /// No description provided for @boldText.
  ///
  /// In pl, this message translates to:
  /// **'Pogrubienie'**
  String get boldText;

  /// No description provided for @italicText.
  ///
  /// In pl, this message translates to:
  /// **'Kursywa'**
  String get italicText;

  /// No description provided for @underlineText.
  ///
  /// In pl, this message translates to:
  /// **'Podkreślenie'**
  String get underlineText;

  /// No description provided for @bulletList.
  ///
  /// In pl, this message translates to:
  /// **'Lista punktowana'**
  String get bulletList;

  /// No description provided for @numberList.
  ///
  /// In pl, this message translates to:
  /// **'Lista numerowana'**
  String get numberList;

  /// No description provided for @insertImage.
  ///
  /// In pl, this message translates to:
  /// **'Wstaw obraz'**
  String get insertImage;

  /// No description provided for @noNotes.
  ///
  /// In pl, this message translates to:
  /// **'Brak notatek. Utwórz swoją pierwszą notatkę!'**
  String get noNotes;

  /// No description provided for @noTitle.
  ///
  /// In pl, this message translates to:
  /// **'Notatka bez tytułu'**
  String get noTitle;

  /// No description provided for @noContent.
  ///
  /// In pl, this message translates to:
  /// **'Brak treści'**
  String get noContent;

  /// No description provided for @dateUnknown.
  ///
  /// In pl, this message translates to:
  /// **'Data nieznana'**
  String get dateUnknown;

  /// No description provided for @newNote.
  ///
  /// In pl, this message translates to:
  /// **'Nowa notatka'**
  String get newNote;

  /// No description provided for @editNote.
  ///
  /// In pl, this message translates to:
  /// **'Edytuj notatkę'**
  String get editNote;

  /// No description provided for @title.
  ///
  /// In pl, this message translates to:
  /// **'Tytuł'**
  String get title;

  /// No description provided for @content.
  ///
  /// In pl, this message translates to:
  /// **'Treść'**
  String get content;

  /// Potwierdzenie usunięcia notatki
  ///
  /// In pl, this message translates to:
  /// **'Czy na pewno chcesz usunąć notatkę \"{title}\"?'**
  String confirmDeleteNote(String title);

  /// Komunikat po usunięciu notatki
  ///
  /// In pl, this message translates to:
  /// **'Notatka została pomyślnie usunięta'**
  String noteDeleted(String title);

  /// No description provided for @deleteError.
  ///
  /// In pl, this message translates to:
  /// **'Nie udało się usunąć notatki'**
  String get deleteError;

  /// No description provided for @loadNotesError.
  ///
  /// In pl, this message translates to:
  /// **'Nie udało się załadować notatek: {error}'**
  String loadNotesError(Object error);

  /// No description provided for @notebooksLabels.
  ///
  /// In pl, this message translates to:
  /// **'--- Etykiety notatników ---'**
  String get notebooksLabels;

  /// No description provided for @noNotebooks.
  ///
  /// In pl, this message translates to:
  /// **'Brak notatników'**
  String get noNotebooks;

  /// No description provided for @newNotebook.
  ///
  /// In pl, this message translates to:
  /// **'Nowy notatnik'**
  String get newNotebook;

  /// No description provided for @notebookName.
  ///
  /// In pl, this message translates to:
  /// **'Nazwa notatnika'**
  String get notebookName;

  /// Potwierdzenie usunięcia notatnika
  ///
  /// In pl, this message translates to:
  /// **'Czy na pewno chcesz usunąć notatnik \"{name}\"?'**
  String confirmDeleteNotebook(String name);

  /// No description provided for @notebookDeleteError.
  ///
  /// In pl, this message translates to:
  /// **'Nie udało się usunąć notatnika'**
  String get notebookDeleteError;

  /// No description provided for @editNotebook.
  ///
  /// In pl, this message translates to:
  /// **'Edytuj notatnik'**
  String get editNotebook;

  /// Komunikat po otwarciu notatnika
  ///
  /// In pl, this message translates to:
  /// **'Otwarto notatnik: {name}'**
  String notebookOpened(String name);

  /// Błąd ładowania notatników
  ///
  /// In pl, this message translates to:
  /// **'Nie udało się załadować notatników: {message}'**
  String loadNotebooksError(String message);

  /// Komunikat wyświetlany po usunięciu notatnika
  ///
  /// In pl, this message translates to:
  /// **'Usunięto notatnik: {name}'**
  String notebookDeleted(String name);

  /// No description provided for @untitledNote.
  ///
  /// In pl, this message translates to:
  /// **'Notatka bez tytułu'**
  String get untitledNote;

  /// No description provided for @retry.
  ///
  /// In pl, this message translates to:
  /// **'Spróbuj ponownie'**
  String get retry;

  /// No description provided for @deleteNote.
  ///
  /// In pl, this message translates to:
  /// **'Usuń notatkę'**
  String get deleteNote;

  /// No description provided for @deleteNoteConfirmTitle.
  ///
  /// In pl, this message translates to:
  /// **'Usuń notatkę'**
  String get deleteNoteConfirmTitle;

  /// No description provided for @deleteNoteConfirmMessage.
  ///
  /// In pl, this message translates to:
  /// **'Czy na pewno chcesz usunąć notatkę \"{title}\"?'**
  String deleteNoteConfirmMessage(Object title);

  /// No description provided for @deleteNoteFailed.
  ///
  /// In pl, this message translates to:
  /// **'Nie udało się usunąć notatki'**
  String get deleteNoteFailed;

  /// No description provided for @sessionExpired.
  ///
  /// In pl, this message translates to:
  /// **'Twoja sesja wygasła. Zaloguj się ponownie.'**
  String get sessionExpired;

  /// Liczba notatek
  ///
  /// In pl, this message translates to:
  /// **'{count, plural, one{# notatka} few{# notatki} many{# notatek} other{# notatek}}'**
  String noteCount(num count);

  /// Wyświetlane, gdy nie ma żadnych tagów
  ///
  /// In pl, this message translates to:
  /// **'Brak tagów'**
  String get noTags;

  /// No description provided for @unsavedChanges.
  ///
  /// In pl, this message translates to:
  /// **'Masz niezapisane zmiany.'**
  String get unsavedChanges;

  /// No description provided for @discardChanges.
  ///
  /// In pl, this message translates to:
  /// **'Czy chcesz porzucić zmiany?'**
  String get discardChanges;

  /// No description provided for @discard.
  ///
  /// In pl, this message translates to:
  /// **'Porzuć'**
  String get discard;

  /// No description provided for @noteTitle.
  ///
  /// In pl, this message translates to:
  /// **'Tytuł notatki'**
  String get noteTitle;

  /// No description provided for @startTyping.
  ///
  /// In pl, this message translates to:
  /// **'Zacznij pisać...'**
  String get startTyping;

  /// No description provided for @notebook.
  ///
  /// In pl, this message translates to:
  /// **'Notatnik'**
  String get notebook;

  /// No description provided for @noNotebook.
  ///
  /// In pl, this message translates to:
  /// **'Brak notatnika'**
  String get noNotebook;
}

class _AppLocalizationsDelegate extends LocalizationsDelegate<AppLocalizations> {
  const _AppLocalizationsDelegate();

  @override
  Future<AppLocalizations> load(Locale locale) {
    return SynchronousFuture<AppLocalizations>(lookupAppLocalizations(locale));
  }

  @override
  bool isSupported(Locale locale) => <String>['en', 'es', 'pl'].contains(locale.languageCode);

  @override
  bool shouldReload(_AppLocalizationsDelegate old) => false;
}

AppLocalizations lookupAppLocalizations(Locale locale) {


  // Lookup logic when only language code is specified.
  switch (locale.languageCode) {
    case 'en': return AppLocalizationsEn();
    case 'es': return AppLocalizationsEs();
    case 'pl': return AppLocalizationsPl();
  }

  throw FlutterError(
    'AppLocalizations.delegate failed to load unsupported locale "$locale". This is likely '
    'an issue with the localizations generation tool. Please file an issue '
    'on GitHub with a reproducible sample app and the gen-l10n configuration '
    'that was used.'
  );
}
