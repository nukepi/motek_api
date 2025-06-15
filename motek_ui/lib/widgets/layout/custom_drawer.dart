import 'package:flutter/material.dart';
import 'package:motek_ui/l10n/app_localizations.dart';
import 'package:motek_ui/models/content_type.dart';
import 'package:motek_ui/services/auth_service.dart';
import 'package:provider/provider.dart';

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
    
    // Nasłuchuj zmian w AuthService
    return Consumer<AuthService>(
      builder: (context, authService, _) {
        return Drawer(
          child: ListView(
            padding: EdgeInsets.zero,
            children: [
              _buildDrawerHeader(theme, l10n, authService),
              _buildMenuItems(context),
              const Divider(),
              _buildAuthSection(context, authService, l10n),
              const Divider(),
              _buildDrawerFooter(l10n),
            ],
          ),
        );
      },
    );
  }

  Widget _buildDrawerHeader(ThemeData theme, AppLocalizations l10n, AuthService authService) {
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
          if (authService.isLoggedIn && authService.userEmail != null)
            Padding(
              padding: const EdgeInsets.only(top: 8.0),
              child: Text(
                authService.userEmail!,
                style: TextStyle(
                  color: isDarkMode ? Colors.amber.withValues(alpha: 0.7) : Colors.white.withValues(alpha: 0.7),
                  fontSize: 14,
                ),
              ),
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

  Widget _buildAuthSection(BuildContext context, AuthService authService, AppLocalizations l10n) {
    if (authService.isLoggedIn) {
      return ListTile(
        leading: const Icon(Icons.logout, color: Colors.red),
        title: Text(l10n.logout, style: const TextStyle(color: Colors.red)),
        onTap: () async {
          // Pokaż dialog potwierdzenia
          final shouldLogout = await showDialog<bool>(
            context: context,
            builder: (context) => AlertDialog(
              title: Text(l10n.logout),
              content: Text('Czy na pewno chcesz się wylogować?'),
              actions: [
                TextButton(
                  onPressed: () => Navigator.pop(context, false),
                  child: Text(l10n.cancel),
                ),
                TextButton(
                  onPressed: () => Navigator.pop(context, true),
                  child: Text(l10n.logout),
                ),
              ],
            ),
          );

          if (shouldLogout == true) {
            await authService.logout();
            
            // Zamknij szufladę
            Navigator.pop(context);
            
            // Przejdź do ekranu logowania
            onContentSelected(ContentType.login);
          }
        },
      );
    } else {
      return ListTile(
        leading: const Icon(Icons.login),
        title: Text(l10n.login),
        onTap: () {
          Navigator.pop(context); // Zamknij drawer
          onContentSelected(ContentType.login);
        },
      );
    }
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