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

String dylibPath({bool release = false}) => Uri.base
    .resolve('polars-wrapper/target/$triple/${release ? 'release' : 'debug'}'
        '/${formatDylib('polars_wrapper')}')
    .toFilePath();

PolarsWrapper initApi({bool release = false}) {
  final dylib = DynamicLibrary.open(dylibPath(release: release));
  final api = PolarsWrapperImpl(dylib);
  return api;
}
