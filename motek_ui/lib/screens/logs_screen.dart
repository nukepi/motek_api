import 'package:flutter/material.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'dart:async';
import 'package:motek_ui/src/rust/api/endpoint.dart';
import 'log_settings_screen.dart';

class LogsScreen extends StatefulWidget {
  @override
  _LogsScreenState createState() => _LogsScreenState();
}

class _LogsScreenState extends State<LogsScreen> {
  String logs = '';
  Timer? _refreshTimer;
  
  @override
  void initState() {
    super.initState();
    _loadLogs();
    // Odświeżaj logi co 2 sekundy
    _refreshTimer = Timer.periodic(Duration(seconds: 2), (_) => _loadLogs());
  }
  
  @override
  void dispose() {
    _refreshTimer?.cancel();
    super.dispose();
  }
  
  Future<void> _loadLogs() async {
    // Pobierz ścieżkę do pliku logów z ustawień
    final prefs = await SharedPreferences.getInstance();
    final logFilePath = prefs.getString('log_file_path');
    
    // Poprawione wywołanie funkcji - dodane await i nazwany parametr
    final logsContent = await getLogs(logFilePath: logFilePath);
    setState(() {
      logs = logsContent;
    });
  }
  
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text('Logi aplikacji'),
        actions: [
          IconButton(
            icon: Icon(Icons.settings),
            onPressed: () async {
              await Navigator.push(
                context, 
                MaterialPageRoute(builder: (_) => LogSettingsScreen())
              );
              // Po powrocie z ustawień, odśwież logi
              _loadLogs();
            },
          ),
          IconButton(
            icon: Icon(Icons.refresh),
            onPressed: _loadLogs,
          ),
        ],
      ),
      body: SingleChildScrollView(
        child: Padding(
          padding: const EdgeInsets.all(8.0),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Text(
                'Zawartość logów:',
                style: TextStyle(fontWeight: FontWeight.bold),
              ),
              SizedBox(height: 8),
              Container(
                padding: EdgeInsets.all(8),
                decoration: BoxDecoration(
                  color: Colors.black,
                  borderRadius: BorderRadius.circular(4),
                ),
                width: double.infinity,
                child: SelectableText(
                  logs,
                  style: TextStyle(
                    fontFamily: 'monospace',
                    color: Colors.lightGreenAccent,
                    fontSize: 12,
                  ),
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}
