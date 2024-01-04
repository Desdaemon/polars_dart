// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.6.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../frb_generated.dart';
import 'entry.dart';
import 'expr.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:freezed_annotation/freezed_annotation.dart' hide protected;
import 'series.dart';
part 'df.freezed.dart';

// Rust type: flutter_rust_bridge::RustOpaque<AssertUnwindSafe < PExpr >>
@sealed
class PExpr extends RustOpaque {
  PExpr.dcoDecode(dynamic wire) : super.dcoDecode(wire, _kStaticData);

  PExpr.sseDecode(int ptr, int externalSizeOnNative)
      : super.sseDecode(ptr, externalSizeOnNative, _kStaticData);

  static final _kStaticData = RustArcStaticData(
    rustArcIncrementStrongCount:
        RustLib.instance.api.rust_arc_increment_strong_count_PExpr,
    rustArcDecrementStrongCount:
        RustLib.instance.api.rust_arc_decrement_strong_count_PExpr,
    rustArcDecrementStrongCountPtr:
        RustLib.instance.api.rust_arc_decrement_strong_count_PExprPtr,
  );
}

// Rust type: flutter_rust_bridge::RustOpaque<AssertUnwindSafe < PSeries >>
@sealed
class PSeries extends RustOpaque {
  PSeries.dcoDecode(dynamic wire) : super.dcoDecode(wire, _kStaticData);

  PSeries.sseDecode(int ptr, int externalSizeOnNative)
      : super.sseDecode(ptr, externalSizeOnNative, _kStaticData);

  static final _kStaticData = RustArcStaticData(
    rustArcIncrementStrongCount:
        RustLib.instance.api.rust_arc_increment_strong_count_PSeries,
    rustArcDecrementStrongCount:
        RustLib.instance.api.rust_arc_decrement_strong_count_PSeries,
    rustArcDecrementStrongCountPtr:
        RustLib.instance.api.rust_arc_decrement_strong_count_PSeriesPtr,
  );
}

// Rust type: flutter_rust_bridge::RustOpaque<AssertUnwindSafe < SpecialEq < PSeries > >>
@sealed
class SpecialEqPSeries extends RustOpaque {
  SpecialEqPSeries.dcoDecode(dynamic wire)
      : super.dcoDecode(wire, _kStaticData);

  SpecialEqPSeries.sseDecode(int ptr, int externalSizeOnNative)
      : super.sseDecode(ptr, externalSizeOnNative, _kStaticData);

  static final _kStaticData = RustArcStaticData(
    rustArcIncrementStrongCount:
        RustLib.instance.api.rust_arc_increment_strong_count_SpecialEqPSeries,
    rustArcDecrementStrongCount:
        RustLib.instance.api.rust_arc_decrement_strong_count_SpecialEqPSeries,
    rustArcDecrementStrongCountPtr: RustLib
        .instance.api.rust_arc_decrement_strong_count_SpecialEqPSeriesPtr,
  );
}

// Rust type: flutter_rust_bridge::RustOpaque<std::sync::RwLock<DataFrame>>
@sealed
class DataFrame extends RustOpaque {
  DataFrame.dcoDecode(dynamic wire) : super.dcoDecode(wire, _kStaticData);

  DataFrame.sseDecode(int ptr, int externalSizeOnNative)
      : super.sseDecode(ptr, externalSizeOnNative, _kStaticData);

  static final _kStaticData = RustArcStaticData(
    rustArcIncrementStrongCount:
        RustLib.instance.api.rust_arc_increment_strong_count_DataFrame,
    rustArcDecrementStrongCount:
        RustLib.instance.api.rust_arc_decrement_strong_count_DataFrame,
    rustArcDecrementStrongCountPtr:
        RustLib.instance.api.rust_arc_decrement_strong_count_DataFramePtr,
  );

  DataFrame clone({dynamic hint}) => RustLib.instance.api.dataFrameClone(
        that: this,
      );

  /// Select a single column by name.
  ///
  /// Note: A clone of the column is returned, rather than a reference.
  Series column({required String column, dynamic hint}) =>
      RustLib.instance.api.dataFrameColumn(
        that: this,
        column: column,
      );

  /// Select the column at the given index.
  Series columnAt({required int index, dynamic hint}) =>
      RustLib.instance.api.dataFrameColumnAt(
        that: this,
        index: index,
      );

  /// Select multiple columns by name.
  ///
  /// Note: Clones of the columns are returned, rather than a reference.
  VecSeries columns({required List<String> columns, dynamic hint}) =>
      RustLib.instance.api.dataFrameColumns(
        that: this,
        columns: columns,
      );

  /// Output statistics about this dataframe.
  Future<DataFrame> describe({Float64List? percentiles, dynamic hint}) =>
      RustLib.instance.api.dataFrameDescribe(
        that: this,
        percentiles: percentiles,
      );

  /// Drops a column by name, producing a new dataframe.
  DataFrame drop({required String column, dynamic hint}) =>
      RustLib.instance.api.dataFrameDrop(
        that: this,
        column: column,
      );

  /// Drops a column in-place and returns it.
  Series dropInPlace({required String column, dynamic hint}) =>
      RustLib.instance.api.dataFrameDropInPlace(
        that: this,
        column: column,
      );

  /// Returns the datatypes of this dataframe's columns.
  List<DataType> dtypes({dynamic hint}) => RustLib.instance.api.dataFrameDtypes(
        that: this,
      );

  /// Dump the contents of this entire dataframe.
  String dump({dynamic hint}) => RustLib.instance.api.dataFrameDump(
        that: this,
      );

  /// Returns the amount of bytes occupied by this series.
  int estimatedSize({dynamic hint}) =>
      RustLib.instance.api.dataFrameEstimatedSize(
        that: this,
      );

  /// Get the names of this dataframe's columns.
  List<String> getColumnNames({dynamic hint}) =>
      RustLib.instance.api.dataFrameGetColumnNames(
        that: this,
      );

  /// Get all columns of this dataframe.
  VecSeries getColumns({dynamic hint}) =>
      RustLib.instance.api.dataFrameGetColumns(
        that: this,
      );

  /// Get a row of data from this dataframe.
  ///
  /// Prefer other functions to this inside a hot loop, as this function performs
  /// data copies and conversions to and from the native representation.
  List<dynamic> getRow({required int index, dynamic hint}) =>
      RustLib.instance.api.dataFrameGetRow(
        that: this,
        index: index,
      );

  /// Returns the first few rows of this dataframe.
  DataFrame head({int? length, dynamic hint}) =>
      RustLib.instance.api.dataFrameHead(
        that: this,
        length: length,
      );

  /// Returns the height of this dataframe, aka the number of rows.
  int height({dynamic hint}) => RustLib.instance.api.dataFrameHeight(
        that: this,
      );

  /// Returns whether this dataframe has no rows.
  bool isEmpty({dynamic hint}) => RustLib.instance.api.dataFrameIsEmpty(
        that: this,
      );

  /// Iterate through this dataframe's rows.
  ///
  /// Use [parseRow] to retrieve the canonical values for these rows.
  Stream<List<dynamic>> iter({dynamic hint}) =>
      RustLib.instance.api.dataFrameIter(
        that: this,
      );

  /// Returns a [LazyFrame] to which operations can be applied lazily.
  /// As opposed to [LazyFrame], [DataFrame] by default applies its operations eagerly.
  LazyFrame lazy(
          {bool? projectionPushdown,
          bool? predicatePushdown,
          bool? typeCoercion,
          bool? simplifyExpressions,
          bool? slicePushdown,
          bool? streaming,
          dynamic hint}) =>
      RustLib.instance.api.dataFrameLazy(
        that: this,
        projectionPushdown: projectionPushdown,
        predicatePushdown: predicatePushdown,
        typeCoercion: typeCoercion,
        simplifyExpressions: simplifyExpressions,
        slicePushdown: slicePushdown,
        streaming: streaming,
      );

  /// Aggregate the columns to their maximum values.
  Future<DataFrame> max({dynamic hint}) => RustLib.instance.api.dataFrameMax(
        that: this,
      );

  /// Returns a new, empty dataframe.
  static DataFrame ofLits({List<(String, Literals)>? series, dynamic hint}) =>
      RustLib.instance.api.dataFrameOfLits(series: series, hint: hint);

  /// Returns a dataframe with columns from this dataframe in reverse order.
  DataFrame reverse({dynamic hint}) => RustLib.instance.api.dataFrameReverse(
        that: this,
      );

  /// Sample [n] datapoints from this dataframe.
  Future<DataFrame> sample(
          {required Series n,
          bool withReplacement = false,
          bool shuffle = false,
          int? seed,
          dynamic hint}) =>
      RustLib.instance.api.dataFrameSample(
        that: this,
        n: n,
        withReplacement: withReplacement,
        shuffle: shuffle,
        seed: seed,
      );

  /// Returns the [Schema] of this dataframe.
  Schema schema({dynamic hint}) => RustLib.instance.api.dataFrameSchema(
        that: this,
      );

  /// Makes a new dataframe with the specified columns from this dataframe.
  DataFrame select({required List<String> columns, dynamic hint}) =>
      RustLib.instance.api.dataFrameSelect(
        that: this,
        columns: columns,
      );

  /// Returns the height and width of this dataframe.
  (int, int) shape({dynamic hint}) => RustLib.instance.api.dataFrameShape(
        that: this,
      );

  /// Sorts this dataframe by the specified columns.
  void sortInPlace(
          {List<String> byColumn = const [],
          List<bool> descending = const [],
          bool maintainOrder = false,
          dynamic hint}) =>
      RustLib.instance.api.dataFrameSortInPlace(
        that: this,
        byColumn: byColumn,
        descending: descending,
        maintainOrder: maintainOrder,
      );

  /// Returns the last few rows of this dataframe.
  DataFrame tail({int? length, dynamic hint}) =>
      RustLib.instance.api.dataFrameTail(
        that: this,
        length: length,
      );

  /// Returns the width of this dataframe, aka the number of columns.
  int width({dynamic hint}) => RustLib.instance.api.dataFrameWidth(
        that: this,
      );

  /// Add a new column at index 0 denoting the row number.
  DataFrame withRowCount({required String name, int? offset, dynamic hint}) =>
      RustLib.instance.api.dataFrameWithRowCount(
        that: this,
        name: name,
        offset: offset,
      );
}

// Rust type: flutter_rust_bridge::RustOpaque<std::sync::RwLock<LazyFrame>>
@sealed
class LazyFrame extends RustOpaque {
  LazyFrame.dcoDecode(dynamic wire) : super.dcoDecode(wire, _kStaticData);

  LazyFrame.sseDecode(int ptr, int externalSizeOnNative)
      : super.sseDecode(ptr, externalSizeOnNative, _kStaticData);

  static final _kStaticData = RustArcStaticData(
    rustArcIncrementStrongCount:
        RustLib.instance.api.rust_arc_increment_strong_count_LazyFrame,
    rustArcDecrementStrongCount:
        RustLib.instance.api.rust_arc_decrement_strong_count_LazyFrame,
    rustArcDecrementStrongCountPtr:
        RustLib.instance.api.rust_arc_decrement_strong_count_LazyFramePtr,
  );

  /// Caches the results into a new [LazyFrame].
  ///
  /// This should be used to prevent computations running multiple times.
  LazyFrame cache({dynamic hint}) => RustLib.instance.api.lazyFrameCache(
        that: this,
      );

  /// Executes all lazy operations and collects results into a [DataFrame].
  ///
  /// Can also optionally be run in [streaming mode](https://docs.pola.rs/user-guide/concepts/streaming).
  Future<DataFrame> collect({bool streaming = false, dynamic hint}) =>
      RustLib.instance.api.lazyFrameCollect(
        that: this,
        streaming: streaming,
      );

  /// Creates the [Cartesian product](https://en.wikipedia.org/wiki/Cartesian_product) from both frames,
  /// preserving the order of this frame's keys.
  LazyFrame crossJoin({required LazyFrame other, dynamic hint}) =>
      RustLib.instance.api.lazyFrameCrossJoin(
        that: this,
        other: other,
      );

  /// Drop null rows.
  ///
  /// Same as `frame.filter(col('*').isNotNull)`.
  LazyFrame dropNulls({List<Expr>? subset, dynamic hint}) =>
      RustLib.instance.api.lazyFrameDropNulls(
        that: this,
        subset: subset,
      );

  /// Explode each column.
  LazyFrame explode({required List<Expr> columns, dynamic hint}) =>
      RustLib.instance.api.lazyFrameExplode(
        that: this,
        columns: columns,
      );

  /// Similar to [collect], but overrides the number of rows read by each operation.
  ///
  /// The final row count is not guaranteed to be equal [nRows].
  Future<DataFrame> fetch({required int nRows, dynamic hint}) =>
      RustLib.instance.api.lazyFrameFetch(
        that: this,
        nRows: nRows,
      );

  /// Filter by the specified predicate expression.
  LazyFrame filter({required Expr pred, dynamic hint}) =>
      RustLib.instance.api.lazyFrameFilter(
        that: this,
        pred: pred,
      );

  /// Get the first row.
  LazyFrame first({dynamic hint}) => RustLib.instance.api.lazyFrameFirst(
        that: this,
      );

  /// Define conditions by which to group and aggregate rows.
  LazyGroupBy groupBy(
          {required List<Expr> exprs,
          bool maintainOrder = false,
          dynamic hint}) =>
      RustLib.instance.api.lazyFrameGroupBy(
        that: this,
        exprs: exprs,
        maintainOrder: maintainOrder,
      );

  /// Performs an [inner join](https://en.wikipedia.org/wiki/Join_(SQL)#Inner_join_and_NULL_values) with [other].
  LazyFrame innerJoin(
          {required LazyFrame other,
          required Expr leftOn,
          required Expr rightOn,
          dynamic hint}) =>
      RustLib.instance.api.lazyFrameInnerJoin(
        that: this,
        other: other,
        leftOn: leftOn,
        rightOn: rightOn,
      );

  /// Joins this table to [other].
  ///
  /// Use [on] to specify columns on both frames to join on, or specify separately
  /// using [leftOn] and [rightOn].
  ///
  /// [suffix] specifies the suffix to add to duplicate columns of [other].
  ///
  /// Example:
  /// ```dart
  /// final joined = left
  ///   .join(
  ///     other: right,
  ///     leftOn: [col('foo'), col('bar')],
  ///     rightOn: [col('foo'), col('bar')],
  ///     how: JoinType.Inner,
  ///   );
  /// ```
  LazyFrame join(
          {required LazyFrame other,
          List<Expr>? on,
          List<Expr>? leftOn,
          List<Expr>? rightOn,
          String suffix = r"_right",
          JoinType how = JoinType.left,
          bool allowParallel = true,
          bool forceParallel = false,
          dynamic hint}) =>
      RustLib.instance.api.lazyFrameJoin(
        that: this,
        other: other,
        on: on,
        leftOn: leftOn,
        rightOn: rightOn,
        suffix: suffix,
        how: how,
        allowParallel: allowParallel,
        forceParallel: forceParallel,
      );

  /// Get the last row.
  LazyFrame last({dynamic hint}) => RustLib.instance.api.lazyFrameLast(
        that: this,
      );

  /// Performs a [left outer join](https://en.wikipedia.org/wiki/Join_(SQL)#Left_outer_join) with [other].
  LazyFrame leftJoin(
          {required LazyFrame other,
          required Expr leftOn,
          required Expr rightOn,
          dynamic hint}) =>
      RustLib.instance.api.lazyFrameLeftJoin(
        that: this,
        other: other,
        leftOn: leftOn,
        rightOn: rightOn,
      );

  /// Limit this dataframe to the first [n] rows.
  ///
  /// To avoid scanning the rows, use [fetch].
  LazyFrame limit({required int n, dynamic hint}) =>
      RustLib.instance.api.lazyFrameLimit(
        that: this,
        n: n,
      );

  /// Aggregate all columns as their max values.
  LazyFrame max({dynamic hint}) => RustLib.instance.api.lazyFrameMax(
        that: this,
      );

  /// Aggregate all columns as their means.
  LazyFrame mean({dynamic hint}) => RustLib.instance.api.lazyFrameMean(
        that: this,
      );

  /// Aggregate all columns as their medians.
  LazyFrame median({dynamic hint}) => RustLib.instance.api.lazyFrameMedian(
        that: this,
      );

  /// [Melt](https://docs.pola.rs/user-guide/transformations/melt) this
  /// dataframe from the wide format to the long format.
  LazyFrame melt(
          {required List<String> idVars,
          required List<String> valueVars,
          String? variableName,
          String? valueName,
          bool streamable = true,
          dynamic hint}) =>
      RustLib.instance.api.lazyFrameMelt(
        that: this,
        idVars: idVars,
        valueVars: valueVars,
        variableName: variableName,
        valueName: valueName,
        streamable: streamable,
      );

  /// Aggregate all columns as their min values.
  LazyFrame min({dynamic hint}) => RustLib.instance.api.lazyFrameMin(
        that: this,
      );

  LazyFrame nullCount({dynamic hint}) =>
      RustLib.instance.api.lazyFrameNullCount(
        that: this,
      );

  /// Performs a [full outer join](https://en.wikipedia.org/wiki/Join_(SQL)#Full_outer_join) with [other].
  LazyFrame outerJoin(
          {required LazyFrame other,
          required Expr leftOn,
          required Expr rightOn,
          dynamic hint}) =>
      RustLib.instance.api.lazyFrameOuterJoin(
        that: this,
        other: other,
        leftOn: leftOn,
        rightOn: rightOn,
      );

  /// Aggregate all columns as their quantiles.
  LazyFrame quantile(
          {required Expr quantile,
          required QuantileInterpolOptions interpol,
          dynamic hint}) =>
      RustLib.instance.api.lazyFrameQuantile(
        that: this,
        quantile: quantile,
        interpol: interpol,
      );

  /// Reverse the order of this dataframe's columns.
  LazyFrame reverse({dynamic hint}) => RustLib.instance.api.lazyFrameReverse(
        that: this,
      );

  /// Select (and rename) columns from the query.
  LazyFrame select({required List<Expr> exprs, dynamic hint}) =>
      RustLib.instance.api.lazyFrameSelect(
        that: this,
        exprs: exprs,
      );

  /// Slice the frame.
  LazyFrame slice({required int offset, required int len, dynamic hint}) =>
      RustLib.instance.api.lazyFrameSlice(
        that: this,
        offset: offset,
        len: len,
      );

  LazyFrame sort(
          {required String byColumn,
          bool descending = false,
          bool nullsLast = false,
          bool multithreaded = true,
          bool maintainOrder = false,
          dynamic hint}) =>
      RustLib.instance.api.lazyFrameSort(
        that: this,
        byColumn: byColumn,
        descending: descending,
        nullsLast: nullsLast,
        multithreaded: multithreaded,
        maintainOrder: maintainOrder,
      );

  /// Aggregate all columns as their standard deviances.
  LazyFrame std({required int ddof, dynamic hint}) =>
      RustLib.instance.api.lazyFrameStd(
        that: this,
        ddof: ddof,
      );

  /// Aggregate all columns as their sums.
  LazyFrame sum({dynamic hint}) => RustLib.instance.api.lazyFrameSum(
        that: this,
      );

  /// Get the last [n] rows.
  LazyFrame tail({required int n, dynamic hint}) =>
      RustLib.instance.api.lazyFrameTail(
        that: this,
        n: n,
      );

  /// Keep unique rows without maintaining order.
  LazyFrame unique(
          {List<String>? subset,
          required UniqueKeepStrategy keepStrategy,
          dynamic hint}) =>
      RustLib.instance.api.lazyFrameUnique(
        that: this,
        subset: subset,
        keepStrategy: keepStrategy,
      );

  /// Aggregate all columns as their variances.
  LazyFrame variance({required int ddof, dynamic hint}) =>
      RustLib.instance.api.lazyFrameVariance(
        that: this,
        ddof: ddof,
      );

  /// Add a column to this dataframe.
  LazyFrame withColumn({required Expr expr, dynamic hint}) =>
      RustLib.instance.api.lazyFrameWithColumn(
        that: this,
        expr: expr,
      );

  /// Add columns to this dataframe.
  LazyFrame withColumns({required List<Expr> exprs, dynamic hint}) =>
      RustLib.instance.api.lazyFrameWithColumns(
        that: this,
        exprs: exprs,
      );

  /// Add a new column at index 0 denoting the row number.
  LazyFrame withRowCount({required String name, int? offset, dynamic hint}) =>
      RustLib.instance.api.lazyFrameWithRowCount(
        that: this,
        name: name,
        offset: offset,
      );
}

// Rust type: flutter_rust_bridge::RustOpaque<std::sync::RwLock<Vec<Series>>>
@sealed
class VecSeries extends RustOpaque {
  VecSeries.dcoDecode(dynamic wire) : super.dcoDecode(wire, _kStaticData);

  VecSeries.sseDecode(int ptr, int externalSizeOnNative)
      : super.sseDecode(ptr, externalSizeOnNative, _kStaticData);

  static final _kStaticData = RustArcStaticData(
    rustArcIncrementStrongCount:
        RustLib.instance.api.rust_arc_increment_strong_count_VecSeries,
    rustArcDecrementStrongCount:
        RustLib.instance.api.rust_arc_decrement_strong_count_VecSeries,
    rustArcDecrementStrongCountPtr:
        RustLib.instance.api.rust_arc_decrement_strong_count_VecSeriesPtr,
  );
}

@freezed
sealed class Literals with _$Literals {
  const Literals._();
  const factory Literals.int64(
    Int64List field0,
  ) = Literals_Int64;
  const factory Literals.nullInt64(
    List<int?> field0,
  ) = Literals_NullInt64;
  const factory Literals.float64(
    Float64List field0,
  ) = Literals_Float64;
  const factory Literals.nullFloat64(
    List<double?> field0,
  ) = Literals_NullFloat64;
  const factory Literals.boolean(
    List<bool> field0,
  ) = Literals_Boolean;
  const factory Literals.duration(
    List<Duration> field0,
  ) = Literals_Duration;
  const factory Literals.nullDuration(
    List<Duration?> field0,
  ) = Literals_NullDuration;
  const factory Literals.stringLike(
    List<String> field0,
    DataType field1,
  ) = Literals_StringLike;
  const factory Literals.nullStringLike(
    List<String?> field0,
    DataType field1,
  ) = Literals_NullStringLike;
  const factory Literals.series(
    PSeries field0,
  ) = Literals_Series;
}

enum UniqueKeepStrategy {
  /// Keep the first unique row.
  first,

  /// Keep the last unique row.
  last,

  /// Keep None of the unique rows.
  none,

  /// Keep any of the unique rows
  /// This allows more optimizations
  any,
}
