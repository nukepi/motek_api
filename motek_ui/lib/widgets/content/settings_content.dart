import 'package:flutter/material.dart';
import 'package:motek_ui/l10n/app_localizations.dart';

class SettingsContent extends StatefulWidget {
  final bool isDarkMode;
  final Function(bool) onThemeChanged;

  const SettingsContent({
    super.key, 
    required this.isDarkMode, 
    required this.onThemeChanged
  });

  @override
  State<SettingsContent> createState() => _SettingsContentState();
}

class _SettingsContentState extends State<SettingsContent> {
  bool _notificationsEnabled = true;
  late bool _darkModeEnabled;
  double _fontSize = 16.0;
  String _selectedLanguage = 'pl';

  final Map<String, String> _availableLanguages = {
    'pl': 'Polski',
    'en': 'English',
  };

  @override
  void initState() {
    super.initState();
    _darkModeEnabled = widget.isDarkMode;
    
    // Ustaw początkowy język na podstawie aktualnej lokalizacji
    WidgetsBinding.instance.addPostFrameCallback((_) {
      final currentLocale = Localizations.localeOf(context).languageCode;
      if (_availableLanguages.containsKey(currentLocale)) {
        setState(() {
          _selectedLanguage = currentLocale;
        });
      }
    });
  }

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final l10n = AppLocalizations.of(context)!;
    
    return ListView(
      padding: const EdgeInsets.all(16.0),
      children: [
        Text(
          l10n.settings,
          style: TextStyle(
            fontSize: 24, 
            fontWeight: FontWeight.bold,
            color: theme.textTheme.titleLarge?.color,
          ),
          textAlign: TextAlign.center,
        ),
        const SizedBox(height: 20),
        _buildSectionTitle(l10n.appearance),
        _buildDarkModeSwitch(l10n),
        _buildFontSizeSlider(l10n),
        const Divider(),
        _buildSectionTitle(l10n.notifications),
        _buildNotificationSwitch(l10n),
        const Divider(),
        _buildSectionTitle(l10n.language),
        _buildLanguageDropdown(l10n),
        const Divider(),
        _buildSectionTitle(l10n.account),
        _buildAccountButtons(l10n),
        const Divider(),
        _buildSectionTitle(l10n.aboutApp),
        _buildAboutInfo(l10n),
      ],
    );
  }

  Widget _buildSectionTitle(String title) {
    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 8.0),
      child: Text(
        title,
        style: TextStyle(
          fontSize: 18,
          fontWeight: FontWeight.bold,
          color: Theme.of(context).primaryColor,
        ),
      ),
    );
  }

  Widget _buildDarkModeSwitch(AppLocalizations l10n) {
    return SwitchListTile(
      title: Text(l10n.darkMode),
      subtitle: Text(l10n.darkModeDescription),
      value: _darkModeEnabled,
      activeColor: Colors.amber,
      onChanged: (value) {
        setState(() {
          _darkModeEnabled = value;
          widget.onThemeChanged(value);
        });
      },
    );
  }

  Widget _buildFontSizeSlider(AppLocalizations l10n) {
    return ListTile(
      title: Text(l10n.fontSize),
      subtitle: Slider(
        value: _fontSize,
        min: 12.0,
        max: 24.0,
        divisions: 6,
        label: _fontSize.round().toString(),
        activeColor: Colors.amber,
        onChanged: (value) {
          setState(() {
            _fontSize = value;
          });
        },
      ),
      trailing: Text(
        '${_fontSize.round()}',
        style: TextStyle(fontSize: _fontSize),
      ),
    );
  }

  Widget _buildNotificationSwitch(AppLocalizations l10n) {
    return SwitchListTile(
      title: Text(l10n.notifications),
      subtitle: Text(l10n.notificationsEnable),
      value: _notificationsEnabled,
      activeColor: Colors.amber,
      onChanged: (value) {
        setState(() {
          _notificationsEnabled = value;
        });
      },
    );
  }

  Widget _buildLanguageDropdown(AppLocalizations l10n) {
    return ListTile(
      title: Text(l10n.language),
      trailing: DropdownButton<String>(
        value: _selectedLanguage,
        onChanged: (String? newValue) {
          if (newValue != null) {
            setState(() {
              _selectedLanguage = newValue;
            });
            
            // Zmień język aplikacji - to jest tylko symulacja, 
            // w rzeczywistej aplikacji należałoby zaimplementować zmianę języka
            ScaffoldMessenger.of(context).showSnackBar(
              SnackBar(content: Text("Język zmieniony na: ${_availableLanguages[newValue]}")),
            );
          }
        },
        items: _availableLanguages.entries.map<DropdownMenuItem<String>>((entry) {
          return DropdownMenuItem<String>(
            value: entry.key,
            child: Text(entry.value),
          );
        }).toList(),
      ),
    );
  }

  Widget _buildAccountButtons(AppLocalizations l10n) {
    return Column(
      children: [
        ListTile(
          leading: const Icon(Icons.person),
          title: Text(l10n.editProfile),
          onTap: () {
            // Tutaj można dodać logikę edycji profilu
          },
        ),
        ListTile(
          leading: const Icon(Icons.security),
          title: Text(l10n.changePassword),
          onTap: () {
            // Tutaj można dodać logikę zmiany hasła
          },
        ),
        ListTile(
          leading: const Icon(Icons.logout, color: Colors.red),
          title: Text(l10n.logout, style: const TextStyle(color: Colors.red)),
          onTap: () {
            // Tutaj można dodać logikę wylogowania
          },
        ),
      ],
    );
  }

  Widget _buildAboutInfo(AppLocalizations l10n) {
    return Column(
      children: [
        ListTile(
          title: Text(l10n.version('1.0.0')),
        ),
        ListTile(
          title: Text(l10n.license),
          trailing: const Text('MIT'),
        ),
        ListTile(
          title: Text(l10n.copyright),
        ),
      ],
    );
  }
}
