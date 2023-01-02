import 'dart:ffi';
import 'dart:io';

import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:test/expect.dart';
import 'package:polars_dart/polars_dart.dart';
export 'package:polars_dart/polars_dart.dart';

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

final dylibPath = Uri.base
    .resolve('polars-wrapper/target/$triple/debug'
        '/${formatDylib('polars_wrapper')}')
    .toFilePath();

final throwsFfiException = throwsA(isA<FfiException>());

PolarsWrapper initApi() {
  final dylib = DynamicLibrary.open(dylibPath);
  final api = PolarsWrapperImpl(dylib);
  return api;
}
