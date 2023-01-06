#![warn(missing_docs)]

use anyhow::{anyhow, Result};
use chrono::prelude::*;
use flutter_rust_bridge::*;
pub use polars::prelude::*;
pub use std::sync::RwLock;
// use std::{fs::File, path::Path};

macro_rules! unlock {
    ($bind:ident, $self:expr, $method:path) => {
        let $bind = $self
            .0
            .read()
            .map_err(|err| anyhow::anyhow!(concat!(stringify!($method), " failed ({})"), err))?;
    };
    (mut $bind:ident, $self:expr, $method:path) => {
        let mut $bind = $self
            .0
            .try_write()
            .map_err(|err| anyhow::anyhow!(concat!(stringify!($method), " failed ({})"), err))?;
    };
}

pub(crate) type PDataFrame = polars::prelude::DataFrame;
pub struct DataFrame(pub RustOpaque<RwLock<PDataFrame>>);
impl DataFrame {
    #[inline]
    fn new(df: PDataFrame) -> Self {
        DataFrame(RustOpaque::new(RwLock::new(df)))
    }
}

pub(crate) type PSeries = polars::prelude::Series;
pub struct Series(pub RustOpaque<RwLock<PSeries>>);
impl Series {
    #[inline]
    fn new(series: PSeries) -> Self {
        Series(RustOpaque::new(RwLock::new(series)))
    }
}

/// Reads a .csv file into a [DataFrame].
pub fn read_csv(
    path: String,
    has_header: Option<bool>,
    // columns: Option<Vec<String>>,
    delimiter: Option<u8>,
    skip_rows: Option<usize>,
    skip_rows_after_header: Option<usize>,
    chunk_size: Option<usize>,
) -> Result<DataFrame> {
    let mut reader = CsvReader::from_path(path)?;
    if let Some(has_header) = has_header {
        reader = reader.has_header(has_header)
    }
    if let Some(delimiter) = delimiter {
        reader = reader.with_delimiter(delimiter)
    }
    if let Some(skip) = skip_rows {
        reader = reader.with_skip_rows(skip)
    }
    if let Some(skip) = skip_rows_after_header {
        reader = reader.with_skip_rows_after_header(skip)
    }
    if let Some(size) = chunk_size {
        reader = reader.with_chunk_size(size)
    }
    Ok(DataFrame::new(reader.finish()?))
}

// TODO(Desdaemon): 'json' doesn't support WASM yet
/// Reads a .json file into a [DataFrame].
// pub fn read_json(path: String) -> Result<DataFrame> {
//     let path = resolve_homedir(Path::new(&path));
//     let file = File::open(path)?;
//     Ok(DataFrame::new(JsonReader::new(file).finish()?))
// }

#[frb(mirror(TimeUnit))]
pub(crate) enum _TimeUnit {
    Nanoseconds,
    Microseconds,
    Milliseconds,
}

impl DataFrame {
    /// Select a single column by name.
    pub fn column(&self, column: String) -> Result<SyncReturn<Series>> {
        unlock!(my, self, DataFrame::column);
        Ok(SyncReturn(Series::new(my.column(&column)?.clone())))
    }

    /// Select multiple columns by name.
    pub fn columns(&self, columns: Vec<String>) -> Result<SyncReturn<Vec<Series>>> {
        unlock!(my, self, DataFrame::columns);
        Ok(SyncReturn(
            my.columns(columns)?
                .into_iter()
                .cloned()
                .map(Series::new)
                .collect(),
        ))
    }

    /// Dump the contents of this entire dataframe.
    pub fn dump(&self) -> Result<SyncReturn<String>> {
        unlock!(my, self, DataFrame::dump);
        Ok(SyncReturn(format!("{}", my)))
    }
}

impl Series {
    // TODO(Desdaemon): Doesn't work on WASM yet.
    /// Create a new series of strings.
    // pub fn of_strings(name: String, values: Option<Vec<String>>) -> SyncReturn<Series> {
    //     SyncReturn(Series::new(if let Some(values) = values {
    //         PSeries::new(&name, values)
    //     } else {
    //         PSeries::new_empty(&name, &DataType::Utf8)
    //     }))
    // }
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
    // pub fn of_durations(
    //     name: String,
    //     values: Option<Vec<chrono::Duration>>,
    //     unit: Option<TimeUnit>,
    // ) -> SyncReturn<Series> {
    //     SyncReturn(Series::new(if let Some(values) = values {
    //         PSeries::new(&name, values)
    //     } else {
    //         PSeries::new_empty(
    //             &name,
    //             &DataType::Duration(unit.unwrap_or(TimeUnit::Nanoseconds)),
    //         )
    //     }))
    // }
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
        unlock!(rhs, other, Series::append);
        unlock!(mut lhs, self, Series::append);
        lhs.append(&rhs)?;
        Ok(())
    }
    /// If this series is a UTF-8 series, returns its Dart representation.
    pub fn as_strings(&self) -> Result<Vec<Option<String>>> {
        unlock!(my, self, Series::as_strings);
        Ok(my
            .utf8()?
            .into_iter()
            .map(|e| e.map(ToOwned::to_owned))
            .collect())
    }
    /// If this series is a 32-bit wide integer series, returns its Dart representation.
    pub fn as_i32(&self) -> Result<Vec<Option<i32>>> {
        unlock!(my, self, Series::as_i32);
        Ok(my.i32()?.into_iter().collect())
    }
    /// If this series is a double series, returns its Dart representation.
    pub fn as_f64(&self) -> Result<Vec<Option<f64>>> {
        unlock!(my, self, Series::as_f64);
        Ok(my.f64()?.into_iter().collect())
    }
    /// If this series contains [Duration]s, returns its Dart representation.
    pub fn as_durations(&self) -> Result<Vec<Option<chrono::Duration>>> {
        unlock!(my, self, Series::as_duration);

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
        unlock!(my, self, Series::as_naive_datetime);
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
        unlock!(my, self, Series::as_datetime_impl);

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
        unlock!(my, self, Series::abs);
        Ok(Series::new(my.abs()?))
    }
    /// Returns a new sorted series.
    #[frb]
    pub fn sort(&self, #[frb(default = false)] reverse: bool) -> Result<Series> {
        unlock!(my, self, Series::sort);
        Ok(Series::new(my.sort(reverse)))
    }
    /// Returns a new shuffled series.
    pub fn shuffle(&self, seed: Option<u64>) -> Result<Series> {
        unlock!(my, self, Series::shuffle);
        Ok(Series::new(my.shuffle(seed)))
    }
    /// Sums all non-null rows in this series to produce a result.
    ///
    /// Returns null if the series only contains null values.
    pub fn sum(&self) -> Result<Option<f64>> {
        unlock!(my, self, Series::sum);
        Ok(my.sum())
    }
    /// Returns the minimum value of this series' values.
    ///
    /// Returns null if one of the values are also null.
    pub fn min(&self) -> Result<Option<f64>> {
        unlock!(my, self, Series::min);
        Ok(my.min())
    }
    /// Returns the maximum value of this series' values.
    ///
    /// Returns null if one of the values are also null.
    pub fn max(&self) -> Result<Option<f64>> {
        unlock!(my, self, Series::max);
        Ok(my.max())
    }
    /// Expands a series of lists into rows of values, or strings into rows of characters.
    pub fn explode(&self) -> Result<Series> {
        unlock!(my, self, Series::explode);
        Ok(Series::new(my.explode()?))
    }
    pub fn explode_by_offsets(&self, offsets: Vec<i64>) -> Result<Series> {
        unlock!(my, self, Series::explode_by_offsets);
        Ok(Series::new(my.explode_by_offsets(&offsets)))
    }
    /// Calculates the cumulative max at each element.
    #[frb]
    pub fn cummax(&self, #[frb(default = false)] reverse: bool) -> Result<Series> {
        unlock!(my, self, Series::cummax);
        Ok(Series::new(my.cummax(reverse)))
    }
    /// Calculates the cumulative min at each element.
    #[frb]
    pub fn cummin(&self, #[frb(default = false)] reverse: bool) -> Result<Series> {
        unlock!(my, self, Series::cummin);
        Ok(Series::new(my.cummin(reverse)))
    }
    /// Calculates the cumulative product at each element.
    #[frb]
    pub fn cumprod(&self, #[frb(default = false)] reverse: bool) -> Result<Series> {
        unlock!(my, self, Series::cumprod);
        Ok(Series::new(my.cumprod(reverse)))
    }
    /// Calculates the cumulative sum at each element.
    #[frb]
    pub fn cumsum(&self, #[frb(default = false)] reverse: bool) -> Result<Series> {
        unlock!(my, self, Series::cumsum);
        Ok(Series::new(my.cumsum(reverse)))
    }
    /// Calculates the product of each element in the series and returns it in a single-element series.
    pub fn product(&self) -> Result<Series> {
        unlock!(my, self, Series::product);
        Ok(Series::new(my.product()))
    }
    /// Get the value at [index] as a string.
    pub fn get_string(&self, index: usize) -> Result<SyncReturn<Option<String>>> {
        unlock!(my, self, Series::get_string);
        Ok(SyncReturn(
            my.str_value(index).ok().map(std::borrow::Cow::into_owned),
        ))
    }
    /// Get the value at [index] as a double.
    pub fn get(&self, index: usize) -> Result<SyncReturn<Option<f64>>> {
        unlock!(my, self, Series::get);
        Ok(SyncReturn(
            my.get(index)
                .ok()
                .and_then(|value| value.try_extract().ok()),
        ))
    }
    /// Get the first few values of this series.
    pub fn head(&self, length: Option<usize>) -> Result<SyncReturn<Series>> {
        unlock!(my, self, Series::head);
        Ok(SyncReturn(Series::new(my.head(length))))
    }
    /// Get the last few values of this series.
    pub fn tail(&self, length: Option<usize>) -> Result<SyncReturn<Series>> {
        unlock!(my, self, Series::tail);
        Ok(SyncReturn(Series::new(my.tail(length))))
    }
    /// Calculates the mean (average) of this series.
    pub fn mean(&self) -> Result<Option<f64>> {
        unlock!(my, self, Series::mean);
        Ok(my.mean())
    }
    /// Calculates the [median](https://en.wikipedia.org/wiki/Median) of this series.
    pub fn median(&self) -> Result<Option<f64>> {
        unlock!(my, self, Series::median);
        Ok(my.median())
    }
    /// Calculates and wraps this series' mean as a single-element series.
    pub fn mean_as_series(&self) -> Result<Series> {
        unlock!(my, self, Series::mean_as_series);
        Ok(Series::new(my.mean_as_series()))
    }
    /// Calculates and wraps this series' median as a single-element series.
    pub fn median_as_series(&self) -> Result<Series> {
        unlock!(my, self, Series::median_as_series);
        Ok(Series::new(my.median_as_series()))
    }
    /// Returns the amount of bytes occupied by this series.
    pub fn estimated_size(&self) -> Result<SyncReturn<usize>> {
        unlock!(my, self, Series::estimated_size);
        Ok(SyncReturn(my.estimated_size()))
    }
    /// Returns a new series with elements from this series added to [other]'s element-wise.
    pub fn add_to(&self, other: Series) -> Result<SyncReturn<Series>> {
        unlock!(rhs, other, Series::add_to);
        unlock!(my, self, Series::add_to);
        Ok(SyncReturn(Series::new(my.add_to(&rhs)?)))
    }
    /// Returns a new series with elements from this series subtracted from [other]'s element-wise.
    pub fn subtract(&self, other: Series) -> Result<SyncReturn<Series>> {
        unlock!(rhs, other, Series::subtract);
        unlock!(my, self, Series::subtract);
        Ok(SyncReturn(Series::new(my.subtract(&rhs)?)))
    }
    /// Returns a new series with elements from this series multiplied with [other]'s element-wise.
    pub fn multiply(&self, other: Series) -> Result<SyncReturn<Series>> {
        unlock!(rhs, other, Series::multiply);
        unlock!(my, self, Series::multiply);
        Ok(SyncReturn(Series::new(my.multiply(&rhs)?)))
    }
    /// Returns a new series with elements from this series divided by [other]'s element-wise.
    pub fn divide(&self, other: Series) -> Result<SyncReturn<Series>> {
        unlock!(rhs, other, Series::divide);
        unlock!(my, self, Series::divide);
        Ok(SyncReturn(Series::new(my.divide(&rhs)?)))
    }
    /// Returns a new series with the [remainder](https://en.wikipedia.org/wiki/Remainder)
    /// between this series' and [other]'s elements.
    pub fn remainder(&self, other: Series) -> Result<SyncReturn<Series>> {
        unlock!(rhs, other, Series::remainder);
        unlock!(my, self, Series::remainder);
        Ok(SyncReturn(Series::new(my.remainder(&rhs)?)))
    }
    /// Returns whether this is a series of booleans.
    pub fn is_bool(&self) -> Result<SyncReturn<bool>> {
        unlock!(my, self, Series::is_bool);
        Ok(SyncReturn(matches!(my.dtype(), DataType::Boolean)))
    }
    /// Returns whether this is a series of UTF-8 strings.
    pub fn is_utf8(&self) -> Result<SyncReturn<bool>> {
        unlock!(my, self, Series::is_utf8);
        Ok(SyncReturn(matches!(my.dtype(), DataType::Utf8)))
    }
    /// Returns whether this is a series of numeric values.
    pub fn is_numeric(&self) -> Result<SyncReturn<bool>> {
        unlock!(my, self, Series::is_numeric);
        Ok(SyncReturn(my.dtype().is_numeric()))
    }
    /// Returns whether this is a series of [DateTime] or [Duration]s.
    pub fn is_temporal(&self) -> Result<SyncReturn<bool>> {
        unlock!(my, self, Series::is_temporal);
        Ok(SyncReturn(my.dtype().is_temporal()))
    }
    /// Dump the contents of this entire series.
    pub fn dump(&self) -> Result<SyncReturn<String>> {
        unlock!(my, self, Series::dump);
        Ok(SyncReturn(format!("{}", my)))
    }
    /// Rename this series to [name] in-place.
    pub fn rename(&self, name: String) -> Result<SyncReturn<()>> {
        unlock!(mut my, self, Series::rename);
        my.rename(&name);
        Ok(SyncReturn(()))
    }
}
