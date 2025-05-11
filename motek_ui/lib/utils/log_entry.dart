// lib/utils/log_entry.dart
enum LogLevel {
  trace,
  debug,
  info,
  warn,
  error,
  fatal
}

class LogEntry {
  final DateTime timestamp;
  final LogLevel level;
  final String message;
  final String? source; // "Flutter", "Rust", etc.
  
  LogEntry({
    required this.level,
    required this.message,
    this.source,
    DateTime? timestamp,
  }) : timestamp = timestamp ?? DateTime.now();
  
  @override
  String toString() {
    return '${timestamp.toIso8601String()} [${level.toString().split('.').last.toUpperCase()}] ${source != null ? '[$source] ' : ''}$message';
  }
  
  static LogLevel stringToLevel(String levelStr) {
    switch (levelStr.toUpperCase()) {
      case 'TRACE':
        return LogLevel.trace;
      case 'DEBUG':
        return LogLevel.debug;
      case 'INFO':
        return LogLevel.info;
      case 'WARN':
      case 'WARNING':
        return LogLevel.warn;
      case 'ERROR':
        return LogLevel.error;
      case 'FATAL':
        return LogLevel.fatal;
      default:
        return LogLevel.info;
    }
  }
}
