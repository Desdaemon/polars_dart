import 'dart:ffi';

import 'package:polars_dart/polars_dart.dart';

void main() async {
  final path = Uri.base.resolve('path/to/libpolars_wrapper.dylib').toFilePath();
  final dylib = DynamicLibrary.open(path);
  final api = PolarsWrapperImpl(dylib);
  final data = await api.readCsv(path: 'path/to/file.csv');
  print(await data.dump());
}
