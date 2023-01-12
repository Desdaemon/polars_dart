import 'dart:ffi';
import 'dart:io';
import 'package:polars/polars.dart';
export 'package:polars/polars.dart' hide PolarsWrapperImpl;

const _libName = 'polars_wrapper';

final DynamicLibrary _dylib = () {
  if (Platform.isMacOS || Platform.isIOS) {
    return DynamicLibrary.executable();
  }
  if (Platform.isAndroid || Platform.isLinux) {
    return DynamicLibrary.open('lib$_libName.so');
  }
  if (Platform.isWindows) {
    return DynamicLibrary.open('$_libName.dll');
  }
  throw UnsupportedError('Unknown platform: ${Platform.operatingSystem}');
}();

final PolarsWrapper pl = PolarsWrapperImpl(_dylib);
