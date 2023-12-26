#![warn(missing_docs)]
#![allow(clippy::too_many_arguments)]

use anyhow::{anyhow, Result};
use chrono::prelude::*;
use flutter_rust_bridge::*;
use std::ops::Rem;

use std::borrow::Cow;
use std::fs::File;
use std::ops::{Add, Div, Mul, Sub};
use std::panic::AssertUnwindSafe;
use std::path::Path;
pub use std::sync::RwLock;

pub(crate) use polars::io::RowCount;
pub(crate) use polars::lazy::dsl::*;
pub(crate) use polars::prelude::*;
pub(crate) use polars::series::IsSorted;

#[macro_use]
mod util;
use util::*;

use crate::bridge::StreamSink;

pub(crate) type PDataFrame = polars::prelude::DataFrame;

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
pub struct DataFrame(AssertUnwindSafe<PDataFrame>);

impl DataFrame {
    #[inline]
    fn new(df: PDataFrame) -> Self {
        DataFrame(AssertUnwindSafe(df))
    }
}

pub(crate) type PLazyFrame = polars::prelude::LazyFrame;

/// Lazily-evaluated version of a [DataFrame].
///
/// Operations applied onto a [LazyFrame] will only be evaluated once
/// `.collect` is called, which returns the results as a new [DataFrame].
#[frb(opaque)]
pub struct LazyFrame(AssertUnwindSafe<PLazyFrame>);

impl LazyFrame {
    #[inline]
    fn new(df: PLazyFrame) -> Self {
        Self(AssertUnwindSafe(df))
    }
}

pub(crate) type PSeries = polars::prelude::Series;

/// The columnar data type for a DataFrame.
///
/// ## Arithmetic
///
/// You can do standard arithmetic on series.
/// ```dart
/// final s = Series.ofI32(name: "a", values: [1, 2, 3]),
/// final outAdd = s + s;
/// final outSub = s - s;
/// final outDiv = s / s;
/// final outMul = s * s;
/// ```
///
/// Or with series and numbers.
///
/// ```dart
/// final s = Series.ofI32(name: "a", values: [1, 2, 3]),
/// final outAddOne = s + 1;
/// final outMultiply = s * 10;
///
/// // When on the right-hand side, methods must be used
/// final outDivide = 1.div(s);
/// final outAdd = 1.add(s);
/// final outSubtract = 1.sub(s);
/// final outMultiply = 1.mul(s);
/// ```
///
/// ## Comparison
/// You can obtain boolean mask by comparing series.
///
/// ```dart
/// import 'package:flutter/foundation.dart' show listEquals;
///
/// final s = Series.ofI32(name: "dollars", values: [1, 2, 3]),
/// final mask = s.equal(1);
/// assert(listEquals(await mask.asBools(), [true, false, false]));
/// ```
///
/// ## Iterators
/// The Series variants contain differently typed `ChunkedArray`s.
/// These structs can be turned into iterators, making it possible to use any function/ closure you want
/// on a Series.
///
/// These iterators return `T?` because the values of a series may be null.
///
/// ```dart
/// const pi = 3.14;
/// final s = Series.ofF64(name: "angle", values: [2 * pi, pi, 1.5 * pi]);
/// final sCos = (await s.asDoubles())
///    .iter()
///    .map((angle) => angle != null ? cos(angle) : null)
///    .toList();
/// ```
///
/// ## Creation
/// Series can be create from different data structures. Below we'll show a few ways we can create
/// a Series object.
///
/// ```
/// // Series can be created from Lists, slices and arrays
/// Series.ofBools(name: "boolean series", values: [true, false, false]);
/// Series.ofI32(name: "int series", values: [1, 2, 3]);
/// // And can be nullable
/// Series.ofI32(name: "got nulls", values: [1, null, 2]);
///
/// ```
#[frb(opaque)]
pub struct Series(AssertUnwindSafe<PSeries>);

impl Series {
    #[inline]
    fn new(series: PSeries) -> Self {
        Series(AssertUnwindSafe(series))
    }
}

pub(crate) type PLazyGroupBy = polars::prelude::LazyGroupBy;

/// A wrapper for group-by opereations on a [LazyFrame].
#[frb(opaque)]
pub struct LazyGroupBy(AssertUnwindSafe<PLazyGroupBy>);

impl LazyGroupBy {
    #[inline]
    fn new(groupby: PLazyGroupBy) -> Self {
        LazyGroupBy(AssertUnwindSafe(groupby))
    }
}

pub(crate) type PSchema = polars::prelude::Schema;

/// Schemas to specify datatypes and optimize operations.
#[frb(opaque)]
pub struct Schema(AssertUnwindSafe<PSchema>);

impl Schema {
    #[inline]
    fn new(schema: PSchema) -> Self {
        Schema(AssertUnwindSafe(schema))
    }
}

/// Reads a [.csv](https://en.wikipedia.org/wiki/Comma-separated_values) file into a [DataFrame].
///
/// - `columns`: Select only columns matching these names
/// - `delimiter`: Specify the delimiter for this file.
/// - `commentChar`: Ignore the rest of a line after encountering this character.
/// - `eolChar`: Stop reading after encountering this character.
/// - `quoteChar`: Specify the quote character, if set to null disables quoting.
/// - `skipRows`: Skip the first few rows, then parse the header and the dataframe.
/// - `skipRowsAfterHeader`: Skip this many rows after the header.
/// - `chunkSize`: Specify the chunk size of the internal parser. Performance knob.
/// - `nRows`: Try to read up to n rows then stop. Might not be honored in multithreading execution.
/// - `nullValues`: Specify values to be interpreted as null.
/// - `projection`: Select only columns at the specified indices.
/// - `rechunk`: Relocate the dataframe into contiguous memory after parsing.
///              Slow, but improves performance for later operations.
#[frb]
pub fn read_csv(
    path: String,
    // dtypes: Option<Schema>,
    dtypes_slice: Option<Vec<DataType>>,
    has_header: Option<bool>,
    columns: Option<Vec<String>>,
    // delimiter: Option<String>,
    comment_char: Option<String>,
    eol_char: Option<String>,
    chunk_size: Option<usize>,
    sample_size: Option<usize>,
    row_count: Option<RowCount>,
    encoding: Option<CsvEncoding>,
    n_rows: Option<usize>,
    n_threads: Option<usize>,
    null_values: Option<NullValues>,
    projection: Option<Vec<u32>>,
    #[frb(default = "'\"'")] quote_char: Option<String>,
    #[frb(default = 0)] skip_rows: usize,
    #[frb(default = 0)] skip_rows_after_header: usize,
    #[frb(default = false)] ignore_errors: bool,
    #[frb(default = false)] rechunk: bool,
    #[frb(default = true)] try_parse_dates: bool,
    #[frb(default = false)] low_memory: bool,
) -> Result<DataFrame> {
    let mut reader = CsvReader::from_path(path)?
        .with_columns(columns)
        .with_dtypes_slice(dtypes_slice.as_deref())
        .with_n_rows(n_rows)
        .with_ignore_errors(ignore_errors)
        .with_rechunk(rechunk)
        .with_null_values(null_values)
        .with_comment_char(comment_char.map(|comment| comment.as_bytes()[0]))
        .with_quote_char(quote_char.map(|quote| quote.as_bytes()[0]))
        .with_projection(projection.map(|proj| proj.into_iter().map(|idx| idx as _).collect()))
        .with_n_threads(n_threads)
        .with_try_parse_dates(try_parse_dates)
        .with_skip_rows(skip_rows)
        .with_skip_rows_after_header(skip_rows_after_header)
        .low_memory(low_memory)
        .with_row_count(row_count);
    if let Some(has_header) = has_header {
        reader = reader.has_header(has_header)
    }
    // if let Some(delimiter) = delimiter {
    //     reader = reader.with_delimiter(delimiter[0]);
    // }
    if let Some(eol) = eol_char {
        reader = reader.with_end_of_line_char(eol.as_bytes()[0]);
    }
    if let Some(size) = chunk_size {
        reader = reader.with_chunk_size(size)
    }
    if let Some(enc) = encoding {
        reader = reader.with_encoding(enc);
    }
    if let Some(sample) = sample_size {
        reader = reader.sample_size(sample);
    }
    // if let Some(dtypes) = dtypes {
    //     Ok(DataFrame::new(
    //         reader.with_dtypes(Some(&dtypes.0 .0)).finish()?,
    //     ))
    // } else {
    Ok(DataFrame::new(reader.finish()?))
    // }
}

/// Prepares a [.csv](https://en.wikipedia.org/wiki/Comma-separated_values) file for reading into a [LazyFrame].
///
/// - `delimiter`: Specify the delimiter for this file.
/// - `commentChar`: Ignore the rest of a line after encountering this character.
/// - `eolChar`: Stop reading after encountering this character.
/// - `quoteChar`: Specify the quote character, if set to null disables quoting.
/// - `skipRows`: Skip the first few rows, then parse the header and the dataframe.
/// - `skipRowsAfterHeader`: Skip this many rows after the header.
/// - `nRows`: Try to read up to n rows then stop. Might not be honored in multithreading execution.
/// - `nullValues`: Specify values to be interpreted as null.
/// - `rechunk`: Relocate the dataframe into contiguous memory after parsing.
///              Slow, but improves performance for later operations.
/// - `inferSchemaLength`: Specify how many rows to read to infer the schema, if null the entire table is scanned.
/// - `cache`: Cache the dataframe after reading.
#[frb]
pub fn scan_csv(
    path: String,
    dtype_overwrite: Option<Schema>,
    has_header: Option<bool>,
    // delimiter: Option<char>,
    comment_char: Option<String>,
    eol_char: Option<String>,
    #[frb(default = "'\"'")] quote_char: Option<String>,
    #[frb(default = 0)] skip_rows: usize,
    #[frb(default = 0)] skip_rows_after_header: usize,
    row_count: Option<RowCount>,
    encoding: Option<CsvEncoding>,
    n_rows: Option<usize>,
    null_values: Option<NullValues>,
    #[frb(default = false)] ignore_errors: bool,
    #[frb(default = false)] rechunk: bool,
    #[frb(default = true)] try_parse_dates: bool,
    #[frb(default = 100)] infer_schema_length: Option<usize>,
    #[frb(default = false)] cache: bool,
) -> Result<LazyFrame> {
    let mut reader = LazyCsvReader::new(path)
        .with_n_rows(n_rows)
        .with_ignore_errors(ignore_errors)
        .with_rechunk(rechunk)
        .with_null_values(null_values)
        .with_comment_char(comment_char.map(|comment| comment.as_bytes()[0]))
        .with_quote_char(quote_char.map(|quote| quote.as_bytes()[0]))
        .with_try_parse_dates(try_parse_dates)
        .with_skip_rows(skip_rows)
        .with_skip_rows_after_header(skip_rows_after_header)
        .with_infer_schema_length(infer_schema_length)
        .with_cache(cache)
        .with_row_count(row_count);
    if let Some(has_header) = has_header {
        reader = reader.has_header(has_header)
    }
    // if let Some(delimiter) = delimiter {
    //     reader = reader.with_delimiter(delimiter as _);
    // }
    if let Some(eol) = eol_char {
        reader = reader.with_end_of_line_char(eol.as_bytes()[0]);
    }
    if let Some(enc) = encoding {
        reader = reader.with_encoding(enc);
    }
    if let Some(dtype) = dtype_overwrite {
        Ok(LazyFrame::new(
            reader.with_dtype_overwrite(Some(&dtype.0)).finish()?,
        ))
    } else {
        Ok(LazyFrame::new(reader.finish()?))
    }
}

/// Reads a [.json](https://en.wikipedia.org/wiki/JSON) file into a [DataFrame].
pub fn read_json(
    path: String,
    // schema: Option<Schema>,
    batch_size: Option<usize>,
    projection: Option<Vec<String>>,
) -> Result<DataFrame> {
    let path = Path::new(&path);
    let file = File::open(path)?;
    let mut reader = JsonReader::new(file).with_projection(projection);
    if let Some(batch) = batch_size {
        reader = reader.with_batch_size(batch);
    }
    // if let Some(schema) = schema {
    //     reader = reader.with_schema(&schema.0);
    // }

    Ok(DataFrame::new(reader.finish()?))
}

/// Possible units of time for dataframe values.
#[frb(mirror(TimeUnit))]
pub(crate) enum _TimeUnit {
    /// One-billionth of a second.
    Nanoseconds,
    /// One-millionth of a second.
    Microseconds,
    /// One-thousandth of a second.
    Milliseconds,
}

impl DataFrame {
    /// Returns a new, empty dataframe.
    #[frb(sync)]
    pub fn of(series: Option<Vec<Series>>) -> Result<DataFrame> {
        Ok(DataFrame::new(match series {
            Some(series) => PDataFrame::new(
                series
                    .into_iter()
                    .map(|series| series.0 .0)
                    .collect::<Vec<_>>(),
            )?,
            None => Default::default(),
        }))
    }
    /// Iterate through this dataframe's rows.
    ///
    /// Use [parseRow] to retrieve the canonical values for these rows.
    pub fn iter(&self, sink: StreamSink<Vec<DartDynamic>>) -> Result<()> {
        let my = self.0;
        let mut buf = make_row(my.width());
        for idx in 0..my.height() {
            my.get_row_amortized(idx, &mut buf)?;
            let row = core::mem::take(&mut buf.0);
            let ok = sink.add(row.into_iter().map(any_value_to_dart).collect::<Vec<_>>());
            if !ok {
                break;
            }
        }
        sink.close();
        Ok(())
    }
    /// Select a single column by name.
    #[frb(sync)]
    pub fn column(&self, column: String) -> Result<Series> {
        // get!(my, self, DataFrame::column);
        Ok(Series::new(self.0.column(&column)?.clone()))
    }
    /// Select multiple columns by name.
    #[frb(sync)]
    pub fn columns(&self, columns: Vec<String>) -> Result<Vec<Series>> {
        // get!(my, self, DataFrame::columns);
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
    #[frb(sync)]
    pub fn estimated_size(&self) -> usize {
        self.0.estimated_size()
    }
    /// Add a new column at index 0 denoting the row number.
    #[frb(sync)]
    pub fn with_row_count(&self, name: String, offset: Option<u32>) -> Result<DataFrame> {
        Ok(DataFrame::new(self.0.with_row_count(&name, offset)?))
    }
    /// Get the names of this dataframe's columns.
    #[frb(sync)]
    pub fn get_column_names(&self) -> Vec<String> {
        self.0
            .get_column_names()
            .into_iter()
            .map(Into::into)
            .collect()
    }
    /// Get all columns of this dataframe.
    #[frb(sync)]
    pub fn get_columns(&self) -> Vec<Series> {
        self.0
            .get_columns()
            .iter()
            .cloned()
            .map(Series::new)
            .collect()
    }
    /// Returns the width of this dataframe, aka the number of columns.
    #[frb(sync)]
    pub fn width(&self) -> usize {
        self.0.width()
    }
    /// Returns the height of this dataframe, aka the number of rows.
    #[frb(sync)]
    pub fn height(&self) -> usize {
        self.0.height()
    }
    /// Returns whether this dataframe has no rows.
    #[frb(sync)]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    /// Sample [n] datapoints from this dataframe.
    #[frb]
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
    #[frb(sync)]
    pub fn shape(&self) -> Result<Shape> {
        // get!(my, self, DataFrame::shape);
        let (height, width) = self.0.shape();
        Ok(Shape { height, width })
    }
    /// Aggregate the columns to their maximum values.
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
        self.0.dtypes()
    }
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
        Ok(LazyFrame::new(self.0 .0.select(cast_exprs(exprs))))
    }
    /// Filter by the specified predicate expression.
    #[frb(sync)]
    pub fn filter(self, pred: Expr) -> Result<LazyFrame> {
        Ok(LazyFrame::new(self.0 .0.filter(pred.0 .0)))
    }
    /// Define conditions by which to group and aggregate rows.
    #[frb(sync)]
    pub fn group_by(
        self,
        exprs: Vec<Expr>,
        #[frb(default = false)] maintain_order: bool,
    ) -> LazyGroupBy {
        let exprs = cast_exprs(exprs);
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
        LazyFrame::new(self.0 .0.with_column(expr.0 .0))
    }
    /// Add columns to this dataframe.
    #[frb(sync)]
    pub fn with_columns(self, exprs: Vec<Expr>) -> LazyFrame {
        LazyFrame::new(self.0.with_columns(cast_exprs(exprs)))
    }
    /// Caches the results into a new [LazyFrame].
    ///
    /// This should be used to prevent computations running multiple times.
    #[frb(sync)]
    pub fn cache(self) -> LazyFrame {
        LazyFrame::new(self.0 .0.cache())
    }
    /// Executes all lazy operations and collects results into a [DataFrame].
    pub fn collect(self) -> Result<DataFrame> {
        Ok(DataFrame::new(self.0 .0.collect()?))
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
            b = b.on(cast_exprs(on));
        }
        if let Some(left) = left_on {
            b = b.left_on(cast_exprs(left));
        }
        if let Some(right) = right_on {
            b = b.right_on(cast_exprs(right));
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
        LazyFrame::new(self.0 .0.quantile(quantile.0 .0, interpol))
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
        LazyFrame::new(self.0 .0.explode(cast_exprs(columns)))
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
        LazyFrame::new(self.0 .0.drop_nulls(subset.map(cast_exprs)))
    }
    /// Slice the frame.
    #[frb(sync)]
    pub fn slice(self, offset: i64, len: u32) -> LazyFrame {
        LazyFrame::new(self.0 .0.slice(offset, len))
    }
    /// Get the first row.
    #[frb(sync)]
    pub fn first(self) -> LazyFrame {
        LazyFrame::new(self.0 .0.first())
    }
    /// Get the last row.
    #[frb(sync)]
    pub fn last(self) -> LazyFrame {
        LazyFrame::new(self.0 .0.last())
    }
    /// Get the last [n] rows.
    #[frb(sync)]
    pub fn tail(self, n: u32) -> LazyFrame {
        LazyFrame::new(self.0 .0.tail(n))
    }
    /// Melt this dataframe from the wide format to the long format.
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
}

impl Series {
    /// Create a new series of strings.
    #[frb(sync)]
    pub fn of_strings(name: String, values: Option<Vec<Option<String>>>) -> Series {
        Series::new(if let Some(values) = values {
            PSeries::new(&name, values)
        } else {
            PSeries::new_empty(&name, &DataType::Utf8)
        })
    }
    /// Create a new series of 32-bit wide integers.
    #[frb(sync)]
    pub fn of_i32(name: String, values: Option<Vec<Option<i32>>>) -> Series {
        Series::new(if let Some(values) = values {
            PSeries::new(&name, values)
        } else {
            PSeries::new_empty(&name, &DataType::Int32)
        })
    }
    /// Create a new series of 64-bit wide integers.
    #[frb(sync)]
    pub fn of_ints(name: String, values: Option<Vec<Option<i64>>>) -> Series {
        Series::new(if let Some(values) = values {
            PSeries::new(&name, values)
        } else {
            PSeries::new_empty(&name, &DataType::Int64)
        })
    }
    /// Create a new series of [Duration]s.
    #[frb(sync)]
    pub fn of_durations(
        name: String,
        values: Option<Vec<Option<chrono::Duration>>>,
        #[frb(default = "TimeUnit.Milliseconds")] unit: TimeUnit,
    ) -> Series {
        Series::new(if let Some(values) = values {
            PSeries::new(&name, values)
        } else {
            PSeries::new_empty(&name, &DataType::Duration(unit))
        })
    }
    /// Create a new series of doubles.
    #[frb(sync)]
    pub fn of_doubles(name: String, values: Option<Vec<Option<f64>>>) -> Series {
        Series::new(if let Some(values) = values {
            PSeries::new(&name, values)
        } else {
            PSeries::new_empty(&name, &DataType::Float64)
        })
    }
    #[frb(sync)]
    pub fn of_bools(name: String, values: Option<Vec<bool>>) -> Series {
        Series::new(if let Some(values) = values {
            PSeries::new(&name, values)
        } else {
            PSeries::new_empty(&name, &DataType::Boolean)
        })
    }
    /// Adds the contents of [other] onto this series.
    ///
    /// Throws if [other] is self.
    #[frb(sync)]
    pub fn append(&mut self, other: &Series) -> Result<()> {
        self.0.append(&other.0)?;
        Ok(())
    }
    /// Casts this series into one with the specified datatype.
    #[frb(sync)]
    pub fn cast(&self, dtype: DataType, #[frb(default = true)] strict: bool) -> Result<Series> {
        Ok(Series::new(if strict {
            self.0.strict_cast(&dtype)?
        } else {
            self.0.cast(&dtype)?
        }))
    }
    /// If this series is a UTF-8 series, returns its Dart representation.
    #[frb(sync)]
    pub fn as_strings(&self) -> Result<Vec<Option<String>>> {
        Ok(self
            .0
            .utf8()?
            .into_iter()
            .map(|e| e.map(ToOwned::to_owned))
            .collect())
    }
    /// If compatible, returns a representation of this series as integers.
    #[frb(sync)]
    pub fn as_ints(&self, #[frb(default = true)] strict: bool) -> Result<Vec<Option<i64>>> {
        let my = if strict {
            self.0.strict_cast(&DataType::Int64)
        } else {
            self.0.cast(&DataType::Int64)
        }?;
        Ok(my.i64().unwrap().into_iter().collect())
    }
    /// If compatible, returns a representation of this series as integers.
    #[frb]
    pub fn as_doubles(&self, #[frb(default = true)] strict: bool) -> Result<Vec<Option<f64>>> {
        let my = if strict {
            self.0.strict_cast(&DataType::Float64)
        } else {
            self.0.cast(&DataType::Float64)
        }?;
        Ok(my
            .cast(&DataType::Float64)?
            .f64()
            .unwrap()
            .into_iter()
            .collect())
    }
    /// If this series contains [Duration]s, returns its Dart representation.
    #[frb(sync)]
    pub fn as_durations(&self) -> Result<Vec<Option<chrono::Duration>>> {
        let ds = self.0.duration()?;
        let ctor = match ds.time_unit() {
            TimeUnit::Nanoseconds => chrono::Duration::nanoseconds,
            TimeUnit::Microseconds => chrono::Duration::microseconds,
            TimeUnit::Milliseconds => chrono::Duration::milliseconds,
        };
        Ok(ds.into_iter().map(|dur| Some(ctor(dur?))).collect())
    }
    /// If this series contains [DateTime]s, returns its Dart representation.
    ///
    /// Datetimes are parsed as-is, without any timezone correction.
    #[frb(sync)]
    pub fn as_naive_datetime(&self) -> Result<Vec<Option<NaiveDateTime>>> {
        Ok(self.0.datetime()?.as_datetime_iter().collect())
    }
    /// If this series contains [DateTime]s, returns its Dart representation.
    ///
    /// If a timezone is defined by this series, the datetimes will be converted to UTC.
    /// Otherwise, the datetimes are assumed to be in UTC.
    #[frb(sync)]
    #[inline]
    pub fn as_utc_datetime(&self) -> Result<Vec<Option<DateTime<Utc>>>> {
        self.as_datetime_impl(Utc)
    }
    /// If this series contains [DateTime]s, returns its Dart representation.
    ///
    /// If a timezone is defined by this series, the datetimes will be converted to the local timezone.
    /// Otherwise, the datetimes are assumed to be in the local timezone.
    #[frb(sync)]
    #[inline]
    pub fn as_local_datetime(&self) -> Result<Vec<Option<DateTime<Local>>>> {
        self.as_datetime_impl(Local)
    }
    fn as_datetime_impl<Tz: chrono::TimeZone>(
        &self,
        target: Tz,
    ) -> Result<Vec<Option<DateTime<Tz>>>> {
        let dt = self.0.datetime()?;
        if let Some(tz) = dt.time_zone().as_deref() {
            let tz = tz
                .parse::<chrono_tz::Tz>()
                .map_err(|err| anyhow!("Couldn't parse timezone ({})", err))?;
            let dt = dt.as_datetime_iter().map(|naive| {
                let dt = naive?.and_local_timezone(tz).single()?;
                Some(dt.with_timezone(&target))
            });
            Ok(dt.collect())
        } else {
            Ok(dt
                .as_datetime_iter()
                .map(|naive| target.from_local_datetime(&naive?).single())
                .collect())
        }
    }
    // /// Returns a new series with each value's absolute value.
    // pub fn abs(&self) -> Result<Series> {
    //     // get!(my, self, Series::abs);
    //     Ok(Series::new(self.0.abs()?))
    // }
    /// Returns a new sorted series.
    #[frb(sync)]
    pub fn sort(&self, #[frb(default = false)] reverse: bool) -> Series {
        Series::new(self.0.sort(reverse))
    }
    /// Returns a new shuffled series.
    #[frb(sync)]
    pub fn shuffle(&self, seed: Option<u64>) -> Series {
        Series::new(self.0.shuffle(seed))
    }
    /// Sums all non-null rows in this series to produce a result.
    ///
    /// Returns null if the series only contains null values.
    #[frb(sync)]
    pub fn sum(&self) -> Option<f64> {
        self.0.sum()
    }
    /// Returns the sum of this series' values as a single-element series.
    #[frb(sync)]
    pub fn sum_as_series(&self) -> Series {
        Series::new(self.0.sum_as_series())
    }
    /// Returns the minimum value of this series' values.
    ///
    /// Returns null if one of the values are also null.
    #[frb(sync)]
    pub fn min(&self) -> Option<f64> {
        self.0.min()
    }
    /// Returns the maximum value of this series' values.
    ///
    /// Returns null if one of the values are also null.
    #[frb(sync)]
    pub fn max(&self) -> Result<Option<f64>> {
        // get!(my, self, Series::max);
        Ok(self.0.max())
    }
    /// Expands a series of lists into rows of values, or strings into rows of characters.
    #[frb(sync)]
    pub fn explode(&self) -> Result<Series> {
        // get!(my, self, Series::explode);
        Ok(Series::new(self.0.explode()?))
    }
    /// TODO: docs
    #[frb(sync)]
    pub fn explode_by_offsets(&self, offsets: Vec<i64>) -> Result<Series> {
        // get!(my, self, Series::explode_by_offsets);
        Ok(Series::new(self.0.explode_by_offsets(&offsets)))
    }
    // /// Calculates the cumulative max at each element.
    // #[frb]
    // pub fn cummax(&self, #[frb(default = false)] reverse: bool) -> Result<Series> {
    //     // get!(my, self, Series::cummax);
    //     Ok(Series::new(self.0.cummax(reverse)))
    // }
    // /// Calculates the cumulative min at each element.
    // #[frb]
    // pub fn cummin(&self, #[frb(default = false)] reverse: bool) -> Result<Series> {
    //     // get!(my, self, Series::cummin);
    //     Ok(Series::new(self.0.cummin(reverse)))
    // }
    // /// Calculates the cumulative product at each element.
    // #[frb]
    // pub fn cumprod(&self, #[frb(default = false)] reverse: bool) -> Result<Series> {
    //     // get!(my, self, Series::cumprod);
    //     Ok(Series::new(self.0.cumprod(reverse)))
    // }
    // /// Calculates the cumulative sum at each element.
    // #[frb]
    // pub fn cumsum(&self, #[frb(default = false)] reverse: bool) -> Result<Series> {
    //     // get!(my, self, Series::cumsum);
    //     Ok(Series::new(self.0.cumsum(reverse)))
    // }
    /// Calculates the product of each element in the series and returns it in a single-element series.
    #[frb(sync)]
    pub fn product(&self) -> Series {
        Series::new(self.0.product())
    }
    /// Get the value at [index] as a string.
    #[frb(sync)]
    pub fn get_string(&self, index: usize) -> Option<String> {
        self.0.str_value(index).ok().map(Cow::into_owned)
    }
    /// Get the value at [index] as a double.
    #[frb(sync)]
    pub fn get(&self, index: usize) -> Result<Option<f64>> {
        Ok(self
            .0
            .get(index)
            .ok()
            .and_then(|value| value.try_extract().ok()))
    }
    /// Get the first few values of this series.
    #[frb(sync)]
    pub fn head(&self, length: Option<usize>) -> Series {
        Series::new(self.0.head(length))
    }
    /// Get the last few values of this series.
    #[frb(sync)]
    pub fn tail(&self, length: Option<usize>) -> Series {
        Series::new(self.0.tail(length))
    }
    /// Calculates the mean (average) of this series.
    #[frb(sync)]
    pub fn mean(&self) -> Option<f64> {
        self.0.mean()
    }
    /// Calculates the [median](https://en.wikipedia.org/wiki/Median) of this series.
    #[frb(sync)]
    pub fn median(&self) -> Option<f64> {
        self.0.median()
    }
    /// Calculates and wraps this series' mean as a single-element series.
    #[frb(sync)]
    pub fn mean_as_series(&self) -> Series {
        Series::new(self.0.mean_as_series())
    }
    /// Calculates and wraps this series' median as a single-element series.
    #[frb(sync)]
    pub fn median_as_series(&self) -> Series {
        Series::new(self.0.median_as_series())
    }
    /// Returns the amount of bytes occupied by this series.
    #[frb(sync)]
    pub fn estimated_size(&self) -> Result<usize> {
        // get!(my, self, Series::estimated_size);
        Ok(self.0.estimated_size())
    }
    /// Returns a new series with elements from this series added to [other]'s element-wise.
    #[frb(sync)]
    pub fn add_to(&self, other: &Series) -> Result<Series> {
        Ok(Series::new(self.0.add_to(&other.0)?))
    }
    /// Returns a new series with elements from this series subtracted from [other]'s element-wise.
    #[frb(sync)]
    pub fn subtract(&self, other: &Series) -> Result<Series> {
        Ok(Series::new(self.0.subtract(&other.0)?))
    }
    /// Returns a new series with elements from this series multiplied with [other]'s element-wise.
    #[frb(sync)]
    pub fn multiply(&self, other: Series) -> Result<Series> {
        Ok(Series::new(self.0.multiply(&other.0)?))
    }
    /// Returns a new series with elements from this series divided by [other]'s element-wise.
    #[frb(sync)]
    pub fn divide(&self, other: Series) -> Result<Series> {
        Ok(Series::new(self.0.divide(&other.0)?))
    }
    /// Returns a new series with the [remainder](https://en.wikipedia.org/wiki/Remainder)
    /// between this series' and [other]'s elements.
    #[frb(sync)]
    pub fn remainder(&self, other: Series) -> Result<Series> {
        Ok(Series::new(self.0.remainder(&other.0)?))
    }
    /// Returns whether this is a series of booleans.
    #[frb(sync)]
    pub fn is_bool(&self) -> bool {
        matches!(self.0.dtype(), DataType::Boolean)
    }
    /// Returns whether this is a series of UTF-8 strings.
    #[frb(sync)]
    pub fn is_utf8(&self) -> Result<bool> {
        // get!(my, self, Series::is_utf8);
        Ok(matches!(self.0.dtype(), DataType::Utf8))
    }
    /// Returns whether this is a series of numeric values.
    #[frb(sync)]
    pub fn is_numeric(&self) -> bool {
        self.0.dtype().is_numeric()
    }
    /// Returns whether this is a series of [DateTime] or [Duration]s.
    #[frb(sync)]
    pub fn is_temporal(&self) -> bool {
        self.0.dtype().is_temporal()
    }
    /// Dump the contents of this entire series.
    #[frb(sync)]
    pub fn dump(&self) -> String {
        format!("{}", self.0 .0)
    }
    /// Rename this series to [name] in-place.
    #[frb(sync)]
    pub fn rename(&mut self, name: String) {
        self.0.rename(&name);
    }
    /// Returns the unique values of this series.
    ///
    /// If `stable` is true, extra work is done to maintain the original order of elements.
    #[frb]
    pub fn unique(&self, #[frb(default = false)] maintain_order: bool) -> Result<Series> {
        Ok(Series::new(if maintain_order {
            self.0.unique_stable()?
        } else {
            self.0.unique()?
        }))
    }
    /// Returns whether this series is identical to [other].
    ///
    /// if `ignoreNull` is true, null values are considered to be equal.
    #[frb(sync)]
    pub fn equal(&self, other: &Series, #[frb(default = false)] ignore_null: bool) -> bool {
        if ignore_null {
            self.0.series_equal_missing(&other.0)
        } else {
            self.0.series_equal(&other.0)
        }
    }
    /// Applies a binary operation onto this series with a scalar value.
    ///
    /// For logic operators, the new series is a boolean mask. Otherwise,
    /// it will be a series of numeric values.
    #[frb(sync)]
    pub fn apply_scalar(&self, op: Operator, value: f64) -> Result<Series> {
        use Operator::*;
        let vec = match op {
            Eq => self.0.equal(value)?.into_series(),
            EqValidity => self.0.equal_missing(value)?.into_series(),
            NotEq => self.0.not_equal(value)?.into_series(),
            NotEqValidity => self.0.not_equal_missing(value)?.into_series(),
            Lt => self.0.lt(value)?.into_series(),
            LtEq => self.0.lt_eq(value)?.into_series(),
            Gt => self.0.gt(value)?.into_series(),
            GtEq => self.0.gt_eq(value)?.into_series(),
            Plus => self.0.add(value).into_series(),
            Minus => self.0.add(-value).into_series(),
            Multiply => self.0.mul(value).into_series(),
            Divide => self.0.div(value).into_series(),
            Modulus => self
                .0
                .remainder(&PSeries::new("", vec![value; self.0.len()]))?,
            And => self
                .0
                .bitand(&PSeries::new("", vec![value; self.0.len()]))?,
            Or => self.0.bitor(&PSeries::new("", vec![value; self.0.len()]))?,
            Xor => self
                .0
                .bitxor(&PSeries::new("", vec![value; self.0.len()]))?,
            TrueDivide | FloorDivide => return Err(anyhow!("Not implemented: {op}")),
        };
        Ok(Series::new(vec.into_series()))
    }
    /// Creates a new series with the specified dimensions.
    #[frb(sync)]
    pub fn reshape(&self, dims: Vec<i64>) -> Result<Series> {
        Ok(Series::new(self.0.reshape(&dims)?))
    }
    /// Calculates the standard deviation of this series with the specified degree of freedom.
    #[frb(sync)]
    pub fn std_as_series(&self, ddof: u8) -> Series {
        Series::new(self.0.std_as_series(ddof))
    }
    /// Calculates the variance of this series with the specified degree of freedom.
    #[frb(sync)]
    pub fn var_as_series(&self, ddof: u8) -> Series {
        Series::new(self.0.var_as_series(ddof))
    }
    /// Returns an untyped list.
    #[frb(sync)]
    pub fn to_list(&self) -> Vec<DartDynamic> {
        self.0.iter().map(any_value_to_dart).collect::<Vec<_>>()
    }
    /// Casts this series into a [DataFrame]. May create a copy.
    #[frb(sync)]
    pub fn into_frame(self) -> DataFrame {
        DataFrame::new(self.0 .0.into_frame())
    }
    /// Iterate over this series' values.
    pub fn iter(&self, sink: StreamSink<DartDynamic>) {
        for value in self.0.iter() {
            let ok = sink.add(any_value_to_dart(value));
            if !ok {
                break;
            }
        }
        sink.close();
    }
}

impl LazyGroupBy {
    /// Group by and aggregate.
    ///
    /// Select a column with [col] and choose an aggregation. If you want to aggregate all columns
    /// use <code>[col]\("*")</code>.
    #[frb(sync)]
    pub fn agg(self, exprs: Vec<Expr>) -> LazyFrame {
        LazyFrame::new(self.0 .0.agg(cast_exprs(exprs)))
    }
    /// Return the first [n] rows of each group.
    #[frb(sync)]
    pub fn head(self, n: Option<usize>) -> LazyFrame {
        LazyFrame::new(self.0 .0.head(n))
    }
    /// Return the last [n] rows of each group.
    #[frb(sync)]
    pub fn tail(self, n: Option<usize>) -> LazyFrame {
        LazyFrame::new(self.0 .0.tail(n))
    }
}

impl Schema {
    /// Create a schema from a list of [Field]s.
    #[frb(sync)]
    pub fn of(fields: Vec<Field>) -> Schema {
        Schema::new(PSchema::from_iter(fields))
    }
}

/// Describes the shape of a [DataFrame].
pub struct Shape {
    /// The number of rows.
    pub height: usize,
    /// The number of columns.
    pub width: usize,
}

/// Expressions for use in query and aggregration operations.
#[frb(opaque)]
#[repr(transparent)]
pub struct Expr(AssertUnwindSafe<PExpr>);
pub(crate) type PExpr = polars::lazy::dsl::Expr;
fn cast_exprs(exprs: Vec<Expr>) -> Vec<PExpr> {
    // SAFETY: We statically asserted that they have the same layout.
    // Even though AssertUnwindSafe does not have an explicit #[repr(transparent)],
    // it still only has one field which is a good enough guarantee.
    unsafe { std::mem::transmute(exprs) }
}

impl From<Expr> for PExpr {
    #[inline]
    fn from(value: Expr) -> Self {
        value.0 .0
    }
}

const _: () = {
    macro_rules! assert_layout {
        ($lhs:ty, $rhs:ty) => {{
            let lhs = core::alloc::Layout::new::<$lhs>();
            let rhs = core::alloc::Layout::new::<$rhs>();
            if lhs.align() != rhs.align() {
                panic!(concat!(
                    "Alignments differ: ",
                    stringify!($lhs),
                    " != ",
                    stringify!($rhs)
                ));
            }
            if lhs.size() != rhs.size() {
                panic!(concat!(
                    "Sizes differ: ",
                    stringify!($lhs),
                    " != ",
                    stringify!($rhs)
                ));
            }
        }};
    }
    assert_layout!(Expr, PExpr);
    assert_layout!(Vec<Expr>, Vec<PExpr>);
};

macro_rules! delegate {
    ($( $(#[$attribute:meta])* $fn:ident(self $(,)? $($param:ident : $(#[$conv:ident])? $ty:ty $(= $default:expr)? ),*) -> $output:ty; )*) => {$(
        $(#[$attribute])*
        pub fn $fn(self, $($(#[frb(default = $default)])? $param : $ty),*) -> $output {
            <$output>::new(self.0 .0.$fn($($param $(.$conv())?),*))
        }
    )*};
}

macro_rules! rolling_series {
    ($($fn:ident;)*) => {$(
        #[frb(sync)]
        pub fn $fn(
            self,
            window_size: Option<Duration>,
            min_periods: Option<usize>,
            weights: Option<Vec<f64>>,
            #[frb(default = false)] center: bool,
            by: Option<String>,
            closed_window: Option<ClosedWindow>,
        ) -> Expr {
            Expr::new(self.0 .$fn(rolling_options(
                window_size,
                min_periods,
                weights,
                center,
                by,
                closed_window,
            )))
        }
    )*};
}

#[frb(sync)]
pub fn col(name: String) -> Expr {
    Expr::new(PExpr::Column(name.into()))
}

#[frb(sync)]
pub fn cols(names: Vec<String>) -> Expr {
    Expr::new(PExpr::Columns(names.into()))
}

#[frb(sync)]
pub fn dtypes(types: Vec<DataType>) -> Expr {
    Expr::new(PExpr::DtypeColumn(types))
}

#[frb(sync)]
pub fn nth(idx: i64) -> Expr {
    Expr::new(PExpr::Nth(idx))
}

#[frb(sync)]
pub fn count() -> Expr {
    Expr::new(PExpr::Count)
}

#[frb(sync)]
pub fn lit(value: LiteralValue) -> Expr {
    Expr::new(PExpr::Literal(value))
}

impl Expr {
    #[inline]
    fn new(expr: PExpr) -> Expr {
        Expr(AssertUnwindSafe(expr))
    }
    #[inline]
    fn erase(self) -> PExpr {
        self.0 .0
    }
    delegate! {
        #[frb(sync)] abs(self) -> Expr;
        #[frb(sync)] add(self, other: #[erase] Expr) -> Expr;
        #[frb(sync)] alias(self, name: #[as_str] String) -> Expr;
        #[frb(sync)] arccos(self) -> Expr;
        #[frb(sync)] arccosh(self) -> Expr;
        #[frb(sync)] arcsin(self) -> Expr;
        #[frb(sync)] arcsinh(self) -> Expr;
        #[frb(sync)] arctan(self) -> Expr;
        #[frb(sync)] arctan2(self, x: #[erase] Expr) -> Expr;
        #[frb(sync)] arctanh(self) -> Expr;
        #[frb(sync)] arg_max(self) -> Expr;
        #[frb(sync)] arg_min(self) -> Expr;
        #[frb(sync)] arg_unique(self) -> Expr;
        #[frb(sync)] agg_groups(self) -> Expr;
        #[frb(sync)] all(self, ignore_nulls: bool = false) -> Expr;
        #[frb(sync)] any(self, ignore_nulls: bool = false) -> Expr;
        #[frb(sync)] and(self, expr: Expr) -> Expr;
        #[frb(sync)] append(self, other: Expr, upcast: bool = true) -> Expr;
        #[frb(sync)] backward_fill(self, limit: Option<u32>) -> Expr;
        #[frb(sync)] cast(self, data_type: DataType) -> Expr;
        #[frb(sync)] cbrt(self) -> Expr;
        #[frb(sync)] ceil(self) -> Expr;
        #[frb(sync)] clip(self, min: #[erase] Expr, max: #[erase] Expr) -> Expr;
        #[frb(sync)] cos(self) -> Expr;
        #[frb(sync)] cosh(self) -> Expr;
        /// Calculate the cotangent of this expression.
        #[frb(sync)] cot(self) -> Expr;
        #[frb(sync)] count(self) -> Expr;
        #[frb(sync)] clip_max(self, max: #[erase] Expr) -> Expr;
        #[frb(sync)] clip_min(self, min: #[erase] Expr) -> Expr;
        #[frb(sync)] cum_count(self, reverse: bool = false) -> Expr;
        #[frb(sync)] cum_max(self, reverse: bool = false) -> Expr;
        #[frb(sync)] cum_min(self, reverse: bool = false) -> Expr;
        #[frb(sync)] cum_prod(self, reverse: bool = false) -> Expr;
        #[frb(sync)] cum_sum(self, reverse: bool = false) -> Expr;
        #[frb(sync)] div(self, other: #[erase] Expr) -> Expr;
        #[frb(sync)] degrees(self) -> Expr;
        #[frb(sync)] dot(self, other: Expr) -> Expr;
        #[frb(sync)] drop_nans(self) -> Expr;
        #[frb(sync)] drop_nulls(self) -> Expr;
        #[frb(sync)] entropy(self, base: f64, normalize: bool = false) -> Expr;
        #[frb(sync)] eq(self, other: Expr) -> Expr;
        #[frb(sync)] eq_missing(self, other: Expr) -> Expr;
        #[frb(sync)] exclude(self, columns: Vec<String>) -> Expr;
        #[frb(sync)] exclude_dtype(self, dtypes: Vec<DataType>) -> Expr;
        #[frb(sync)] exp(self) -> Expr;
        #[frb(sync)] explode(self) -> Expr;
        #[frb(sync)] fill_nan(self, value: Expr) -> Expr;
        #[frb(sync)] fill_null(self, value: Expr) -> Expr;
        #[frb(sync)] filter(self, cond: Expr) -> Expr;
        #[frb(sync)] first(self) -> Expr;
        #[frb(sync)] flatten(self) -> Expr;
        #[frb(sync)] floor(self) -> Expr;
        #[frb(sync)] floor_div(self, rhs: #[erase] Expr) -> Expr;
        #[frb(sync)] forward_fill(self, limit: Option<u32>) -> Expr;
        #[frb(sync)] gather(self, idx: Expr) -> Expr;
        /// Similar to [gather] but allows for scalars.
        #[frb(sync)] get(self, idx: Expr) -> Expr;
        #[frb(sync)] gt(self, other: Expr) -> Expr;
        #[frb(sync)] gt_eq(self, other: Expr) -> Expr;
        #[frb(sync)] head(self, length: Option<usize>) -> Expr;
        #[frb(sync)] implode(self) -> Expr;
        #[frb(sync)] is_finite(self) -> Expr;
        #[frb(sync)] is_in(self, other: Expr) -> Expr;
        #[frb(sync)] is_nan(self) -> Expr;
        #[frb(sync)] is_not_nan(self) -> Expr;
        #[frb(sync)] is_not_null(self) -> Expr;
        #[frb(sync)] is_null(self) -> Expr;
        #[frb(sync)] last(self) -> Expr;
        #[frb(sync)] log(self, base: f64) -> Expr;
        #[frb(sync)] log1p(self) -> Expr;
        #[frb(sync)] lower_bound(self) -> Expr;
        #[frb(sync)] lt(self, other: Expr) -> Expr;
        #[frb(sync)] lt_eq(self, other: Expr) -> Expr;
        #[frb(sync)] mul(self, other: #[erase] Expr) -> Expr;
        #[frb(sync)] n_unique(self) -> Expr;
        #[frb(sync)] nan_max(self) -> Expr;
        #[frb(sync)] nan_min(self) -> Expr;
        #[frb(sync)] neq(self, other: Expr) -> Expr;
        #[frb(sync)] neq_missing(self, other: Expr) -> Expr;
        #[frb(sync)] not(self) -> Expr;
        #[frb(sync)] null_count(self) -> Expr;
        #[frb(sync)] or(self, expr: Expr) -> Expr;
        #[frb(sync)] pow(self, exponent: f64) -> Expr;
        #[frb(sync)] product(self) -> Expr;
        #[frb(sync)] radians(self) -> Expr;
        #[frb(sync)] reshape(self, dims: #[as_slice] Vec<i64>) -> Expr;
        #[frb(sync)] rem(self, other: #[erase] Expr) -> Expr;
        #[frb(sync)] reverse(self) -> Expr;
        #[frb(sync)] round(self, decimals: u32) -> Expr;
        #[frb(sync)] round_sig_figs(self, digits: i32) -> Expr;
        #[frb(sync)] set_sorted_flag(self, sorted: IsSorted) -> Expr;
        #[frb(sync)] shift(self, n: #[erase] Expr) -> Expr;
        #[frb(sync)] shift_and_fill(self, n: Expr, fill_value: Expr) -> Expr;
        #[frb(sync)] shrink_dtype(self) -> Expr;
        #[frb(sync)] sin(self) -> Expr;
        #[frb(sync)] sinh(self) -> Expr;
        #[frb(sync)] slice(self, offset: Expr, length: Expr) -> Expr;
        // #[frb(sync)] sort_by(self, by: Expr, descending: bool = false) -> Expr;
        #[frb(sync)] sqrt(self) -> Expr;
        #[frb(sync)] sub(self, other: #[erase] Expr) -> Expr;
        #[frb(sync)] std(self, ddof: u8) -> Expr;
        #[frb(sync)] strict_cast(self, data_type: DataType) -> Expr;
        #[frb(sync)] sum(self) -> Expr;
        #[frb(sync)] tail(self, length: Option<usize>) -> Expr;
        #[frb(sync)] tan(self) -> Expr;
        #[frb(sync)] tanh(self) -> Expr;
        #[frb(sync)] to_physical(self) -> Expr;
        #[frb(sync)] unique(self) -> Expr;
        #[frb(sync)] unique_stable(self) -> Expr;
        #[frb(sync)] upper_bound(self) -> Expr;
        #[frb(sync)] value_counts(self, sort: bool = false, parallel: bool = true) -> Expr;
        #[frb(sync)] xor(self, expr: Expr) -> Expr;
    }
    rolling_series! {
        rolling_min;
        rolling_max;
        rolling_mean;
        rolling_median;
        rolling_quantile;
        rolling_std;
        rolling_sum;
        rolling_var;
    }
    pub fn what(self) {
        // namespaces
        // self.0.binary();
        // self.0.list();
        // self.0.name();
        // self.0.str();
        // self.0.struct_();
    }
    #[frb(sync)]
    pub fn arg_sort(
        self,
        #[frb(default = false)] descending: bool,
        #[frb(default = false)] nulls_last: bool,
        #[frb(default = true)] multithreaded: bool,
        #[frb(default = false)] maintain_order: bool,
    ) -> Expr {
        Expr::new(self.0 .0.arg_sort(SortOptions {
            descending,
            nulls_last,
            multithreaded,
            maintain_order,
        }))
    }
    #[frb(sync)]
    pub fn over(self, partiion_by: Vec<Expr>, kind: Option<WindowMapping>) -> Expr {
        Expr::new(
            self.0
                 .0
                .over_with_options(cast_exprs(partiion_by), kind.unwrap_or_default()),
        )
    }
    #[frb(sync)]
    pub fn quantile(self, quantile: Expr, interpol: Option<QuantileInterpolOptions>) -> Expr {
        Expr::new(self.0.quantile(quantile.0 .0, interpol.unwrap_or_default()))
    }
    #[frb(sync)]
    pub fn sort(
        self,
        #[frb(default = false)] descending: bool,
        #[frb(default = false)] nulls_last: bool,
        #[frb(default = true)] multithreaded: bool,
        #[frb(default = false)] maintain_order: bool,
    ) -> Expr {
        Expr::new(self.0 .0.sort_with(SortOptions {
            descending,
            maintain_order,
            multithreaded,
            nulls_last,
        }))
    }
    /// Returns a dot representation of this expression.
    #[frb(sync)]
    pub fn to_dot(&self) -> Result<String> {
        Ok(self.0.to_dot()?)
    }
    #[frb(sync)]
    pub fn variance(self, ddof: u8) -> Expr {
        Expr::new(self.0.var(ddof))
    }
    #[frb(sync)]
    pub fn then(self, value: Expr, otherwise: Expr) -> Expr {
        Expr::new(PExpr::Ternary {
            predicate: Box::new(self.erase()),
            truthy: Box::new(value.erase()),
            falsy: Box::new(otherwise.erase()),
        })
    }
}

fn rolling_options(
    window_size: Option<Duration>,
    min_periods: Option<usize>,
    weights: Option<Vec<f64>>,
    center: bool,
    by: Option<String>,
    closed_window: Option<ClosedWindow>,
) -> RollingOptions {
    let mut options = RollingOptions::default();
    if let Some(window_size) = window_size {
        options.window_size = window_size;
    }
    if let Some(min_periods) = min_periods {
        options.min_periods = min_periods;
    }
    options.weights = weights;
    options.center = center;
    options.by = by;
    options.closed_window = closed_window;
    options
}

#[frb(mirror(WindowMapping))]
pub enum _WindowMapping {
    /// Map the group vlues to the position
    GroupsToRows,
    /// Explode the aggregated list and just do a hstack instead of a join
    /// this requires the groups to be sorted to make any sense
    Explode,
    /// Join the groups as 'List<group_dtype>' to the row positions.
    /// warning: this can be memory intensive
    Join,
}

#[frb(mirror(IsSorted))]
pub enum _IsSorted {
    Ascending,
    Descending,
    Not,
}

#[frb(mirror(ClosedWindow))]
pub enum _ClosedWindow {
    Left,
    Right,
    Both,
    None,
}

/// Supported datatypes in a [DataFrame].
#[frb(mirror(DataType))]
#[non_exhaustive]
pub enum _DataTypeMirror {
    /// Boolean
    Boolean,
    /// Unsigned 8-bit integer
    UInt8,
    /// Unsigned 16-bit integer
    UInt16,
    /// Unsigned 32-bit integer
    UInt32,
    /// Unsigned 64-bit integer
    UInt64,
    /// Signed 8-bit integer
    Int8,
    /// Signed 16-bit integer
    Int16,
    /// Signed 32-bit integer
    Int32,
    /// Signed 64-bit integer, the default [int] on native platforms.
    Int64,
    /// Single-precision floating point number
    Float32,
    /// Double-precision floating point number, aka a [double].
    Float64,
    /// String data
    Utf8,
    /// Raw bytes.
    Binary,
    /// A 32-bit date representing the elapsed time since UNIX epoch (1970-01-01)
    /// in days (32 bits).
    Date,
    /// A 64-bit date representing the elapsed time since UNIX epoch (1970-01-01)
    /// in the given timeunit (64 bits).
    Datetime(TimeUnit, Option<String>),
    /// 64-bit integer representing difference between times in milliseconds or nanoseconds
    Duration(TimeUnit),
    /// A 64-bit time representing the elapsed time since midnight in nanoseconds
    Time,
    /// A typed list.
    List(Box<DataType>),
    // /// A generic type that can be used in a `Series`
    // /// &'static str can be used to determine/set inner type
    // Object(&'static str),
    /// Null value.
    Null,
    /// Structured data.
    Struct(Vec<Field>),
    /// Some logical types we cannot know statically, e.g. Datetime
    Unknown,
}

/// Literal values for use in [Expr]essions.
#[frb(mirror(LiteralValue))]
pub enum _LiteralValueMirror {
    /// Null value.
    Null,
    /// A binary true or false.
    Boolean(bool),
    /// A UTF8 encoded string type.
    Utf8(String),
    /// A raw binary array
    Binary(Vec<u8>),
    /// An unsigned 8-bit integer number.
    UInt8(u8),
    /// An unsigned 16-bit integer number.
    UInt16(u16),
    /// An unsigned 32-bit integer number.
    UInt32(u32),
    /// An unsigned 64-bit integer number.
    UInt64(u64),
    /// An 8-bit integer number.
    Int8(i8),
    /// A 16-bit integer number.
    Int16(i16),
    /// A 32-bit integer number.
    Int32(i32),
    /// A 64-bit integer number.
    Int64(i64),
    /// A 32-bit floating point number.
    Float32(f32),
    /// A 64-bit floating point number.
    Float64(f64),
    /// A range between integers.
    Range {
        /// The starting value of the range.
        low: i64,
        /// The ending value of the range.
        high: i64,
        /// The datatype of this range's ends.
        data_type: DataType,
    },
    // #[cfg(all(feature = "temporal", feature = "dtype-datetime"))]
    /// Datetimes.
    DateTime(i64, TimeUnit, Option<String>),
    /// Durations.
    Duration(i64, TimeUnit),
    // Series(SpecialEq<Series>),
    Date(i32),
    Time(i64),
}

/// Operators for binary operations between [Expr]essions.
#[frb(mirror(Operator))]
pub enum _OperatorMirror {
    /// ==
    Eq,
    EqValidity,
    /// !=
    NotEq,
    NotEqValidity,
    /// <
    Lt,
    /// <=
    LtEq,
    /// >
    Gt,
    /// >=
    GtEq,
    /// +
    Plus,
    /// -
    Minus,
    /// *
    Multiply,
    /// /
    Divide,
    /// ~/
    TrueDivide,
    /// Divides and floors to the nearest integer.
    FloorDivide,
    /// %
    Modulus,
    /// &&
    And,
    /// ||
    Or,
    /// ^
    Xor,
}

// #[frb(mirror(AggExpr))]
// pub(crate) enum _AggExprMirror {
//     Min {
//         input: Box<Expr>,
//         propagate_nans: bool,
//     },
//     Max {
//         input: Box<Expr>,
//         propagate_nans: bool,
//     },
//     Median(Box<Expr>),
//     NUnique(Box<Expr>),
//     First(Box<Expr>),
//     Last(Box<Expr>),
//     Implode(Box<Expr>),
//     Mean(Box<Expr>),
//     Count(Box<Expr>),
//     Quantile {
//         expr: Box<Expr>,
//         quantile: Box<Expr>,
//         interpol: QuantileInterpolOptions,
//     },
//     Sum(Box<Expr>),
//     AggGroups(Box<Expr>),
//     Std(Box<Expr>, u8),
//     Var(Box<Expr>, u8),
// }

/// Fields in a struct.
#[frb(mirror(Field))]
pub struct _FieldMirror {
    /// The field's name.
    pub name: String,
    /// The field's data type.
    pub dtype: DataType,
}

#[frb(mirror(QuantileInterpolOptions))]
pub(crate) enum _QuantileInterpolOptionsMirror {
    Nearest,
    Lower,
    Higher,
    Midpoint,
    Linear,
}

/// Options for including a row count column.
#[frb(mirror(RowCount))]
pub struct _RowCountMirror {
    /// Name of the new column.
    pub name: String,
    /// The value from which to start counting.
    pub offset: u32,
}

/// Options for CSV encoding.
#[frb(mirror(CsvEncoding))]
pub enum _CsvEncodingMirror {
    /// Utf8 encoding
    Utf8,
    /// Utf8 encoding and unknown bytes are replaced with �
    LossyUtf8,
}

/// Options for filling null values.
#[frb(mirror(NullValues))]
pub enum _NullValuesMirror {
    /// A single value that's used for all columns
    AllColumnsSingle(String),
    /// Multiple values that are used for all columns
    AllColumns(Vec<String>),
    /// Tuples that map column names to null value of that column
    Named(Vec<(String, String)>), // TODO
}

/// Options for excluding columns.
#[frb(mirror(Excluded))]
pub enum _ExcludedMirror {
    /// By name
    Name(/* Arc<str> */ String),
    /// By type
    Dtype(DataType),
}

/// Options for joining.
#[frb(mirror(JoinType))]
pub enum _JoinTypeMirror {
    /// Left outer join.
    Left,
    /// Inner join.
    Inner,
    /// Full outer join.
    Outer,
    // #[cfg(feature = "asof_join")]
    // AsOf(AsOfOptions),
    /// Cartesian (cross-product) join.
    Cross,
    /// [Semijoin](https://en.wikipedia.org/wiki/Relational_algebra#Semijoin_(%E2%8B%89_and_%E2%8B%8A)).
    Semi,
    /// [Antijoin](https://en.wikipedia.org/wiki/Relational_algebra#Antijoin_(%E2%96%B7)).
    Anti,
}

/// Options for keeping unique values.
#[frb(mirror(UniqueKeepStrategy))]
pub enum _UniqueKeepStrategyMirror {
    /// TODO: Doc
    First,
    /// TODO: Doc
    Last,
    None,
    Any,
}
