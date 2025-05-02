import 'package:flutter/material.dart';

class ThemeManager {
  // Kolor magenta jako akcent
  static const Color accentColor = Color(0xFFFF00FF); // Magenta

  // Jasny motyw
  static ThemeData lightTheme = ThemeData(
    brightness: Brightness.light,
    primaryColor: const Color(0xFF151026),
    primarySwatch: Colors.amber,
    appBarTheme: const AppBarTheme(
      backgroundColor: Colors.amber,
      foregroundColor: Colors.black,
    ),
    scaffoldBackgroundColor: Colors.white,
    cardColor: Colors.white,
    iconTheme: const IconThemeData(color: Colors.black87),
    textTheme: const TextTheme(
      bodyMedium: TextStyle(color: Colors.black87),
      titleMedium: TextStyle(color: Colors.black),
      titleLarge: TextStyle(color: Colors.black),
    ),
    elevatedButtonTheme: ElevatedButtonThemeData(
      style: ElevatedButton.styleFrom(
        backgroundColor: const Color(0xFF2B2B2D),
        foregroundColor: Colors.black,
      ),
    ),
  );

  // Ciemny motyw
  static ThemeData darkTheme = ThemeData(
    brightness: Brightness.dark,
    primaryColor: accentColor, // Używamy magenta jako kolor główny
    primarySwatch: Colors.pink, // Najbliższy odpowiednik dla magenta w MaterialColor
    appBarTheme: const AppBarTheme(
      backgroundColor: Color(0xFF121212), // Ciemne tło
      foregroundColor: accentColor, // Magenta tekst
    ),
    scaffoldBackgroundColor: const Color(0xFF121212), // Ciemne tło jak na obrazku
    cardColor: const Color(0xFF1E1E1E), // Ciemne karty
    iconTheme: IconThemeData(color: accentColor), // Ikony w kolorze magenta
    textTheme: const TextTheme(
      bodyMedium: TextStyle(color: Colors.white70),
      titleMedium: TextStyle(color: Colors.white),
      titleLarge: TextStyle(color: Colors.white),
    ),
    elevatedButtonTheme: ElevatedButtonThemeData(
      style: ElevatedButton.styleFrom(
        backgroundColor: accentColor, // Przyciski w kolorze magenta
        foregroundColor: Colors.black, // Czarny tekst na przyciskach
      ),
    ),
    dividerColor: Colors.white24,
    colorScheme: ColorScheme.dark(
      primary: accentColor, // Magenta jako kolor primary
      secondary: accentColor, // Magenta jako kolor secondary
      surface: const Color(0xFF1E1E1E), // Ciemne tło
    ),
    switchTheme: SwitchThemeData(
      thumbColor: WidgetStateProperty.resolveWith<Color>((states) {
        if (states.contains(WidgetState.selected)) {
          return accentColor; // Kolor magenta dla włączonego przełącznika
        }
        return Colors.grey;
      }),
      trackColor: WidgetStateProperty.resolveWith<Color>((states) {
        if (states.contains(WidgetState.selected)) {
          return accentColor.withOpacity(0.5); // Półprzezroczysty magenta dla toru przełącznika
        }
        return Colors.grey.withOpacity(0.5);
      }),
    ),
  );
}
