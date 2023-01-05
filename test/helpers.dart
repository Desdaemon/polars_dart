import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:test/expect.dart';
export 'package:polars_dart/polars_dart.dart';

export 'helpers.io.dart' if (dart.library.html) 'helpers.web.dart';

final throwsFfiException = throwsA(isA<FfiException>());
