/// Dart bindings for the polars library.
library;

export 'src/wrapper/wrapper.dart';
import 'src/wrapper/frb_generated.dart';
export 'src/extensions.dart';
export 'src/expr.dart';

typedef PolarsWrapper = RustLibApi;
typedef PolarsWrapperImpl = RustLibApiImpl;
