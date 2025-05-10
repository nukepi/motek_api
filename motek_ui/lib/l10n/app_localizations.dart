import 'dart:async';

import 'package:flutter/foundation.dart';
import 'package:flutter/widgets.dart';
import 'package:flutter_localizations/flutter_localizations.dart';
import 'package:intl/intl.dart' as intl;

import 'app_localizations_en.dart';
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
  /// **'Logowanie'**
  String get login;

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

  /// No description provided for @logout.
  ///
  /// In pl, this message translates to:
  /// **'Wyloguj się'**
  String get logout;

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

  /// No description provided for @copyright.
  ///
  /// In pl, this message translates to:
  /// **'© 2025 Motek UI'**
  String get copyright;

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

  /// No description provided for @noNotes.
  ///
  /// In pl, this message translates to:
  /// **'Brak notatek'**
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

  /// No description provided for @cancel.
  ///
  /// In pl, this message translates to:
  /// **'Anuluj'**
  String get cancel;

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

  /// No description provided for @save.
  ///
  /// In pl, this message translates to:
  /// **'Zapisz'**
  String get save;

  /// No description provided for @confirmDelete.
  ///
  /// In pl, this message translates to:
  /// **'Potwierdź usunięcie'**
  String get confirmDelete;

  /// Potwierdzenie usunięcia notatki
  ///
  /// In pl, this message translates to:
  /// **'Czy na pewno chcesz usunąć notatkę \"{title}\"?'**
  String confirmDeleteNote(String title);

  /// No description provided for @delete.
  ///
  /// In pl, this message translates to:
  /// **'Usuń'**
  String get delete;

  /// Komunikat po usunięciu notatki
  ///
  /// In pl, this message translates to:
  /// **'Usunięto notatkę: {title}'**
  String noteDeleted(String title);

  /// No description provided for @undoAction.
  ///
  /// In pl, this message translates to:
  /// **'Cofnij'**
  String get undoAction;

  /// No description provided for @deleteError.
  ///
  /// In pl, this message translates to:
  /// **'Nie udało się usunąć notatki'**
  String get deleteError;

  /// Komunikat błędu
  ///
  /// In pl, this message translates to:
  /// **'Błąd: {message}'**
  String error(String message);

  /// No description provided for @tryAgain.
  ///
  /// In pl, this message translates to:
  /// **'Spróbuj ponownie'**
  String get tryAgain;

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

  /// Data utworzenia
  ///
  /// In pl, this message translates to:
  /// **'Utworzono: {date}'**
  String created(String date);

  /// Błąd ładowania notatek
  ///
  /// In pl, this message translates to:
  /// **'Nie udało się załadować notatek: {message}'**
  String loadNotesError(String message);

  /// Błąd ładowania notatników
  ///
  /// In pl, this message translates to:
  /// **'Nie udało się załadować notatników: {message}'**
  String loadNotebooksError(String message);
}

class _AppLocalizationsDelegate extends LocalizationsDelegate<AppLocalizations> {
  const _AppLocalizationsDelegate();

  @override
  Future<AppLocalizations> load(Locale locale) {
    return SynchronousFuture<AppLocalizations>(lookupAppLocalizations(locale));
  }

  @override
  bool isSupported(Locale locale) => <String>['en', 'pl'].contains(locale.languageCode);

  @override
  bool shouldReload(_AppLocalizationsDelegate old) => false;
}

AppLocalizations lookupAppLocalizations(Locale locale) {


  // Lookup logic when only language code is specified.
  switch (locale.languageCode) {
    case 'en': return AppLocalizationsEn();
    case 'pl': return AppLocalizationsPl();
  }

  throw FlutterError(
    'AppLocalizations.delegate failed to load unsupported locale "$locale". This is likely '
    'an issue with the localizations generation tool. Please file an issue '
    'on GitHub with a reproducible sample app and the gen-l10n configuration '
    'that was used.'
  );
}
