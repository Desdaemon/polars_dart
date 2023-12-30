/// Dart bindings for the polars library.
library;

export 'src/wrapper/df.dart' hide SpecialEqPSeries, PExpr;
export 'src/wrapper/entry.dart';
export 'src/wrapper/expr.dart';
export 'src/wrapper/series.dart';

import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

import 'src/frb_generated.dart';

export 'src/extensions.dart';
export 'src/expr.dart';

typedef PolarsWrapper = RustLibApi;
typedef PolarsWrapperImpl = RustLibApiImpl;

Future<void> initialize({ExternalLibrary? dylib}) {
  return RustLib.init(externalLibrary: dylib);
}
