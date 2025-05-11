import 'package:flutter/material.dart';
import 'package:motek_ui/src/rust/frb_generated.dart';
import 'package:motek_ui/src/rust/api/endpoint.dart';
import 'package:motek_ui/services/auth_service.dart';
import 'package:provider/provider.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'app.dart';

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();
  
  // Inicjalizacja mostu Rust
  await RustLib.init();
  
  // Inicjalizacja logowania
  // Wczytaj ustawienia logowania
  final prefs = await SharedPreferences.getInstance();
  final logLevel = prefs.getString('log_level') ?? 'info';
  final logFilePath = prefs.getString('log_file_path');
  
  // Skonfiguruj logowanie z nazwanymi parametrami
  await setupLoggingBridge(logLevel: logLevel, logFilePath: logFilePath);
  
  // Inicjalizacja AuthService
  final authService = AuthService();
  await authService.init();
  
  // Sprawdzenie stanu logowania
  bool isLoggedIn = await authService.checkLoginStatus();
  debugPrint('Stan logowania przy starcie: ${isLoggedIn ? 'zalogowany' : 'niezalogowany'}');
  
  runApp(
    MultiProvider(
      providers: [
        ChangeNotifierProvider.value(value: authService),
      ],
      child: MotekApp(initiallyLoggedIn: isLoggedIn),
    ),
  );
}
