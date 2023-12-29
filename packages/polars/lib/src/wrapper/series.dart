// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.6.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../frb_generated.dart';
import 'df.dart';
import 'entry.dart';
import 'expr.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

// Rust type: flutter_rust_bridge::RustOpaque<std::sync::RwLock<LazyGroupBy>>
@sealed
class LazyGroupBy extends RustOpaque {
  LazyGroupBy.dcoDecode(dynamic wire) : super.dcoDecode(wire, _kStaticData);

  LazyGroupBy.sseDecode(int ptr, int externalSizeOnNative)
      : super.sseDecode(ptr, externalSizeOnNative, _kStaticData);

  static final _kStaticData = RustArcStaticData(
    rustArcIncrementStrongCount:
        RustLib.instance.api.rust_arc_increment_strong_count_LazyGroupBy,
    rustArcDecrementStrongCount:
        RustLib.instance.api.rust_arc_decrement_strong_count_LazyGroupBy,
    rustArcDecrementStrongCountPtr:
        RustLib.instance.api.rust_arc_decrement_strong_count_LazyGroupByPtr,
  );

  /// Group by and aggregate.
  ///
  /// Select a column with [col] and choose an aggregation. If you want to aggregate all columns
  /// use <code>[col]\("*")</code>.
  LazyFrame agg({required VecExpr exprs, dynamic hint}) =>
      RustLib.instance.api.lazyGroupByAgg(
        that: this,
        exprs: exprs,
      );

  /// Return the first [n] rows of each group.
  LazyFrame head({int? n, dynamic hint}) =>
      RustLib.instance.api.lazyGroupByHead(
        that: this,
        n: n,
      );

  /// Return the last [n] rows of each group.
  LazyFrame tail({int? n, dynamic hint}) =>
      RustLib.instance.api.lazyGroupByTail(
        that: this,
        n: n,
      );
}

// Rust type: flutter_rust_bridge::RustOpaque<std::sync::RwLock<Schema>>
@sealed
class Schema extends RustOpaque {
  Schema.dcoDecode(dynamic wire) : super.dcoDecode(wire, _kStaticData);

  Schema.sseDecode(int ptr, int externalSizeOnNative)
      : super.sseDecode(ptr, externalSizeOnNative, _kStaticData);

  static final _kStaticData = RustArcStaticData(
    rustArcIncrementStrongCount:
        RustLib.instance.api.rust_arc_increment_strong_count_Schema,
    rustArcDecrementStrongCount:
        RustLib.instance.api.rust_arc_decrement_strong_count_Schema,
    rustArcDecrementStrongCountPtr:
        RustLib.instance.api.rust_arc_decrement_strong_count_SchemaPtr,
  );

  /// Create a schema from a list of [Field]s.
  static Schema of({required List<Field> fields, dynamic hint}) =>
      RustLib.instance.api.schemaOf(fields: fields, hint: hint);
}

// Rust type: flutter_rust_bridge::RustOpaque<std::sync::RwLock<Series>>
@sealed
class Series extends RustOpaque {
  Series.dcoDecode(dynamic wire) : super.dcoDecode(wire, _kStaticData);

  Series.sseDecode(int ptr, int externalSizeOnNative)
      : super.sseDecode(ptr, externalSizeOnNative, _kStaticData);

  static final _kStaticData = RustArcStaticData(
    rustArcIncrementStrongCount:
        RustLib.instance.api.rust_arc_increment_strong_count_Series,
    rustArcDecrementStrongCount:
        RustLib.instance.api.rust_arc_decrement_strong_count_Series,
    rustArcDecrementStrongCountPtr:
        RustLib.instance.api.rust_arc_decrement_strong_count_SeriesPtr,
  );

  /// Returns a new series with elements from this series added to [other]'s element-wise.
  Series addTo({required Series other, dynamic hint}) =>
      RustLib.instance.api.seriesAddTo(
        that: this,
        other: other,
      );

  /// Adds the contents of [other] onto this series.
  ///
  /// Throws if [other] is self.
  void append({required Series other, dynamic hint}) =>
      RustLib.instance.api.seriesAppend(
        that: this,
        other: other,
      );

  /// Applies a binary operation onto this series with a scalar value.
  ///
  /// For logic operators, the new series is a boolean mask. Otherwise,
  /// it will be a series of numeric values.
  Series applyScalar(
          {required Operator op, required double value, dynamic hint}) =>
      RustLib.instance.api.seriesApplyScalar(
        that: this,
        op: op,
        value: value,
      );

  /// If compatible, returns a representation of this series as integers.
  Future<List<double?>> asDoubles({bool strict = true, dynamic hint}) =>
      RustLib.instance.api.seriesAsDoubles(
        that: this,
        strict: strict,
      );

  /// If this series contains [Duration]s, returns its Dart representation.
  List<Duration?> asDurations({dynamic hint}) =>
      RustLib.instance.api.seriesAsDurations(
        that: this,
      );

  /// If compatible, returns a representation of this series as integers.
  List<int?> asInts({bool strict = true, dynamic hint}) =>
      RustLib.instance.api.seriesAsInts(
        that: this,
        strict: strict,
      );

  /// If this series contains [DateTime]s, returns its Dart representation.
  ///
  /// If a timezone is defined by this series, the datetimes will be converted to the local timezone.
  /// Otherwise, the datetimes are assumed to be in the local timezone.
  List<DateTime?> asLocalDatetime({dynamic hint}) =>
      RustLib.instance.api.seriesAsLocalDatetime(
        that: this,
      );

  /// If this series contains [DateTime]s, returns its Dart representation.
  ///
  /// Datetimes are parsed as-is, without any timezone correction.
  List<DateTime?> asNaiveDatetime({dynamic hint}) =>
      RustLib.instance.api.seriesAsNaiveDatetime(
        that: this,
      );

  /// If this series is a UTF-8 series, returns its Dart representation.
  List<String?> asStrings({dynamic hint}) =>
      RustLib.instance.api.seriesAsStrings(
        that: this,
      );

  /// If this series contains [DateTime]s, returns its Dart representation.
  ///
  /// If a timezone is defined by this series, the datetimes will be converted to UTC.
  /// Otherwise, the datetimes are assumed to be in UTC.
  List<DateTime?> asUtcDatetime({dynamic hint}) =>
      RustLib.instance.api.seriesAsUtcDatetime(
        that: this,
      );

  /// Casts this series into one with the specified datatype.
  Series cast({required DataType dtype, bool strict = true, dynamic hint}) =>
      RustLib.instance.api.seriesCast(
        that: this,
        dtype: dtype,
        strict: strict,
      );

  /// Returns a new series with elements from this series divided by [other]'s element-wise.
  Series divide({required Series other, dynamic hint}) =>
      RustLib.instance.api.seriesDivide(
        that: this,
        other: other,
      );

  /// Dump the contents of this entire series.
  String dump({dynamic hint}) => RustLib.instance.api.seriesDump(
        that: this,
      );

  /// Returns whether this series is identical to [other].
  ///
  /// if `ignoreNull` is true, null values are considered to be equal.
  bool equal({required Series other, bool ignoreNull = false, dynamic hint}) =>
      RustLib.instance.api.seriesEqual(
        that: this,
        other: other,
        ignoreNull: ignoreNull,
      );

  /// Returns the amount of bytes occupied by this series.
  int estimatedSize({dynamic hint}) => RustLib.instance.api.seriesEstimatedSize(
        that: this,
      );

  /// Expands a series of lists into rows of values, or strings into rows of characters.
  Series explode({dynamic hint}) => RustLib.instance.api.seriesExplode(
        that: this,
      );

  /// TODO: docs
  Series explodeByOffsets({required Int64List offsets, dynamic hint}) =>
      RustLib.instance.api.seriesExplodeByOffsets(
        that: this,
        offsets: offsets,
      );

  /// Get the value at [index] as a double.
  double? get({required int index, dynamic hint}) =>
      RustLib.instance.api.seriesGet(
        that: this,
        index: index,
      );

  /// Get the value at [index] as a string.
  String? getString({required int index, dynamic hint}) =>
      RustLib.instance.api.seriesGetString(
        that: this,
        index: index,
      );

  /// Get the first few values of this series.
  Series head({int? length, dynamic hint}) => RustLib.instance.api.seriesHead(
        that: this,
        length: length,
      );

  /// Casts this series into a [DataFrame]. May create a copy.
  DataFrame intoFrame({dynamic hint}) => RustLib.instance.api.seriesIntoFrame(
        that: this,
      );

  LiteralValue intoLiteral({dynamic hint}) =>
      RustLib.instance.api.seriesIntoLiteral(
        that: this,
      );

  /// Returns whether this is a series of booleans.
  bool isBool({dynamic hint}) => RustLib.instance.api.seriesIsBool(
        that: this,
      );

  /// Returns whether this is a series of numeric values.
  bool isNumeric({dynamic hint}) => RustLib.instance.api.seriesIsNumeric(
        that: this,
      );

  /// Returns whether this is a series of [DateTime] or [Duration]s.
  bool isTemporal({dynamic hint}) => RustLib.instance.api.seriesIsTemporal(
        that: this,
      );

  /// Returns whether this is a series of UTF-8 strings.
  bool isUtf8({dynamic hint}) => RustLib.instance.api.seriesIsUtf8(
        that: this,
      );

  /// Iterate over this series' values.
  Stream<dynamic> iter({dynamic hint}) => RustLib.instance.api.seriesIter(
        that: this,
      );

  /// Returns the maximum value of this series' values.
  ///
  /// Returns null if one of the values are also null.
  double? max({dynamic hint}) => RustLib.instance.api.seriesMax(
        that: this,
      );

  /// Calculates the mean (average) of this series.
  double? mean({dynamic hint}) => RustLib.instance.api.seriesMean(
        that: this,
      );

  /// Calculates and wraps this series' mean as a single-element series.
  Series meanAsSeries({dynamic hint}) =>
      RustLib.instance.api.seriesMeanAsSeries(
        that: this,
      );

  /// Calculates the [median](https://en.wikipedia.org/wiki/Median) of this series.
  double? median({dynamic hint}) => RustLib.instance.api.seriesMedian(
        that: this,
      );

  /// Calculates and wraps this series' median as a single-element series.
  Series medianAsSeries({dynamic hint}) =>
      RustLib.instance.api.seriesMedianAsSeries(
        that: this,
      );

  /// Returns the minimum value of this series' values.
  ///
  /// Returns null if one of the values are also null.
  double? min({dynamic hint}) => RustLib.instance.api.seriesMin(
        that: this,
      );

  /// Returns a new series with elements from this series multiplied with [other]'s element-wise.
  Series multiply({required Series other, dynamic hint}) =>
      RustLib.instance.api.seriesMultiply(
        that: this,
        other: other,
      );

  /// Create a new series of booleans.
  static Series ofBools(
          {String name = r"", List<bool?>? values, dynamic hint}) =>
      RustLib.instance.api
          .seriesOfBools(name: name, values: values, hint: hint);

  /// Create a new series of doubles.
  static Series ofDoubles(
          {String name = r"", List<double?>? values, dynamic hint}) =>
      RustLib.instance.api
          .seriesOfDoubles(name: name, values: values, hint: hint);

  /// Create a new series of [Duration]s.
  static Series ofDurations(
          {String name = r"",
          List<Duration?>? values,
          TimeUnit unit = TimeUnit.milliseconds,
          dynamic hint}) =>
      RustLib.instance.api.seriesOfDurations(
          name: name, values: values, unit: unit, hint: hint);

  /// Create a new series of 32-bit wide integers.
  static Series ofI32({String name = r"", List<int?>? values, dynamic hint}) =>
      RustLib.instance.api.seriesOfI32(name: name, values: values, hint: hint);

  /// Create a new series of 64-bit wide integers.
  static Series ofInts({String name = r"", List<int?>? values, dynamic hint}) =>
      RustLib.instance.api.seriesOfInts(name: name, values: values, hint: hint);

  /// Create a new series of strings.
  static Series ofStrings(
          {String name = r"", List<String?>? values, dynamic hint}) =>
      RustLib.instance.api
          .seriesOfStrings(name: name, values: values, hint: hint);

  /// Calculates the product of each element in the series and returns it in a single-element series.
  Series product({dynamic hint}) => RustLib.instance.api.seriesProduct(
        that: this,
      );

  /// Returns a new series with the [remainder](https://en.wikipedia.org/wiki/Remainder)
  /// between this series' and [other]'s elements.
  Series remainder({required Series other, dynamic hint}) =>
      RustLib.instance.api.seriesRemainder(
        that: this,
        other: other,
      );

  /// Rename this series to [name] in-place.
  void rename({required String name, dynamic hint}) =>
      RustLib.instance.api.seriesRename(
        that: this,
        name: name,
      );

  /// Creates a new series with the specified dimensions.
  Series reshape({required Int64List dims, dynamic hint}) =>
      RustLib.instance.api.seriesReshape(
        that: this,
        dims: dims,
      );

  /// TODO: Docs for rolling_max
  Series rollingMax(
          {Duration? windowSize,
          int minPeriods = 1,
          Float64List? weights,
          bool center = false,
          Int64List? by,
          ClosedWindow? closedWindow,
          TimeUnit? timeUnit,
          String? timezone,
          dynamic hint}) =>
      RustLib.instance.api.seriesRollingMax(
        that: this,
        windowSize: windowSize,
        minPeriods: minPeriods,
        weights: weights,
        center: center,
        by: by,
        closedWindow: closedWindow,
        timeUnit: timeUnit,
        timezone: timezone,
      );

  /// TODO: Docs for rolling_mean
  Series rollingMean(
          {Duration? windowSize,
          int minPeriods = 1,
          Float64List? weights,
          bool center = false,
          Int64List? by,
          ClosedWindow? closedWindow,
          TimeUnit? timeUnit,
          String? timezone,
          dynamic hint}) =>
      RustLib.instance.api.seriesRollingMean(
        that: this,
        windowSize: windowSize,
        minPeriods: minPeriods,
        weights: weights,
        center: center,
        by: by,
        closedWindow: closedWindow,
        timeUnit: timeUnit,
        timezone: timezone,
      );

  /// TODO: Docs for rolling_median
  Series rollingMedian(
          {Duration? windowSize,
          int minPeriods = 1,
          Float64List? weights,
          bool center = false,
          Int64List? by,
          ClosedWindow? closedWindow,
          TimeUnit? timeUnit,
          String? timezone,
          dynamic hint}) =>
      RustLib.instance.api.seriesRollingMedian(
        that: this,
        windowSize: windowSize,
        minPeriods: minPeriods,
        weights: weights,
        center: center,
        by: by,
        closedWindow: closedWindow,
        timeUnit: timeUnit,
        timezone: timezone,
      );

  /// TODO: Docs for rolling_min
  Series rollingMin(
          {Duration? windowSize,
          int minPeriods = 1,
          Float64List? weights,
          bool center = false,
          Int64List? by,
          ClosedWindow? closedWindow,
          TimeUnit? timeUnit,
          String? timezone,
          dynamic hint}) =>
      RustLib.instance.api.seriesRollingMin(
        that: this,
        windowSize: windowSize,
        minPeriods: minPeriods,
        weights: weights,
        center: center,
        by: by,
        closedWindow: closedWindow,
        timeUnit: timeUnit,
        timezone: timezone,
      );

  /// TODO: Docs for rolling_quantile
  Series rollingQuantile(
          {Duration? windowSize,
          int minPeriods = 1,
          Float64List? weights,
          bool center = false,
          Int64List? by,
          ClosedWindow? closedWindow,
          TimeUnit? timeUnit,
          String? timezone,
          dynamic hint}) =>
      RustLib.instance.api.seriesRollingQuantile(
        that: this,
        windowSize: windowSize,
        minPeriods: minPeriods,
        weights: weights,
        center: center,
        by: by,
        closedWindow: closedWindow,
        timeUnit: timeUnit,
        timezone: timezone,
      );

  /// TODO: Docs for rolling_std
  Series rollingStd(
          {Duration? windowSize,
          int minPeriods = 1,
          Float64List? weights,
          bool center = false,
          Int64List? by,
          ClosedWindow? closedWindow,
          TimeUnit? timeUnit,
          String? timezone,
          dynamic hint}) =>
      RustLib.instance.api.seriesRollingStd(
        that: this,
        windowSize: windowSize,
        minPeriods: minPeriods,
        weights: weights,
        center: center,
        by: by,
        closedWindow: closedWindow,
        timeUnit: timeUnit,
        timezone: timezone,
      );

  /// TODO: Docs for rolling_sum
  Series rollingSum(
          {Duration? windowSize,
          int minPeriods = 1,
          Float64List? weights,
          bool center = false,
          Int64List? by,
          ClosedWindow? closedWindow,
          TimeUnit? timeUnit,
          String? timezone,
          dynamic hint}) =>
      RustLib.instance.api.seriesRollingSum(
        that: this,
        windowSize: windowSize,
        minPeriods: minPeriods,
        weights: weights,
        center: center,
        by: by,
        closedWindow: closedWindow,
        timeUnit: timeUnit,
        timezone: timezone,
      );

  /// TODO: Docs for rolling_var
  Series rollingVar(
          {Duration? windowSize,
          int minPeriods = 1,
          Float64List? weights,
          bool center = false,
          Int64List? by,
          ClosedWindow? closedWindow,
          TimeUnit? timeUnit,
          String? timezone,
          dynamic hint}) =>
      RustLib.instance.api.seriesRollingVar(
        that: this,
        windowSize: windowSize,
        minPeriods: minPeriods,
        weights: weights,
        center: center,
        by: by,
        closedWindow: closedWindow,
        timeUnit: timeUnit,
        timezone: timezone,
      );

  /// Returns a new shuffled series.
  Series shuffle({int? seed, dynamic hint}) =>
      RustLib.instance.api.seriesShuffle(
        that: this,
        seed: seed,
      );

  /// Returns a new sorted series.
  Series sort({bool reverse = false, dynamic hint}) =>
      RustLib.instance.api.seriesSort(
        that: this,
        reverse: reverse,
      );

  /// Calculates the standard deviation of this series with the specified degree of freedom.
  Series stdAsSeries({required int ddof, dynamic hint}) =>
      RustLib.instance.api.seriesStdAsSeries(
        that: this,
        ddof: ddof,
      );

  /// Returns a new series with elements from this series subtracted from [other]'s element-wise.
  Series subtract({required Series other, dynamic hint}) =>
      RustLib.instance.api.seriesSubtract(
        that: this,
        other: other,
      );

  /// Sums all non-null rows in this series to produce a result.
  ///
  /// Returns null if the series only contains null values.
  double? sum({dynamic hint}) => RustLib.instance.api.seriesSum(
        that: this,
      );

  /// Returns the sum of this series' values as a single-element series.
  Series sumAsSeries({dynamic hint}) => RustLib.instance.api.seriesSumAsSeries(
        that: this,
      );

  /// Get the last few values of this series.
  Series tail({int? length, dynamic hint}) => RustLib.instance.api.seriesTail(
        that: this,
        length: length,
      );

  /// Returns an untyped list.
  List<dynamic> toList({dynamic hint}) => RustLib.instance.api.seriesToList(
        that: this,
      );

  /// Returns the unique values of this series.
  ///
  /// If `stable` is true, extra work is done to maintain the original order of elements.
  Series unique({bool maintainOrder = false, dynamic hint}) =>
      RustLib.instance.api.seriesUnique(
        that: this,
        maintainOrder: maintainOrder,
      );

  /// Calculates the variance of this series with the specified degree of freedom.
  Series varAsSeries({required int ddof, dynamic hint}) =>
      RustLib.instance.api.seriesVarAsSeries(
        that: this,
        ddof: ddof,
      );
}
