import 'package:flutter/material.dart';
import 'package:motek_ui/src/rust/frb_generated.dart';
import 'app.dart';

Future<void> main() async {
  await RustLib.init();
  runApp(const MotekApp());
}
