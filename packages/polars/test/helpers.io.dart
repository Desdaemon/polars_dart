import 'dart:ffi';
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

String formatDylib(String name) {
  if (Platform.isWindows) {
    return '$name.dll';
  }
  if (Platform.isMacOS) {
    return 'lib$name.dylib';
  }
  return 'lib$name.so';
}

String dylibPath(String profile) => Uri.base
    .resolve(p.joinAll([
      '../../polars-wrapper/target',
      if (Platform.isMacOS && hostTriple.startsWith('aarch64')) hostTriple,
      profile,
      formatDylib('polars_wrapper')
    ]))
    .toFilePath();

PolarsWrapper initApi({String profile = 'debug'}) {
  final dylib = DynamicLibrary.open(dylibPath(profile));
  final api = PolarsWrapperImpl(dylib);
  return api;
}
