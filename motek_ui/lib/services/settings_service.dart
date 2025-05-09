import 'package:flutter/material.dart';
import 'package:shared_preferences/shared_preferences.dart';

class SettingsService {
  // Klucze dla SharedPreferences
  static const String _themeKey = 'theme_mode';
  static const String _localeKey = 'locale';
  static const String _notificationsKey = 'notifications_enabled';
  static const String _fontSizeKey = 'font_size';

  // Singleton
  static final SettingsService _instance = SettingsService._internal();
  factory SettingsService() => _instance;
  SettingsService._internal();

  // Wartości domyślne
  bool _isDarkMode = false;
  Locale _locale = const Locale('pl');
  bool _notificationsEnabled = true;
  double _fontSize = 16.0;

  // Gettery
  bool get isDarkMode => _isDarkMode;
  Locale get locale => _locale;
  bool get notificationsEnabled => _notificationsEnabled;
  double get fontSize => _fontSize;

  // Inicjalizacja - wczytuje zapisane ustawienia
  Future<void> init() async {
    final prefs = await SharedPreferences.getInstance();
    _isDarkMode = prefs.getBool(_themeKey) ?? false;
    final localeString = prefs.getString(_localeKey);
    if (localeString != null) {
      _locale = Locale(localeString);
    }
    _notificationsEnabled = prefs.getBool(_notificationsKey) ?? true;
    _fontSize = prefs.getDouble(_fontSizeKey) ?? 16.0;
  }

  // Metody do zmiany ustawień
  Future<void> setThemeMode(bool isDarkMode) async {
    _isDarkMode = isDarkMode;
    final prefs = await SharedPreferences.getInstance();
    await prefs.setBool(_themeKey, isDarkMode);
  }

  Future<void> setLocale(String languageCode) async {
    _locale = Locale(languageCode);
    final prefs = await SharedPreferences.getInstance();
    await prefs.setString(_localeKey, languageCode);
  }

  Future<void> setNotificationsEnabled(bool enabled) async {
    _notificationsEnabled = enabled;
    final prefs = await SharedPreferences.getInstance();
    await prefs.setBool(_notificationsKey, enabled);
  }

  Future<void> setFontSize(double size) async {
    _fontSize = size;
    final prefs = await SharedPreferences.getInstance();
    await prefs.setDouble(_fontSizeKey, size);
  }
}
