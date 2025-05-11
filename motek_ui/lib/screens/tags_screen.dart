import 'package:flutter/material.dart';
import 'package:motek_ui/l10n/app_localizations.dart';
import 'package:motek_ui/src/rust/api_handlers/notes.dart';
import 'package:motek_ui/src/rust/api/endpoint.dart';
import 'package:motek_ui/utils/logger.dart';
import 'package:motek_ui/screens/note_editor_screen.dart';
import 'dart:convert';

class TagsScreen extends StatefulWidget {
  const TagsScreen({Key? key}) : super(key: key);

  @override
  _TagsScreenState createState() => _TagsScreenState();
}

class _TagsScreenState extends State<TagsScreen> {
  List<String> _tags = [];
  bool _isLoading = true;
  String? _errorMessage;
  Map<String, List<Note>> _notesByTag = {};

  @override
  void initState() {
    super.initState();
    _loadTags();
  }

  Future<void> _loadTags() async {
    if (!mounted) return;
    
    setState(() {
      _isLoading = true;
      _errorMessage = null;
    });

    try {
      // Pobierz wszystkie notatki
      final notes = await listNotes();
      
      // Wyodrębnij unikalne tagi
      final Set<String> uniqueTags = {};
      final Map<String, List<Note>> notesByTag = {};
      
      for (final note in notes) {
        if (note.tags.isNotEmpty) {
          final noteTags = note.tags.split(',').where((tag) => tag.isNotEmpty).toList();
          
          for (final tag in noteTags) {
            uniqueTags.add(tag);
            
            if (!notesByTag.containsKey(tag)) {
              notesByTag[tag] = [];
            }
            notesByTag[tag]!.add(note);
          }
        }
      }
      
      if (!mounted) return;
      
      setState(() {
        _tags = uniqueTags.toList()..sort();
        _notesByTag = notesByTag;
        _isLoading = false;
      });
    } catch (e) {
      Logger.error('Failed to load tags: $e');
      if (!mounted) return;
      
      setState(() {
        _errorMessage = e.toString();
        _isLoading = false;
      });
    }
  }

  void _showNotesWithTag(String tag) {
    final notes = _notesByTag[tag] ?? [];
    
    Navigator.push(
      context,
      MaterialPageRoute(
        builder: (context) => _TagNotesScreen(tag: tag, notes: notes),
      ),
    ).then((_) => _loadTags());
  }

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;
    
    return Scaffold(
      body: _isLoading
          ? const Center(child: CircularProgressIndicator())
          : _errorMessage != null
              ? Center(
                  child: Column(
                    mainAxisAlignment: MainAxisAlignment.center,
                    children: [
                      Text(
                        _errorMessage!,
                        style: const TextStyle(color: Colors.red),
                        textAlign: TextAlign.center,
                      ),
                      const SizedBox(height: 16),
                      ElevatedButton(
                        onPressed: _loadTags,
                        child: Text(l10n.tryAgain),
                      ),
                    ],
                  ),
                )
              : _tags.isEmpty
                  ? Center(
                      child: Text(l10n.noTags),
                    )
                  : ListView.builder(
                      itemCount: _tags.length,
                      itemBuilder: (context, index) {
                        final tag = _tags[index];
                        final noteCount = _notesByTag[tag]?.length ?? 0;
                        
                        return ListTile(
                          title: Text('#$tag'),
                          subtitle: Text(
                            l10n.noteCount(noteCount),
                          ),
                          leading: const Icon(Icons.tag),
                          trailing: const Icon(Icons.arrow_forward_ios, size: 16),
                          onTap: () => _showNotesWithTag(tag),
                        );
                      },
                    ),
    );
  }
}

class _TagNotesScreen extends StatelessWidget {
  final String tag;
  final List<Note> notes;

  const _TagNotesScreen({
    Key? key,
    required this.tag,
    required this.notes,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;
    
    return Scaffold(
      appBar: AppBar(
        title: Text('#$tag'),
      ),
      body: notes.isEmpty
          ? Center(
              child: Text(l10n.noNotes),
            )
          : ListView.builder(
              itemCount: notes.length,
              itemBuilder: (context, index) {
                final note = notes[index];
                return ListTile(
                  title: Text(note.title.isEmpty ? l10n.untitledNote : note.title),
                  subtitle: Text(
                    _getContentPreview(note.content),
                    maxLines: 2,
                    overflow: TextOverflow.ellipsis,
                  ),
                  leading: const Icon(Icons.note),
                  onTap: () {
                    Navigator.push(
                      context,
                      MaterialPageRoute(
                        builder: (context) => NoteEditorScreen(
                          initialTitle: note.title,
                          initialContent: note.content,
                          initialTags: note.tags.split(',').where((s) => s.isNotEmpty).toList(),
                          initialNotebookId: note.notebookId?.isEmpty == true ? null : note.notebookId,
                          onSave: (title, content) async {
                            try {
                              await updateNote(
                                noteId: note.id,
                                title: title.isEmpty ? l10n.untitledNote : title,
                                content: content,
                              );
                              return true;
                            } catch (e) {
                              Logger.error('Failed to update note: $e');
                              ScaffoldMessenger.of(context).showSnackBar(
                                SnackBar(content: Text(l10n.error(e.toString()))),
                              );
                              return false;
                            }
                          },
                        ),
                      ),
                    );
                  },
                );
              },
            ),
    );
  }

  String _getContentPreview(String content) {
    // Próba parsowania JSON
    try {
      final jsonData = json.decode(content);
      if (jsonData is List) {
        // To może być format Delta z QuillJS
        String plainText = '';
        for (var op in jsonData) {
          if (op is Map && op.containsKey('insert')) {
            plainText += op['insert'].toString();
          }
        }
        return plainText.length > 50 ? '${plainText.substring(0, 50)}...' : plainText;
      }
    } catch (_) {
      // Jeśli nie da się sparsować, zwróć surowy tekst
    }
    
    return content.length > 50 ? '${content.substring(0, 50)}...' : content;
  }
}
