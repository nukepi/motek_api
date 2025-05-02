import 'package:flutter/material.dart';
import 'package:motek_ui/l10n/app_localizations.dart';
import 'package:motek_ui/models/content_type.dart';

class CustomDrawer extends StatelessWidget {
  final Function(ContentType) onContentSelected;
  final ContentType currentContent;
  final bool isDarkMode;

  const CustomDrawer({
    super.key,
    required this.onContentSelected,
    required this.currentContent,
    required this.isDarkMode,
  });

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final l10n = AppLocalizations.of(context)!;
    
    return Drawer(
      child: ListView(
        padding: EdgeInsets.zero,
        children: [
          _buildDrawerHeader(theme, l10n),
          _buildMenuItems(context),
          const Divider(),
          _buildDrawerFooter(l10n),
        ],
      ),
    );
  }

  Widget _buildDrawerHeader(ThemeData theme, AppLocalizations l10n) {
    return DrawerHeader(
      decoration: BoxDecoration(
        color: isDarkMode ? const Color(0xFF151026) : Colors.amber,
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          CircleAvatar(
            radius: 30,
            backgroundColor: isDarkMode ? Colors.amber : Colors.white,
            child: Icon(
              Icons.person, 
              size: 40, 
              color: isDarkMode ? Colors.black : Colors.amber
            ),
          ),
          const SizedBox(height: 10),
          Text(
            l10n.appTitle, 
            style: TextStyle(
              color: isDarkMode ? Colors.amber : Colors.white, 
              fontSize: 24
            )
          ),
        ],
      ),
    );
  }

  Widget _buildMenuItems(BuildContext context) {
    return Column(
      children: ContentType.values.map((type) {
        return ListTile(
          leading: Icon(type.icon),
          title: Text(type.title(context)),
          selected: currentContent == type,
          onTap: () {
            Navigator.pop(context); // Zamknij drawer
            onContentSelected(type);
          },
        );
      }).toList(),
    );
  }

  Widget _buildDrawerFooter(AppLocalizations l10n) {
    return Padding(
      padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 8),
      child: Text(
        l10n.version('1.0.0'), 
        style: TextStyle(color: Colors.grey[600])
      ),
    );
  }
}
