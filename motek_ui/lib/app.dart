import 'package:flutter/material.dart';
import 'package:flutter_localizations/flutter_localizations.dart';
import 'package:motek_ui/l10n/app_localizations.dart';
import 'package:motek_ui/services/settings_service.dart';
import 'screens/main_layout.dart';
import 'utils/theme_manager.dart';

// Globalny klucz do nawigacji, aby móc zmieniać język z dowolnego miejsca
final GlobalKey<NavigatorState> navigatorKey = GlobalKey<NavigatorState>();

class MotekApp extends StatefulWidget {
  const MotekApp({super.key});

  @override
  State<MotekApp> createState() => _MotekAppState();
}

class _MotekAppState extends State<MotekApp> {
  final SettingsService _settingsService = SettingsService();
  bool _isInitialized = false;

  @override
  void initState() {
    super.initState();
    _initSettings();
  }

  Future<void> _initSettings() async {
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

    return MaterialApp(
      navigatorKey: navigatorKey,
      title: 'Motek UI',
      // Konfiguracja lokalizacji
      localizationsDelegates: const [
        AppLocalizations.delegate,
        GlobalMaterialLocalizations.delegate,
        GlobalWidgetsLocalizations.delegate,
        GlobalCupertinoLocalizations.delegate,
      ],
      supportedLocales: const [
        Locale('pl'),
        Locale('en'),
      ],
      locale: _settingsService.locale,
      theme: _settingsService.isDarkMode ? ThemeManager.darkTheme : ThemeManager.lightTheme,
      home: MainLayout(
        isDarkMode: _settingsService.isDarkMode,
        onThemeChanged: toggleTheme,
        onLocaleChanged: setLocale,
      ),
    );
  }
}
