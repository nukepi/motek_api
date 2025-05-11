import 'dart:convert';
import 'package:flutter/material.dart';
import 'package:flutter_quill/flutter_quill.dart';
import 'package:motek_ui/l10n/app_localizations.dart';
import 'package:motek_ui/src/rust/api_handlers/notes.dart';
import 'package:motek_ui/screens/note_editor_screen.dart';
import 'package:motek_ui/services/note_sevice.dart';
import 'package:provider/provider.dart';

class NotesScreen extends StatefulWidget {
  final String notebookId;
  final String notebookName;

  const NotesScreen({
    Key? key,
    required this.notebookId,
    required this.notebookName,
  }) : super(key: key);

  @override
  State<NotesScreen> createState() => _NotesScreenState();
}

class _NotesScreenState extends State<NotesScreen> {
  List<Note> _notes = [];
  bool _isLoading = false;
  String? _errorMessage;

  @override
  void initState() {
    super.initState();
    _loadNotes();
  }

  Future<void> _loadNotes() async {
    setState(() {
      _isLoading = true;
      _errorMessage = null;
    });

    try {
      final noteService = Provider.of<NoteService>(context, listen: false);
      final notes = await noteService.getNotes(widget.notebookId);
      
      setState(() {
        _notes = notes;
        _isLoading = false;
      });
    } catch (e) {
      setState(() {
        _errorMessage = e.toString();
        _isLoading = false;
      });
    }
  }

  void _openNoteEditor(Note? note) async {
    final noteService = Provider.of<NoteService>(context, listen: false);
    
    await Navigator.push(
      context,
      MaterialPageRoute(
        builder: (context) => NoteEditorScreen(
          initialTitle: note?.title,
          initialContent: note != null ? note.content : null,
          onSave: (title, content) async {
            if (note == null) {
              // Tworzenie nowej notatki
              await noteService.createNote(
                title: title,
                content: content,
                notebookId: widget.notebookId,
              );
            } else {
              // Aktualizacja istniejącej notatki
              await noteService.updateNote(
                noteId: note.id,
                title: title,
                content: content,
              );
            }
            // Odświeżenie listy notatek
            if (mounted) {
              _loadNotes();
            }
          },
        ),
      ),
    );
  }

  Future<void> _deleteNote(Note note) async {
    final l10n = AppLocalizations.of(context)!;
    
    final confirmed = await showDialog<bool>(
      context: context,
      builder: (context) => AlertDialog(
        title: Text(l10n.deleteNote),
        content: Text(l10n.confirmDeleteNote(note.title.isEmpty ? l10n.untitledNote : note.title)),
        actions: [
          TextButton(
            onPressed: () => Navigator.pop(context, false),
            child: Text(l10n.cancel),
          ),
          TextButton(
            onPressed: () => Navigator.pop(context, true),
            child: Text(l10n.delete),
          ),
        ],
      ),
    );

    if (confirmed == true) {
      setState(() {
        _isLoading = true;
      });

      try {
        final noteService = Provider.of<NoteService>(context, listen: false);
        await noteService.deleteNote(note.id);
        
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(content: Text(l10n.noteDeleted(note.title))),
        );
        
        _loadNotes();
      } catch (e) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(content: Text(l10n.error(e.toString()))),
        );
        setState(() {
          _isLoading = false;
        });
      }
    }
  }

  String _getContentPreview(String content) {
    try {
      final json = jsonDecode(content);
      final document = Document.fromJson(json);
      final plainText = document.toPlainText();
      return plainText.length > 50 ? plainText.substring(0, 50) + '...' : plainText;
    } catch (e) {
      return content.length > 50 ? content.substring(0, 50) + '...' : content;
    }
  }

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;
    
    return Scaffold(
      appBar: AppBar(
        title: Text(widget.notebookName),
      ),
      body: _isLoading
          ? const Center(child: CircularProgressIndicator())
          : _errorMessage != null
              ? Center(
                  child: Column(
                    mainAxisAlignment: MainAxisAlignment.center,
                    children: [
                      Text(
                        l10n.loadNotesError(_errorMessage!),
                        style: const TextStyle(fontSize: 18),
                        textAlign: TextAlign.center,
                      ),
                      const SizedBox(height: 16),
                      ElevatedButton(
                        onPressed: _loadNotes,
                        child: Text(l10n.retry),
                      ),
                    ],
                  ),
                )
              : _notes.isEmpty
                  ? Center(
                      child: Text(
                        l10n.noNotes,
                        style: const TextStyle(fontSize: 18),
                      ),
                    )
                  : ListView.builder(
                      itemCount: _notes.length,
                      itemBuilder: (context, index) {
                        final note = _notes[index];
                        return Dismissible(
                          key: Key(note.id),
                          background: Container(
                            color: Colors.red,
                            alignment: Alignment.centerRight,
                            padding: const EdgeInsets.only(right: 16.0),
                            child: const Icon(
                              Icons.delete,
                              color: Colors.white,
                            ),
                          ),
                          direction: DismissDirection.endToStart,
                          confirmDismiss: (_) async {
                            await _deleteNote(note);
                            return false; // Zawsze zwracamy false, bo sami obsługujemy usuwanie
                          },
                          child: ListTile(
                            title: Text(
                              note.title.isEmpty ? l10n.untitledNote : note.title,
                              style: const TextStyle(fontWeight: FontWeight.bold),
                            ),
                            subtitle: Text(_getContentPreview(note.content)),
                            onTap: () => _openNoteEditor(note),
                          ),
                        );
                      },
                    ),
      floatingActionButton: FloatingActionButton(
        onPressed: () => _openNoteEditor(null),
        tooltip: l10n.newNote,
        child: const Icon(Icons.add),
      ),
    );
  }
}
