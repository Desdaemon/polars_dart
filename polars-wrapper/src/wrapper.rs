#![warn(missing_docs)]
#![allow(clippy::too_many_arguments)]

use anyhow::{anyhow, Result};
use chrono::prelude::*;
use flutter_rust_bridge::*;
pub use polars::io::RowCount;
pub use polars::prelude::*;
pub use polars::{frame::row::Row, lazy::dsl::Expr};
use std::fs::File;
use std::path::Path;
pub use std::sync::RwLock;

#[macro_use]
mod utils;
use utils::*;

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
/// final df = DataFrame.of(bridge: pl);
/// assert(df.isEmpty());
/// ```
///
/// ## Wrapping a `List<Series>`
///
/// A `DataFrame` is built upon a `List<Series>` where the [Series] have the same length.
///
/// ```dart
/// final s1 = Series.ofStrings(
///     bridge: pl, name: "Fruit",
///     values: ["Apple", "Apple", "Pear"]);
/// final s2 = Series.ofStrings(
///     bridge: pl, name: "Color",
///     values: ["Red", "Yellow", "Green"]);
/// final df = DataFrame.of(bridge: pl, series: [s1, s2]);
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
/// final df = DataFrame.of(bridge: pl, series: [
///     Series.ofStrings(bridge: pl, name: "Fruit", values: ["Apple", "Apple", "Pear"]),
///     Series.ofStrings(bridge: pl, name: "Color", values: ["Red", "Yellow", "Green"]),
/// ]);
///
/// assert(await df[0].asStrings(), ["Apple", "Apple", "Pear"]);
/// assert(await df[1].asStrings(), ["Red", "Yellow", "Green"]);
/// ```
///
/// ## By a [Series] name
///
/// ```dart
/// final df = DataFrame.of(bridge: pl, series: [
///     Series.ofStrings(bridge: pl, name: "Fruit", values: ["Apple", "Apple", "Pear"]),
///     Series.ofStrings(bridge: pl, name: "Color", values: ["Red", "Yellow", "Green"]),
/// ]);
///
/// assert(await df["Fruit"].asStrings(), ["Apple", "Apple", "Pear"]);
/// assert(await df["Color"].asStrings(), ["Red", "Yellow", "Green"]);
/// ```
pub struct DataFrame(
    /// @nodoc
    pub RustOpaque<RwLock<PDataFrame>>,
);
impl DataFrame {
    #[inline]
    fn new(df: PDataFrame) -> Self {
        DataFrame(RustOpaque::new(RwLock::new(df)))
    }
}

pub(crate) type PLazyFrame = polars::prelude::LazyFrame;
/// Lazily-evaluated version of a [DataFrame].
pub struct LazyFrame(
    /// @nodoc
    pub RustOpaque<RwLock<PLazyFrame>>,
);

impl LazyFrame {
    #[inline]
    fn new(df: PLazyFrame) -> Self {
        Self(RustOpaque::new(RwLock::new(df)))
    }
}

pub(crate) type PSeries = polars::prelude::Series;

/// Represents a sequence of values of uniform type.
pub struct Series(
    /// @nodoc
    pub RustOpaque<RwLock<PSeries>>,
);
impl Series {
    #[inline]
    fn new(series: PSeries) -> Self {
        Series(RustOpaque::new(RwLock::new(series)))
    }
}

pub(crate) type PLazyGroupBy = polars::prelude::LazyGroupBy;
/// A wrapper for group-by opereations on a [LazyFrame].
pub struct LazyGroupBy(
    /// @nodoc
    pub RustOpaque<RwLock<PLazyGroupBy>>,
);

impl LazyGroupBy {
    #[inline]
    fn new(groupby: PLazyGroupBy) -> Self {
        LazyGroupBy(RustOpaque::new(RwLock::new(groupby)))
    }
}

pub(crate) type PSchema = polars::prelude::Schema;
/// Schemas to specify datatypes and optimize operations.
pub struct Schema(
    /// @nodoc  
    pub RustOpaque<RwLock<PSchema>>,
);

impl Schema {
    #[inline]
    fn new(schema: PSchema) -> Self {
        Schema(RustOpaque::new(RwLock::new(schema)))
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
    dtypes: Option<Schema>,
    dtypes_slice: Option<Vec<DataType>>,
    has_header: Option<bool>,
    columns: Option<Vec<String>>,
    delimiter: Option<char>,
    comment_char: Option<char>,
    eol_char: Option<char>,
    chunk_size: Option<usize>,
    sample_size: Option<usize>,
    row_count: Option<RowCount>,
    encoding: Option<CsvEncoding>,
    n_rows: Option<usize>,
    n_threads: Option<usize>,
    null_values: Option<NullValues>,
    projection: Option<Vec<u32>>,
    #[frb(default = "'\"'")] quote_char: Option<char>,
    #[frb(default = 0)] skip_rows: usize,
    #[frb(default = 0)] skip_rows_after_header: usize,
    #[frb(default = false)] ignore_parser_errors: bool,
    #[frb(default = false)] rechunk: bool,
    #[frb(default = true)] parse_dates: bool,
    #[frb(default = false)] low_memory: bool,
) -> Result<DataFrame> {
    let mut reader = CsvReader::from_path(path)?
        .with_columns(columns)
        .with_dtypes_slice(dtypes_slice.as_deref())
        .with_n_rows(n_rows)
        .with_ignore_parser_errors(ignore_parser_errors)
        .with_rechunk(rechunk)
        .with_null_values(null_values)
        .with_comment_char(comment_char.map(|comment| comment as _))
        .with_quote_char(quote_char.map(|quote| quote as _))
        .with_projection(projection.map(|proj| proj.into_iter().map(|idx| idx as _).collect()))
        .with_n_threads(n_threads)
        .with_parse_dates(parse_dates)
        .with_skip_rows(skip_rows)
        .with_skip_rows_after_header(skip_rows_after_header)
        .low_memory(low_memory)
        .with_row_count(row_count);
    if let Some(has_header) = has_header {
        reader = reader.has_header(has_header)
    }
    if let Some(delimiter) = delimiter {
        reader = reader.with_delimiter(delimiter as _);
    }
    if let Some(eol) = eol_char {
        reader = reader.with_end_of_line_char(eol as _);
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
    if let Some(dtypes) = dtypes {
        get!(dtypes, dtypes, read_csv);
        Ok(DataFrame::new(reader.with_dtypes(Some(&dtypes)).finish()?))
    } else {
        Ok(DataFrame::new(reader.finish()?))
    }
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
    delimiter: Option<char>,
    comment_char: Option<char>,
    eol_char: Option<char>,
    #[frb(default = "'\"'")] quote_char: Option<char>,
    #[frb(default = 0)] skip_rows: usize,
    #[frb(default = 0)] skip_rows_after_header: usize,
    row_count: Option<RowCount>,
    encoding: Option<CsvEncoding>,
    n_rows: Option<usize>,
    null_values: Option<NullValues>,
    #[frb(default = false)] ignore_parser_errors: bool,
    #[frb(default = false)] rechunk: bool,
    #[frb(default = true)] parse_dates: bool,
    #[frb(default = 100)] infer_schema_length: Option<usize>,
    #[frb(default = false)] cache: bool,
) -> Result<LazyFrame> {
    let mut reader = LazyCsvReader::new(path)
        .with_n_rows(n_rows)
        .with_ignore_parser_errors(ignore_parser_errors)
        .with_rechunk(rechunk)
        .with_null_values(null_values)
        .with_comment_char(comment_char.map(|comment| comment as _))
        .with_quote_char(quote_char.map(|quote| quote as _))
        .with_parse_dates(parse_dates)
        .with_skip_rows(skip_rows)
        .with_skip_rows_after_header(skip_rows_after_header)
        .with_infer_schema_length(infer_schema_length)
        .with_cache(cache)
        .with_row_count(row_count);
    if let Some(has_header) = has_header {
        reader = reader.has_header(has_header)
    }
    if let Some(delimiter) = delimiter {
        reader = reader.with_delimiter(delimiter as _);
    }
    if let Some(eol) = eol_char {
        reader = reader.with_end_of_line_char(eol as _);
    }
    if let Some(enc) = encoding {
        reader = reader.with_encoding(enc);
    }
    if let Some(dtype) = dtype_overwrite {
        get!(dtype, dtype, scan_csv);
        Ok(LazyFrame::new(
            reader.with_dtype_overwrite(Some(&dtype)).finish()?,
        ))
    } else {
        Ok(LazyFrame::new(reader.finish()?))
    }
}

/// Reads a [.json](https://en.wikipedia.org/wiki/JSON) file into a [DataFrame].
pub fn read_json(
    path: String,
    schema: Option<Schema>,
    batch_size: Option<usize>,
    projection: Option<Vec<String>>,
) -> Result<DataFrame> {
    let path = Path::new(&path);
    let file = File::open(path)?;
    let mut reader = JsonReader::new(file).with_projection(projection);
    if let Some(batch) = batch_size {
        reader = reader.with_batch_size(batch);
    }
    if let Some(schema) = schema {
        get!(schema, schema, read_json);
        reader = reader.with_schema(&schema);
    }

    Ok(DataFrame::new(reader.finish()?))
}

#[frb(mirror(TimeUnit))]
pub(crate) enum _TimeUnit {
    Nanoseconds,
    Microseconds,
    Milliseconds,
}

impl DataFrame {
    /// Returns a new, empty dataframe.
    pub fn of(series: Option<Vec<Series>>) -> Result<SyncReturn<DataFrame>> {
        Ok(SyncReturn(DataFrame::new(match series {
            Some(series) => PDataFrame::new(
                series
                    .into_iter()
                    .map(|series| -> Result<_> {
                        Ok(series
                            .0
                            .try_unwrap()
                            .map_err(|err| anyhow!("Failed to acquire lock for Series ({err:?})"))?
                            .into_inner()?)
                    })
                    .collect::<Result<Vec<_>>>()?,
            )?,
            None => Default::default(),
        })))
    }
    /// Iterate through this dataframe's rows.
    ///
    /// Use [parseRow] to retrieve the canonical values for these rows.
    pub fn iter(&self, sink: StreamSink<Vec<DartAbi>>) -> Result<()> {
        get!(my, self, DataFrame::iter);
        let mut buf = make_row(my.width());
        for idx in 0..my.height() {
            my.get_row_amortized(idx, &mut buf)?;
            let row = core::mem::take(&mut buf.0);
            let ok = sink.add(row.into_iter().map(any_value_to_dart).collect());
            if !ok {
                break;
            }
            buf = make_row(my.width());
        }
        sink.close();
        Ok(())
    }
    /// Select a single column by name.
    pub fn column(&self, column: String) -> Result<SyncReturn<Series>> {
        get!(my, self, DataFrame::column);
        Ok(SyncReturn(Series::new(my.column(&column)?.clone())))
    }
    /// Select multiple columns by name.
    pub fn columns(&self, columns: Vec<String>) -> Result<SyncReturn<Vec<Series>>> {
        get!(my, self, DataFrame::columns);
        Ok(SyncReturn(
            my.columns(columns)?
                .into_iter()
                .cloned()
                .map(Series::new)
                .collect(),
        ))
    }
    /// Select the column at the given index.
    pub fn column_at(&self, index: usize) -> Result<SyncReturn<Series>> {
        get!(my, self, DataFrame::column_at);
        Ok(SyncReturn(Series::new(my[index].clone())))
    }
    /// Dump the contents of this entire dataframe.
    pub fn dump(&self) -> Result<String> {
        get!(my, self, DataFrame::dump);
        Ok(format!("{}", my))
    }
    /// Returns the amount of bytes occupied by this series.
    pub fn estimated_size(&self) -> Result<SyncReturn<usize>> {
        get!(my, self, DataFrame::estimated_size);
        Ok(SyncReturn(my.estimated_size()))
    }
    /// Add a new column at index 0 denoting the row number.
    pub fn with_row_count(&self, name: String, offset: Option<u32>) -> Result<DataFrame> {
        get!(my, self, DataFrame::with_row_count);
        Ok(DataFrame::new(my.with_row_count(&name, offset)?))
    }
    /// Get the names of this dataframe's columns.
    pub fn get_column_names(&self) -> Result<SyncReturn<Vec<String>>> {
        get!(my, self, DataFrame::get_column_names);
        Ok(SyncReturn(my.get_column_names_owned()))
    }
    /// Get all columns of this dataframe.
    pub fn get_columns(&self) -> Result<Vec<Series>> {
        get!(my, self, DataFarme::get_columns);
        Ok(my.get_columns().iter().cloned().map(Series::new).collect())
    }
    /// Returns the width of this dataframe, aka the number of columns.
    pub fn width(&self) -> Result<SyncReturn<usize>> {
        get!(my, self, DataFrame::width);
        Ok(SyncReturn(my.width()))
    }
    /// Returns the height of this dataframe, aka the number of rows.
    pub fn height(&self) -> Result<SyncReturn<usize>> {
        get!(my, self, DataFrame::height);
        Ok(SyncReturn(my.height()))
    }
    /// Returns whether this dataframe has no rows.
    pub fn is_empty(&self) -> Result<SyncReturn<bool>> {
        get!(my, self, DataFrame::is_empty);
        Ok(SyncReturn(my.is_empty()))
    }
    /// Sample [n] datapoints from this dataframe.
    #[frb]
    pub fn sample(
        &self,
        n: usize,
        #[frb(default = false)] with_replacement: bool,
        #[frb(default = false)] shuffle: bool,
        seed: Option<u64>,
    ) -> Result<DataFrame> {
        get!(my, self, DataFrame::sample);
        Ok(DataFrame::new(my.sample_n(
            n,
            with_replacement,
            shuffle,
            seed,
        )?))
    }
    /// Makes a new dataframe with the specified columns from this dataframe.
    pub fn select(&self, columns: Vec<String>) -> Result<SyncReturn<DataFrame>> {
        get!(my, self, DataFrame::select);
        Ok(SyncReturn(DataFrame::new(my.select(columns)?)))
    }
    /// Returns the first few rows of this dataframe.
    pub fn head(&self, length: Option<usize>) -> Result<SyncReturn<DataFrame>> {
        get!(my, self, DataFrame::head);
        Ok(SyncReturn(DataFrame::new(my.head(length))))
    }
    /// Returns the last few rows of this dataframe.
    pub fn tail(&self, length: Option<usize>) -> Result<SyncReturn<DataFrame>> {
        get!(my, self, DataFrame::tail);
        Ok(SyncReturn(DataFrame::new(my.tail(length))))
    }
    /// Output statistics about this dataframe.
    pub fn describe(&self, percentiles: Option<Vec<f64>>) -> Result<DataFrame> {
        get!(my, self, DataFrame::describe);
        Ok(DataFrame::new(my.describe(percentiles.as_deref())))
    }
    /// Drops a column by name, producing a new dataframe.
    pub fn drop(&self, column: String) -> Result<SyncReturn<DataFrame>> {
        get!(my, self, DataFrame::drop);
        Ok(SyncReturn(DataFrame::new(PDataFrame::drop(&my, &column)?)))
    }
    /// Drops a column in-place and returns it.
    pub fn drop_in_place(&self, column: String) -> Result<SyncReturn<Series>> {
        get!(mut my, self, DataFrame::drop_in_place);
        Ok(SyncReturn(Series::new(my.drop_in_place(&column)?)))
    }
    /// Returns a dataframe with columns from this dataframe in reverse order.
    pub fn reverse(&self) -> Result<SyncReturn<DataFrame>> {
        get!(my, self, DataFrame::reverse);
        Ok(SyncReturn(DataFrame::new(my.reverse())))
    }
    /// Returns the height and width of this dataframe.
    pub fn shape(&self) -> Result<SyncReturn<Shape>> {
        get!(my, self, DataFrame::shape);
        let (height, width) = my.shape();
        Ok(SyncReturn(Shape { height, width }))
    }
    /// Aggregate the columns to their maximum values.
    pub fn max(&self) -> Result<DataFrame> {
        get!(my, self, DataFrame::max);
        Ok(DataFrame::new(my.max()))
    }
    /// Get a row of data from this dataframe.
    ///
    /// This method may be slow due to conversions between different data formats.
    pub fn get_row(&self, index: usize) -> Result<Vec<DartAbi>> {
        get!(my, self, DataFrame::row);
        let row = my.get_row(index)?;
        Ok(row.0.into_iter().map(any_value_to_dart).collect())
    }
    // pub fn sort_in_place(&self) -> Result<()> {
    //     unlock!(mut my, self, DataFrame::sort_in_place);
    //     my.sort_in_place()
    // }
    /// Returns a [LazyFrame] to which operations can be applied lazily.
    /// As opposed to [LazyFrame], [DataFrame] by default applies its operations eagerly.
    ///
    /// This operation will fail if this dataframe is currently being shared, unless
    /// `allowCopy` is true in which case this dataframe will be copied.
    #[frb]
    pub fn lazy(
        self,
        #[frb(default = false)] allow_copy: bool,
        projection_pushdown: Option<bool>,
        predicate_pushdown: Option<bool>,
        type_coercion: Option<bool>,
        simplify_expressions: Option<bool>,
        slice_pushdown: Option<bool>,
        streaming: Option<bool>,
    ) -> Result<SyncReturn<LazyFrame>> {
        let my = match self.0.try_unwrap() {
            Ok(my) => my.into_inner()?,
            Err(lock) if allow_copy => {
                let my = lock
                    .read()
                    .map_err(|err| anyhow!("Could not acquire lock ({err})"))?;
                my.clone()
            }
            Err(_) => return Err(anyhow!("Cannot make a shared dataframe lazy.")),
        };

        let mut lazy = my.lazy();
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

        Ok(SyncReturn(LazyFrame::new(lazy)))
    }
}

impl LazyFrame {
    /// Select (and rename) columns from the query.
    pub fn select(self, exprs: Vec<Expr>) -> Result<SyncReturn<LazyFrame>> {
        let my = self.unwrap(false)?;
        Ok(SyncReturn(LazyFrame::new(my.select(exprs))))
    }
    /// Filter by the specified predicate expression.
    pub fn filter(self, pred: Expr) -> Result<SyncReturn<LazyFrame>> {
        let my = self.unwrap(false)?;
        Ok(SyncReturn(LazyFrame::new(my.filter(pred))))
    }
    /// Define conditions by which to group and aggregate rows.
    #[frb]
    pub fn groupby(
        self,
        exprs: Vec<Expr>,
        #[frb(default = false)] stable: bool,
    ) -> Result<SyncReturn<LazyGroupBy>> {
        let my = self.unwrap(false)?;
        Ok(SyncReturn(LazyGroupBy::new(if stable {
            my.groupby_stable(exprs)
        } else {
            my.groupby(exprs)
        })))
    }
    /// Reverse the order of this dataframe's columns.
    pub fn reverse(self) -> Result<SyncReturn<LazyFrame>> {
        let my = self.unwrap(false)?;
        Ok(SyncReturn(LazyFrame::new(my.reverse())))
    }
    /// Add a column to this dataframe.
    pub fn with_column(self, expr: Expr) -> Result<SyncReturn<LazyFrame>> {
        let my = self.unwrap(false)?;
        Ok(SyncReturn(LazyFrame::new(my.with_column(expr))))
    }
    /// Add columns to this dataframe.
    pub fn with_columns(self, expr: Vec<Expr>) -> Result<SyncReturn<LazyFrame>> {
        let my = self.unwrap(false)?;
        Ok(SyncReturn(LazyFrame::new(my.with_columns(expr))))
    }
    /// Caches the results into a new [LazyFrame].
    ///
    /// This should be used to prevent computations running multiple times.
    pub fn cache(self) -> Result<SyncReturn<LazyFrame>> {
        let my = self.unwrap(false)?;
        Ok(SyncReturn(LazyFrame::new(my.cache())))
    }
    /// Executes all lazy operations and collects results into a [DataFrame].
    pub fn collect(self) -> Result<DataFrame> {
        let my = self.unwrap(false)?;
        Ok(DataFrame::new(my.collect()?))
    }
    /// Creates the [Cartesian product](https://en.wikipedia.org/wiki/Cartesian_product) from both frames,
    /// preserving the order of this frame's keys.
    #[frb]
    pub fn cross_join(self, other: LazyFrame) -> Result<SyncReturn<LazyFrame>> {
        let my = self.unwrap(true)?;
        let rhs = other.unwrap(true)?;
        Ok(SyncReturn(LazyFrame::new(my.cross_join(rhs))))
    }
    /// Performs a [left outer join](https://en.wikipedia.org/wiki/Join_(SQL)#Left_outer_join) with [other].
    #[frb]
    pub fn left_join(
        self,
        other: LazyFrame,
        left_on: Expr,
        right_on: Expr,
    ) -> Result<SyncReturn<LazyFrame>> {
        let my = self.unwrap(true)?;
        let rhs = other.unwrap(true)?;
        Ok(SyncReturn(LazyFrame::new(
            my.left_join(rhs, left_on, right_on),
        )))
    }
    /// Performs a [full outer join](https://en.wikipedia.org/wiki/Join_(SQL)#Full_outer_join) with [other].
    #[frb]
    pub fn outer_join(
        self,
        other: LazyFrame,
        left_on: Expr,
        right_on: Expr,
    ) -> Result<SyncReturn<LazyFrame>> {
        let my = self.unwrap(true)?;
        let rhs = other.unwrap(true)?;
        Ok(SyncReturn(LazyFrame::new(
            my.outer_join(rhs, left_on, right_on),
        )))
    }
    /// Performs an [inner join](https://en.wikipedia.org/wiki/Join_(SQL)#Inner_join_and_NULL_values) with [other].
    #[frb]
    pub fn inner_join(
        self,
        other: LazyFrame,
        left_on: Expr,
        right_on: Expr,
    ) -> Result<SyncReturn<LazyFrame>> {
        let my = self.unwrap(true)?;
        let rhs = other.unwrap(true)?;
        Ok(SyncReturn(LazyFrame::new(
            my.inner_join(rhs, left_on, right_on),
        )))
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
    #[frb]
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
    ) -> Result<SyncReturn<LazyFrame>> {
        let my = self.unwrap(true)?;
        let rhs = other.unwrap(true)?;
        let mut b = my
            .join_builder()
            .with(rhs)
            .how(how)
            .suffix(suffix)
            .allow_parallel(allow_parallel)
            .force_parallel(force_parallel);
        if let Some(on) = on {
            b = b.on(on);
        }
        if let Some(left) = left_on {
            b = b.left_on(left);
        }
        if let Some(right) = right_on {
            b = b.right_on(right);
        }
        Ok(SyncReturn(LazyFrame::new(b.finish())))
    }
    /// Aggregate all columns as their max values.
    pub fn max(self) -> Result<SyncReturn<LazyFrame>> {
        let my = self.unwrap(false)?;
        Ok(SyncReturn(LazyFrame::new(my.max())))
    }
    /// Aggregate all columns as their min values.
    pub fn min(self) -> Result<SyncReturn<LazyFrame>> {
        let my = self.unwrap(false)?;
        Ok(SyncReturn(LazyFrame::new(my.min())))
    }
    /// Aggregate all columns as their sums.
    pub fn sum(self) -> Result<SyncReturn<LazyFrame>> {
        let my = self.unwrap(false)?;
        Ok(SyncReturn(LazyFrame::new(my.sum())))
    }
    /// Aggregate all columns as their means.
    pub fn mean(self) -> Result<SyncReturn<LazyFrame>> {
        let my = self.unwrap(false)?;
        Ok(SyncReturn(LazyFrame::new(my.mean())))
    }
    /// Aggregate all columns as their medians.
    pub fn median(self) -> Result<SyncReturn<LazyFrame>> {
        let my = self.unwrap(false)?;
        Ok(SyncReturn(LazyFrame::new(my.median())))
    }
    /// Aggregate all columns as their quantiles.
    pub fn quantile(
        self,
        quantile: Expr,
        interpol: QuantileInterpolOptions,
    ) -> Result<SyncReturn<LazyFrame>> {
        let my = self.unwrap(false)?;
        Ok(SyncReturn(LazyFrame::new(my.quantile(quantile, interpol))))
    }
    /// Aggregate all columns as their standard deviances.
    pub fn std(self, ddof: u8) -> Result<SyncReturn<LazyFrame>> {
        let my = self.unwrap(false)?;
        Ok(SyncReturn(LazyFrame::new(my.std(ddof))))
    }
    /// Aggregate all columns as their variances.
    pub fn variance(self, ddof: u8) -> Result<SyncReturn<LazyFrame>> {
        let my = self.unwrap(false)?;
        Ok(SyncReturn(LazyFrame::new(my.var(ddof))))
    }
    /// Explode each column.
    pub fn explode(self, columns: Vec<Expr>) -> Result<SyncReturn<LazyFrame>> {
        let my = self.unwrap(false)?;
        Ok(SyncReturn(LazyFrame::new(my.explode(columns))))
    }
    /// Keep unique rows without maintaining order.
    pub fn unique(
        self,
        subset: Option<Vec<String>>,
        keep_strategy: UniqueKeepStrategy,
    ) -> Result<SyncReturn<LazyFrame>> {
        let my = self.unwrap(false)?;
        Ok(SyncReturn(LazyFrame::new(my.unique(subset, keep_strategy))))
    }
    /// Drop null rows.
    ///
    /// Same as `frame.filter(col('*').isNotNull)`.
    pub fn drop_nulls(self, subset: Option<Vec<Expr>>) -> Result<SyncReturn<LazyFrame>> {
        let my = self.unwrap(false)?;
        Ok(SyncReturn(LazyFrame::new(my.drop_nulls(subset))))
    }
    /// Slice the frame.
    pub fn slice(self, offset: i64, len: u32) -> Result<SyncReturn<LazyFrame>> {
        let my = self.unwrap(false)?;
        Ok(SyncReturn(LazyFrame::new(my.slice(offset, len))))
    }
    /// Get the first row.
    pub fn first(self) -> Result<SyncReturn<LazyFrame>> {
        let my = self.unwrap(false)?;
        Ok(SyncReturn(LazyFrame::new(my.first())))
    }
    /// Get the last row.
    pub fn last(self) -> Result<SyncReturn<LazyFrame>> {
        let my = self.unwrap(false)?;
        Ok(SyncReturn(LazyFrame::new(my.last())))
    }
    /// Get the last [n] rows.
    pub fn tail(self, n: u32) -> Result<SyncReturn<LazyFrame>> {
        let my = self.unwrap(false)?;
        Ok(SyncReturn(LazyFrame::new(my.tail(n))))
    }
    /// Melt this dataframe from the wide format to the long format.
    pub fn melt(
        self,
        id_vars: Vec<String>,
        value_vars: Vec<String>,
        variable_name: Option<String>,
        value_name: Option<String>,
    ) -> Result<SyncReturn<LazyFrame>> {
        let my = self.unwrap(false)?;
        Ok(SyncReturn(LazyFrame::new(my.melt(MeltArgs {
            id_vars,
            value_vars,
            variable_name,
            value_name,
        }))))
    }
    /// Limit this dataframe to the first [n] rows.
    ///
    /// To avoid scanning the rows, use [fetch].
    pub fn limit(self, n: u32) -> Result<SyncReturn<LazyFrame>> {
        let my = self.unwrap(false)?;
        Ok(SyncReturn(LazyFrame::new(my.tail(n))))
    }
    /// Similar to [collect], but overrides the number of rows read by each operation.
    ///
    /// The final row count is not guaranteed to be equal [nRows].
    pub fn fetch(self, n_rows: usize) -> Result<DataFrame> {
        let my = self.unwrap(false)?;
        Ok(DataFrame::new(my.fetch(n_rows)?))
    }
    /// Add a new column at index 0 denoting the row number.
    pub fn with_row_count(
        self,
        name: String,
        offset: Option<u32>,
    ) -> Result<SyncReturn<LazyFrame>> {
        let my = self.unwrap(false)?;
        Ok(SyncReturn(LazyFrame::new(my.with_row_count(&name, offset))))
    }
    #[inline]
    fn unwrap(self, allow_copy: bool) -> Result<PLazyFrame> {
        Ok(match self.0.try_unwrap() {
            Ok(my) => my.into_inner()?,
            Err(lock) if allow_copy => {
                let my = lock
                    .read()
                    .map_err(|err| anyhow!("Could not acquire lock ({err})"))?;
                my.clone()
            }
            Err(_) => return Err(anyhow!("Cannot use this operation on a shared LazyFrame.")),
        })
    }
}

impl Series {
    /// Create a new series of strings.
    pub fn of_strings(name: String, values: Option<Vec<String>>) -> SyncReturn<Series> {
        SyncReturn(Series::new(if let Some(values) = values {
            PSeries::new(&name, values)
        } else {
            PSeries::new_empty(&name, &DataType::Utf8)
        }))
    }
    /// Create a new series of 32-bit wide integers.
    pub fn of_i32(name: String, values: Option<Vec<i32>>) -> SyncReturn<Series> {
        SyncReturn(Series::new(if let Some(values) = values {
            PSeries::new(&name, values)
        } else {
            PSeries::new_empty(&name, &DataType::Int32)
        }))
    }
    /// Create a new series of 64-bit wide integers.
    pub fn of_i64(name: String, values: Option<Vec<i64>>) -> SyncReturn<Series> {
        SyncReturn(Series::new(if let Some(values) = values {
            PSeries::new(&name, values)
        } else {
            PSeries::new_empty(&name, &DataType::Int64)
        }))
    }
    /// Create a new series of [Duration]s.
    #[frb]
    pub fn of_durations(
        name: String,
        values: Option<Vec<chrono::Duration>>,
        #[frb(default = "TimeUnit.Milliseconds")] unit: TimeUnit,
    ) -> SyncReturn<Series> {
        SyncReturn(Series::new(if let Some(values) = values {
            PSeries::new(&name, values)
        } else {
            PSeries::new_empty(&name, &DataType::Duration(unit))
        }))
    }
    /// Create a new series of doubles.
    pub fn of_f64(name: String, values: Option<Vec<f64>>) -> SyncReturn<Series> {
        SyncReturn(Series::new(if let Some(values) = values {
            PSeries::new(&name, values)
        } else {
            PSeries::new_empty(&name, &DataType::Float64)
        }))
    }
    // TODO(Desdaemon): implement Vec<bool> upstream
    // pub fn of_bools(name: String, values: Option<Vec<bool>>) -> PSeries {
    //     PSeries::new(if let Some(values) = values {
    //         Series::new(&name, values)
    //     } else {
    //         Series::new_empty(&name, &DataType::Boolean)
    //     })
    // }

    /// Adds the contents of [other] onto this series.
    ///
    /// Throws if [other] is self.
    pub fn append(&self, other: Series) -> Result<()> {
        get!(rhs, other, Series::append);
        get!(mut lhs, self, Series::append);
        lhs.append(&rhs)?;
        Ok(())
    }
    /// If this series is a UTF-8 series, returns its Dart representation.
    pub fn as_strings(&self) -> Result<Vec<Option<String>>> {
        get!(my, self, Series::as_strings);
        Ok(my
            .utf8()?
            .into_iter()
            .map(|e| e.map(ToOwned::to_owned))
            .collect())
    }
    /// If this series is a 32-bit wide integer series, returns its Dart representation.
    pub fn as_i32(&self) -> Result<Vec<Option<i32>>> {
        get!(my, self, Series::as_i32);
        Ok(my.i32()?.into_iter().collect())
    }
    /// If this series is a double series, returns its Dart representation.
    pub fn as_f64(&self) -> Result<Vec<Option<f64>>> {
        get!(my, self, Series::as_f64);
        Ok(my.f64()?.into_iter().collect())
    }
    /// If this series contains [Duration]s, returns its Dart representation.
    pub fn as_durations(&self) -> Result<Vec<Option<chrono::Duration>>> {
        get!(my, self, Series::as_duration);

        let ds = my.duration()?;
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
    pub fn as_naive_datetime(&self) -> Result<Vec<Option<NaiveDateTime>>> {
        get!(my, self, Series::as_naive_datetime);
        Ok(my.datetime()?.as_datetime_iter().collect())
    }
    /// If this series contains [DateTime]s, returns its Dart representation.
    ///
    /// If a timezone is defined by this series, the datetimes will be converted to UTC.
    /// Otherwise, the datetimes are assumed to be in UTC.
    #[inline]
    pub fn as_utc_datetime(&self) -> Result<Vec<Option<DateTime<Utc>>>> {
        self.as_datetime_impl(Utc)
    }
    /// If this series contains [DateTime]s, returns its Dart representation.
    ///
    /// If a timezone is defined by this series, the datetimes will be converted to the local timezone.
    /// Otherwise, the datetimes are assumed to be in the local timezone.
    #[inline]
    pub fn as_local_datetime(&self) -> Result<Vec<Option<DateTime<Local>>>> {
        self.as_datetime_impl(Local)
    }
    fn as_datetime_impl<Tz: chrono::TimeZone>(
        &self,
        target: Tz,
    ) -> Result<Vec<Option<DateTime<Tz>>>> {
        get!(my, self, Series::as_datetime_impl);

        let dt = my.datetime()?;
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
    /// Returns a new series with each value's absolute value.
    pub fn abs(&self) -> Result<Series> {
        get!(my, self, Series::abs);
        Ok(Series::new(my.abs()?))
    }
    /// Returns a new sorted series.
    #[frb]
    pub fn sort(&self, #[frb(default = false)] reverse: bool) -> Result<Series> {
        get!(my, self, Series::sort);
        Ok(Series::new(my.sort(reverse)))
    }
    /// Returns a new shuffled series.
    pub fn shuffle(&self, seed: Option<u64>) -> Result<Series> {
        get!(my, self, Series::shuffle);
        Ok(Series::new(my.shuffle(seed)))
    }
    /// Sums all non-null rows in this series to produce a result.
    ///
    /// Returns null if the series only contains null values.
    pub fn sum(&self) -> Result<Option<f64>> {
        get!(my, self, Series::sum);
        Ok(my.sum())
    }
    /// Returns the sum of this series' values as a single-element series.
    pub fn sum_as_series(&self) -> Result<Series> {
        get!(my, self, Series::sum_as_series);
        Ok(Series::new(my.sum_as_series()))
    }
    /// Returns the minimum value of this series' values.
    ///
    /// Returns null if one of the values are also null.
    pub fn min(&self) -> Result<Option<f64>> {
        get!(my, self, Series::min);
        Ok(my.min())
    }
    /// Returns the maximum value of this series' values.
    ///
    /// Returns null if one of the values are also null.
    pub fn max(&self) -> Result<Option<f64>> {
        get!(my, self, Series::max);
        Ok(my.max())
    }
    /// Expands a series of lists into rows of values, or strings into rows of characters.
    pub fn explode(&self) -> Result<Series> {
        get!(my, self, Series::explode);
        Ok(Series::new(my.explode()?))
    }
    /// TODO: docs
    pub fn explode_by_offsets(&self, offsets: Vec<i64>) -> Result<Series> {
        get!(my, self, Series::explode_by_offsets);
        Ok(Series::new(my.explode_by_offsets(&offsets)))
    }
    /// Calculates the cumulative max at each element.
    #[frb]
    pub fn cummax(&self, #[frb(default = false)] reverse: bool) -> Result<Series> {
        get!(my, self, Series::cummax);
        Ok(Series::new(my.cummax(reverse)))
    }
    /// Calculates the cumulative min at each element.
    #[frb]
    pub fn cummin(&self, #[frb(default = false)] reverse: bool) -> Result<Series> {
        get!(my, self, Series::cummin);
        Ok(Series::new(my.cummin(reverse)))
    }
    /// Calculates the cumulative product at each element.
    #[frb]
    pub fn cumprod(&self, #[frb(default = false)] reverse: bool) -> Result<Series> {
        get!(my, self, Series::cumprod);
        Ok(Series::new(my.cumprod(reverse)))
    }
    /// Calculates the cumulative sum at each element.
    #[frb]
    pub fn cumsum(&self, #[frb(default = false)] reverse: bool) -> Result<Series> {
        get!(my, self, Series::cumsum);
        Ok(Series::new(my.cumsum(reverse)))
    }
    /// Calculates the product of each element in the series and returns it in a single-element series.
    pub fn product(&self) -> Result<Series> {
        get!(my, self, Series::product);
        Ok(Series::new(my.product()))
    }
    /// Get the value at [index] as a string.
    pub fn get_string(&self, index: usize) -> Result<SyncReturn<Option<String>>> {
        get!(my, self, Series::get_string);
        Ok(SyncReturn(
            my.str_value(index).ok().map(std::borrow::Cow::into_owned),
        ))
    }
    /// Get the value at [index] as a double.
    pub fn get(&self, index: usize) -> Result<SyncReturn<Option<f64>>> {
        get!(my, self, Series::get);
        Ok(SyncReturn(
            my.get(index)
                .ok()
                .and_then(|value| value.try_extract().ok()),
        ))
    }
    /// Get the first few values of this series.
    pub fn head(&self, length: Option<usize>) -> Result<SyncReturn<Series>> {
        get!(my, self, Series::head);
        Ok(SyncReturn(Series::new(my.head(length))))
    }
    /// Get the last few values of this series.
    pub fn tail(&self, length: Option<usize>) -> Result<SyncReturn<Series>> {
        get!(my, self, Series::tail);
        Ok(SyncReturn(Series::new(my.tail(length))))
    }
    /// Calculates the mean (average) of this series.
    pub fn mean(&self) -> Result<Option<f64>> {
        get!(my, self, Series::mean);
        Ok(my.mean())
    }
    /// Calculates the [median](https://en.wikipedia.org/wiki/Median) of this series.
    pub fn median(&self) -> Result<Option<f64>> {
        get!(my, self, Series::median);
        Ok(my.median())
    }
    /// Calculates and wraps this series' mean as a single-element series.
    pub fn mean_as_series(&self) -> Result<Series> {
        get!(my, self, Series::mean_as_series);
        Ok(Series::new(my.mean_as_series()))
    }
    /// Calculates and wraps this series' median as a single-element series.
    pub fn median_as_series(&self) -> Result<Series> {
        get!(my, self, Series::median_as_series);
        Ok(Series::new(my.median_as_series()))
    }
    /// Returns the amount of bytes occupied by this series.
    pub fn estimated_size(&self) -> Result<SyncReturn<usize>> {
        get!(my, self, Series::estimated_size);
        Ok(SyncReturn(my.estimated_size()))
    }
    /// Returns a new series with elements from this series added to [other]'s element-wise.
    pub fn add_to(&self, other: Series) -> Result<SyncReturn<Series>> {
        get!(rhs, other, Series::add_to);
        get!(my, self, Series::add_to);
        Ok(SyncReturn(Series::new(my.add_to(&rhs)?)))
    }
    /// Returns a new series with elements from this series subtracted from [other]'s element-wise.
    pub fn subtract(&self, other: Series) -> Result<SyncReturn<Series>> {
        get!(rhs, other, Series::subtract);
        get!(my, self, Series::subtract);
        Ok(SyncReturn(Series::new(my.subtract(&rhs)?)))
    }
    /// Returns a new series with elements from this series multiplied with [other]'s element-wise.
    pub fn multiply(&self, other: Series) -> Result<SyncReturn<Series>> {
        get!(rhs, other, Series::multiply);
        get!(my, self, Series::multiply);
        Ok(SyncReturn(Series::new(my.multiply(&rhs)?)))
    }
    /// Returns a new series with elements from this series divided by [other]'s element-wise.
    pub fn divide(&self, other: Series) -> Result<SyncReturn<Series>> {
        get!(rhs, other, Series::divide);
        get!(my, self, Series::divide);
        Ok(SyncReturn(Series::new(my.divide(&rhs)?)))
    }
    /// Returns a new series with the [remainder](https://en.wikipedia.org/wiki/Remainder)
    /// between this series' and [other]'s elements.
    pub fn remainder(&self, other: Series) -> Result<SyncReturn<Series>> {
        get!(rhs, other, Series::remainder);
        get!(my, self, Series::remainder);
        Ok(SyncReturn(Series::new(my.remainder(&rhs)?)))
    }
    /// Returns whether this is a series of booleans.
    pub fn is_bool(&self) -> Result<SyncReturn<bool>> {
        get!(my, self, Series::is_bool);
        Ok(SyncReturn(matches!(my.dtype(), DataType::Boolean)))
    }
    /// Returns whether this is a series of UTF-8 strings.
    pub fn is_utf8(&self) -> Result<SyncReturn<bool>> {
        get!(my, self, Series::is_utf8);
        Ok(SyncReturn(matches!(my.dtype(), DataType::Utf8)))
    }
    /// Returns whether this is a series of numeric values.
    pub fn is_numeric(&self) -> Result<SyncReturn<bool>> {
        get!(my, self, Series::is_numeric);
        Ok(SyncReturn(my.dtype().is_numeric()))
    }
    /// Returns whether this is a series of [DateTime] or [Duration]s.
    pub fn is_temporal(&self) -> Result<SyncReturn<bool>> {
        get!(my, self, Series::is_temporal);
        Ok(SyncReturn(my.dtype().is_temporal()))
    }
    /// Dump the contents of this entire series.
    pub fn dump(&self) -> Result<String> {
        get!(my, self, Series::dump);
        Ok(format!("{}", my))
    }
    /// Rename this series to [name] in-place.
    pub fn rename(&self, name: String) -> Result<SyncReturn<()>> {
        get!(mut my, self, Series::rename);
        my.rename(&name);
        Ok(SyncReturn(()))
    }
    /// Returns the unique values of this series.
    ///
    /// If `stable` is true, extra work is done to maintain the original order of elements.
    #[frb]
    pub fn unique(&self, #[frb(default = false)] stable: bool) -> Result<Series> {
        get!(my, self, Series::unique);
        // my.series_equal
        Ok(Series::new(if stable {
            my.unique_stable()?
        } else {
            my.unique()?
        }))
    }
    /// Returns whether this series is identical to [other].
    ///
    /// if `ignoreNull` is true, null values are considered to be equal.
    #[frb]
    pub fn equal(&self, other: Series, #[frb(default = false)] ignore_null: bool) -> Result<bool> {
        get!(rhs, other, Series::equal);
        get!(my, self, Series::equal);
        Ok(if ignore_null {
            my.series_equal_missing(&rhs)
        } else {
            my.series_equal(&rhs)
        })
    }
    /// Creates a new series with the specified dimensions.
    pub fn reshape(&self, dims: Vec<i64>) -> Result<Series> {
        get!(my, self, Series::reshape);
        Ok(Series::new(my.reshape(&dims)?))
    }
    /// Calculates the standard deviation of this series with the specified degree of freedom.
    pub fn std_as_series(&self, ddof: u8) -> Result<Series> {
        get!(my, self, Series::std_as_series);
        Ok(Series::new(my.std_as_series(ddof)))
    }
}

impl LazyGroupBy {
    /// Group by and aggregate.
    ///
    /// Select a column with [col] and choose an aggregation. If you want to aggregate all columns
    /// use `col("*")`.
    pub fn agg(self, exprs: Vec<Expr>) -> Result<SyncReturn<LazyFrame>> {
        let my = self.unwrap()?;
        Ok(SyncReturn(LazyFrame::new(my.agg(exprs))))
    }
    /// Return the first [n] rows of each group.
    pub fn head(self, n: Option<usize>) -> Result<SyncReturn<LazyFrame>> {
        let my = self.unwrap()?;
        Ok(SyncReturn(LazyFrame::new(my.head(n))))
    }
    /// Return the last [n] rows of each group.
    pub fn tail(self, n: Option<usize>) -> Result<SyncReturn<LazyFrame>> {
        let my = self.unwrap()?;
        Ok(SyncReturn(LazyFrame::new(my.tail(n))))
    }
    #[inline]
    fn unwrap(self) -> Result<PLazyGroupBy> {
        Ok(self
            .0
            .try_unwrap()
            .map_err(|_| anyhow!("failed to unwrap LazyGroupBy"))?
            .into_inner()?)
    }
}

impl Schema {
    /// Create a schema from a list of [Field]s.
    pub fn of(fields: Vec<Field>) -> SyncReturn<Schema> {
        SyncReturn(Schema::new(fields.into_iter().into()))
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
#[frb(mirror(Expr))]
pub enum _Expr {
    /// Give this expression a new name.
    Alias(Box<Expr>, Arc<str>),
    /// Get the column matching this name.
    Column(Arc<str>),
    /// Get all columns matching these names.
    Columns(Vec<String>),
    /// Get columns of these datatypes.
    DtypeColumn(Vec<DataType>),
    /// Represents a literal value, i.e. strings, numebrs and so on.
    Literal(LiteralValue),
    /// A binary expression.
    BinaryExpr {
        /// The left-hand side column.
        left: Box<Expr>,
        /// The operator, e.g. ==, >, <.
        op: Operator,
        /// The right-hand side column.
        right: Box<Expr>,
    },
    /// Cast a column into one of another type.
    Cast {
        /// The column to be cast.
        expr: Box<Expr>,
        /// The new desired datatype.
        data_type: DataType,
        /// Whether incompatible values should be coerced.
        strict: bool,
    },
    /// Sort the column.
    Sort {
        /// The column to be sorted.
        expr: Box<Expr>,
        /// Options for sorting.
        options: SortOptions,
    },
    /// Take a column.
    Take {
        /// The column from which to take.
        expr: Box<Expr>,
        /// The index to take at.
        idx: Box<Expr>,
    },
    // SortBy {
    //     expr: Box<Expr>,
    //     by: Vec<Expr>,
    //     reverse: Vec<bool>, // TODO
    // },
    /// Aggregating options.
    Agg(AggExpr),
    /// A ternary operation.
    Ternary {
        /// The condition for this ternary.
        predicate: Box<Expr>,
        /// If `predicate` is true, evaluate to this.
        truthy: Box<Expr>,
        /// If `predicate` is false, evaluate to this.
        falsy: Box<Expr>,
    },
    // Function {
    //     /// function arguments
    //     input: Vec<Expr>,
    //     /// function to apply
    //     function: FunctionExpr,
    //     options: FunctionOptions,
    // },
    /// Expand columns of strings or lists.
    Explode(Box<Expr>),
    /// Filter columns' values.
    Filter {
        /// The column to be filtered.
        input: Box<Expr>,
        /// The conditions by which this column should be filtered.
        by: Box<Expr>,
    },
    // /// See postgres window functions
    // Window {
    //     /// Also has the input. i.e. avg("foo")
    //     function: Box<Expr>,
    //     partition_by: Vec<Expr>,
    //     order_by: Option<Box<Expr>>,
    //     options: WindowOptions,
    // },
    /// Matches any value.
    Wildcard,
    /// Take slices of series.
    Slice {
        /// The column to take slices of.
        input: Box<Expr>,
        /// Length is not yet known so we accept negative offsets
        offset: Box<Expr>,
        /// How long the slice should be.
        length: Box<Expr>,
    },
    /// Can be used in a select statement to exclude a column from selection
    Exclude(Box<Expr>, Vec<Excluded>),
    /// Set root name as Alias
    KeepName(Box<Expr>),
    /// Special case that does not need columns
    Count,
    /// Take the nth column in the `DataFrame`
    Nth(i64),
    // skipped fields must be last otherwise serde fails in pickle
    // RenameAlias {
    //     function: SpecialEq<Arc<dyn RenameAliasFn>>,
    //     expr: Box<Expr>,
    // },
    // AnonymousFunction {
    //     /// function arguments
    //     input: Vec<Expr>,
    //     /// function to apply
    //     function: SpecialEq<Arc<dyn SeriesUdf>>,
    //     /// output dtype of the function
    //     output_type: GetOutput,
    //     options: FunctionOptions,
    // },
}

/// Options for sorting
#[frb(mirror(SortOptions))]
pub struct _SortOptions {
    /// Whether it should be sorted from smallest or largest.
    pub descending: bool,
    /// Whether nulls get pushed to the top or bottom.
    pub nulls_last: bool,
}

/// Supported datatypes in a [DataFrame].
#[frb(mirror(DataType))]
pub enum _DataType {
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
    // #[cfg(feature = "dtype-binary")]
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
    /// A generic type that can be used in a `Series`
    /// &'static str can be used to determine/set inner type
    // Object(&'static str),
    // Null,
    // #[cfg(feature = "dtype-categorical")]
    // The RevMapping has the internal state.
    // This is ignored with casts, comparisons, hashing etc.
    // Categorical(Option<Arc<RevMapping>>),
    // #[cfg(feature = "dtype-struct")]
    /// Structured data.
    Struct(Vec<Field>),
    /// Some logical types we cannot know statically, e.g. Datetime
    Unknown,
}

/// Literal values for use in [Expr]essions.
#[frb(mirror(LiteralValue))]
pub enum _LiteralValue {
    // Null,
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
    DateTime(NaiveDateTime, TimeUnit),
    /// Durations.
    Duration(chrono::Duration, TimeUnit),
    // Series(SpecialEq<Series>),
}

#[frb(mirror(Operator))]
pub(crate) enum _Operator {
    Eq,
    NotEq,
    Lt,
    LtEq,
    Gt,
    GtEq,
    Plus,
    Minus,
    Multiply,
    Divide,
    TrueDivide,
    FloorDivide,
    Modulus,
    And,
    Or,
    Xor,
}

#[frb(mirror(AggExpr))]
pub(crate) enum _AggExpr {
    Min {
        input: Box<Expr>,
        propagate_nans: bool,
    },
    Max {
        input: Box<Expr>,
        propagate_nans: bool,
    },
    Median(Box<Expr>),
    NUnique(Box<Expr>),
    First(Box<Expr>),
    Last(Box<Expr>),
    Mean(Box<Expr>),
    List(Box<Expr>),
    Count(Box<Expr>),
    Quantile {
        expr: Box<Expr>,
        quantile: Box<Expr>,
        interpol: QuantileInterpolOptions,
    },
    Sum(Box<Expr>),
    AggGroups(Box<Expr>),
    Std(Box<Expr>, u8),
    // Var(Box<Expr>, u8),
}

/// Fields in a struct.
#[frb(mirror(Field))]
pub struct _Field {
    /// The field's name.
    pub name: String,
    /// The field's data type.
    pub dtype: DataType,
}

#[frb(mirror(QuantileInterpolOptions))]
pub(crate) enum _QuantileInterpolOptions {
    Nearest,
    Lower,
    Higher,
    Midpoint,
    Linear,
}

/// Options for including a row count column.
#[frb(mirror(RowCount))]
pub struct _RowCount {
    /// Name of the new column.
    pub name: String,
    /// The value from which to start counting.
    pub offset: u32,
}

/// Options for CSV encoding.
#[frb(mirror(CsvEncoding))]
pub enum _CsvEncoding {
    /// Utf8 encoding
    Utf8,
    /// Utf8 encoding and unknown bytes are replaced with �
    LossyUtf8,
}

/// Options for filling null values.
#[frb(mirror(NullValues))]
pub enum _NullValues {
    /// A single value that's used for all columns
    AllColumnsSingle(String),
    /// Multiple values that are used for all columns
    AllColumns(Vec<String>),
    // /// Tuples that map column names to null value of that column
    // Named(Vec<(String, String)>), // TODO
}

/// Options for excluding columns.
#[frb(mirror(Excluded))]
pub enum _Excluded {
    /// By name
    Name(Arc<str>),
    /// By type
    Dtype(DataType),
}

/// Options for joining.
#[frb(mirror(JoinType))]
pub enum _JoinType {
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
pub enum _UniqueKeepStrategy {
    /// TODO: Doc
    First,
    /// TODO: Doc
    Last,
}
