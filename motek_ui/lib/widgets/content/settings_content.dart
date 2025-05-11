import 'package:flutter/material.dart';
import 'package:motek_ui/l10n/app_localizations.dart';
import 'package:motek_ui/services/settings_service.dart';
import 'package:motek_ui/services/auth_service.dart';
import 'package:provider/provider.dart';
import 'package:motek_ui/screens/main_layout.dart';
import 'package:motek_ui/models/content_type.dart';

class SettingsContent extends StatefulWidget {
  final bool isDarkMode;
  final Function(bool) onThemeChanged;
  final Function(String) onLocaleChanged;

  const SettingsContent({
    super.key, 
    required this.isDarkMode, 
    required this.onThemeChanged,
    required this.onLocaleChanged,
  });

  @override
  State<SettingsContent> createState() => _SettingsContentState();
}

class _SettingsContentState extends State<SettingsContent> {
  final SettingsService _settingsService = SettingsService();
  late bool _notificationsEnabled;
  late bool _darkModeEnabled;
  late double _fontSize;
  late String _selectedLanguage;

  final Map<String, String> _availableLanguages = {
    'pl': 'Polski',
    'en': 'English',
    'es': 'Español',
  };

  @override
  void initState() {
    super.initState();
    _darkModeEnabled = widget.isDarkMode;
    _notificationsEnabled = _settingsService.notificationsEnabled;
    _fontSize = _settingsService.fontSize;
    _selectedLanguage = _settingsService.locale.languageCode;
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
      onChanged: (value) async {
        setState(() {
          _darkModeEnabled = value;
        });
        widget.onThemeChanged(value);
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
        onChanged: (value) async {
          setState(() {
            _fontSize = value;
          });
          await _settingsService.setFontSize(value);
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
      onChanged: (value) async {
        setState(() {
          _notificationsEnabled = value;
        });
        await _settingsService.setNotificationsEnabled(value);
      },
    );
  }

  Widget _buildLanguageDropdown(AppLocalizations l10n) {
    return ListTile(
      title: Text(l10n.language),
      trailing: DropdownButton<String>(
        value: _selectedLanguage,
        onChanged: (String? newValue) async {
          if (newValue != null) {
            setState(() {
              _selectedLanguage = newValue;
            });
            
            // Zmień język aplikacji
            widget.onLocaleChanged(newValue);
            
            // Pokaż komunikat o zmianie języka
            if (mounted) {
              ScaffoldMessenger.of(context).showSnackBar(
                SnackBar(content: Text("Język zmieniony na: ${_availableLanguages[newValue]}")),
              );
            }
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
    return Consumer<AuthService>(
      builder: (context, authService, _) {
        if (!authService.isLoggedIn) {
          return ListTile(
            leading: const Icon(Icons.login),
            title: Text(l10n.login),
            onTap: () {
              // Przejdź do ekranu logowania
              final mainLayout = context.findAncestorStateOfType<State<MainLayout>>();
              if (mainLayout != null) {
                final mainLayoutState = mainLayout as dynamic;
                mainLayoutState.changeContent(ContentType.login);
              }
            },
          );
        }
        
        return Column(
          children: [
            if (authService.userEmail != null)
              ListTile(
                leading: const Icon(Icons.person),
                title: Text(authService.userEmail!),
                subtitle: Text('Zalogowano jako'), // Dodaj to tłumaczenie
              ),
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
              onTap: () async {
                final shouldLogout = await showDialog<bool>(
                  context: context,
                  builder: (context) => AlertDialog(
                    title: Text(l10n.logout),
                    content: Text('Czy na pewno chcesz się wylogować?'), // Dodaj to tłumaczenie
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
                  
                  if (context.mounted) {
                    ScaffoldMessenger.of(context).showSnackBar(
                      SnackBar(content: Text('Wylogowano pomyślnie')), // Dodaj to tłumaczenie
                    );
                  }
                }
              },
            ),
          ],
        );
      },
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
