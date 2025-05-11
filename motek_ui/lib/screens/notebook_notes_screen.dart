import 'dart:convert';
import 'package:flutter/material.dart';
import 'package:motek_ui/l10n/app_localizations.dart';
import 'package:motek_ui/src/rust/api_handlers/notebooks.dart';
import 'package:motek_ui/src/rust/api_handlers/notes.dart';
import 'package:motek_ui/src/rust/api/endpoint.dart';
import 'package:motek_ui/utils/logger.dart';
import 'package:motek_ui/screens/note_editor_screen.dart';

class NotebookNotesScreen extends StatefulWidget {
  final Notebook notebook;

  const NotebookNotesScreen({Key? key, required this.notebook}) : super(key: key);

  @override
  _NotebookNotesScreenState createState() => _NotebookNotesScreenState();
}

class _NotebookNotesScreenState extends State<NotebookNotesScreen> {
  List<Note> _notes = [];
  bool _isLoading = true;
  String? _errorMessage;

  @override
  void initState() {
    super.initState();
    _loadNotes();
  }

  Future<void> _loadNotes() async {
    if (!mounted) return;
    
    setState(() {
      _isLoading = true;
      _errorMessage = null;
    });

    try {
      // Pobierz wszystkie notatki i filtruj po ID notatnika
      final allNotes = await listNotes();
      final notebookNotes = allNotes.where((note) => note.notebookId == widget.notebook.id).toList();
      
      if (!mounted) return;
      
      setState(() {
        _notes = notebookNotes;
        _isLoading = false;
      });
    } catch (e) {
      Logger.error('Failed to load notes for notebook: $e');
      if (!mounted) return;
      
      setState(() {
        _errorMessage = e.toString();
        _isLoading = false;
      });
    }
  }

  Future<void> _createNote() async {
    final l10n = AppLocalizations.of(context)!;
    
    final result = await Navigator.push<bool>(
      context,
      MaterialPageRoute(
        builder: (context) => NoteEditorScreen(
          onSave: (title, content) async {
            try {
              // Tworzenie notatki z przypisaniem do notatnika
              await createNote(
                title: title.isEmpty ? l10n.untitledNote : title,
                content: content,
              );
              return true;
            } catch (e) {
              Logger.error('Failed to create note: $e');
              if (mounted) {
                ScaffoldMessenger.of(context).showSnackBar(
                  SnackBar(content: Text(l10n.error(e.toString()))),
                );
              }
              return false;
            }
          },
        ),
      ),
    );

    if (result == true && mounted) {
      _loadNotes();
    }
  }

  Future<void> _editNote(Note note) async {
    final l10n = AppLocalizations.of(context)!;
    
    final result = await Navigator.push<bool>(
      context,
      MaterialPageRoute(
        builder: (context) => NoteEditorScreen(
          initialTitle: note.title,
          initialContent: note.content,
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
              if (mounted) {
                ScaffoldMessenger.of(context).showSnackBar(
                  SnackBar(content: Text(l10n.error(e.toString()))),
                );
              }
              return false;
            }
          },
        ),
      ),
    );

    if (result == true && mounted) {
      _loadNotes();
    }
  }

  Future<void> _deleteNote(Note note) async {
    final l10n = AppLocalizations.of(context)!;
    
    final confirmed = await showDialog<bool>(
      context: context,
      builder: (context) => AlertDialog(
        title: Text(l10n.confirmDelete),
        content: Text(l10n.confirmDeleteNote(note.title.isEmpty ? l10n.untitledNote : note.title)),
        actions: [
          TextButton(
            onPressed: () => Navigator.pop(context, false),
            child: Text(l10n.cancel),
          ),
          TextButton(
            onPressed: () => Navigator.pop(context, true),
            child: Text(l10n.delete, style: const TextStyle(color: Colors.red)),
          ),
        ],
      ),
    );

    if (confirmed == true && mounted) {
      try {
        setState(() {
          _isLoading = true;
        });
        
        final success = await deleteNote(noteId: note.id);
        
        if (mounted) {
          if (success) {
            ScaffoldMessenger.of(context).showSnackBar(
              SnackBar(content: Text(l10n.noteDeleted(note.title.isEmpty ? l10n.untitledNote : note.title))),
            );
            _loadNotes();
          } else {
            setState(() {
              _isLoading = false;
            });
            ScaffoldMessenger.of(context).showSnackBar(
              SnackBar(content: Text(l10n.deleteError)),
            );
          }
        }
      } catch (e) {
        Logger.error('Failed to delete note: $e');
        if (mounted) {
          setState(() {
            _isLoading = false;
          });
          ScaffoldMessenger.of(context).showSnackBar(
            SnackBar(content: Text(l10n.error(e.toString()))),
          );
        }
      }
    }
  }

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;
    
    return Scaffold(
      appBar: AppBar(
        title: Text(widget.notebook.name),
        actions: [
          IconButton(
            icon: const Icon(Icons.refresh),
            onPressed: _loadNotes,
            tooltip: l10n.refresh,
          ),
        ],
      ),
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
                        onPressed: _loadNotes,
                        child: Text(l10n.tryAgain),
                      ),
                    ],
                  ),
                )
              : _notes.isEmpty
                  ? Center(
                      child: Text(l10n.noNotes),
                    )
                  : ListView.builder(
                      itemCount: _notes.length,
                      itemBuilder: (context, index) {
                        final note = _notes[index];
                        return ListTile(
                          title: Text(note.title.isEmpty ? l10n.untitledNote : note.title),
                          subtitle: Text(
                            _getContentPreview(note.content),
                            maxLines: 2,
                            overflow: TextOverflow.ellipsis,
                          ),
                          leading: const Icon(Icons.note),
                          trailing: Row(
                            mainAxisSize: MainAxisSize.min,
                            children: [
                              IconButton(
                                icon: const Icon(Icons.edit),
                                onPressed: () => _editNote(note),
                                tooltip: l10n.edit,
                              ),
                              IconButton(
                                icon: const Icon(Icons.delete),
                                onPressed: () => _deleteNote(note),
                                tooltip: l10n.delete,
                              ),
                            ],
                          ),
                          onTap: () => _editNote(note),
                        );
                      },
                    ),
      floatingActionButton: FloatingActionButton(
        onPressed: _createNote,
        tooltip: l10n.newNote,
        child: const Icon(Icons.add),
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
