/// Dart bindings for the polars library.
library polars;

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
        Excluded,
        Schema,
        // enums
        Operator,
        TimeUnit,
        QuantileInterpolOptions,
        CsvEncoding,
        JoinType,
        UniqueKeepStrategy,
        // bridge
        PolarsWrapper,
        PolarsWrapperImpl;
export 'src/extensions.dart';
export 'src/expr.dart';
