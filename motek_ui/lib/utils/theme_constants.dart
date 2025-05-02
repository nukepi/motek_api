import 'package:flutter/material.dart';

/// Stałe dotyczące motywu aplikacji
class ThemeConstants {
  // Kolory
  static const Color primaryColor = Color(0xFF151026);
  static const Color accentColor = Color.fromARGB(255, 186, 4, 76);

  // Rozmiary
  static const double footerHeight = 50.0;

  // Padding
  static const EdgeInsets contentPadding = EdgeInsets.all(16.0);

  // Stylistyka tekstu
  static const TextStyle headerStyle = TextStyle(
    fontSize: 24,
    fontWeight: FontWeight.bold,
  );

  static const TextStyle subtitleStyle = TextStyle(
    fontSize: 16,
    color: Colors.grey,
  );
}
