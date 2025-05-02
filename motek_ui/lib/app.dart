import 'package:flutter/material.dart';
import 'package:flutter_localizations/flutter_localizations.dart';
import 'package:motek_ui/l10n/app_localizations.dart';
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
  bool _isDarkMode = false;
  Locale _locale = const Locale('pl');

  void toggleTheme(bool isDarkMode) {
    setState(() {
      _isDarkMode = isDarkMode;
    });
  }

  void setLocale(Locale locale) {
    setState(() {
      _locale = locale;
    });
  }

  @override
  Widget build(BuildContext context) {
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
      locale: _locale,
      theme: _isDarkMode ? ThemeManager.darkTheme : ThemeManager.lightTheme,
      home: MainLayout(
        isDarkMode: _isDarkMode,
        onThemeChanged: toggleTheme,
      ),
    );
  }
}
