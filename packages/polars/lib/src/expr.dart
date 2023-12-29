import 'wrapper/entry.dart';
import 'wrapper/expr.dart';

final _kIsWeb = 0 == 0.0;

extension LiteralValueExt on LiteralValue {
  /// Returns an expression representing this literal value.
  Expr get expr => Expr.literal(value: this);
}

/// Extensions for [Expr].
extension ExprExt on Expr {
  /// Returns an expression evaluating whether this is less than [other].
  Expr operator <(Expr other) => lt(other: other);

  /// Returns an expression evaluating whether this is no greater than [other].
  Expr operator <=(Expr other) => ltEq(other: other);

  /// Returns an expression evaluating whether this is greater than [other].
  Expr operator >(Expr other) => gt(other: other);

  /// Returns an expression evaluating whether this is no lesser than [other].
  Expr operator >=(Expr other) => gtEq(other: other);

  /// Returns an expression representing the sum of this and [other].
  Expr operator +(Expr other) => add(other: other);

  /// Returns an expression representing the difference of this and [other].
  Expr operator -(Expr other) => sub(other: other);

  /// Returns an expression representing the product of this and [other].
  Expr operator *(Expr other) => mul(other: other);

  /// Returns an expression representing the division of this and [other].
  Expr operator /(Expr other) => div(other: other);

  /// Returns an expression representing the integral division of this and [other].
  Expr operator ~/(Expr other) => floorDiv(rhs: other);

  /// Performs a modulo operation on this and [other].
  Expr operator %(Expr other) => rem(other: other);

  /// Returns an expression evaluating whether both this and [other] are true.
  Expr operator &(Expr other) => and(expr: other);

  /// Returns an expression evaluating whether either this or [other] is true.
  Expr operator |(Expr other) => or(expr: other);

  /// Returns an expression evaluating whether at most one of this and [other] is true,
  /// i.e. whether this and [other] are not equal.
  Expr operator ^(Expr other) => xor(expr: other);
}

/// Extensions on [String].
extension StringPolars on String {
  Expr get expr => Expr.literal(value: LiteralValue.utf8(this));
  DataType get dtype => const DataType.utf8();
}

/// Extensions on [int].
extension IntPolars on int {
  Expr get i32 => Expr.literal(value: LiteralValue.int32(this));
  Expr get i64 => Expr.literal(value: LiteralValue.int64(this));

  Expr get u32 =>
      Expr.literal(value: LiteralValue.uint32(_assertNonNegative(this)));
  Expr get u64 =>
      Expr.literal(value: LiteralValue.uint64(_assertNonNegative(this)));

  Expr range(int other, {DataType? dataType}) {
    return Expr.literal(
      value: LiteralValue.range(
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
  Expr get f32 => Expr.literal(value: LiteralValue.float32(this));
  Expr get f64 => Expr.literal(value: LiteralValue.float64(this));
  Expr get expr => f64;
  DataType get dtype => const DataType.float64();
}

/// Extensions on [bool].
extension BoolPolars on bool {
  Expr get expr => Expr.literal(
      value: this
          ? const LiteralValue.boolean(true)
          : const LiteralValue.boolean(false));
  DataType get dtype => const DataType.boolean();
}

/// Extensions on [DateTime].
extension DateTimePolars on DateTime {
  DataType get dtype => DataType.datetime(
      _kIsWeb ? TimeUnit.milliseconds : TimeUnit.microseconds);
  Expr get expr => Expr.literal(
      value: _kIsWeb
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
  Expr get expr => Expr.literal(
      value: _kIsWeb
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
        null => Expr.literal(value: const LiteralValue.Null()),
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
