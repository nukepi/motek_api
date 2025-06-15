import 'package:flutter/material.dart';
import 'package:motek_ui/utils/logger.dart';

class TagSelector extends StatefulWidget {
  final List<String> initialTags;
  final Function(List<String>) onTagsChanged;
  final Future<List<String>> Function()? getAllTagsCallback;

  const TagSelector({
    Key? key,
    required this.initialTags,
    required this.onTagsChanged,
    this.getAllTagsCallback,
  }) : super(key: key);

  @override
  _TagSelectorState createState() => _TagSelectorState();
}

class _TagSelectorState extends State<TagSelector> {
  late List<String> _selectedTags;
  final TextEditingController _tagController = TextEditingController();
  List<String> _availableTags = [];
  bool _isLoading = false;

  @override
  void initState() {
    super.initState();
    _selectedTags = List.from(widget.initialTags);
    _loadAvailableTags();
  }

  Future<void> _loadAvailableTags() async {
    if (widget.getAllTagsCallback == null) return;

    setState(() {
      _isLoading = true;
    });

    try {
      final tags = await widget.getAllTagsCallback!();
      if (mounted) {
        setState(() {
          _availableTags = tags;
          _isLoading = false;
        });
      }
    } catch (e) {
      Logger.error('Failed to load available tags: $e');
      if (mounted) {
        setState(() {
          _isLoading = false;
        });
      }
    }
  }

  void _addTag(String tag) {
    if (tag.isEmpty) return;
    
    if (!_selectedTags.contains(tag)) {
      setState(() {
        _selectedTags.add(tag);
        _tagController.clear();
      });
      widget.onTagsChanged(_selectedTags);
    }
  }

  void _removeTag(String tag) {
    setState(() {
      _selectedTags.remove(tag);
    });
    widget.onTagsChanged(_selectedTags);
  }

  @override
  Widget build(BuildContext context) {
    
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Text(
          'Tags',
          style: Theme.of(context).textTheme.titleMedium,
        ),
        const SizedBox(height: 8),
        Wrap(
          spacing: 8,
          runSpacing: 4,
          children: _selectedTags.map((tag) => Chip(
            label: Text(tag),
            deleteIcon: const Icon(Icons.close, size: 18),
            onDeleted: () => _removeTag(tag),
          )).toList(),
        ),
        const SizedBox(height: 8),
        Row(
          children: [
            Expanded(
              child: TextField(
                controller: _tagController,
                decoration: InputDecoration(
                  hintText: 'Add tag...',
                  border: OutlineInputBorder(
                    borderRadius: BorderRadius.circular(8),
                  ),
                  isDense: true,
                  contentPadding: const EdgeInsets.symmetric(horizontal: 12, vertical: 8),
                ),
                onSubmitted: _addTag,
              ),
            ),
            const SizedBox(width: 8),
            ElevatedButton(
              onPressed: () => _addTag(_tagController.text),
              style: ElevatedButton.styleFrom(
                padding: const EdgeInsets.symmetric(vertical: 10),
              ),
              child: const Text('Add'),
            ),
          ],
        ),
        if (_availableTags.isNotEmpty) ...[
          const SizedBox(height: 16),
          Text(
            'Suggested tags:',
            style: Theme.of(context).textTheme.bodySmall,
          ),
          const SizedBox(height: 4),
          _isLoading
              ? const Center(child: CircularProgressIndicator(strokeWidth: 2))
              : Wrap(
                  spacing: 8,
                  runSpacing: 4,
                  children: _availableTags
                      .where((tag) => !_selectedTags.contains(tag))
                      .take(10)
                      .map((tag) => ActionChip(
                        label: Text(tag),
                        onPressed: () => _addTag(tag),
                      ))
                      .toList(),
                ),
        ],
      ],
    );
  }

  @override
  void dispose() {
    _tagController.dispose();
    super.dispose();
  }
}
