import 'dart:convert';
import 'package:flutter/material.dart';
import 'package:motek_ui/l10n/app_localizations.dart';
import 'package:motek_ui/src/rust/api_handlers/notes.dart';
import 'package:motek_ui/src/rust/api/endpoint.dart';
import 'package:motek_ui/services/auth_service.dart';
import 'package:provider/provider.dart';
import 'package:motek_ui/screens/note_editor_screen.dart';
import 'package:motek_ui/models/content_type.dart';
import 'package:motek_ui/screens/main_layout.dart';
import 'dart:async';

class NotesContent extends StatefulWidget {
  const NotesContent({super.key});

  @override
  State<NotesContent> createState() => _NotesContentState();
}

class _NotesContentState extends State<NotesContent> {
  bool _isLoading = true;
  List<Note> _notes = [];
  String? _errorMessage;
  Timer? _refreshTimer;

  @override
  void initState() {
    super.initState();
    _refreshTimer = Timer.periodic(const Duration(seconds: 30), (_) {
      if (mounted) {
        _loadNotes();
      }
    });
    _loadNotes();
  }

  @override
  void dispose() {
    _refreshTimer?.cancel();
    super.dispose();
  }

  Future<void> _loadNotes() async {
    // Check if the user is logged in
    final authService = Provider.of<AuthService>(context, listen: false);
    if (!authService.isLoggedIn) {
      setState(() {
        _isLoading = false;
      });
      return;
    }

    setState(() {
      _isLoading = true;
      _errorMessage = null;
    });

    try {
      final notes = await listNotes();

      if (mounted) {
        setState(() {
          _notes = notes;
          _isLoading = false;
        });
      }
    } catch (e) {
      if (mounted) {
        final l10n = AppLocalizations.of(context)!;
        setState(() {
          _errorMessage = l10n.loadNotesError(e.toString());
          _isLoading = false;
        });
      }
    }
  }

  // Function to parse note content to plain text
  String _getPlainTextFromContent(String content) {
    try {
      final jsonData = json.decode(content);

      if (jsonData is List) {
        // Delta format from Quill editor
        String plainText = '';
        for (var op in jsonData) {
          if (op is Map && op.containsKey('insert')) {
            plainText += op['insert'].toString();
          }
        }
        return plainText;
      }

      return jsonData.toString();
    } catch (e) {
      return content;
    }
  }

  Future<void> _addNewNote() async {
    final result = await Navigator.push(
      context,
      MaterialPageRoute(
        builder:
            (context) => NoteEditorScreen(
              onSave: (title, content) async {
                try {
                  await createNote(title: title, content: content);
                  return true;
                } catch (e) {
                  if (mounted) {
                    final l10n = AppLocalizations.of(context)!;
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
    final result = await Navigator.push(
      context,
      MaterialPageRoute(
        builder:
            (context) => NoteEditorScreen(
              initialTitle: note.title,
              initialContent: note.content,
              onSave: (title, content) async {
                try {
                  await updateNote(
                    noteId: note.id,
                    title: title,
                    content: content,
                  );
                  return true;
                } catch (e) {
                  if (mounted) {
                    final l10n = AppLocalizations.of(context)!;
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
      builder:
          (context) => AlertDialog(
            title: Text(l10n.deleteNoteConfirmTitle),
            content: Text(l10n.deleteNoteConfirmMessage(note.title)),
            actions: [
              TextButton(
                onPressed: () => Navigator.of(context).pop(false),
                child: Text(l10n.cancel),
              ),
              TextButton(
                onPressed: () => Navigator.of(context).pop(true),
                child: Text(
                  l10n.delete,
                  style: const TextStyle(color: Colors.red),
                ),
              ),
            ],
          ),
    );

    if (confirmed == true) {
      setState(() {
        _isLoading = true;
      });

      try {
        final success = await deleteNote(noteId: note.id);
        if (mounted) {
          if (success) {
            ScaffoldMessenger.of(context).showSnackBar(
              SnackBar(content: Text(l10n.noteDeleted(note.title))),
            );
            _loadNotes();
          } else {
            setState(() {
              _isLoading = false;
            });
            ScaffoldMessenger.of(
              context,
            ).showSnackBar(SnackBar(content: Text(l10n.deleteNoteFailed)));
          }
        }
      } catch (e) {
        if (mounted) {
          setState(() {
            _isLoading = false;
          });
          ScaffoldMessenger.of(
            context,
          ).showSnackBar(SnackBar(content: Text(l10n.error(e.toString()))));
        }
      }
    }
  }

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;
    final authService = Provider.of<AuthService>(context);

    // Check if the user is logged in
    if (!authService.isLoggedIn) {
      return Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            Text(l10n.loginRequired),
            const SizedBox(height: 16),
            ElevatedButton(
              onPressed: () {
                // Przejdź do ekranu logowania
                final mainLayout =
                    context.findAncestorStateOfType<State<MainLayout>>();
                if (mainLayout != null) {
                  final mainLayoutState = mainLayout as dynamic;
                  mainLayoutState.changeContent(ContentType.login);
                }
              },
              child: Text(l10n.login),
            ),
          ],
        ),
      );
    }

    if (_isLoading) {
      return const Center(child: CircularProgressIndicator());
    }

    if (_errorMessage != null) {
      return Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            Text(_errorMessage!, style: const TextStyle(color: Colors.red)),
            const SizedBox(height: 16),
            ElevatedButton(onPressed: _loadNotes, child: Text(l10n.tryAgain)),
          ],
        ),
      );
    }

    return Scaffold(
      appBar: AppBar(
        title: Text(l10n.notes),
        actions: [
          IconButton(
            icon: const Icon(Icons.refresh),
            onPressed: () => _loadNotes(),
            tooltip: l10n.refresh,
          ),
        ],
      ),
      body:
          _notes.isEmpty
              ? Center(child: Text(l10n.noNotes))
              : ListView.builder(
                itemCount: _notes.length,
                itemBuilder: (context, index) {
                  final note = _notes[index];
                  // Parse note content to plain text
                  final plainTextContent = _getPlainTextFromContent(
                    note.content,
                  );

                  return Dismissible(
                    key: Key(note.id),
                    background: Container(
                      color: Colors.red,
                      alignment: Alignment.centerRight,
                      padding: const EdgeInsets.only(right: 20),
                      child: const Icon(Icons.delete, color: Colors.white),
                    ),
                    direction: DismissDirection.endToStart,
                    confirmDismiss: (_) async {
                      await _deleteNote(note);
                      return false; // Nie usuwamy elementu z listy, bo _loadNotes() to zrobi
                    },
                    child: ListTile(
                      title: Text(note.title),
                      subtitle: Text(
                        plainTextContent.length > 50
                            ? '${plainTextContent.substring(0, 50)}...'
                            : plainTextContent,
                        maxLines: 1,
                        overflow: TextOverflow.ellipsis,
                      ),
                      trailing: Row(
                        mainAxisSize: MainAxisSize.min,
                        children: [
                          IconButton(
                            icon: const Icon(Icons.edit),
                            onPressed: () => _editNote(note),
                          ),
                          IconButton(
                            icon: const Icon(Icons.delete),
                            onPressed: () => _deleteNote(note),
                          ),
                        ],
                      ),
                      onTap: () => _editNote(note),
                    ),
                  );
                },
              ),
      floatingActionButton: FloatingActionButton(
        onPressed: _addNewNote,
        backgroundColor: Colors.amber,
        child: const Icon(Icons.add),
      ),
    );
  }
}
