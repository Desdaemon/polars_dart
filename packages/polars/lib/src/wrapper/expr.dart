// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.6.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../frb_generated.dart';
import 'df.dart';
import 'entry.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:freezed_annotation/freezed_annotation.dart' hide protected;
part 'expr.freezed.dart';

@freezed
sealed class AggExpr with _$AggExpr {
  const AggExpr._();
  const factory AggExpr.min({
    required Expr input,
    required bool propagateNans,
  }) = AggExpr_Min;
  const factory AggExpr.max({
    required Expr input,
    required bool propagateNans,
  }) = AggExpr_Max;
  const factory AggExpr.median(
    Expr field0,
  ) = AggExpr_Median;
  const factory AggExpr.nUnique(
    Expr field0,
  ) = AggExpr_NUnique;
  const factory AggExpr.first(
    Expr field0,
  ) = AggExpr_First;
  const factory AggExpr.last(
    Expr field0,
  ) = AggExpr_Last;
  const factory AggExpr.mean(
    Expr field0,
  ) = AggExpr_Mean;
  const factory AggExpr.implode(
    Expr field0,
  ) = AggExpr_Implode;
  const factory AggExpr.count(
    Expr field0,
  ) = AggExpr_Count;
  const factory AggExpr.quantile({
    required Expr expr,
    required Expr quantile,
    required QuantileInterpolOptions interpol,
  }) = AggExpr_Quantile;
  const factory AggExpr.sum(
    Expr field0,
  ) = AggExpr_Sum;
  const factory AggExpr.aggGroups(
    Expr field0,
  ) = AggExpr_AggGroups;
  const factory AggExpr.std(
    Expr field0,
    int field1,
  ) = AggExpr_Std;
  const factory AggExpr.Var(
    Expr field0,
    int field1,
  ) = AggExpr_Var;
}

enum ClosedWindow {
  left,
  right,
  both,
  none,
}

/// Supported datatypes in a [DataFrame].
@freezed
sealed class DataType with _$DataType {
  const DataType._();

  /// Boolean
  const factory DataType.boolean() = DataType_Boolean;

  /// Unsigned 8-bit integer
  const factory DataType.uint8() = DataType_Uint8;

  /// Unsigned 16-bit integer
  const factory DataType.uint16() = DataType_Uint16;

  /// Unsigned 32-bit integer
  const factory DataType.uint32() = DataType_Uint32;

  /// Unsigned 64-bit integer
  const factory DataType.uint64() = DataType_Uint64;

  /// Signed 8-bit integer
  const factory DataType.int8() = DataType_Int8;

  /// Signed 16-bit integer
  const factory DataType.int16() = DataType_Int16;

  /// Signed 32-bit integer
  const factory DataType.int32() = DataType_Int32;

  /// Signed 64-bit integer, the default [int] on native platforms.
  const factory DataType.int64() = DataType_Int64;

  /// Single-precision floating point number
  const factory DataType.float32() = DataType_Float32;

  /// Double-precision floating point number, aka a [double].
  const factory DataType.float64() = DataType_Float64;

  /// String data
  const factory DataType.utf8() = DataType_Utf8;

  /// Raw bytes.
  const factory DataType.binary() = DataType_Binary;

  /// A 32-bit date representing the elapsed time since UNIX epoch (1970-01-01)
  /// in days (32 bits).
  const factory DataType.date() = DataType_Date;

  /// A 64-bit date representing the elapsed time since UNIX epoch (1970-01-01)
  /// in the given timeunit (64 bits), with optional timezone.
  const factory DataType.datetime(
    TimeUnit field0, [
    String? field1,
  ]) = DataType_Datetime;

  /// 64-bit integer representing difference between times in milliseconds or nanoseconds
  const factory DataType.duration(
    TimeUnit field0,
  ) = DataType_Duration;

  /// A 64-bit time representing the elapsed time since midnight in nanoseconds
  const factory DataType.time() = DataType_Time;

  /// A typed list.
  const factory DataType.list(
    DataType field0,
  ) = DataType_List;
  const factory DataType.struct(
    List<Field> field0,
  ) = DataType_Struct;

  /// Null value.
  const factory DataType.Null() = DataType_Null;

  /// Some logical types we cannot know statically, e.g. Datetime
  const factory DataType.unknown() = DataType_Unknown;
}

@freezed
sealed class Excluded with _$Excluded {
  const Excluded._();
  const factory Excluded.name(
    String field0,
  ) = Excluded_Name;
  const factory Excluded.dtype(
    DataType field0,
  ) = Excluded_Dtype;
}

/// Expressions for use in query and aggregration operations.
@freezed
sealed class Expr with _$Expr {
  const Expr._();
  const factory Expr.alias(
    Expr field0,
    String field1,
  ) = Expr_Alias;
  const factory Expr.column(
    String field0,
  ) = Expr_Column;
  const factory Expr.columns(
    List<String> field0,
  ) = Expr_Columns;
  const factory Expr.dtypeColumn(
    List<DataType> field0,
  ) = Expr_DtypeColumn;
  const factory Expr.literal(
    LiteralValue field0,
  ) = Expr_Literal;
  const factory Expr.binaryExpr({
    required Expr left,
    required Operator op,
    required Expr right,
  }) = Expr_BinaryExpr;
  const factory Expr.cast({
    required Expr expr,
    required DataType dataType,
    required bool strict,
  }) = Expr_Cast;
  const factory Expr.sort({
    required Expr expr,
    @Default(const SortOptions()) SortOptions options,
  }) = Expr_Sort;
  const factory Expr.gather({
    required Expr expr,
    required Expr idx,
    required bool returnsScalar,
  }) = Expr_Gather;
  const factory Expr.sortBy({
    required Expr expr,
    @Default(const []) List<Expr> by,
    @Default(const []) List<bool> descending,
  }) = Expr_SortBy;
  const factory Expr.agg(
    AggExpr field0,
  ) = Expr_Agg;
  const factory Expr.ternary({
    required Expr predicate,
    required Expr truthy,
    required Expr falsy,
  }) = Expr_Ternary;
  const factory Expr.explode(
    Expr field0,
  ) = Expr_Explode;
  const factory Expr.filter({
    required Expr input,
    required Expr by,
  }) = Expr_Filter;
  const factory Expr.wildcard() = Expr_Wildcard;
  const factory Expr.window({
    required Expr function,
    required List<Expr> partitionBy,
    required WindowType options,
  }) = Expr_Window;
  const factory Expr.slice({
    required Expr input,
    required Expr offset,
    required Expr length,
  }) = Expr_Slice;
  const factory Expr.exclude(
    Expr field0,
    List<Excluded> field1,
  ) = Expr_Exclude;
  const factory Expr.keepName(
    Expr field0,
  ) = Expr_KeepName;
  const factory Expr.count() = Expr_Count;
  const factory Expr.nth(
    int field0,
  ) = Expr_Nth;
  const factory Expr.internal(
    PExpr field0,
  ) = Expr_Internal;

  /// Similar to [gather] but allows for scalars.
  Expr abs({dynamic hint}) => RustLib.instance.api.exprAbs(
        that: this,
      );

  Expr all({bool ignoreNulls = false, dynamic hint}) =>
      RustLib.instance.api.exprAll(
        that: this,
        ignoreNulls: ignoreNulls,
      );

  Expr any({bool ignoreNulls = false, dynamic hint}) =>
      RustLib.instance.api.exprAny(
        that: this,
        ignoreNulls: ignoreNulls,
      );

  Expr append({required Expr other, bool upcast = true, dynamic hint}) =>
      RustLib.instance.api.exprAppend(
        that: this,
        other: other,
        upcast: upcast,
      );

  Expr arccos({dynamic hint}) => RustLib.instance.api.exprArccos(
        that: this,
      );

  Expr arccosh({dynamic hint}) => RustLib.instance.api.exprArccosh(
        that: this,
      );

  Expr arcsin({dynamic hint}) => RustLib.instance.api.exprArcsin(
        that: this,
      );

  Expr arcsinh({dynamic hint}) => RustLib.instance.api.exprArcsinh(
        that: this,
      );

  Expr arctan({dynamic hint}) => RustLib.instance.api.exprArctan(
        that: this,
      );

  Expr arctan2({required Expr x, dynamic hint}) =>
      RustLib.instance.api.exprArctan2(
        that: this,
        x: x,
      );

  Expr arctanh({dynamic hint}) => RustLib.instance.api.exprArctanh(
        that: this,
      );

  Expr argMax({dynamic hint}) => RustLib.instance.api.exprArgMax(
        that: this,
      );

  Expr argMin({dynamic hint}) => RustLib.instance.api.exprArgMin(
        that: this,
      );

  Expr argSort(
          {bool descending = false,
          bool nullsLast = false,
          bool multithreaded = true,
          bool maintainOrder = false,
          dynamic hint}) =>
      RustLib.instance.api.exprArgSort(
        that: this,
        descending: descending,
        nullsLast: nullsLast,
        multithreaded: multithreaded,
        maintainOrder: maintainOrder,
      );

  Expr argUnique({dynamic hint}) => RustLib.instance.api.exprArgUnique(
        that: this,
      );

  Expr backwardFill({int? limit, dynamic hint}) =>
      RustLib.instance.api.exprBackwardFill(
        that: this,
        limit: limit,
      );

  Expr cbrt({dynamic hint}) => RustLib.instance.api.exprCbrt(
        that: this,
      );

  Expr ceil({dynamic hint}) => RustLib.instance.api.exprCeil(
        that: this,
      );

  Expr clip({required Expr min, required Expr max, dynamic hint}) =>
      RustLib.instance.api.exprClip(
        that: this,
        min: min,
        max: max,
      );

  Expr clipMax({required Expr max, dynamic hint}) =>
      RustLib.instance.api.exprClipMax(
        that: this,
        max: max,
      );

  Expr clipMin({required Expr min, dynamic hint}) =>
      RustLib.instance.api.exprClipMin(
        that: this,
        min: min,
      );

  Expr cos({dynamic hint}) => RustLib.instance.api.exprCos(
        that: this,
      );

  Expr cosh({dynamic hint}) => RustLib.instance.api.exprCosh(
        that: this,
      );

  /// Calculate the cotangent of this expression.
  Expr cot({dynamic hint}) => RustLib.instance.api.exprCot(
        that: this,
      );

  Expr count({dynamic hint}) => RustLib.instance.api.exprCount(
        that: this,
      );

  Expr cumCount({bool reverse = false, dynamic hint}) =>
      RustLib.instance.api.exprCumCount(
        that: this,
        reverse: reverse,
      );

  Expr cumMax({bool reverse = false, dynamic hint}) =>
      RustLib.instance.api.exprCumMax(
        that: this,
        reverse: reverse,
      );

  Expr cumMin({bool reverse = false, dynamic hint}) =>
      RustLib.instance.api.exprCumMin(
        that: this,
        reverse: reverse,
      );

  Expr cumProd({bool reverse = false, dynamic hint}) =>
      RustLib.instance.api.exprCumProd(
        that: this,
        reverse: reverse,
      );

  Expr cumSum({bool reverse = false, dynamic hint}) =>
      RustLib.instance.api.exprCumSum(
        that: this,
        reverse: reverse,
      );

  Expr degrees({dynamic hint}) => RustLib.instance.api.exprDegrees(
        that: this,
      );

  Expr div({required Expr other, dynamic hint}) => RustLib.instance.api.exprDiv(
        that: this,
        other: other,
      );

  Expr dot({required Expr other, dynamic hint}) => RustLib.instance.api.exprDot(
        that: this,
        other: other,
      );

  Expr dropNans({dynamic hint}) => RustLib.instance.api.exprDropNans(
        that: this,
      );

  Expr dropNulls({dynamic hint}) => RustLib.instance.api.exprDropNulls(
        that: this,
      );

  Expr entropy({required double base, bool normalize = false, dynamic hint}) =>
      RustLib.instance.api.exprEntropy(
        that: this,
        base: base,
        normalize: normalize,
      );

  Expr exp({dynamic hint}) => RustLib.instance.api.exprExp(
        that: this,
      );

  Expr fillNan({required Expr value, dynamic hint}) =>
      RustLib.instance.api.exprFillNan(
        that: this,
        value: value,
      );

  Expr fillNull({required Expr value, dynamic hint}) =>
      RustLib.instance.api.exprFillNull(
        that: this,
        value: value,
      );

  Expr floor({dynamic hint}) => RustLib.instance.api.exprFloor(
        that: this,
      );

  Expr forwardFill({int? limit, dynamic hint}) =>
      RustLib.instance.api.exprForwardFill(
        that: this,
        limit: limit,
      );

  Expr isFinite({dynamic hint}) => RustLib.instance.api.exprIsFinite(
        that: this,
      );

  Expr isIn({required Expr other, dynamic hint}) =>
      RustLib.instance.api.exprIsIn(
        that: this,
        other: other,
      );

  Expr isNan({dynamic hint}) => RustLib.instance.api.exprIsNan(
        that: this,
      );

  Expr isNotNan({dynamic hint}) => RustLib.instance.api.exprIsNotNan(
        that: this,
      );

  Expr isNotNull({dynamic hint}) => RustLib.instance.api.exprIsNotNull(
        that: this,
      );

  Expr isNull({dynamic hint}) => RustLib.instance.api.exprIsNull(
        that: this,
      );

  Expr log({required double base, dynamic hint}) =>
      RustLib.instance.api.exprLog(
        that: this,
        base: base,
      );

  Expr log1P({dynamic hint}) => RustLib.instance.api.exprLog1P(
        that: this,
      );

  Expr lowerBound({dynamic hint}) => RustLib.instance.api.exprLowerBound(
        that: this,
      );

  Expr not({dynamic hint}) => RustLib.instance.api.exprNot(
        that: this,
      );

  Expr nullCount({dynamic hint}) => RustLib.instance.api.exprNullCount(
        that: this,
      );

  Expr pow({required double exponent, dynamic hint}) =>
      RustLib.instance.api.exprPow(
        that: this,
        exponent: exponent,
      );

  Expr product({dynamic hint}) => RustLib.instance.api.exprProduct(
        that: this,
      );

  Expr radians({dynamic hint}) => RustLib.instance.api.exprRadians(
        that: this,
      );

  Expr reshape({required Int64List dims, dynamic hint}) =>
      RustLib.instance.api.exprReshape(
        that: this,
        dims: dims,
      );

  Expr reverse({dynamic hint}) => RustLib.instance.api.exprReverse(
        that: this,
      );

  /// TODO: Docs for rolling_max
  Expr rollingMax(
          {Duration? windowSize,
          int minPeriods = 1,
          Float64List? weights,
          bool center = false,
          String? by,
          ClosedWindow? closedWindow,
          dynamic hint}) =>
      RustLib.instance.api.exprRollingMax(
        that: this,
        windowSize: windowSize,
        minPeriods: minPeriods,
        weights: weights,
        center: center,
        by: by,
        closedWindow: closedWindow,
      );

  /// TODO: Docs for rolling_mean
  Expr rollingMean(
          {Duration? windowSize,
          int minPeriods = 1,
          Float64List? weights,
          bool center = false,
          String? by,
          ClosedWindow? closedWindow,
          dynamic hint}) =>
      RustLib.instance.api.exprRollingMean(
        that: this,
        windowSize: windowSize,
        minPeriods: minPeriods,
        weights: weights,
        center: center,
        by: by,
        closedWindow: closedWindow,
      );

  /// TODO: Docs for rolling_median
  Expr rollingMedian(
          {Duration? windowSize,
          int minPeriods = 1,
          Float64List? weights,
          bool center = false,
          String? by,
          ClosedWindow? closedWindow,
          dynamic hint}) =>
      RustLib.instance.api.exprRollingMedian(
        that: this,
        windowSize: windowSize,
        minPeriods: minPeriods,
        weights: weights,
        center: center,
        by: by,
        closedWindow: closedWindow,
      );

  /// TODO: Docs for rolling_min
  Expr rollingMin(
          {Duration? windowSize,
          int minPeriods = 1,
          Float64List? weights,
          bool center = false,
          String? by,
          ClosedWindow? closedWindow,
          dynamic hint}) =>
      RustLib.instance.api.exprRollingMin(
        that: this,
        windowSize: windowSize,
        minPeriods: minPeriods,
        weights: weights,
        center: center,
        by: by,
        closedWindow: closedWindow,
      );

  /// TODO: Docs for rolling_quantile
  Expr rollingQuantile(
          {Duration? windowSize,
          int minPeriods = 1,
          Float64List? weights,
          bool center = false,
          String? by,
          ClosedWindow? closedWindow,
          dynamic hint}) =>
      RustLib.instance.api.exprRollingQuantile(
        that: this,
        windowSize: windowSize,
        minPeriods: minPeriods,
        weights: weights,
        center: center,
        by: by,
        closedWindow: closedWindow,
      );

  /// TODO: Docs for rolling_std
  Expr rollingStd(
          {Duration? windowSize,
          int minPeriods = 1,
          Float64List? weights,
          bool center = false,
          String? by,
          ClosedWindow? closedWindow,
          dynamic hint}) =>
      RustLib.instance.api.exprRollingStd(
        that: this,
        windowSize: windowSize,
        minPeriods: minPeriods,
        weights: weights,
        center: center,
        by: by,
        closedWindow: closedWindow,
      );

  /// TODO: Docs for rolling_sum
  Expr rollingSum(
          {Duration? windowSize,
          int minPeriods = 1,
          Float64List? weights,
          bool center = false,
          String? by,
          ClosedWindow? closedWindow,
          dynamic hint}) =>
      RustLib.instance.api.exprRollingSum(
        that: this,
        windowSize: windowSize,
        minPeriods: minPeriods,
        weights: weights,
        center: center,
        by: by,
        closedWindow: closedWindow,
      );

  /// TODO: Docs for rolling_var
  Expr rollingVar(
          {Duration? windowSize,
          int minPeriods = 1,
          Float64List? weights,
          bool center = false,
          String? by,
          ClosedWindow? closedWindow,
          dynamic hint}) =>
      RustLib.instance.api.exprRollingVar(
        that: this,
        windowSize: windowSize,
        minPeriods: minPeriods,
        weights: weights,
        center: center,
        by: by,
        closedWindow: closedWindow,
      );

  Expr round({required int decimals, dynamic hint}) =>
      RustLib.instance.api.exprRound(
        that: this,
        decimals: decimals,
      );

  Expr roundSigFigs({required int digits, dynamic hint}) =>
      RustLib.instance.api.exprRoundSigFigs(
        that: this,
        digits: digits,
      );

  Expr setSortedFlag({required IsSorted sorted, dynamic hint}) =>
      RustLib.instance.api.exprSetSortedFlag(
        that: this,
        sorted: sorted,
      );

  Expr shift({required Expr n, dynamic hint}) => RustLib.instance.api.exprShift(
        that: this,
        n: n,
      );

  Expr shiftAndFill({required Expr n, required Expr fillValue, dynamic hint}) =>
      RustLib.instance.api.exprShiftAndFill(
        that: this,
        n: n,
        fillValue: fillValue,
      );

  Expr shrinkDtype({dynamic hint}) => RustLib.instance.api.exprShrinkDtype(
        that: this,
      );

  Expr sin({dynamic hint}) => RustLib.instance.api.exprSin(
        that: this,
      );

  Expr sinh({dynamic hint}) => RustLib.instance.api.exprSinh(
        that: this,
      );

  Expr sqrt({dynamic hint}) => RustLib.instance.api.exprSqrt(
        that: this,
      );

  Expr tan({dynamic hint}) => RustLib.instance.api.exprTan(
        that: this,
      );

  Expr tanh({dynamic hint}) => RustLib.instance.api.exprTanh(
        that: this,
      );

  /// Returns a dot representation of this expression.
  String toDot({dynamic hint}) => RustLib.instance.api.exprToDot(
        that: this,
      );

  Expr toPhysical({dynamic hint}) => RustLib.instance.api.exprToPhysical(
        that: this,
      );

  Expr unique({dynamic hint}) => RustLib.instance.api.exprUnique(
        that: this,
      );

  Expr uniqueStable({dynamic hint}) => RustLib.instance.api.exprUniqueStable(
        that: this,
      );

  Expr upperBound({dynamic hint}) => RustLib.instance.api.exprUpperBound(
        that: this,
      );

  Expr valueCounts({bool sort = false, bool parallel = true, dynamic hint}) =>
      RustLib.instance.api.exprValueCounts(
        that: this,
        sort: sort,
        parallel: parallel,
      );
}

class Field {
  final String name;
  final DataType dtype;

  const Field({
    required this.name,
    required this.dtype,
  });

  @override
  int get hashCode => name.hashCode ^ dtype.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is Field &&
          runtimeType == other.runtimeType &&
          name == other.name &&
          dtype == other.dtype;
}

enum IsSorted {
  ascending,
  descending,
  not,
}

/// Literal values for use in [Expr]essions.
@freezed
sealed class LiteralValue with _$LiteralValue {
  const LiteralValue._();

  /// Null value.
  const factory LiteralValue.Null() = LiteralValue_Null;

  /// A binary true or false.
  const factory LiteralValue.boolean(
    bool field0,
  ) = LiteralValue_Boolean;

  /// A UTF8 encoded string type.
  const factory LiteralValue.utf8(
    String field0,
  ) = LiteralValue_Utf8;

  /// A raw binary array
  const factory LiteralValue.binary(
    Uint8List field0,
  ) = LiteralValue_Binary;

  /// An unsigned 32-bit integer number.
  const factory LiteralValue.uint32(
    int field0,
  ) = LiteralValue_Uint32;

  /// An unsigned 64-bit integer number.
  const factory LiteralValue.uint64(
    int field0,
  ) = LiteralValue_Uint64;

  /// A 32-bit integer number.
  const factory LiteralValue.int32(
    int field0,
  ) = LiteralValue_Int32;

  /// A 64-bit integer number.
  const factory LiteralValue.int64(
    int field0,
  ) = LiteralValue_Int64;

  /// A 32-bit floating point number.
  const factory LiteralValue.float32(
    double field0,
  ) = LiteralValue_Float32;

  /// A 64-bit floating point number.
  const factory LiteralValue.float64(
    double field0,
  ) = LiteralValue_Float64;

  /// A range between integers.
  const factory LiteralValue.range({
    /// The starting value of the range.
    required int low,

    /// The ending value of the range.
    required int high,

    /// The datatype of this range's ends.
    required DataType dataType,
  }) = LiteralValue_Range;

  /// Datetimes, with optional timezone.
  const factory LiteralValue.dateTime(
    int field0,
    TimeUnit field1, [
    String? field2,
  ]) = LiteralValue_DateTime;

  /// Durations.
  const factory LiteralValue.duration(
    int field0,
    TimeUnit field1,
  ) = LiteralValue_Duration;
  const factory LiteralValue.series(
    SpecialEqPSeries field0,
  ) = LiteralValue_Series;
  const factory LiteralValue.date(
    int field0,
  ) = LiteralValue_Date;

  /// Nanoseconds elapsed since midnight.
  const factory LiteralValue.time(
    int field0,
  ) = LiteralValue_Time;
}

enum Operator {
  eq,
  eqValidity,
  notEq,
  notEqValidity,
  lt,
  ltEq,
  gt,
  gtEq,
  plus,
  minus,
  multiply,
  divide,
  trueDivide,
  floorDivide,
  modulus,
  and,
  or,
  xor,
}

class SortOptions {
  final bool descending;
  final bool nullsLast;
  final bool multithreaded;
  final bool maintainOrder;

  const SortOptions({
    this.descending = false,
    this.nullsLast = false,
    this.multithreaded = true,
    this.maintainOrder = false,
  });

  @override
  int get hashCode =>
      descending.hashCode ^
      nullsLast.hashCode ^
      multithreaded.hashCode ^
      maintainOrder.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is SortOptions &&
          runtimeType == other.runtimeType &&
          descending == other.descending &&
          nullsLast == other.nullsLast &&
          multithreaded == other.multithreaded &&
          maintainOrder == other.maintainOrder;
}

enum WindowMapping {
  /// Map the group vlues to the position
  groupsToRows,

  /// Explode the aggregated list and just do a hstack instead of a join
  /// this requires the groups to be sorted to make any sense
  explode,

  /// Join the groups as 'List<group_dtype>' to the row positions.
  /// warning: this can be memory intensive
  join,
}

@freezed
sealed class WindowType with _$WindowType {
  const WindowType._();

  /// Explode the aggregated list and just do a hstack instead of a join
  /// this requires the groups to be sorted to make any sense
  const factory WindowType.over(
    WindowMapping field0,
  ) = WindowType_Over;
}
