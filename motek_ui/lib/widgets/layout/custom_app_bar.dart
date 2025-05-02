import 'package:flutter/material.dart';
import 'package:motek_ui/l10n/app_localizations.dart';
import 'package:motek_ui/models/content_type.dart';

class CustomAppBar extends StatelessWidget implements PreferredSizeWidget {
  final ContentType contentType;
  final VoidCallback? onSettingsPressed;

  const CustomAppBar({
    super.key, 
    required this.contentType, 
    this.onSettingsPressed
  });

  @override
  Size get preferredSize => const Size.fromHeight(kToolbarHeight);

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;
    
    return AppBar(
      title: Text(contentType.title(context)),
      elevation: 4,
      actions: [
        IconButton(
          icon: const Icon(Icons.settings),
          onPressed: onSettingsPressed,
        ),
      ],
    );
  }
}
