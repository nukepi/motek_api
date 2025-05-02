import 'package:flutter/material.dart';
import '../../models/content_type.dart';

class CustomFooter extends StatelessWidget {
  final Function(ContentType) onContentSelected;
  final ContentType currentContent;

  const CustomFooter({
    super.key,
    required this.onContentSelected,
    required this.currentContent,
  });

  @override
  Widget build(BuildContext context) {
    final isDarkMode = Theme.of(context).brightness == Brightness.dark;
    
    return Container(
      height: 50,
      color: isDarkMode 
          ? const Color(0xFF151026) 
          : Colors.amber.withOpacity(0.8),
      child: Row(
        mainAxisAlignment: MainAxisAlignment.spaceEvenly,
        children: _buildFooterItems(isDarkMode),
      ),
    );
  }

  List<Widget> _buildFooterItems(bool isDarkMode) {
    // Wybieramy tylko niektóre typy zawartości do wyświetlenia w stopce
    final footerContentTypes = [
      ContentType.home,
      ContentType.notebooks, // Dodaj notatniki do stopki
      ContentType.notes,
      ContentType.login,
    ];
    
    return footerContentTypes.map((type) {
      final isSelected = currentContent == type;
      return IconButton(
        icon: Icon(type.icon),
        color: isDarkMode
            ? (isSelected ? Colors.amber : Colors.white60)
            : (isSelected ? Colors.white : Colors.black54),
        onPressed: () => onContentSelected(type),
      );
    }).toList();
  }
}
