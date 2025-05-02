import 'package:flutter/material.dart';
import 'package:motek_ui/l10n/app_localizations.dart';

enum ContentType { home, settings, notes, notebooks, login }

/// Rozszerzenie dla ContentType dodające pomocnicze metody
extension ContentTypeExtension on ContentType {
  String title(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;
    switch (this) {
      case ContentType.home:
        return l10n.home;
      case ContentType.settings:
        return l10n.settings;
      case ContentType.notes:
        return l10n.notes;
      case ContentType.notebooks:
        return l10n.notebooks;
      case ContentType.login:
        return l10n.login;
    }
  }

  IconData get icon {
    switch (this) {
      case ContentType.home:
        return Icons.home;
      case ContentType.settings:
        return Icons.settings;
      case ContentType.notes:
        return Icons.note;
      case ContentType.notebooks:
        return Icons.book;
      case ContentType.login:
        return Icons.login;
    }
  }
}
