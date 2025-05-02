import 'package:flutter/material.dart';
import 'package:motek_ui/l10n/app_localizations.dart';
import 'package:motek_ui/models/content_type.dart';
import 'package:motek_ui/widgets/layout/custom_app_bar.dart';
import 'package:motek_ui/widgets/layout/custom_drawer.dart';
import 'package:motek_ui/widgets/layout/custom_footer.dart';
import 'package:motek_ui/widgets/content/home_content.dart';
import 'package:motek_ui/widgets/content/settings_content.dart';
import 'package:motek_ui/widgets/content/notes_content.dart';
import 'package:motek_ui/widgets/content/notebook_content.dart';
import 'package:motek_ui/widgets/content/login_content.dart';

class MainLayout extends StatefulWidget {
  final bool isDarkMode;
  final Function(bool) onThemeChanged;

  const MainLayout({
    super.key, 
    required this.isDarkMode, 
    required this.onThemeChanged
  });

  @override
  State<MainLayout> createState() => _MainLayoutState();
}

class _MainLayoutState extends State<MainLayout> {
  // Aktualny typ zawartości
  ContentType _currentContent = ContentType.home;

  // Metoda do zmiany zawartości
  void changeContent(ContentType newContent) {
    setState(() {
      _currentContent = newContent;
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: CustomAppBar(
        contentType: _currentContent,
        onSettingsPressed: () => changeContent(ContentType.settings),
      ),
      drawer: CustomDrawer(
        onContentSelected: changeContent,
        currentContent: _currentContent,
        isDarkMode: widget.isDarkMode,
      ),
      body: _buildCurrentContent(),
      bottomNavigationBar: CustomFooter(
        onContentSelected: changeContent,
        currentContent: _currentContent,
      ),
    );
  }

  // Metoda budująca aktualną zawartość
  Widget _buildCurrentContent() {
    switch (_currentContent) {
      case ContentType.home:
        return const HomeContent();
      case ContentType.settings:
        return SettingsContent(
          isDarkMode: widget.isDarkMode,
          onThemeChanged: widget.onThemeChanged,
        );
      case ContentType.notes:
        return const NotesContent();
      case ContentType.notebooks:
        return const NotebookContent();
      case ContentType.login:
        return const LoginContent();
    }
  }
}
