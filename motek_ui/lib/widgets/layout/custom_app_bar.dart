// lib/widgets/layout/custom_app_bar.dart
import 'package:flutter/material.dart';
import 'package:motek_ui/models/content_type.dart';
import 'package:motek_ui/widgets/components/auth_status_indicator.dart';

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
    return AppBar(
      title: Text(contentType.title(context)),
      elevation: 4,
      actions: [
        // Dodajemy wskaźnik logowania
        Padding(
          padding: const EdgeInsets.symmetric(vertical: 8.0, horizontal: 8.0),
          child: AuthStatusIndicator(),
        ),
        IconButton(
          icon: const Icon(Icons.settings),
          onPressed: onSettingsPressed,
        ),
      ],
    );
  }
}
