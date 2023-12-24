import 'wrapper/wrapper.dart';

final _kIsWeb = 0 == 0.0;

/// Extensions for [Expr].
extension ExprExt on Expr {
  /// Returns an expression evaluating whether this and [other] is equal.
  Expr equals(Expr other) =>
      Expr.binaryExpr(left: this, right: other, op: Operator.eq);

  /// Returns an expression evaluating whether this and [other] does not equal.
  Expr notEquals(Expr other) =>
      Expr.binaryExpr(left: this, right: other, op: Operator.notEq);

  /// Returns an expression evaluating whether this is less than [other].
  Expr operator <(Expr other) =>
      Expr.binaryExpr(left: this, right: other, op: Operator.lt);

  /// Returns an expression evaluating whether this is no greater than [other].
  Expr operator <=(Expr other) =>
      Expr.binaryExpr(left: this, right: other, op: Operator.ltEq);

  /// Returns an expression evaluating whether this is greater than [other].
  Expr operator >(Expr other) =>
      Expr.binaryExpr(left: this, right: other, op: Operator.gt);

  /// Returns an expression evaluating whether this is no lesser than [other].
  Expr operator >=(Expr other) =>
      Expr.binaryExpr(left: this, right: other, op: Operator.gtEq);

  /// Returns an expression representing the sum of this and [other].
  Expr operator +(Expr other) =>
      Expr.binaryExpr(left: this, right: other, op: Operator.plus);

  /// Returns an expression representing the difference of this and [other].
  Expr operator -(Expr other) =>
      Expr.binaryExpr(left: this, right: other, op: Operator.minus);

  /// Returns an expression representing the product of this and [other].
  Expr operator *(Expr other) =>
      Expr.binaryExpr(left: this, right: other, op: Operator.multiply);

  /// Returns an expression representing the division of this and [other].
  Expr operator /(Expr other) =>
      Expr.binaryExpr(left: this, right: other, op: Operator.divide);

  /// Returns an expression representing the integral division of this and [other].
  Expr operator ~/(Expr other) =>
      Expr.binaryExpr(left: this, right: other, op: Operator.trueDivide);
  Expr operator %(Expr other) =>
      Expr.binaryExpr(left: this, right: other, op: Operator.modulus);
  Expr operator &(Expr other) =>
      Expr.binaryExpr(left: this, right: other, op: Operator.and);
  Expr operator |(Expr other) =>
      Expr.binaryExpr(left: this, right: other, op: Operator.or);
  Expr operator ^(Expr other) =>
      Expr.binaryExpr(left: this, right: other, op: Operator.xor);
  Expr floorDivide(Expr other) =>
      Expr.binaryExpr(left: this, right: other, op: Operator.floorDivide);

  Expr alias(String name) => Expr.alias(this, name);
  Expr cast(DataType dataType, {bool strict = true}) =>
      Expr.cast(expr: this, dataType: dataType, strict: strict);
  Expr sort({
    bool descending = false,
    bool nullsLast = false,
    bool multithreaded = true,
    bool maintainOrder = true,
  }) =>
      Expr.sort(
          expr: this,
          options: SortOptions(
            descending: descending,
            nullsLast: nullsLast,
            multithreaded: multithreaded,
            maintainOrder: maintainOrder,
          ));
  Expr gather(Expr index, {bool returnsScalar = false}) => Expr.gather(
        expr: this,
        idx: index,
        returnsScalar: returnsScalar,
      );
  Expr filter(Expr by) => Expr.filter(input: this, by: by);
  Expr get explode => Expr.explode(this);
  Expr get keepName => Expr.keepName(this);

  // Aggregations
  Expr get min => Expr.agg(AggExpr.min(input: this, propagateNans: false));
  Expr get max => Expr.agg(AggExpr.max(input: this, propagateNans: false));
  Expr get nanMin => Expr.agg(AggExpr.min(input: this, propagateNans: true));
  Expr get nanMax => Expr.agg(AggExpr.max(input: this, propagateNans: true));
  Expr get median => Expr.agg(AggExpr.median(this));
  Expr get nUnique => Expr.agg(AggExpr.nUnique(this));
  Expr get first => Expr.agg(AggExpr.first(this));
  Expr get last => Expr.agg(AggExpr.last(this));
  Expr get mean => Expr.agg(AggExpr.mean(this));
  Expr get count => Expr.agg(AggExpr.count(this));
  Expr get aggGroups => Expr.agg(AggExpr.aggGroups(this));

  /// Gets the standard deviation of this column with the specified degrees of freedom.
  Expr std({int ddof = 1}) => Expr.agg(AggExpr.std(this, ddof));

  Expr quantile(Expr quantile, {required QuantileInterpolOptions interpol}) =>
      Expr.agg(AggExpr.quantile(
        expr: this,
        quantile: quantile,
        interpol: interpol,
      ));

  /// Returns a ternary expression evaluating this expression's truthiness.
  Expr then(Expr truthy, {required Expr orElse}) =>
      Expr.ternary(predicate: this, truthy: truthy, falsy: orElse);

  Expr exclude(List<Excluded> excluded) => Expr.exclude(this, excluded);
}

/// Select a column with [name].
Expr col(String name) => Expr.column(name);

/// Select columns matching [names].
Expr cols(Iterable<String> names) =>
    Expr.columns(names.toList(growable: false));

/// Select columns matching [dtypes].
Expr dtypes(Iterable<DataType> dtypes) =>
    Expr.dtypeColumn(dtypes.toList(growable: false));

/// TODO: Docs
Expr count() => const Expr.count();

/// Select the column at [index].
Expr nth(int index) => Expr.nth(index);

/// Extensions on [String].
extension StringPolars on String {
  Expr get expr => Expr.literal(LiteralValue.utf8(this));
  static const dtype = DataType.utf8();
}

/// Extensions on [int].
extension IntPolars on int {
  Expr get i8 => Expr.literal(LiteralValue.int8(this));
  Expr get i16 => Expr.literal(LiteralValue.int16(this));
  Expr get i32 => Expr.literal(LiteralValue.int32(this));
  Expr get i64 => Expr.literal(LiteralValue.int64(this));

  Expr get u8 => Expr.literal(LiteralValue.uInt8(_assertNonNegative(this)));
  Expr get u16 => Expr.literal(LiteralValue.uInt16(_assertNonNegative(this)));
  Expr get u32 => Expr.literal(LiteralValue.uInt32(_assertNonNegative(this)));
  Expr get u64 => Expr.literal(LiteralValue.uInt64(_assertNonNegative(this)));

  Expr range(int other, {DataType? dataType}) {
    return Expr.literal(LiteralValue.range(
      low: this,
      high: other >= this ? other : this,
      dataType: dataType ?? dtype,
    ));
  }

  static final dtype = _kIsWeb ? DataType.int32() : DataType.int64();

  Expr get expr => _kIsWeb ? i32 : i64;
}

/// Extensions on [double].
extension DoublePolars on double {
  Expr get f32 => Expr.literal(LiteralValue.float32(this));
  Expr get expr => Expr.literal(LiteralValue.float64(this));
  static const dtype = DataType.float64();
}

/// Extensions on [bool].
extension BoolPolars on bool {
  Expr get expr => Expr.literal(LiteralValue.boolean(this));
  static const dtype = DataType.boolean();
}

/// Extensions on [DateTime].
extension DateTimePolars on DateTime {
  static final dtype = DataType.datetime(
      _kIsWeb ? TimeUnit.milliseconds : TimeUnit.microseconds);
}

/// Extensions on [Duration].
extension DurationPolars on Duration {
  static final dtype = DataType.duration(
      _kIsWeb ? TimeUnit.milliseconds : TimeUnit.microseconds);
}

int _assertNonNegative(int value) {
  assert(value >= 0, 'Expected non-negative integer, got $value');
  return value;
}
