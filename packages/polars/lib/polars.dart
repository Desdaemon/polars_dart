/// Dart bindings for the polars library.
library;

export 'src/wrapper/df.dart';
export 'src/wrapper/entry.dart';
export 'src/wrapper/expr.dart';
export 'src/wrapper/series.dart';

import 'src/frb_generated.dart';

export 'src/extensions.dart';
export 'src/expr.dart';

typedef PolarsWrapper = RustLibApi;
typedef PolarsWrapperImpl = RustLibApiImpl;
