use super::series::PSeries;
pub(crate) use super::{
    expr::{into_vec, DataType, Expr},
    series::{LazyGroupBy, Schema, Series},
    util::{any_value_to_dart, make_row},
};
use crate::bridge::StreamSink;
use anyhow::{Context, Result};
use flutter_rust_bridge::{frb, DartDynamic, RustOpaque};

pub(crate) type PDataFrame = polars::prelude::DataFrame;
pub(crate) type PLazyFrame = polars::prelude::LazyFrame;

pub(crate) use super::prelude::UniqueKeepStrategy;
pub(crate) use super::{expr::PExpr, prelude::*};
use chrono::Duration;
pub(crate) use core::panic::AssertUnwindSafe;
use std::fs::File;

/// A contiguous growable collection of [Series] that have the same length.
///
/// ## Import declarations
///
/// ```dart
/// import 'package:polars/polars.dart';                 // in Dart library
/// import 'package:flutter_polars/flutter_polars.dart'; // in Flutter
/// ```
///
/// # Initialization
/// ## Default
///
/// A `DataFrame` can be initialized empty:
///
/// ```dart
/// final df = DataFrame.of();
/// assert(df.isEmpty());
/// ```
///
/// ## Wrapping a `List<Series>`
///
/// A `DataFrame` is built upon a `List<Series>` where the [Series] have the same length.
///
/// ```dart
/// final s1 = Series.ofStrings(
///     name: "Fruit",
///     values: ["Apple", "Apple", "Pear"]);
/// final s2 = Series.ofStrings(
///     name: "Color",
///     values: ["Red", "Yellow", "Green"]);
/// final df = DataFrame.of(series: [s1, s2]);
/// ```
///
/// ## Using a CSV file
///
/// See [readCsv] and [scanCsv].
///
/// # Indexing
/// ## By a number
///
/// ```dart
/// final df = DataFrame.of(series: [
///     Series.ofStrings(name: "Fruit", values: ["Apple", "Apple", "Pear"]),
///     Series.ofStrings(name: "Color", values: ["Red", "Yellow", "Green"]),
/// ]);
///
/// assert(await df[0].asStrings(), ["Apple", "Apple", "Pear"]);
/// assert(await df[1].asStrings(), ["Red", "Yellow", "Green"]);
/// ```
///
/// ## By a [Series] name
///
/// ```dart
/// final df = DataFrame.of(series: [
///     Series.ofStrings(name: "Fruit", values: ["Apple", "Apple", "Pear"]),
///     Series.ofStrings(name: "Color", values: ["Red", "Yellow", "Green"]),
/// ]);
///
/// assert(await df["Fruit"].asStrings(), ["Apple", "Apple", "Pear"]);
/// assert(await df["Color"].asStrings(), ["Red", "Yellow", "Green"]);
/// ```
#[frb(opaque)]
pub struct DataFrame(pub(crate) AssertUnwindSafe<PDataFrame>);

impl DataFrame {
    #[inline]
    pub(crate) fn new(df: PDataFrame) -> Self {
        DataFrame(AssertUnwindSafe(df))
    }
}

/// Lazily-evaluated version of a [DataFrame].
///
/// Operations applied onto a [LazyFrame] will only be evaluated once
/// `.collect` is called, which returns the results as a new [DataFrame].
#[frb(opaque)]
pub struct LazyFrame(pub(crate) AssertUnwindSafe<PLazyFrame>);

impl LazyFrame {
    #[inline]
    pub(crate) fn new(df: PLazyFrame) -> Self {
        Self(AssertUnwindSafe(df))
    }
}

impl DataFrame {
    #[frb]
    pub fn write_csv(
        &mut self,
        path: String,
        #[frb(default = false)] include_bom: bool,
        #[frb(default = true)] include_header: bool,
        #[frb(default = false)] append: bool,
        #[frb(default = false)] create_new: bool,
        null_value: Option<String>,
    ) -> Result<()> {
        let file = if append {
            File::options().create_new(create_new).append(true)
        } else {
            File::options()
                .write(true)
                .truncate(true)
                .create_new(create_new)
        };
        let mut writer = CsvWriter::new(file.open(path)?)
            .include_bom(include_bom)
            .include_header(include_header);
        if let Some(null_value) = null_value {
            writer = writer.with_null_value(null_value);
        }
        writer.finish(&mut self.0)?;
        Ok(())
    }
    /// Returns a new, empty dataframe.
    #[frb(sync)]
    pub fn of_lits(series: Option<Vec<(String, Literals)>>) -> Result<DataFrame> {
        Ok(DataFrame::new(match series {
            Some(series) => PDataFrame::new(
                series
                    .into_iter()
                    .map(|(name, lits)| lits.into_series(&name))
                    .collect::<Result<Vec<_>>>()?,
            )?,
            None => Default::default(),
        }))
    }
    #[frb(sync)]
    pub fn clone(&self) -> DataFrame {
        DataFrame::new(self.0.clone())
    }
    /// Iterate through this dataframe's rows.
    ///
    /// Use [parseRow] to retrieve the canonical values for these rows.
    pub fn iter(&self, sink: StreamSink<Vec<DartDynamic>>) -> Result<()> {
        let my = &self.0;
        let mut buf = make_row(my.width());
        for idx in 0..my.height() {
            my.get_row_amortized(idx, &mut buf)?;
            let row = core::mem::take(&mut buf.0);
            sink.add(row.into_iter().map(any_value_to_dart).collect::<Vec<_>>())?;
        }
        Ok(())
    }
    /// Select a single column by name.
    ///
    /// Note: A clone of the column is returned, rather than a reference.
    #[frb(sync)]
    pub fn column(&self, column: String) -> Result<Series> {
        Ok(Series::new(self.0.column(&column)?.clone()))
    }
    /// Select multiple columns by name.
    ///
    /// Note: Clones of the columns are returned, rather than a reference.
    #[frb(sync)]
    pub fn columns(&self, columns: Vec<String>) -> Result<Vec<Series>> {
        Ok(self
            .0
            .columns(columns)?
            .into_iter()
            .cloned()
            .map(Series::new)
            .collect())
    }
    /// Select the column at the given index.
    #[frb(sync)]
    pub fn column_at(&self, index: usize) -> Series {
        Series::new(self.0[index].clone())
    }
    /// Dump the contents of this entire dataframe.
    #[frb(sync)]
    pub fn dump(&self) -> String {
        format!("{}", self.0 .0)
    }
    /// Returns the amount of bytes occupied by this series.
    #[frb(sync, getter)]
    pub fn estimated_size(&self) -> usize {
        self.0.estimated_size()
    }
    /// Add a new column at index 0 denoting the row number.
    #[frb(sync)]
    pub fn with_row_count(&self, name: String, offset: Option<u32>) -> Result<DataFrame> {
        Ok(DataFrame::new(self.0.with_row_count(&name, offset)?))
    }
    /// Get the names of this dataframe's columns.
    #[frb(sync, getter)]
    pub fn column_names(&self) -> Vec<String> {
        self.0
            .get_column_names()
            .into_iter()
            .map(Into::into)
            .collect()
    }
    /// Get all columns of this dataframe.
    #[frb(sync, getter)]
    pub fn get_columns(&self) -> Vec<Series> {
        self.0
            .get_columns()
            .iter()
            .cloned()
            .map(Series::new)
            .collect()
    }
    /// Returns the width of this dataframe, aka the number of columns.
    #[frb(sync, getter)]
    pub fn width(&self) -> usize {
        self.0.width()
    }
    /// Returns the height of this dataframe, aka the number of rows.
    #[frb(sync, getter)]
    pub fn height(&self) -> usize {
        self.0.height()
    }
    /// Returns whether this dataframe has no rows.
    #[frb(sync, getter)]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    /// Sample [n] datapoints from this dataframe.
    #[frb(sync)]
    pub fn sample(
        &self,
        n: &Series,
        #[frb(default = false)] with_replacement: bool,
        #[frb(default = false)] shuffle: bool,
        seed: Option<u64>,
    ) -> Result<DataFrame> {
        Ok(DataFrame::new(self.0.sample_n(
            &n.0,
            with_replacement,
            shuffle,
            seed,
        )?))
    }
    /// Makes a new dataframe with the specified columns from this dataframe.
    #[frb(sync)]
    pub fn select(&self, columns: Vec<String>) -> Result<DataFrame> {
        Ok(DataFrame::new(self.0.select(columns)?))
    }
    /// Returns the first few rows of this dataframe.
    #[frb(sync)]
    pub fn head(&self, length: Option<usize>) -> DataFrame {
        DataFrame::new(self.0.head(length))
    }
    /// Returns the last few rows of this dataframe.
    #[frb(sync)]
    pub fn tail(&self, length: Option<usize>) -> DataFrame {
        DataFrame::new(self.0.tail(length))
    }
    /// Output statistics about this dataframe.
    pub fn describe(&self, percentiles: Option<Vec<f64>>) -> Result<DataFrame> {
        Ok(DataFrame::new(self.0.describe(percentiles.as_deref())?))
    }
    /// Drops a column by name, producing a new dataframe.
    #[frb(sync)]
    pub fn drop(&self, column: String) -> Result<DataFrame> {
        Ok(DataFrame::new(PDataFrame::drop(&self.0, &column)?))
    }
    /// Drops a column in-place and returns it.
    #[frb(sync)]
    pub fn drop_in_place(&mut self, column: String) -> Result<Series> {
        Ok(Series::new(self.0.drop_in_place(&column)?))
    }
    /// Returns a dataframe with columns from this dataframe in reverse order.
    #[frb(sync)]
    pub fn reverse(&self) -> DataFrame {
        DataFrame::new(self.0.reverse())
    }
    /// Returns the height and width of this dataframe.
    #[frb(sync, getter)]
    pub fn shape(&self) -> (usize, usize) {
        self.0.shape()
    }
    /// Aggregate the columns to their maximum values.
    #[frb(sync)]
    pub fn max(&self) -> DataFrame {
        DataFrame::new(self.0.max())
    }
    /// Get a row of data from this dataframe.
    ///
    /// Prefer other functions to this inside a hot loop, as this function performs
    /// data copies and conversions to and from the native representation.
    #[frb(sync)]
    pub fn get_row(&self, index: usize) -> Result<Vec<DartDynamic>> {
        let row = self.0.get_row(index)?;
        Ok(row.0.into_iter().map(any_value_to_dart).collect())
    }
    /// Returns the [Schema] of this dataframe.
    #[frb(sync)]
    pub fn schema(&self) -> Schema {
        Schema::new(self.0.schema())
    }
    /// Returns the datatypes of this dataframe's columns.
    #[frb(sync)]
    pub fn dtypes(&self) -> Vec<DataType> {
        self.0.dtypes().into_iter().map(Into::into).collect()
    }
    /// Sorts this dataframe by the specified columns.
    #[frb(sync)]
    pub fn sort_in_place(
        &mut self,
        #[frb(default = [])] by_column: Vec<String>,
        #[frb(default = [])] descending: Vec<bool>,
        #[frb(default = false)] maintain_order: bool,
    ) -> Result<()> {
        self.0
            .sort_in_place(by_column, descending, maintain_order)?;
        Ok(())
    }
    /// Returns a [LazyFrame] to which operations can be applied lazily.
    /// As opposed to [LazyFrame], [DataFrame] by default applies its operations eagerly.
    #[frb(sync)]
    pub fn lazy(
        self,
        projection_pushdown: Option<bool>,
        predicate_pushdown: Option<bool>,
        type_coercion: Option<bool>,
        simplify_expressions: Option<bool>,
        slice_pushdown: Option<bool>,
        streaming: Option<bool>,
    ) -> LazyFrame {
        let mut lazy = self.0 .0.lazy();
        if let Some(opt) = projection_pushdown {
            lazy = lazy.with_projection_pushdown(opt);
        }
        if let Some(opt) = predicate_pushdown {
            lazy = lazy.with_predicate_pushdown(opt);
        }
        if let Some(opt) = type_coercion {
            lazy = lazy.with_type_coercion(opt);
        }
        if let Some(opt) = simplify_expressions {
            lazy = lazy.with_simplify_expr(opt);
        }
        if let Some(opt) = slice_pushdown {
            lazy = lazy.with_slice_pushdown(opt);
        }
        if let Some(opt) = streaming {
            lazy = lazy.with_streaming(opt);
        }

        LazyFrame::new(lazy)
    }
}

impl LazyFrame {
    /// Select (and rename) columns from the query.
    #[frb(sync)]
    pub fn select(self, exprs: Vec<Expr>) -> Result<LazyFrame> {
        Ok(LazyFrame::new(self.0 .0.select(into_vec(exprs))))
    }
    /// Filter by the specified predicate expression.
    #[frb(sync)]
    pub fn filter(self, pred: Expr) -> Result<LazyFrame> {
        Ok(LazyFrame::new(self.0 .0.filter(pred.into_internal())))
    }
    /// Define conditions by which to group and aggregate rows.
    #[frb(sync)]
    pub fn group_by(
        self,
        exprs: Vec<Expr>,
        #[frb(default = false)] maintain_order: bool,
    ) -> LazyGroupBy {
        let exprs = into_vec::<_, PExpr>(exprs);
        LazyGroupBy::new(if maintain_order {
            self.0 .0.group_by_stable(exprs)
        } else {
            self.0 .0.group_by(exprs)
        })
    }
    /// Reverse the order of this dataframe's columns.
    #[frb(sync)]
    pub fn reverse(self) -> LazyFrame {
        LazyFrame::new(self.0 .0.reverse())
    }
    /// Add a column to this dataframe.
    #[frb(sync)]
    pub fn with_column(self, expr: Expr) -> LazyFrame {
        LazyFrame::new(self.0 .0.with_column(expr.into_internal()))
    }
    /// Add columns to this dataframe.
    #[frb(sync)]
    pub fn with_columns(self, exprs: Vec<Expr>) -> LazyFrame {
        LazyFrame::new(self.0 .0.with_columns(into_vec(exprs)))
    }
    /// Caches the results into a new [LazyFrame].
    ///
    /// This should be used to prevent computations running multiple times.
    #[frb(sync)]
    pub fn cache(self) -> LazyFrame {
        LazyFrame::new(self.0 .0.cache())
    }
    /// Executes all lazy operations and collects results into a [DataFrame].
    ///
    /// Can also optionally be run in [streaming mode](https://docs.pola.rs/user-guide/concepts/streaming).
    #[frb]
    pub fn collect(self, #[frb(default = false)] streaming: bool) -> Result<DataFrame> {
        Ok(DataFrame::new(
            self.0 .0.with_streaming(streaming).collect()?,
        ))
    }
    /// Creates the [Cartesian product](https://en.wikipedia.org/wiki/Cartesian_product) from both frames,
    /// preserving the order of this frame's keys.
    #[frb(sync)]
    pub fn cross_join(self, other: LazyFrame) -> LazyFrame {
        LazyFrame::new(self.0 .0.cross_join(other.0 .0))
    }
    /// Performs a [left outer join](https://en.wikipedia.org/wiki/Join_(SQL)#Left_outer_join) with [other].
    #[frb(sync)]
    pub fn left_join(self, other: LazyFrame, left_on: Expr, right_on: Expr) -> LazyFrame {
        LazyFrame::new(self.0 .0.left_join(other.0 .0, left_on, right_on))
    }
    /// Performs a [full outer join](https://en.wikipedia.org/wiki/Join_(SQL)#Full_outer_join) with [other].
    #[frb(sync)]
    pub fn outer_join(self, other: LazyFrame, left_on: Expr, right_on: Expr) -> LazyFrame {
        LazyFrame::new(self.0 .0.outer_join(other.0 .0, left_on, right_on))
    }
    /// Performs an [inner join](https://en.wikipedia.org/wiki/Join_(SQL)#Inner_join_and_NULL_values) with [other].
    #[frb(sync)]
    pub fn inner_join(self, other: LazyFrame, left_on: Expr, right_on: Expr) -> LazyFrame {
        LazyFrame::new(self.0 .0.inner_join(other.0 .0, left_on, right_on))
    }
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
    #[frb(sync)]
    pub fn join(
        self,
        other: LazyFrame,
        on: Option<Vec<Expr>>,
        left_on: Option<Vec<Expr>>,
        right_on: Option<Vec<Expr>>,
        #[frb(default = "_right")] suffix: String,
        #[frb(default = "JoinType.Left")] how: JoinType,
        #[frb(default = true)] allow_parallel: bool,
        #[frb(default = false)] force_parallel: bool,
    ) -> LazyFrame {
        let mut b = self
            .0
             .0
            .join_builder()
            .with(other.0 .0)
            .how(how)
            .suffix(suffix)
            .allow_parallel(allow_parallel)
            .force_parallel(force_parallel);
        if let Some(on) = on {
            b = b.on(into_vec(on));
        }
        if let Some(left) = left_on {
            b = b.left_on(into_vec(left));
        }
        if let Some(right) = right_on {
            b = b.right_on(into_vec(right));
        }
        LazyFrame::new(b.finish())
    }
    /// Aggregate all columns as their max values.
    #[frb(sync)]
    pub fn max(self) -> LazyFrame {
        LazyFrame::new(self.0 .0.max())
    }
    /// Aggregate all columns as their min values.
    #[frb(sync)]
    pub fn min(self) -> LazyFrame {
        LazyFrame::new(self.0 .0.min())
    }
    /// Aggregate all columns as their sums.
    #[frb(sync)]
    pub fn sum(self) -> LazyFrame {
        LazyFrame::new(self.0 .0.sum())
    }
    /// Aggregate all columns as their means.
    #[frb(sync)]
    pub fn mean(self) -> LazyFrame {
        LazyFrame::new(self.0 .0.mean())
    }
    /// Aggregate all columns as their medians.
    #[frb(sync)]
    pub fn median(self) -> LazyFrame {
        LazyFrame::new(self.0 .0.median())
    }
    /// Aggregate all columns as their quantiles.
    #[frb(sync)]
    pub fn quantile(self, quantile: Expr, interpol: QuantileInterpolOptions) -> LazyFrame {
        LazyFrame::new(self.0 .0.quantile(quantile.into_internal(), interpol))
    }
    /// Aggregate all columns as their standard deviances.
    #[frb(sync)]
    pub fn std(self, ddof: u8) -> LazyFrame {
        LazyFrame::new(self.0 .0.std(ddof))
    }
    /// Aggregate all columns as their variances.
    #[frb(sync)]
    pub fn variance(self, ddof: u8) -> LazyFrame {
        LazyFrame::new(self.0 .0.var(ddof))
    }
    /// Explode each column.
    #[frb(sync)]
    pub fn explode(self, columns: Vec<Expr>) -> LazyFrame {
        LazyFrame::new(self.0 .0.explode(into_vec::<_, PExpr>(columns)))
    }
    /// Keep unique rows without maintaining order.
    #[frb(sync)]
    pub fn unique(
        self,
        subset: Option<Vec<String>>,
        keep_strategy: UniqueKeepStrategy,
    ) -> LazyFrame {
        LazyFrame::new(self.0 .0.unique(subset, keep_strategy))
    }
    /// Drop null rows.
    ///
    /// Same as `frame.filter(col('*').isNotNull)`.
    #[frb(sync)]
    pub fn drop_nulls(self, subset: Option<Vec<Expr>>) -> LazyFrame {
        LazyFrame::new(self.0 .0.drop_nulls(subset.map(into_vec)))
    }
    /// Slice the frame.
    #[frb(sync)]
    pub fn slice(self, offset: i64, len: u32) -> LazyFrame {
        LazyFrame::new(self.0 .0.slice(offset, len))
    }
    /// Get the first row.
    #[frb(sync, getter)]
    pub fn first(self) -> LazyFrame {
        LazyFrame::new(self.0 .0.first())
    }
    /// Get the last row.
    #[frb(sync, getter)]
    pub fn last(self) -> LazyFrame {
        LazyFrame::new(self.0 .0.last())
    }
    /// Get the last [n] rows.
    #[frb(sync)]
    pub fn tail(self, n: u32) -> LazyFrame {
        LazyFrame::new(self.0 .0.tail(n))
    }
    /// [Melt](https://docs.pola.rs/user-guide/transformations/melt) this
    /// dataframe from the wide format to the long format.
    #[frb(sync)]
    pub fn melt(
        self,
        id_vars: Vec<String>,
        value_vars: Vec<String>,
        variable_name: Option<String>,
        value_name: Option<String>,
        #[frb(default = true)] streamable: bool,
    ) -> LazyFrame {
        LazyFrame::new(self.0 .0.melt(MeltArgs {
            id_vars: id_vars.into_iter().map(Into::into).collect(),
            value_vars: value_vars.into_iter().map(Into::into).collect(),
            variable_name: variable_name.map(Into::into),
            value_name: value_name.map(Into::into),
            streamable,
        }))
    }
    /// Limit this dataframe to the first [n] rows.
    ///
    /// To avoid scanning the rows, use [fetch].
    #[frb(sync)]
    pub fn limit(self, n: u32) -> LazyFrame {
        LazyFrame::new(self.0 .0.tail(n))
    }
    /// Similar to [collect], but overrides the number of rows read by each operation.
    ///
    /// The final row count is not guaranteed to be equal [nRows].
    pub fn fetch(self, n_rows: usize) -> Result<DataFrame> {
        Ok(DataFrame::new(self.0 .0.fetch(n_rows)?))
    }
    /// Add a new column at index 0 denoting the row number.
    #[frb(sync)]
    pub fn with_row_count(self, name: String, offset: Option<u32>) -> LazyFrame {
        LazyFrame::new(self.0 .0.with_row_count(&name, offset))
    }
    #[frb(sync)]
    pub fn sort(
        self,
        by_column: String,
        #[frb(default = false)] descending: bool,
        #[frb(default = false)] nulls_last: bool,
        #[frb(default = true)] multithreaded: bool,
        #[frb(default = false)] maintain_order: bool,
    ) -> LazyFrame {
        LazyFrame::new(self.0 .0.sort(
            &by_column,
            SortOptions {
                descending,
                nulls_last,
                multithreaded,
                maintain_order,
            },
        ))
    }
    #[frb(sync)]
    pub fn null_count(self) -> LazyFrame {
        LazyFrame::new(self.0 .0.null_count())
    }
}

#[frb(mirror(UniqueKeepStrategy))]
pub enum _UniqueKeepStrategy {
    /// Keep the first unique row.
    First,
    /// Keep the last unique row.
    Last,
    /// Keep None of the unique rows.
    None,
    /// Keep any of the unique rows
    /// This allows more optimizations
    Any,
}

pub enum Literals {
    Int64(Vec<i64>),
    NullInt64(Vec<Option<i64>>),
    Float64(Vec<f64>),
    NullFloat64(Vec<Option<f64>>),
    Boolean(Vec<bool>),
    Duration(Vec<Duration>),
    NullDuration(Vec<Option<Duration>>),
    StringLike(Vec<String>, DataType),
    NullStringLike(Vec<Option<String>>, DataType),
    Series(RustOpaque<AssertUnwindSafe<PSeries>>),
}

impl Literals {
    pub(crate) fn into_series(self, name: &str) -> Result<PSeries> {
        match self {
            Literals::Int64(v) => Ok(PSeries::new(name, v)),
            Literals::NullInt64(v) => Ok(PSeries::new(name, v)),
            Literals::Float64(v) => Ok(PSeries::new(name, v)),
            Literals::NullFloat64(v) => Ok(PSeries::new(name, v)),
            Literals::Boolean(v) => Ok(PSeries::new(name, v)),
            Literals::Duration(v) => Ok(PSeries::new(name, v)),
            Literals::NullDuration(v) => Ok(PSeries::new(name, v)),
            Literals::StringLike(v, dt) => Ok(PSeries::new(name, v).strict_cast(&dt.into())?),
            Literals::NullStringLike(v, dt) => Ok(PSeries::new(name, v).strict_cast(&dt.into())?),
            Literals::Series(opaque) => Ok(opaque
                .into_inner()
                .context("cannot acquire unique series")?
                .0),
        }
    }
}
