import 'package:flutter/material.dart';
import 'package:motek_ui/l10n/app_localizations.dart';
import 'package:motek_ui/src/rust/api_handlers/notebooks.dart';
import 'package:motek_ui/src/rust/api/endpoint.dart';
import 'package:motek_ui/utils/logger.dart';
import 'package:motek_ui/screens/notebook_notes_screen.dart';

class NotebooksScreen extends StatefulWidget {
  const NotebooksScreen({Key? key}) : super(key: key);

  @override
  _NotebooksScreenState createState() => _NotebooksScreenState();
}

class _NotebooksScreenState extends State<NotebooksScreen> {
  List<Notebook> _notebooks = [];
  bool _isLoading = true;
  String? _errorMessage;

  @override
  void initState() {
    super.initState();
    _loadNotebooks();
  }

  Future<void> _loadNotebooks() async {
    if (!mounted) return;
    
    setState(() {
      _isLoading = true;
      _errorMessage = null;
    });

    try {
      final notebooks = await listNotebooks();
      
      if (!mounted) return;
      
      setState(() {
        _notebooks = notebooks;
        _isLoading = false;
      });
    } catch (e) {
      Logger.error('Failed to load notebooks: $e');
      if (!mounted) return;
      
      setState(() {
        _errorMessage = e.toString();
        _isLoading = false;
      });
    }
  }

  Future<void> _createNotebook() async {
    final l10n = AppLocalizations.of(context)!;
    final nameController = TextEditingController();
    
    final result = await showDialog<bool>(
      context: context,
      builder: (context) => AlertDialog(
        title: Text(l10n.newNotebook),
        content: TextField(
          controller: nameController,
          decoration: InputDecoration(
            labelText: l10n.notebookName,
            hintText: 'My Notebook',
          ),
          autofocus: true,
        ),
        actions: [
          TextButton(
            onPressed: () => Navigator.pop(context, false),
            child: Text(l10n.cancel),
          ),
          TextButton(
            onPressed: () => Navigator.pop(context, true),
            child: Text(l10n.create),
          ),
        ],
      ),
    );

    if (result == true && nameController.text.isNotEmpty && mounted) {
      try {
        setState(() {
          _isLoading = true;
        });
        
        await createNotebook(name: nameController.text, parentId: null);
        
        if (mounted) {
          _loadNotebooks();
          ScaffoldMessenger.of(context).showSnackBar(
            SnackBar(content: Text('Notebook "${nameController.text}" created')),
          );
        }
      } catch (e) {
        Logger.error('Failed to create notebook: $e');
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

  Future<void> _editNotebook(Notebook notebook) async {
    final l10n = AppLocalizations.of(context)!;
    final nameController = TextEditingController(text: notebook.name);
    
    final result = await showDialog<bool>(
      context: context,
      builder: (context) => AlertDialog(
        title: Text(l10n.editNotebook),
        content: TextField(
          controller: nameController,
          decoration: InputDecoration(
            labelText: l10n.notebookName,
          ),
          autofocus: true,
        ),
        actions: [
          TextButton(
            onPressed: () => Navigator.pop(context, false),
            child: Text(l10n.cancel),
          ),
          TextButton(
            onPressed: () => Navigator.pop(context, true),
            child: Text(l10n.save),
          ),
        ],
      ),
    );

    if (result == true && nameController.text.isNotEmpty && mounted) {
      try {
        setState(() {
          _isLoading = true;
        });
        
        await updateNotebook(
          notebookId: notebook.id,
          name: nameController.text,
          parentId: null,
        );
        
        if (mounted) {
          _loadNotebooks();
          ScaffoldMessenger.of(context).showSnackBar(
            SnackBar(content: Text('Notebook updated')),
          );
        }
      } catch (e) {
        Logger.error('Failed to update notebook: $e');
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

  Future<void> _deleteNotebook(Notebook notebook) async {
    final l10n = AppLocalizations.of(context)!;
    
    final confirmed = await showDialog<bool>(
      context: context,
      builder: (context) => AlertDialog(
        title: Text(l10n.confirmDelete),
        content: Text(l10n.confirmDeleteNotebook(notebook.name)),
        actions: [
          TextButton(
            onPressed: () => Navigator.pop(context, false),
            child: Text(l10n.cancel),
          ),
          TextButton(
            onPressed: () => Navigator.pop(context, true),
            child: Text(l10n.delete, style: TextStyle(color: Colors.red)),
          ),
        ],
      ),
    );

    if (confirmed == true && mounted) {
      try {
        setState(() {
          _isLoading = true;
        });
        
        final success = await deleteNotebook(notebookId: notebook.id);
        
        if (mounted) {
          if (success) {
            ScaffoldMessenger.of(context).showSnackBar(
              SnackBar(content: Text(l10n.notebookDeleted(notebook.name))),
            );
            _loadNotebooks();
          } else {
            setState(() {
              _isLoading = false;
            });
            ScaffoldMessenger.of(context).showSnackBar(
              SnackBar(content: Text(l10n.notebookDeleteError)),
            );
          }
        }
      } catch (e) {
        Logger.error('Failed to delete notebook: $e');
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

  void _openNotebookNotes(Notebook notebook) {
    Navigator.push(
      context,
      MaterialPageRoute(
        builder: (context) => NotebookNotesScreen(notebook: notebook),
      ),
    ).then((_) => _loadNotebooks());
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
                        onPressed: _loadNotebooks,
                        child: Text(l10n.tryAgain),
                      ),
                    ],
                  ),
                )
              : _notebooks.isEmpty
                  ? Center(
                      child: Text(l10n.noNotebooks),
                    )
                  : ListView.builder(
                      itemCount: _notebooks.length,
                      itemBuilder: (context, index) {
                        final notebook = _notebooks[index];
                        return ListTile(
                          title: Text(notebook.name),
                          subtitle: Text(
                            _formatDate(DateTime.fromMicrosecondsSinceEpoch(notebook.createdAt)),
                          ),
                          leading: const Icon(Icons.book),
                          trailing: Row(
                            mainAxisSize: MainAxisSize.min,
                            children: [
                              IconButton(
                                icon: const Icon(Icons.edit),
                                onPressed: () => _editNotebook(notebook),
                                tooltip: l10n.edit,
                              ),
                              IconButton(
                                icon: const Icon(Icons.delete),
                                onPressed: () => _deleteNotebook(notebook),
                                tooltip: l10n.delete,
                              ),
                            ],
                          ),
                          onTap: () => _openNotebookNotes(notebook),
                        );
                      },
                    ),
      floatingActionButton: FloatingActionButton(
        onPressed: _createNotebook,
        tooltip: l10n.newNotebook,
        child: const Icon(Icons.add),
      ),
    );
  }

  String _formatDate(DateTime date) {
    return '${date.day}.${date.month}.${date.year}';
  }
}
