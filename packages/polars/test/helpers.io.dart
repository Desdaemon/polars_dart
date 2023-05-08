import 'dart:io';

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

PolarsWrapper initApi({String profile = 'debug'}) {
  initialize(path: dylibPath(profile));
  return wrapper;
}
