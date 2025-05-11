// lib/utils/logger.dart
import 'dart:developer' as developer;
import 'package:motek_ui/utils/logs_manager.dart';
import 'package:motek_ui/utils/log_entry.dart';
// Tymczasowo zakomentowane do czasu wygenerowania pliku
// import 'package:motek_ui/bridge_generated.dart';

class Logger {
  // Tymczasowo zakomentowane do czasu wygenerowania pliku
  // static final api = RustApi.instance;
  
  static final LogsManager _logsManager = LogsManager.instance;
  
  /// Pobiera zawartość logów z pliku
  static Future<String?> getLogContent() async {
    try {
      // Zwracamy logi jako sformatowany string
      return _logsManager.getLogsAsString();
    } catch (e) {
      developer.log('Error getting logs: $e', name: 'Logger');
      return 'Error getting logs: $e';
    }
  }
  
  /// Inicjalizuje system logowania
  static Future<void> initLogging({String logLevel = 'info'}) async {
    try {
      // Tymczasowo zakomentowane do czasu wygenerowania pliku
      // await api.setupLoggingBridge(logLevel: logLevel);
      developer.log('Logging initialized with level: $logLevel', name: 'Logger');
      
      _logsManager.addLog(LogEntry(
        level: LogLevel.info,
        message: 'Logging initialized with level: $logLevel',
        source: 'Flutter'
      ));
    } catch (e) {
      developer.log('Error initializing logging: $e', name: 'Logger');
      
      _logsManager.addLog(LogEntry(
        level: LogLevel.error,
        message: 'Error initializing logging: $e',
        source: 'Flutter'
      ));
    }
  }
  
  /// Generuje przykładowe logi testowe
  static Future<void> generateTestLogs() async {
    try {
      // Tymczasowo zakomentowane do czasu wygenerowania pliku
      // await api.testRustLogging();
      developer.log('Test logs generated', name: 'Logger');
      
      _logsManager.addLog(LogEntry(
        level: LogLevel.info,
        message: 'Test log generated',
        source: 'Flutter'
      ));
      
      _logsManager.addLog(LogEntry(
        level: LogLevel.debug,
        message: 'This is a debug message',
        source: 'Flutter'
      ));
      
      _logsManager.addLog(LogEntry(
        level: LogLevel.warn,
        message: 'This is a warning message',
        source: 'Flutter'
      ));
      
      _logsManager.addLog(LogEntry(
        level: LogLevel.error,
        message: 'This is an error message',
        source: 'Flutter'
      ));
    } catch (e) {
      developer.log('Error generating test logs: $e', name: 'Logger');
      
      _logsManager.addLog(LogEntry(
        level: LogLevel.error,
        message: 'Error generating test logs: $e',
        source: 'Flutter'
      ));
    }
  }
  
  /// Loguje wiadomość z poziomu Dart
  static void log(LogLevel level, String message, {String source = 'Flutter'}) {
    final logEntry = LogEntry(
      level: level,
      message: message,
      source: source
    );
    
    developer.log(message, name: '${source}[${level.toString().split('.').last.toUpperCase()}]');
    _logsManager.addLog(logEntry);
  }
  
  /// Odbiera log z Rust i dodaje go do managera logów
  static void handleRustLog(String levelStr, String message, {String? module}) {
    final level = LogEntry.stringToLevel(levelStr);
    final source = module != null ? 'Rust:$module' : 'Rust';
    
    _logsManager.addLog(LogEntry(
      level: level,
      message: message,
      source: source
    ));
  }
  
  /// Parsuje log z terminala Rust do formatu LogEntry
  static void parseRustLog(String logLine) {
    // Format: 2025-05-10T06:53:35.613498Z  INFO rust_lib_motek_ui::api_handlers::auth: message
    try {
      final regex = RegExp(r'(\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}\.\d+Z)\s+(\w+)\s+([^:]+):\s+(.+)');
      final match = regex.firstMatch(logLine);
      
      if (match != null) {
        final timestamp = DateTime.parse(match.group(1)!);
        final levelStr = match.group(2)!;
        final module = match.group(3);
        final message = match.group(4)!;
        
        final level = LogEntry.stringToLevel(levelStr);
        
        _logsManager.addLog(LogEntry(
          timestamp: timestamp,
          level: level,
          message: message,
          source: 'Rust:$module'
        ));
      }
    } catch (e) {
      // Jeśli nie udało się sparsować, dodaj jako surowy log
      _logsManager.addLog(LogEntry(
        level: LogLevel.debug,
        message: 'Raw log: $logLine',
        source: 'Rust'
      ));
    }
  }
  
  static void trace(String message) => log(LogLevel.trace, message);
  static void debug(String message) => log(LogLevel.debug, message);
  static void info(String message) => log(LogLevel.info, message);
  static void warn(String message) => log(LogLevel.warn, message);
  static void error(String message) => log(LogLevel.error, message);
  static void fatal(String message) => log(LogLevel.fatal, message);
}
