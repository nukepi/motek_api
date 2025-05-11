import 'dart:convert';
import 'package:http/http.dart' as http;
import 'package:motek_ui/src/rust/api_handlers/notes.dart';
import 'package:motek_ui/services/auth_service.dart';

class NoteAdapter {
  static Note fromJson(Map<String, dynamic> json) {
    return Note(
      id: json['id'] ?? '',
      userId: json['user_id'] ?? '',
      notebookId: json['notebook_id'] ?? '',
      title: json['title'] ?? '',
      content: json['content'] ?? '',
      isArchived: json['is_archived'] ?? false,
      isPinned: json['is_pinned'] ?? false,
      tags: json['tags'] ?? '',
      // Konwersja timestampów
      createdAt: _parseTimestamp(json['created_at']),
      updatedAt: _parseTimestamp(json['updated_at']),
    );
  }
  
  // Metoda pomocnicza do konwersji różnych formatów timestampów
  static int _parseTimestamp(dynamic timestamp) {
    if (timestamp is int) {
      return timestamp;
    } else if (timestamp is String) {
      // Zakładamy, że string to ISO 8601
      return DateTime.parse(timestamp).microsecondsSinceEpoch;
    }
    // Domyślnie zwróć aktualny czas
    return DateTime.now().microsecondsSinceEpoch;
  }
}

class NoteService {
  final String baseUrl;
  final AuthService authService;

  NoteService({required this.baseUrl, required this.authService});

  Future<List<Note>> getNotes(String notebookId) async {
    final token = await authService.getToken();
    if (token == null) {
      throw Exception('User not authenticated');
    }
    
    final response = await http.get(
      Uri.parse('$baseUrl/notebooks/$notebookId/notes'),
      headers: {
        'Authorization': 'Bearer $token',
        'Content-Type': 'application/json',
      },
    );

    if (response.statusCode == 200) {
      final List<dynamic> data = json.decode(response.body);
      return data.map((json) => NoteAdapter.fromJson(json)).toList();
    } else {
      throw Exception('Failed to load notes: ${response.body}');
    }
  }

  Future<Note> createNote({
    required String title,
    required String content,
    required String notebookId,
  }) async {
    final token = await authService.getToken();
    if (token == null) {
      throw Exception('User not authenticated');
    }
    
    final response = await http.post(
      Uri.parse('$baseUrl/notes'),
      headers: {
        'Authorization': 'Bearer $token',
        'Content-Type': 'application/json',
      },
      body: json.encode({
        'title': title,
        'content': content,
        'notebook_id': notebookId,
      }),
    );

    if (response.statusCode == 201) {
      return NoteAdapter.fromJson(json.decode(response.body));
    } else {
      throw Exception('Failed to create note: ${response.body}');
    }
  }

  Future<Note> updateNote({
    required String noteId,
    required String title,
    required String content,
  }) async {
    final token = await authService.getToken();
    if (token == null) {
      throw Exception('User not authenticated');
    }
    
    final response = await http.put(
      Uri.parse('$baseUrl/notes/$noteId'),
      headers: {
        'Authorization': 'Bearer $token',
        'Content-Type': 'application/json',
      },
      body: json.encode({
        'title': title,
        'content': content,
      }),
    );

    if (response.statusCode == 200) {
      return NoteAdapter.fromJson(json.decode(response.body));
    } else {
      throw Exception('Failed to update note: ${response.body}');
    }
  }

  Future<void> deleteNote(String noteId) async {
    final token = await authService.getToken();
    if (token == null) {
      throw Exception('User not authenticated');
    }
    
    final response = await http.delete(
      Uri.parse('$baseUrl/notes/$noteId'),
      headers: {
        'Authorization': 'Bearer $token',
      },
    );

    if (response.statusCode != 204) {
      throw Exception('Failed to delete note: ${response.body}');
    }
  }
}
