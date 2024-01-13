/// Dart bindings for the polars library.
library;

export 'src/wrapper/df.dart'
    show DataFrame, LazyFrame, Literals, UniqueKeepStrategy;
export 'src/wrapper/entry.dart'
    hide NullValues_AllColumnsSingle, NullValues_AllColumns, NullValues_Named;
export 'src/wrapper/expr.dart'
    show
        AggExpr,
        Ambiguous,
        ClosedWindow,
        DataType,
        Excluded,
        Expr,
        Field,
        IsSorted,
        LiteralValue,
        Operator,
        SortOptions,
        WindowMapping,
        WindowType;
export 'src/wrapper/series.dart';
export 'src/extensions.dart';
export 'src/expr.dart';
export 'src/str.dart';
export 'src/list.dart';
export 'src/df.dart';
export 'src/series.dart';

import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'src/frb_generated.dart';

typedef PolarsWrapper = RustLibApi;
typedef PolarsWrapperImpl = RustLibApiImpl;

Future<void> initialize({ExternalLibrary? dylib}) {
  return RustLib.init(externalLibrary: dylib);
}
