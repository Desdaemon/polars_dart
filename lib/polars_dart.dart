/// Dart bindings for the polars library.
library polars_dart;

export 'src/wrapper.dart'
    show
        AggExpr,
        DataFrame,
        DataType,
        Expr,
        LazyFrame,
        LazyGroupBy,
        LiteralValue,
        Series,
        Shape,
        SortOptions,
        Field,
        RowCount,
        NullValues,
        // enums
        Operator,
        TimeUnit,
        QuantileInterpolOptions,
        CsvEncoding,
        // bridge
        PolarsWrapper,
        PolarsWrapperImpl;
export 'src/wrapper.io.dart' if (dart.library.html) 'src/wrapper.web.dart'
    show PolarsWrapperImpl;
export 'src/extensions.dart';
export 'src/expr.dart';
