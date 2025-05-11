import 'package:flutter/material.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'dart:io';
import 'package:motek_ui/src/rust/api/endpoint.dart';

class LogSettingsScreen extends StatefulWidget {
  @override
  _LogSettingsScreenState createState() => _LogSettingsScreenState();
}

class _LogSettingsScreenState extends State<LogSettingsScreen> {
  final TextEditingController _logFilePathController = TextEditingController();
  String _logLevel = 'info'; // domyślny poziom
  
  @override
  void initState() {
    super.initState();
    _loadSettings();
  }
  
  Future<void> _loadSettings() async {
    // Wczytaj ustawienia z lokalnego storage
    final prefs = await SharedPreferences.getInstance();
    setState(() {
      _logFilePathController.text = prefs.getString('log_file_path') ?? 
        '${Directory.systemTemp.path}/motek_ui.log';
      _logLevel = prefs.getString('log_level') ?? 'info';
    });
  }
  
  Future<void> _saveSettings() async {
    final prefs = await SharedPreferences.getInstance();
    await prefs.setString('log_file_path', _logFilePathController.text);
    await prefs.setString('log_level', _logLevel);
    
    // Zastosuj nowe ustawienia
    await setupLoggingBridge(
      logLevel: _logLevel,  // Zmienione na nazwany parametr
      logFilePath: _logFilePathController.text.isNotEmpty ? _logFilePathController.text : null
    );
    
    ScaffoldMessenger.of(context).showSnackBar(
      SnackBar(content: Text('Ustawienia zapisane'))
    );
  }
  
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: Text('Ustawienia logowania')),
      body: Padding(
        padding: const EdgeInsets.all(16.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text('Ścieżka do pliku logów:'),
            TextField(
              controller: _logFilePathController,
              decoration: InputDecoration(
                hintText: 'np. /storage/emulated/0/Android/data/com.motek.ui/logs/app.log'
              ),
            ),
            SizedBox(height: 20),
            Text('Poziom logowania:'),
            DropdownButton<String>(
              value: _logLevel,
              items: ['trace', 'debug', 'info', 'warn', 'error'].map((level) {
                return DropdownMenuItem<String>(
                  value: level,
                  child: Text(level),
                );
              }).toList(),
              onChanged: (value) {
                setState(() {
                  _logLevel = value!;
                });
              },
            ),
            SizedBox(height: 20),
            ElevatedButton(
              onPressed: _saveSettings,
              child: Text('Zapisz ustawienia'),
            ),
          ],
        ),
      ),
    );
  }
}
