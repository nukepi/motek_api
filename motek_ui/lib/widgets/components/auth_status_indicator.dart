// lib/widgets/auth_status_indicator.dart
import 'package:flutter/material.dart';
import 'package:motek_ui/l10n/app_localizations.dart';
import 'package:motek_ui/services/auth_service.dart';
import 'package:provider/provider.dart';

class AuthStatusIndicator extends StatelessWidget {
  const AuthStatusIndicator({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;
    
    return Consumer<AuthService>(
      builder: (context, authService, _) {
        final isLoggedIn = authService.isLoggedIn;
        final email = authService.userEmail ?? '';
        
        return Container(
          padding: EdgeInsets.symmetric(horizontal: 12, vertical: 6),
          decoration: BoxDecoration(
            color: isLoggedIn ? Colors.green.shade100 : Colors.red.shade100,
            borderRadius: BorderRadius.circular(16),
            border: Border.all(
              color: isLoggedIn ? Colors.green : Colors.red,
              width: 1,
            ),
          ),
          child: Row(
            mainAxisSize: MainAxisSize.min,
            children: [
              Icon(
                isLoggedIn ? Icons.check_circle : Icons.error_outline,
                color: isLoggedIn ? Colors.green : Colors.red,
                size: 16,
              ),
              SizedBox(width: 8),
              Text(
                isLoggedIn 
                  ? '${l10n.loggedInAs}: $email'
                  : 'Niezalogowany', // Dodaj to tłumaczenie do pliku l10n
                style: TextStyle(
                  color: isLoggedIn ? Colors.green.shade800 : Colors.red.shade800,
                  fontSize: 12,
                  fontWeight: FontWeight.bold,
                ),
              ),
            ],
          ),
        );
      },
    );
  }
}
