import 'dart:ffi';
import 'dart:io';

import 'package:polars_dart/polars_dart.dart';

final triple = () {
  final result = Process.runSync('rustc', const ['-vV']);
  final stdout = result.stdout;
  if (stdout is! String) throw Exception('invalid stdout');

  return stdout
      .split('\n')
      .firstWhere((line) => line.startsWith('host:'))
      .substring(6);
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

String dylibPath({String profile = 'debug'}) => Uri.base
    .resolve([
      '../../polars-wrapper/target',
      if (Platform.isMacOS) triple,
      profile,
      formatDylib('polars_wrapper')
    ].join('/'))
    .toFilePath();

PolarsWrapper initApi({String profile = 'debug'}) {
  final dylib = DynamicLibrary.open(dylibPath(profile: profile));
  final api = PolarsWrapperImpl(dylib);
  return api;
}
