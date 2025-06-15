import 'package:flutter/material.dart';
import 'package:motek_ui/l10n/app_localizations.dart';
import 'package:motek_ui/models/content_type.dart';
import 'package:motek_ui/services/auth_service.dart';
import 'package:motek_ui/src/rust/api/endpoint.dart';
import 'package:motek_ui/utils/logger.dart';
import 'package:provider/provider.dart';
import 'package:motek_ui/screens/main_layout.dart';

class LoginContent extends StatefulWidget {
  const LoginContent({super.key});

  @override
  State<LoginContent> createState() => _LoginContentState();
}

class _LoginContentState extends State<LoginContent> {
  final _formKey = GlobalKey<FormState>();
  final _emailController = TextEditingController();
  final _passwordController = TextEditingController();
  bool _isLoading = false;
  String? _errorMessage;
  bool _showRegisterContent = false;

  @override
  void initState() {
    super.initState();
    Logger.info('LoginContent initialized');
    
    // Sprawdź czy użytkownik jest już zalogowany
    WidgetsBinding.instance.addPostFrameCallback((_) {
      final authService = Provider.of<AuthService>(context, listen: false);
      if (authService.isLoggedIn) {
        // Jeśli zalogowany, przejdź do ekranu głównego
        _navigateToHome();
      }
    });
  }

  void _navigateToHome() {
    // Znajdź najbliższy MainLayout i zmień zawartość
    final mainLayout = context.findAncestorStateOfType<State<MainLayout>>();
    if (mainLayout != null) {
      final mainLayoutState = mainLayout as dynamic; // Używamy dynamic, bo nie mamy bezpośredniego dostępu do _MainLayoutState
      mainLayoutState.changeContent(ContentType.home);
    }
  }

  @override
  void dispose() {
    _emailController.dispose();
    _passwordController.dispose();
    super.dispose();
  }

  Future<void> _login() async {
    if (!_formKey.currentState!.validate()) {
      Logger.warn('Form validation failed');
      return;
    }
    
    final l10n = AppLocalizations.of(context)!;
    final email = _emailController.text;
    final password = _passwordController.text;
    
    Logger.info('Attempting to login with email: $email');
    
    setState(() {
      _isLoading = true;
      _errorMessage = null;
    });
    
    try {
      Logger.debug('Calling login API');
      final response = await login(
        email: email,
        password: password,
      );
      
      Logger.info('Login response received: success=${response.success}');
      Logger.debug('Login message: ${response.message}');
      
      setState(() {
        _isLoading = false;
      });
      
      if (response.success) {
        Logger.info('Login successful');
        
        // Aktualizuj stan autoryzacji
        final authService = Provider.of<AuthService>(context, listen: false);
        await authService.setLoggedIn(email);
        
        if (mounted) {
          ScaffoldMessenger.of(context).showSnackBar(
            SnackBar(content: Text(l10n.loggingIn)),
          );
          
          // Przejdź do ekranu głównego
          _navigateToHome();
        }
      } else {
        Logger.warn('Login failed: ${response.message}');
        setState(() {
          _errorMessage = response.message;
        });
      }
    } catch (e, stackTrace) {
      Logger.error('Exception during login. + ${e.toString()}  ${stackTrace.toString()}');
      setState(() {
        _isLoading = false;
        _errorMessage = e.toString();
      });
    }
  }

  void _toggleRegisterView() {
    Logger.debug('Toggling register view');
    setState(() {
      _showRegisterContent = !_showRegisterContent;
    });
  }

  @override
  Widget build(BuildContext context) {
    // Sprawdź czy użytkownik jest zalogowany
    final authService = Provider.of<AuthService>(context);
    if (authService.isLoggedIn) {
      return Material(
        color: Colors.transparent,
        child: Center(
          child: Column(
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              Text('Zalogowano jako: ${authService.userEmail}'),
              const SizedBox(height: 20),
              ElevatedButton(
                onPressed: () async {
                  await authService.logout();
                  if (mounted) {
                    ScaffoldMessenger.of(context).showSnackBar(
                      const SnackBar(content: Text('Wylogowano pomyślnie')),
                    );
                  }
                },
                child: const Text('Wyloguj się'),
              ),
            ],
          ),
        ),
      );
    }
    
    if (_showRegisterContent) {
      return RegisterContent(
        onBackToLogin: _toggleRegisterView,
      );
    }
    
    final l10n = AppLocalizations.of(context)!;
    
    return Material(
      color: Colors.transparent,
      child: Padding(
        padding: const EdgeInsets.all(16.0),
        child: Form(
          key: _formKey,
          child: Column(
            mainAxisAlignment: MainAxisAlignment.center,
            crossAxisAlignment: CrossAxisAlignment.stretch,
            children: [
              Text(
                l10n.login,
                style: const TextStyle(fontSize: 24, fontWeight: FontWeight.bold),
                textAlign: TextAlign.center,
              ),
              const SizedBox(height: 30),
              if (_errorMessage != null)
                Container(
                  padding: const EdgeInsets.all(10),
                  margin: const EdgeInsets.only(bottom: 16),
                  decoration: BoxDecoration(
                    color: Colors.red.withValues(alpha: 0.1),
                    borderRadius: BorderRadius.circular(8),
                    border: Border.all(color: Colors.red),
                  ),
                  child: Text(
                    _errorMessage!,
                    style: const TextStyle(color: Colors.red),
                  ),
                ),
              TextFormField(
                controller: _emailController,
                decoration: InputDecoration(
                  labelText: l10n.email,
                  border: const OutlineInputBorder(),
                  prefixIcon: const Icon(Icons.email),
                ),
                keyboardType: TextInputType.emailAddress,
                validator: (value) {
                  if (value == null || value.isEmpty) {
                    return l10n.pleaseEnterEmail;
                  }
                  return null;
                },
              ),
              const SizedBox(height: 16),
              TextFormField(
                controller: _passwordController,
                decoration: InputDecoration(
                  labelText: l10n.password,
                  border: const OutlineInputBorder(),
                  prefixIcon: const Icon(Icons.lock),
                ),
                obscureText: true,
                validator: (value) {
                  if (value == null || value.isEmpty) {
                    return l10n.pleaseEnterPassword;
                  }
                  return null;
                },
              ),
              const SizedBox(height: 24),
              ElevatedButton(
                onPressed: _isLoading ? null : _login,
                style: ElevatedButton.styleFrom(
                  backgroundColor: Colors.amber,
                  padding: const EdgeInsets.symmetric(vertical: 12),
                ),
                child: _isLoading 
                  ? const SizedBox(
                      height: 20,
                      width: 20,
                      child: CircularProgressIndicator(
                        strokeWidth: 2,
                        color: Colors.white,
                      ),
                    )
                  : Text(l10n.loginButton),
              ),
              const SizedBox(height: 16),
              TextButton(
                onPressed: _toggleRegisterView,
                child: Text(l10n.registerPrompt),
              ),
            ],
          ),
        ),
      ),
    );
  }
}

// Klasa RegisterContent zdefiniowana w tym samym pliku dla uproszczenia
class RegisterContent extends StatefulWidget {
  final VoidCallback onBackToLogin;

  const RegisterContent({
    super.key, 
    required this.onBackToLogin,
  });

  @override
  State<RegisterContent> createState() => _RegisterContentState();
}

class _RegisterContentState extends State<RegisterContent> {
  final _formKey = GlobalKey<FormState>();
  final _emailController = TextEditingController();
  final _passwordController = TextEditingController();
  final _confirmPasswordController = TextEditingController();
  bool _isLoading = false;
  String? _errorMessage;

  @override
  void initState() {
    super.initState();
    Logger.info('RegisterContent initialized');
  }

  @override
  void dispose() {
    Logger.debug('Disposing RegisterContent controllers');
    _emailController.dispose();
    _passwordController.dispose();
    _confirmPasswordController.dispose();
    super.dispose();
  }

  Future<void> _register() async {
    if (!_formKey.currentState!.validate()) {
      Logger.warn('Register form validation failed');
      return;
    }
    
    final email = _emailController.text;
    final password = _passwordController.text;
    
    Logger.info('Attempting to register with email: $email');
    
    setState(() {
      _isLoading = true;
      _errorMessage = null;
    });
    
    try {
      Logger.debug('Calling register API');
      final response = await register(
        email: email,
        password: password,
      );
      
      Logger.info('Register response received: success=${response.success}');
      Logger.debug('Register message: ${response.message}');
      
      setState(() {
        _isLoading = false;
      });
      
      if (response.success) {
        Logger.info('Registration successful');
        if (mounted) {
          ScaffoldMessenger.of(context).showSnackBar(
            const SnackBar(content: Text('Rejestracja udana! Możesz się teraz zalogować.')),
          );
          widget.onBackToLogin();
        }
      } else {
        Logger.warn('Registration failed: ${response.message}');
        setState(() {
          _errorMessage = response.message;
        });
      }
    } catch (e, stackTrace) {
      Logger.error('Exception during registration. + ${e.toString()}  ${stackTrace.toString()}');
      setState(() {
        _isLoading = false;
        _errorMessage = e.toString();
      });
    }
  }

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;
    
    return Material(
      color: Colors.transparent,
      child: Padding(
        padding: const EdgeInsets.all(16.0),
        child: Form(
          key: _formKey,
          child: Column(
            mainAxisAlignment: MainAxisAlignment.center,
            crossAxisAlignment: CrossAxisAlignment.stretch,
            children: [
              Text(
                'Rejestracja', // Można dodać tłumaczenie do plików l10n
                style: const TextStyle(fontSize: 24, fontWeight: FontWeight.bold),
                textAlign: TextAlign.center,
              ),
              const SizedBox(height: 30),
              if (_errorMessage != null)
                Container(
                  padding: const EdgeInsets.all(10),
                  margin: const EdgeInsets.only(bottom: 16),
                  decoration: BoxDecoration(
                    color: Colors.red.withValues(alpha: .1),
                    borderRadius: BorderRadius.circular(8),
                    border: Border.all(color: Colors.red),
                  ),
                  child: Text(
                    _errorMessage!,
                    style: const TextStyle(color: Colors.red),
                  ),
                ),
              TextFormField(
                controller: _emailController,
                decoration: InputDecoration(
                  labelText: l10n.email,
                  border: const OutlineInputBorder(),
                  prefixIcon: const Icon(Icons.email),
                ),
                keyboardType: TextInputType.emailAddress,
                validator: (value) {
                  if (value == null || value.isEmpty) {
                    return l10n.pleaseEnterEmail;
                  }
                  if (!value.contains('@') || !value.contains('.')) {
                    return 'Proszę podać poprawny adres email';
                  }
                  return null;
                },
              ),
              const SizedBox(height: 16),
              TextFormField(
                controller: _passwordController,
                decoration: InputDecoration(
                  labelText: l10n.password,
                  border: const OutlineInputBorder(),
                  prefixIcon: const Icon(Icons.lock),
                ),
                obscureText: true,
                validator: (value) {
                  if (value == null || value.isEmpty) {
                    return l10n.pleaseEnterPassword;
                  }
                  if (value.length < 6) {
                    return 'Hasło musi mieć co najmniej 6 znaków';
                  }
                  return null;
                },
              ),
              const SizedBox(height: 16),
              TextFormField(
                controller: _confirmPasswordController,
                decoration: const InputDecoration(
                  labelText: 'Potwierdź hasło', // Można dodać tłumaczenie
                  border: OutlineInputBorder(),
                  prefixIcon: Icon(Icons.lock_outline),
                ),
                obscureText: true,
                validator: (value) {
                  if (value == null || value.isEmpty) {
                    return 'Proszę potwierdzić hasło';
                  }
                  if (value != _passwordController.text) {
                    return 'Hasła nie są identyczne';
                  }
                  return null;
                },
              ),
              const SizedBox(height: 24),
              ElevatedButton(
                onPressed: _isLoading ? null : _register,
                style: ElevatedButton.styleFrom(
                  backgroundColor: Colors.amber,
                  padding: const EdgeInsets.symmetric(vertical: 12),
                ),
                child: _isLoading 
                  ? const SizedBox(
                      height: 20,
                      width: 20,
                      child: CircularProgressIndicator(
                        strokeWidth: 2,
                        color: Colors.white,
                      ),
                    )
                  : const Text('Zarejestruj się'), // Można dodać tłumaczenie
              ),
              const SizedBox(height: 16),
              TextButton(
                onPressed: widget.onBackToLogin,
                child: const Text('Masz już konto? Zaloguj się'), // Można dodać tłumaczenie
              ),
            ],
          ),
        ),
      ),
    );
  }
}
