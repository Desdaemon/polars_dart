import 'wrapper/entry.dart';
import 'wrapper/expr.dart';

final _kIsWeb = 0 == 0.0;

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
  Expr get expr => lit(value: LiteralValue.utf8(this));
  static const dtype = DataType.utf8();
}

/// Extensions on [int].
extension IntPolars on int {
  // Expr get i8 => lit(value: LiteralValue.int8(this));
  // Expr get i16 => lit(value: LiteralValue.int16(this));
  Expr get i32 => lit(value: LiteralValue.int32(this));
  Expr get i64 => lit(value: LiteralValue.int64(this));

  // Expr get u8 => lit(value: LiteralValue.uInt8(_assertNonNegative(this)));
  // Expr get u16 => lit(value: LiteralValue.uInt16(_assertNonNegative(this)));
  Expr get u32 => lit(value: LiteralValue.uInt32(_assertNonNegative(this)));
  Expr get u64 => lit(value: LiteralValue.uInt64(_assertNonNegative(this)));

  Expr range(int other, {DataType? dataType}) {
    return lit(
      value: LiteralValue.range(
        low: this,
        high: other >= this ? other : this,
        dataType: dataType ?? dtype,
      ),
    );
  }

  static final dtype = _kIsWeb ? DataType.int32() : DataType.int64();

  Expr get expr => _kIsWeb ? i32 : i64;
}

/// Extensions on [double].
extension DoublePolars on double {
  Expr get f32 => lit(value: LiteralValue.float32(this));
  Expr get expr => lit(value: LiteralValue.float64(this));
  static const dtype = DataType.float64();
}

/// Extensions on [bool].
extension BoolPolars on bool {
  Expr get expr => lit(
      value: this
          ? const LiteralValue.boolean(true)
          : const LiteralValue.boolean(false));
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
