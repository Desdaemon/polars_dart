import 'dart:ffi';
import 'dart:io';

import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:polars/polars.dart';
import 'package:path/path.dart' as p;

final hostTriple = () {
  final result = Process.runSync('rustc', const ['-vV']);

  return (result.stdout as String)
      .split('\n')
      .firstWhere((line) => line.startsWith('host:'))
      .split(':')
      .last
      .trim();
}();

extension FileExt on String {
  String get dylib {
    if (Platform.isWindows) {
      return '$this.dll';
    }
    if (Platform.isMacOS) {
      return 'lib$this.dylib';
    }
    return 'lib$this.so';
  }
}

String dylibPath(String profile) => Uri.base
    .resolve(p.joinAll([
      '../../target',
      if (Platform.isMacOS && hostTriple.startsWith('aarch64')) hostTriple,
      profile,
      'polars_wrapper'.dylib,
    ]))
    .toFilePath();

Future<void> initApi({String profile = 'debug'}) {
  return initialize(dylib: ExternalLibrary.open(dylibPath(profile)));
}
