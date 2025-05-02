import 'package:flutter/material.dart';
import 'package:motek_ui/l10n/app_localizations.dart';
import 'package:motek_ui/src/rust/api/endpoint.dart';
import 'package:motek_ui/src/rust/api_handlers/notebooks.dart';

class NotebookContent extends StatefulWidget {
  const NotebookContent({super.key});

  @override
  State<NotebookContent> createState() => _NotebookContentState();
}

class _NotebookContentState extends State<NotebookContent> {
  bool _isLoading = true;
  List<Notebook> _notebooks = [];
  String? _errorMessage;

  @override
  void initState() {
    super.initState();
    _loadNotebooks();
  }

  Future<void> _loadNotebooks() async {
    final l10n = AppLocalizations.of(context)!;
    try {
      setState(() {
        _isLoading = true;
        _errorMessage = null;
      });
      
      final notebooks = await listNotebooks();
      
      setState(() {
        _notebooks = notebooks;
        _isLoading = false;
      });
    } catch (e) {
      setState(() {
        _errorMessage = l10n.loadNotebooksError(e.toString());
        _isLoading = false;
      });
    }
  }

  Future<void> _createNotebook() async {
    final l10n = AppLocalizations.of(context)!;
    final nameController = TextEditingController();
    
    return showDialog(
      context: context,
      builder: (context) => AlertDialog(
        title: Text(l10n.newNotebook),
        content: TextField(
          controller: nameController,
          decoration: InputDecoration(
            labelText: l10n.notebookName,
          ),
          autofocus: true,
        ),
        actions: [
          TextButton(
            onPressed: () => Navigator.pop(context),
            child: Text(l10n.cancel),
          ),
          TextButton(
            onPressed: () async {
              if (nameController.text.isNotEmpty) {
                Navigator.pop(context);
                try {
                  await createNotebook(name: nameController.text, parentId: null);
                  _loadNotebooks(); // Odśwież listę
                } catch (e) {
                  if (mounted) {
                    ScaffoldMessenger.of(context).showSnackBar(
                      SnackBar(content: Text(l10n.error(e.toString()))),
                    );
                  }
                }
              }
            },
            child: Text(l10n.create),
          ),
        ],
      ),
    );
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
            child: Text(l10n.delete, style: const TextStyle(color: Colors.red)),
          ),
        ],
      ),
    ) ?? false;

    if (confirmed) {
      try {
        final success = await deleteNotebook(notebookId: notebook.id);
        if (success) {
          _loadNotebooks(); // Odśwież listę
        } else {
          if (mounted) {
            ScaffoldMessenger.of(context).showSnackBar(
              SnackBar(content: Text(l10n.notebookDeleteError)),
            );
          }
        }
      } catch (e) {
        if (mounted) {
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
    
    return showDialog(
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
            onPressed: () => Navigator.pop(context),
            child: Text(l10n.cancel),
          ),
          TextButton(
            onPressed: () async {
              if (nameController.text.isNotEmpty) {
                Navigator.pop(context);
                try {
                  await updateNotebook(
                    notebookId: notebook.id, 
                    name: nameController.text,
                    parentId: null
                  );
                  _loadNotebooks(); // Odśwież listę
                } catch (e) {
                  if (mounted) {
                    ScaffoldMessenger.of(context).showSnackBar(
                      SnackBar(content: Text(l10n.error(e.toString()))),
                    );
                  }
                }
              }
            },
            child: Text(l10n.save),
          ),
        ],
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;
    
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
            ElevatedButton(
              onPressed: _loadNotebooks,
              child: Text(l10n.tryAgain),
            ),
          ],
        ),
      );
    }

    return Scaffold(
      body: _notebooks.isEmpty
          ? Center(child: Text(l10n.noNotebooks))
          : ListView.builder(
              itemCount: _notebooks.length,
              itemBuilder: (context, index) {
                final notebook = _notebooks[index];
                return Card(
                  margin: const EdgeInsets.symmetric(horizontal: 16, vertical: 8),
                  child: ListTile(
                    title: Text(
                      notebook.name,
                      style: const TextStyle(fontWeight: FontWeight.bold),
                    ),
                    subtitle: Text(
                      l10n.created(_formatDate(DateTime.fromMicrosecondsSinceEpoch(notebook.createdAt))),
                      style: const TextStyle(fontSize: 12),
                    ),
                    trailing: Row(
                      mainAxisSize: MainAxisSize.min,
                      children: [
                        IconButton(
                          icon: const Icon(Icons.edit, color: Colors.blue),
                          onPressed: () => _editNotebook(notebook),
                        ),
                        IconButton(
                          icon: const Icon(Icons.delete, color: Colors.red),
                          onPressed: () => _deleteNotebook(notebook),
                        ),
                      ],
                    ),
                    onTap: () {
                      ScaffoldMessenger.of(context).showSnackBar(
                        SnackBar(content: Text(l10n.notebookOpened(notebook.name))),
                      );
                    },
                  ),
                );
              },
            ),
      floatingActionButton: FloatingActionButton(
        onPressed: _createNotebook,
        backgroundColor: Colors.amber,
        child: const Icon(Icons.add),
      ),
    );
  }

  String _formatDate(DateTime date) {
    return '${date.day}.${date.month}.${date.year}';
  }
}
