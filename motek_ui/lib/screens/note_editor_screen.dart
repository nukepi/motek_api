import 'dart:convert';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_quill/flutter_quill.dart';
import 'package:motek_ui/l10n/app_localizations.dart';
import 'package:motek_ui/src/rust/api/endpoint.dart';
import 'package:motek_ui/src/rust/api_handlers/notebooks.dart';
import 'package:motek_ui/utils/logger.dart';
import 'package:motek_ui/widgets/components/tag_selector.dart';

class NoteEditorScreen extends StatefulWidget {
  final String? initialContent;
  final String? initialTitle;
  final List<String> initialTags;
  final String? initialNotebookId;
  final Function(String title, String content) onSave;

  const NoteEditorScreen({
    Key? key,
    this.initialContent,
    this.initialTitle,
    this.initialTags = const [],
    this.initialNotebookId,
    required this.onSave,
  }) : super(key: key);

  @override
  State<NoteEditorScreen> createState() => _NoteEditorScreenState();
}

class _NoteEditorScreenState extends State<NoteEditorScreen> {
  final TextEditingController _titleController = TextEditingController();
  late QuillController _quillController;
  bool _isLoading = false;
  final FocusNode _editorFocusNode = FocusNode();
  final ScrollController _scrollController = ScrollController();
  bool _hasUnsavedChanges = false;
  final FocusNode _keyboardFocusNode = FocusNode();

  // Fields for tags and notebooks
  List<String> _selectedTags = [];
  String? _selectedNotebookId;
  List<Notebook> _notebooks = [];
  bool _isLoadingNotebooks = false;

  @override
  void initState() {
    super.initState();

    if (widget.initialTitle != null) {
      _titleController.text = widget.initialTitle!;
    }

    // Inicjalizacja kontrolera Quill z bezpiecznym parsowaniem JSON
    if (widget.initialContent != null && widget.initialContent!.isNotEmpty) {
      try {
        final jsonData = json.decode(widget.initialContent!);
        if (jsonData is List) {
          _quillController = QuillController(
            document: Document.fromJson(jsonData),
            selection: const TextSelection.collapsed(offset: 0),
          );
        } else {
          _quillController = QuillController(
            document: Document()..insert(0, widget.initialContent!),
            selection: const TextSelection.collapsed(offset: 0),
          );
        }
      } catch (e) {
        _quillController = QuillController(
          document: Document()..insert(0, widget.initialContent ?? ''),
          selection: const TextSelection.collapsed(offset: 0),
        );
      }
    } else {
      _quillController = QuillController.basic();
    }

    // Nasłuchiwanie zmian w edytorze
    _quillController.document.changes.listen((event) {
      if (!_hasUnsavedChanges) {
        setState(() {
          _hasUnsavedChanges = true;
        });
      }
    });

    // Nasłuchiwanie zmian w tytule
    _titleController.addListener(() {
      if (!_hasUnsavedChanges) {
        setState(() {
          _hasUnsavedChanges = true;
        });
      }
    });

    // Inicjalizacja tagów i notatnika
    _selectedTags = List.from(widget.initialTags);
    _selectedNotebookId = widget.initialNotebookId;

    // Ładowanie notatników
    _loadNotebooks();
  }

  Future<void> _loadNotebooks() async {
    setState(() {
      _isLoadingNotebooks = true;
    });

    try {
      final notebooks = await listNotebooks();
      if (mounted) {
        setState(() {
          _notebooks = notebooks;
          _isLoadingNotebooks = false;
        });
      }
    } catch (e) {
      Logger.error('Failed to load notebooks: $e');
      if (mounted) {
        setState(() {
          _isLoadingNotebooks = false;
        });
      }
    }
  }

  Future<List<String>> _getAllTags() async {
    // Example tags - in the future, an API for fetching tags can be added
    return ['important', 'work', 'personal', 'ideas', 'todo'];
  }

  @override
  void dispose() {
    _titleController.dispose();
    _quillController.dispose();
    _editorFocusNode.dispose();
    _scrollController.dispose();
    _keyboardFocusNode.dispose();
    super.dispose();
  }

  void _saveNote() {
    final title = _titleController.text.trim();
    final content = jsonEncode(_quillController.document.toDelta().toJson());
    widget.onSave(title, content);
    setState(() {
      _hasUnsavedChanges = false;
    });

    final l10n = AppLocalizations.of(context)!;
    ScaffoldMessenger.of(
      context,
    ).showSnackBar(SnackBar(content: Text(l10n.saveNote)));
  }

  // Keyboard shortcut handling
  KeyEventResult _handleKeyEvent(FocusNode node, KeyEvent event) {
    // Check if it's Ctrl+S
    if (event is KeyDownEvent &&
        event.logicalKey == LogicalKeyboardKey.keyS &&
        (HardwareKeyboard.instance.isControlPressed ||
            HardwareKeyboard.instance.isMetaPressed)) {
      _saveNote();
      return KeyEventResult.handled;
    }
    return KeyEventResult.ignored;
  }

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;

    return PopScope(
      canPop: true,
      onPopInvokedWithResult: (didPop, result) async {
        if (!didPop && _hasUnsavedChanges) {
          final shouldDiscard = await showDialog<bool>(
            context: context,
            builder:
                (context) => AlertDialog(
                  title: Text(l10n.unsavedChanges),
                  content: Text(l10n.discardChanges),
                  actions: [
                    TextButton(
                      onPressed: () => Navigator.pop(context, false),
                      child: Text(l10n.cancel),
                    ),
                    TextButton(
                      onPressed: () => Navigator.pop(context, true),
                      child: Text(l10n.discard),
                    ),
                    TextButton(
                      onPressed: () {
                        _saveNote();
                        Navigator.pop(context, true);
                      },
                      child: Text(l10n.save),
                    ),
                  ],
                ),
          );
          if (shouldDiscard != true) return;
        }
      },
      child: KeyboardListener(
        focusNode: _keyboardFocusNode,
        onKeyEvent: (event) {
          _handleKeyEvent(_keyboardFocusNode, event);
        },
        child: Scaffold(
          appBar: AppBar(
            title: Text(l10n.editNote),
            actions: [
              IconButton(
                icon: const Icon(Icons.save),
                onPressed: _saveNote,
                tooltip: l10n.save,
              ),
            ],
          ),
          body:
              _isLoading
                  ? const Center(child: CircularProgressIndicator())
                  : Column(
                    children: [
                      Padding(
                        padding: const EdgeInsets.all(8.0),
                        child: TextField(
                          controller: _titleController,
                          decoration: InputDecoration(
                            hintText: l10n.noteTitle,
                            border: InputBorder.none,
                            contentPadding: const EdgeInsets.all(8),
                          ),
                          style: Theme.of(context).textTheme.headlineSmall,
                        ),
                      ),
                      const Divider(),
                      Expanded(
                        child: Padding(
                          padding: const EdgeInsets.all(8.0),
                          child: QuillEditor(
                            focusNode: _editorFocusNode,
                            controller: _quillController,
                            scrollController: _scrollController,
                            config: QuillEditorConfig(
                              scrollable: true,
                              autoFocus: false,
                              placeholder: l10n.startTyping,
                              expands: true,
                              padding: const EdgeInsets.all(8),
                              customStyles: DefaultStyles(
                                h1: DefaultTextBlockStyle(
                                  Theme.of(context).textTheme.headlineLarge!,
                                  const HorizontalSpacing(16, 0),
                                  const VerticalSpacing(16, 0),
                                  const VerticalSpacing(0, 0),
                                  null,
                                ),
                                h2: DefaultTextBlockStyle(
                                  Theme.of(context).textTheme.headlineMedium!,
                                  const HorizontalSpacing(16, 0),
                                  const VerticalSpacing(16, 0),
                                  const VerticalSpacing(0, 0),
                                  null,
                                ),
                                h3: DefaultTextBlockStyle(
                                  Theme.of(context).textTheme.headlineSmall!,
                                  const HorizontalSpacing(16, 0),
                                  const VerticalSpacing(16, 0),
                                  const VerticalSpacing(0, 0),
                                  null,
                                ),
                                paragraph: DefaultTextBlockStyle(
                                  Theme.of(context).textTheme.bodyLarge!,
                                  const HorizontalSpacing(0, 0),
                                  const VerticalSpacing(0, 0),
                                  const VerticalSpacing(0, 0),
                                  null,
                                ),
                              ),
                            ),
                          ),
                        ),
                      ),
                      const Divider(),
                      // Notebook and tag selection section
                      Padding(
                        padding: const EdgeInsets.all(8.0),
                        child: Column(
                          crossAxisAlignment: CrossAxisAlignment.start,
                          children: [
                            // Notebook selection
                            DropdownButtonFormField<String?>(
                              decoration: InputDecoration(
                                labelText: l10n.notebook,
                                border: const OutlineInputBorder(),
                              ),
                              value: _selectedNotebookId,
                              items: [
                                DropdownMenuItem<String?>(
                                  value: null,
                                  child: Text(l10n.noNotebook),
                                ),
                                ..._notebooks.map(
                                  (notebook) => DropdownMenuItem<String?>(
                                    value: notebook.id,
                                    child: Text(notebook.name),
                                  ),
                                ),
                              ],
                              onChanged:
                                  _isLoadingNotebooks
                                      ? null
                                      : (value) {
                                        setState(() {
                                          _selectedNotebookId = value;
                                          _hasUnsavedChanges = true;
                                        });
                                      },
                            ),
                            const SizedBox(height: 16),
                            // Tag selection
                            TagSelector(
                              initialTags: _selectedTags,
                              onTagsChanged: (tags) {
                                setState(() {
                                  _selectedTags = tags;
                                  _hasUnsavedChanges = true;
                                });
                              },
                              getAllTagsCallback: _getAllTags,
                            ),
                          ],
                        ),
                      ),
                    ],
                  ),
          bottomNavigationBar: QuillSimpleToolbar(
            controller: _quillController,
            config: QuillSimpleToolbarConfig(
              showFontFamily: false,
              showFontSize: false,
              showBackgroundColorButton: false,
              showClearFormat: false,
              showCodeBlock: false,
              showInlineCode: false,
              showColorButton: true,
              showLink: true,
              showDividers: false,
              showQuote: true,
              showListCheck: true,
              showIndent: true,
              showSearchButton: false,
              showSubscript: false,
              showSuperscript: false,
              multiRowsDisplay: false,
              showAlignmentButtons: true,
              toolbarSectionSpacing: 4,
              toolbarIconAlignment: WrapAlignment.start,
            ),
          ),
        ),
      ),
    );
  }
}
