// This file is automatically generated, so please do not edit it.
// @generated by `flutter_rust_bridge`@ 2.9.0.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

class Note {
  final String id;
  final String userId;
  final String? notebookId;
  final String title;
  final String content;
  final bool isArchived;
  final bool isPinned;
  final String tags;
  final PlatformInt64 createdAt;
  final PlatformInt64 updatedAt;

  const Note({
    required this.id,
    required this.userId,
    this.notebookId,
    required this.title,
    required this.content,
    required this.isArchived,
    required this.isPinned,
    required this.tags,
    required this.createdAt,
    required this.updatedAt,
  });

  @override
  int get hashCode =>
      id.hashCode ^
      userId.hashCode ^
      notebookId.hashCode ^
      title.hashCode ^
      content.hashCode ^
      isArchived.hashCode ^
      isPinned.hashCode ^
      tags.hashCode ^
      createdAt.hashCode ^
      updatedAt.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is Note &&
          runtimeType == other.runtimeType &&
          id == other.id &&
          userId == other.userId &&
          notebookId == other.notebookId &&
          title == other.title &&
          content == other.content &&
          isArchived == other.isArchived &&
          isPinned == other.isPinned &&
          tags == other.tags &&
          createdAt == other.createdAt &&
          updatedAt == other.updatedAt;
}
