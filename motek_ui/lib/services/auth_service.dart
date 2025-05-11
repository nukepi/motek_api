// lib/services/auth_service.dart
import 'dart:convert';
import 'package:flutter/foundation.dart';
import 'package:flutter_secure_storage/flutter_secure_storage.dart';
import 'package:http/http.dart' as http;

class AuthService extends ChangeNotifier {
  final FlutterSecureStorage _secureStorage = const FlutterSecureStorage();
  final String _tokenKey = 'auth_token';
  final String _refreshTokenKey = 'refresh_token';
  final String _tokenExpiryKey = 'token_expiry';
  final String _userEmailKey = 'user_email';
  
  bool _isLoggedIn = false;
  String? _userEmail;
  
  // Gettery
  bool get isLoggedIn => _isLoggedIn;
  String? get userEmail => _userEmail;
  
  // Inicjalizacja - sprawdza czy użytkownik jest zalogowany
  Future<void> init() async {
    await checkLoginStatus();
  }
  
  // Sprawdza czy użytkownik jest zalogowany i czy token jest ważny
  Future<bool> checkLoginStatus() async {
    final token = await getToken();
    final expiryString = await _secureStorage.read(key: _tokenExpiryKey);
    final email = await _secureStorage.read(key: _userEmailKey);
    
    if (token == null || expiryString == null) {
      _isLoggedIn = false;
      _userEmail = null;
      notifyListeners();
      return false;
    }
    
    try {
      final expiry = DateTime.parse(expiryString);
      // Dodaj bufor 5 minut przed faktycznym wygaśnięciem
      final isValid = expiry.isAfter(DateTime.now().add(const Duration(minutes: 5)));
      
      if (!isValid) {
        // Token wygasł, próba odświeżenia
        final refreshed = await refreshTokenIfNeeded();
        _isLoggedIn = refreshed;
        _userEmail = refreshed ? email : null;
        notifyListeners();
        return refreshed;
      }
      
      _isLoggedIn = true;
      _userEmail = email;
      notifyListeners();
      return true;
    } catch (e) {
      _isLoggedIn = false;
      _userEmail = null;
      notifyListeners();
      return false;
    }
  }
  
  Future<bool> refreshTokenIfNeeded() async {
    final refreshToken = await _secureStorage.read(key: _refreshTokenKey);
    if (refreshToken == null) {
      return false;
    }
    
    try {
      // Wywołanie API do odświeżenia tokenu
      final response = await http.post(
        Uri.parse('http://139.59.138.164:3000/api/auth/refresh'),
        headers: {'Content-Type': 'application/json'},
        body: json.encode({'refreshToken': refreshToken}),
      );
      
      if (response.statusCode == 200) {
        final data = json.decode(response.body);
        await saveAuthData(
          token: data['token'],
          refreshToken: data['refreshToken'],
          expiry: data['expiry'],
        );
        return true;
      } else {
        // Odświeżenie nie powiodło się, wyloguj użytkownika
        await logout();
        return false;
      }
    } catch (e) {
      await logout();
      return false;
    }
  }
  
  Future<String?> getToken() async {
    return await _secureStorage.read(key: _tokenKey);
  }
  
  Future<void> saveAuthData({
    required String token,
    required String refreshToken,
    required String expiry,
    String? email,
  }) async {
    await _secureStorage.write(key: _tokenKey, value: token);
    await _secureStorage.write(key: _refreshTokenKey, value: refreshToken);
    await _secureStorage.write(key: _tokenExpiryKey, value: expiry);
    
    if (email != null) {
      await _secureStorage.write(key: _userEmailKey, value: email);
      _userEmail = email;
    }
    
    _isLoggedIn = true;
    notifyListeners();
  }
  
  // Metoda do ustawiania stanu zalogowania (używana po udanym logowaniu)
  Future<void> setLoggedIn(String email) async {
    _isLoggedIn = true;
    _userEmail = email;
    await _secureStorage.write(key: _userEmailKey, value: email);
    notifyListeners();
  }
  
  Future<void> logout() async {
    await _secureStorage.delete(key: _tokenKey);
    await _secureStorage.delete(key: _refreshTokenKey);
    await _secureStorage.delete(key: _tokenExpiryKey);
    // Możemy zachować email dla wygody przy ponownym logowaniu
    
    _isLoggedIn = false;
    _userEmail = null;
    notifyListeners();
  }
}
