use anyhow::Result;
use flutter_rust_bridge::*;
use polars::io::SerReader;
use polars::lazy::frame::LazyFileListReader;

use super::prelude::*;
pub(crate) use super::prelude::{
    CsvEncoding, JoinType, NullValues, QuantileInterpolOptions, TimeUnit,
};

use std::fs::File;
use std::path::Path;

pub use super::df::{DataFrame, LazyFrame};
pub use super::expr::DataType;
pub use super::series::Schema;
pub(crate) use polars::io::RowCount;

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
    let dtypes_slice = match dtypes_slice {
        Some(dtypes) => Some(dtypes.into_iter().map(Into::into).collect::<Vec<_>>()),
        None => None,
    };
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
    Named(Vec<(String, String)>),
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
