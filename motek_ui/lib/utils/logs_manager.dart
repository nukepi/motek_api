// lib/utils/logs_manager.dart
import 'package:flutter/foundation.dart';
import 'package:flutter/scheduler.dart';
import 'log_entry.dart';

class LogsManager extends ChangeNotifier {
  static final LogsManager _instance = LogsManager._internal();

  factory LogsManager() => _instance;

  LogsManager._internal();

  static LogsManager get instance => _instance;

  final List<LogEntry> _logs = [];

  List<LogEntry> get logs => List.unmodifiable(_logs);

  void addLog(LogEntry log) {
    _logs.add(log);

    // Bezpieczne powiadomienie słuchaczy
    SchedulerBinding.instance.addPostFrameCallback((_) {
      if (hasListeners) {
        notifyListeners();
      }
    });
  }

  void clearLogs() {
    _logs.clear();
    notifyListeners();
  }

  String getLogsAsString() {
    return _logs.map((log) => log.toString()).join('\n');
  }
}
