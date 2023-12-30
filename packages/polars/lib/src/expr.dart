import 'package:polars/src/str.dart';

import 'wrapper/entry.dart';
import 'wrapper/expr.dart';

final _kIsWeb = 0 == 0.0;

extension LiteralValueExt on LiteralValue {
  /// Returns an expression representing this literal value.
  Expr get expr => Expr.literal(this);
}

/// Extensions for [Expr].
extension ExprExt on Expr {
  /// Returns an expression evaluating whether this is less than [other].
  Expr operator <(Object? other) =>
      Expr.binaryExpr(left: this, op: Operator.lt, right: other.expr);

  /// Returns an expression evaluating whether this is no greater than [other].
  Expr operator <=(Object? other) =>
      Expr.binaryExpr(left: this, op: Operator.ltEq, right: other.expr);

  /// Returns an expression evaluating whether this is greater than [other].
  Expr operator >(Object? other) =>
      Expr.binaryExpr(left: this, op: Operator.gt, right: other.expr);

  /// Returns an expression evaluating whether this is no lesser than [other].
  Expr operator >=(Object? other) =>
      Expr.binaryExpr(left: this, op: Operator.gtEq, right: other.expr);

  /// Returns an expression representing the sum of this and [other].
  Expr operator +(Object? other) =>
      Expr.binaryExpr(left: this, op: Operator.plus, right: other.expr);

  /// Returns an expression representing the difference of this and [other].
  Expr operator -(Object? other) =>
      Expr.binaryExpr(left: this, op: Operator.minus, right: other.expr);

  /// Returns an expression representing the product of this and [other].
  Expr operator *(Object? other) =>
      Expr.binaryExpr(left: this, op: Operator.multiply, right: other.expr);

  /// Returns an expression representing the division of this and [other].
  Expr operator /(Object? other) =>
      Expr.binaryExpr(left: this, op: Operator.divide, right: other.expr);

  /// Returns an expression representing the integral division of this and [other].
  Expr operator ~/(Object? other) =>
      Expr.binaryExpr(left: this, op: Operator.floorDivide, right: other.expr);

  /// Performs a modulo operation on this and [other].
  Expr operator %(Object? other) =>
      Expr.binaryExpr(left: this, op: Operator.modulus, right: other.expr);

  /// Returns an expression evaluating whether both this and [other] are true.
  Expr operator &(Object? other) =>
      Expr.binaryExpr(left: this, op: Operator.and, right: other.expr);

  /// Returns an expression evaluating whether either this or [other] is true.
  Expr operator |(Object? other) =>
      Expr.binaryExpr(left: this, op: Operator.or, right: other.expr);

  /// Returns an expression evaluating whether at most one of this and [other] is true,
  /// i.e. whether this and [other] are not equal.
  Expr operator ^(Object? other) =>
      Expr.binaryExpr(left: this, op: Operator.xor, right: other.expr);

  Expr head({int length = 10}) => Expr.slice(
      input: this,
      offset: const Expr.literal(LiteralValue.int64(0)),
      length: length.expr);

  Expr tail({int length = 10}) =>
      Expr.slice(input: this, offset: (-length).expr, length: length.expr);

  Expr alias(String name) => Expr.alias(this, name);
  Expr get aggGroups => Expr.agg(AggExpr.aggGroups(this));

  /// Attempt to [cast](https://docs.pola.rs/user-guide/expressions/casting) a column's [DataType] to a new one.
  ///
  /// By default, strict mode is enabled and restricts certain types of casts:
  /// - String-to-numeric casts will throw if the string cannot be parsed as a number.
  /// - Downcasts (e.g. [int64] to [int32]) that result in overflowing values will throw.
  ///
  /// When strict mode is disabled, these casts will return null instead.
  Expr cast(DataType dataType, {bool strict = true}) =>
      Expr.cast(expr: this, dataType: dataType, strict: strict);
  Expr equalMissing(Object? other) =>
      Expr.binaryExpr(left: this, op: Operator.eqValidity, right: other.expr);
  Expr notEqualMissing(Object? other) => Expr.binaryExpr(
      left: this, op: Operator.notEqValidity, right: other.expr);
  Expr exclude(Iterable<String> columns) =>
      Expr.exclude(this, columns.map(Excluded.name).toList(growable: false));
  Expr get explode => Expr.explode(this);

  /// Alias for [explode].
  Expr get flatten => Expr.explode(this);
  Expr filter({required Object? by}) => Expr.filter(input: this, by: by.expr);
  Expr get first => Expr.agg(AggExpr.first(this));
  Expr get last => Expr.agg(AggExpr.last(this));
  Expr get({required Object? index, bool returnsScalar = true}) =>
      Expr.gather(expr: this, idx: index.expr, returnsScalar: returnsScalar);
  Expr get implode => Expr.agg(AggExpr.implode(this));
  Expr get nUnique => Expr.agg(AggExpr.nUnique(this));
  Expr get nanMax => Expr.agg(AggExpr.max(input: this, propagateNans: true));
  Expr get nanMin => Expr.agg(AggExpr.min(input: this, propagateNans: true));
  Expr slice(int offset, int length) =>
      Expr.slice(input: this, offset: offset.expr, length: length.expr);

  /// Calculate the standard deviation of this expression with the specified
  /// [ddof] or [delta degrees of freedom](https://en.wikipedia.org/wiki/Degrees_of_freedom_(statistics)).
  Expr std(int ddof) => Expr.agg(AggExpr.std(this, ddof));

  StrNamespace get str => StrNamespace(this);
}

Expr col(String column) => Expr.column(column);
Expr cols(Iterable<String> columns) =>
    Expr.columns(columns.toList(growable: false));
Expr dtypes(Iterable<DataType> dtypes) =>
    Expr.dtypeColumn(dtypes.toList(growable: false));

/// Begin a chain of [when-then-otherwise](https://docs.pola.rs/user-guide/expressions/functions/#conditionals) expressions.
///
/// ### Example:
/// ```dart
/// final data = await df.clone().lazy().select([
///   when(col('a') > 0, then: col('a') * 2)
///     .when(col('a') < 0, then: col('a') * 3)
///     .otherwise(col('a'))
///     .alias('new_a'),
/// ]).collect();
/// ```
When when(
  Object? condition, {
  required Object? then,
}) =>
    When(condition, then);

class When {
  final List<(Expr, Expr)> _chains;
  Expr _otherwise = const Expr.literal(LiteralValue.Null());

  When(Object? condition, Object? then)
      : _chains = [(condition.expr, then.expr)];

  When when(Object? condition, {required Object? then}) {
    _chains.add((condition.expr, then.expr));
    return this;
  }

  Expr otherwise(Object? otherwise) {
    _otherwise = otherwise.expr;
    return expr;
  }

  Expr get expr {
    var root = _otherwise;
    for (final (cond, truthy) in _chains.reversed) {
      root = Expr.ternary(predicate: cond, truthy: truthy, falsy: root);
    }
    return root;
  }
}

/// Extensions on [String].
extension StringPolars on String {
  Expr get expr => Expr.literal(LiteralValue.utf8(this));
  DataType get dtype => const DataType.utf8();
}

/// Extensions on [int].
extension IntPolars on int {
  Expr get i32 => Expr.literal(LiteralValue.int32(this));
  Expr get i64 => Expr.literal(LiteralValue.int64(this));

  Expr get u32 => Expr.literal(LiteralValue.uint32(_assertNonNegative(this)));
  Expr get u64 => Expr.literal(LiteralValue.uint64(_assertNonNegative(this)));

  Expr range(int other, {DataType? dataType}) {
    return Expr.literal(
      LiteralValue.range(
        low: this,
        high: other >= this ? other : this,
        dataType: dataType ?? dtype,
      ),
    );
  }

  DataType get dtype => const DataType.int64();
  Expr get expr => i64;
}

/// Extensions on [double].
extension DoublePolars on double {
  Expr get f32 => Expr.literal(LiteralValue.float32(this));
  Expr get f64 => Expr.literal(LiteralValue.float64(this));
  Expr get expr => f64;
  DataType get dtype => const DataType.float64();
}

/// Extensions on [bool].
extension BoolPolars on bool {
  Expr get expr => Expr.literal(this
      ? const LiteralValue.boolean(true)
      : const LiteralValue.boolean(false));
  DataType get dtype => const DataType.boolean();
}

/// Extensions on [DateTime].
extension DateTimePolars on DateTime {
  DataType get dtype => DataType.datetime(
      _kIsWeb ? TimeUnit.milliseconds : TimeUnit.microseconds);
  Expr get expr => Expr.literal(_kIsWeb
      ? LiteralValue.dateTime(
          millisecondsSinceEpoch,
          TimeUnit.milliseconds,
        )
      : LiteralValue.dateTime(
          microsecondsSinceEpoch,
          TimeUnit.microseconds,
        ));
}

/// Extensions on [Duration].
extension DurationPolars on Duration {
  DataType get dtype => DataType.duration(
      _kIsWeb ? TimeUnit.milliseconds : TimeUnit.microseconds);
  Expr get expr => Expr.literal(_kIsWeb
      ? LiteralValue.duration(
          inMilliseconds,
          TimeUnit.milliseconds,
        )
      : LiteralValue.duration(
          inMicroseconds,
          TimeUnit.microseconds,
        ));
}

extension DynamicPolars on dynamic {
  Expr get expr => switch (this) {
        int value => value.expr,
        double value => value.expr,
        String value => value.expr,
        bool value => value.expr,
        DateTime value => value.expr,
        Duration value => value.expr,
        Expr expr => expr,
        When ternary => ternary.expr,
        StrNamespace ns => ns.expr,
        LiteralValue lit => Expr.literal(lit),
        null => const Expr.literal(LiteralValue.Null()),
        _ => '$this'.expr,
      };
  DataType get dtype => switch (this) {
        int value => value.dtype,
        double value => value.dtype,
        String value => value.dtype,
        bool value => value.dtype,
        DateTime value => value.dtype,
        Duration value => value.dtype,
        null => const DataType.Null(),
        _ => const DataType.unknown(),
      };
}

int _assertNonNegative(int value) {
  assert(value >= 0, 'Value must be non-negative.');
  return value;
}

const int8 = DataType.int8();
const int16 = DataType.int16();
const int32 = DataType.int32();
const int64 = DataType.int64();
const uint8 = DataType.uint8();
const uint16 = DataType.uint16();
const uint32 = DataType.uint32();
const uint64 = DataType.uint64();
const utf8 = DataType.utf8();
const boolean = DataType.boolean();
const binary = DataType.binary();
const time = DataType.time();
const unknown = DataType.unknown();
