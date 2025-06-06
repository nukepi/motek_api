// This file is automatically generated, so please do not edit it.
// @generated by `flutter_rust_bridge`@ 2.9.0.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

class NoteSettings {
  final String id;
  final String noteId;
  final String color;
  final String font;
  final String viewMode;
  final PlatformInt64 createdAt;
  final PlatformInt64 updatedAt;

  const NoteSettings({
    required this.id,
    required this.noteId,
    required this.color,
    required this.font,
    required this.viewMode,
    required this.createdAt,
    required this.updatedAt,
  });

  @override
  int get hashCode =>
      id.hashCode ^
      noteId.hashCode ^
      color.hashCode ^
      font.hashCode ^
      viewMode.hashCode ^
      createdAt.hashCode ^
      updatedAt.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is NoteSettings &&
          runtimeType == other.runtimeType &&
          id == other.id &&
          noteId == other.noteId &&
          color == other.color &&
          font == other.font &&
          viewMode == other.viewMode &&
          createdAt == other.createdAt &&
          updatedAt == other.updatedAt;
}
