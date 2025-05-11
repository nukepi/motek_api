// lib/app.dart
import 'package:flutter/material.dart';
import 'package:flutter_localizations/flutter_localizations.dart';
import 'package:motek_ui/l10n/app_localizations.dart';
import 'package:motek_ui/services/auth_service.dart';
import 'package:motek_ui/screens/notebooks_screen.dart';
import 'package:motek_ui/screens/notebook_notes_screen.dart';
import 'package:motek_ui/services/settings_service.dart';
import 'package:motek_ui/src/rust/api_handlers/notebooks.dart';
import 'package:flutter_quill/flutter_quill.dart';
import 'package:provider/provider.dart';
import 'screens/main_layout.dart';
import 'package:motek_ui/widgets/content/login_content.dart'; // Dodaj import ekranu logowania
import 'utils/theme_manager.dart';

// Globalny klucz do nawigacji, aby móc zmieniać język z dowolnego miejsca
final GlobalKey<NavigatorState> navigatorKey = GlobalKey<NavigatorState>();

class MotekApp extends StatefulWidget {
  final bool initiallyLoggedIn; // Dodajemy parametr do przekazania początkowego stanu logowania
  
  const MotekApp({super.key, this.initiallyLoggedIn = false});

  @override
  State<MotekApp> createState() => _MotekAppState();
}

class _MotekAppState extends State<MotekApp> {
  final SettingsService _settingsService = SettingsService();
  bool _isInitialized = false;

  @override
  void initState() {
    super.initState();
    _initApp();
  }

  Future<void> _initApp() async {
    await _settingsService.init();
    setState(() {
      _isInitialized = true;
    });
  }

  void toggleTheme(bool isDarkMode) async {
    await _settingsService.setThemeMode(isDarkMode);
    setState(() {});
  }

  void setLocale(String languageCode) async {
    await _settingsService.setLocale(languageCode);
    setState(() {});
  }

  @override
  Widget build(BuildContext context) {
    if (!_isInitialized) {
      return const MaterialApp(
        home: Scaffold(
          body: Center(
            child: CircularProgressIndicator(),
          ),
        ),
      );
    }

    // Używamy Consumer, aby aplikacja reagowała na zmiany stanu logowania
    return Consumer<AuthService>(
      builder: (context, authService, _) {
        return MaterialApp(
          navigatorKey: navigatorKey,
          title: 'Motek UI',
          // Konfiguracja lokalizacji
          localizationsDelegates: const [
            AppLocalizations.delegate,
            GlobalMaterialLocalizations.delegate,
            GlobalWidgetsLocalizations.delegate,
            GlobalCupertinoLocalizations.delegate,
            FlutterQuillLocalizations.delegate,
          ],
          supportedLocales: const [
            Locale('pl'),
            Locale('en'),
            Locale('es'),
          ],
          locale: _settingsService.locale,
          theme: _settingsService.isDarkMode ? ThemeManager.darkTheme : ThemeManager.lightTheme,
          // Sprawdzamy stan logowania i wyświetlamy odpowiedni ekran
          home: authService.isLoggedIn
              ? MainLayout(
                  isDarkMode: _settingsService.isDarkMode,
                  onThemeChanged: toggleTheme,
                  onLocaleChanged: setLocale,
                )
              : const LoginContent(), // Ekran logowania
          routes: {
            '/login': (context) => const LoginContent(),
            '/main': (context) => MainLayout(
                  isDarkMode: _settingsService.isDarkMode,
                  onThemeChanged: toggleTheme,
                  onLocaleChanged: setLocale,
                ),
                '/notebooks': (context) => const NotebooksScreen(),
                '/notebook_notes': (context) => NotebookNotesScreen(
                  notebook: ModalRoute.of(context)!.settings.arguments as Notebook,
                ),
          },
        );
      },
    );
  }
}
