import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:test/test.dart';
export 'package:polars_dart/polars_dart.dart';

export 'helpers.io.dart';

final throwsFfiException = throwsA(isA<FfiException>());
