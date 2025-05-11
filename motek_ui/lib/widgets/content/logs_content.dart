import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:motek_ui/utils/logger.dart';
import 'package:motek_ui/utils/logs_manager.dart';

class LogsContent extends StatefulWidget {
  const LogsContent({super.key});

  @override
  State<LogsContent> createState() => _LogsContentState();
}

class _LogsContentState extends State<LogsContent> {
  final ScrollController _scrollController = ScrollController();
  final LogsManager _logsManager = LogsManager();
  
  @override
  void initState() {
    super.initState();
    // Dodaj słuchacza zmian w LogsManager
    _logsManager.addListener(_scrollToBottom);
    
    // Dodaj log o otwarciu ekranu logów
    Logger.info('Logs screen opened');
  }

  @override
  void dispose() {
    _scrollController.dispose();
    // Usuń słuchacza przy usunięciu widgetu
    _logsManager.removeListener(_scrollToBottom);
    super.dispose();
  }
  
  // Funkcja przewijająca do dołu po dodaniu nowego logu
  void _scrollToBottom() {
    WidgetsBinding.instance.addPostFrameCallback((_) {
      if (_scrollController.hasClients) {
        _scrollController.animateTo(
          _scrollController.position.maxScrollExtent,
          duration: const Duration(milliseconds: 300),
          curve: Curves.easeOut,
        );
      }
    });
  }

  // ignore: unused_element
  Future<void> _copyToClipboard() async {
    final logs = _logsManager.logs.join('\n');
    await Clipboard.setData(ClipboardData(text: logs));
    if (mounted) {
      ScaffoldMessenger.of(context).showSnackBar(
        const SnackBar(content: Text('Logi skopiowane do schowka')),
      );
    }
  }

  Future<void> _generateTestLogs() async {
    await Logger.generateTestLogs();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Padding(
        padding: const EdgeInsets.all(8.0),
        child: AnimatedBuilder(
          animation: _logsManager,
          builder: (context, _) {
            final logs = _logsManager.logs;
            return logs.isEmpty
                ? const Center(child: Text('Brak logów'))
                : SingleChildScrollView(
                    controller: _scrollController,
                    child: SelectableText(
                      logs.join('\n'),
                      style: const TextStyle(fontFamily: 'monospace', fontSize: 12),
                    ),
                  );
          },
        ),
      ),
      floatingActionButton: FloatingActionButton(
        onPressed: _generateTestLogs,
        tooltip: 'Generuj testowe logi',
        child: const Icon(Icons.bug_report),
      ),
    );
  }
}
