use std::{
    borrow::Cow,
    ops::{Add, Div, Mul},
    panic::AssertUnwindSafe,
};

use super::expr::{DataType, Field, PDataType};
use anyhow::{anyhow, Result};
use chrono::{DateTime, Local, NaiveDateTime, Utc};
use flutter_rust_bridge::{frb, DartDynamic};

use crate::bridge::StreamSink;

pub(crate) use super::{
    df::{DataFrame, LazyFrame},
    expr::{cast_exprs, Expr},
    util::any_value_to_dart,
};

pub(crate) type PSeries = polars::prelude::Series;
use super::prelude::*;

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
pub struct Series(pub(crate) AssertUnwindSafe<PSeries>);

impl Series {
    #[inline]
    pub(crate) fn new(series: PSeries) -> Self {
        Series(AssertUnwindSafe(series))
    }
}

pub(crate) type PLazyGroupBy = polars::prelude::LazyGroupBy;

/// A wrapper for group-by opereations on a [LazyFrame].
#[frb(opaque)]
pub struct LazyGroupBy(AssertUnwindSafe<PLazyGroupBy>);

impl LazyGroupBy {
    #[inline]
    pub(crate) fn new(groupby: PLazyGroupBy) -> Self {
        LazyGroupBy(AssertUnwindSafe(groupby))
    }
}

pub(crate) type PSchema = polars::prelude::Schema;

/// Schemas to specify datatypes and optimize operations.
#[frb(opaque)]
pub struct Schema(pub(crate) AssertUnwindSafe<PSchema>);

impl Schema {
    #[inline]
    pub(crate) fn new(schema: PSchema) -> Self {
        Schema(AssertUnwindSafe(schema))
    }
}

impl Series {
    /// Create a new series of strings.
    #[frb(sync)]
    pub fn of_strings(name: String, values: Option<Vec<Option<String>>>) -> Series {
        Series::new(if let Some(values) = values {
            PSeries::new(&name, values)
        } else {
            PSeries::new_empty(&name, &PDataType::Utf8)
        })
    }
    /// Create a new series of 32-bit wide integers.
    #[frb(sync)]
    pub fn of_i32(name: String, values: Option<Vec<Option<i32>>>) -> Series {
        Series::new(if let Some(values) = values {
            PSeries::new(&name, values)
        } else {
            PSeries::new_empty(&name, &PDataType::Int32)
        })
    }
    /// Create a new series of 64-bit wide integers.
    #[frb(sync)]
    pub fn of_ints(name: String, values: Option<Vec<Option<i64>>>) -> Series {
        Series::new(if let Some(values) = values {
            PSeries::new(&name, values)
        } else {
            PSeries::new_empty(&name, &PDataType::Int64)
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
            PSeries::new_empty(&name, &PDataType::Duration(unit))
        })
    }
    /// Create a new series of doubles.
    #[frb(sync)]
    pub fn of_doubles(name: String, values: Option<Vec<Option<f64>>>) -> Series {
        Series::new(if let Some(values) = values {
            PSeries::new(&name, values)
        } else {
            PSeries::new_empty(&name, &PDataType::Float64)
        })
    }
    /// Create a new series of booleans.
    #[frb(sync)]
    pub fn of_bools(name: String, values: Option<Vec<bool>>) -> Series {
        Series::new(if let Some(values) = values {
            PSeries::new(&name, values)
        } else {
            PSeries::new_empty(&name, &PDataType::Boolean)
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
            self.0.strict_cast(&dtype.into())?
        } else {
            self.0.cast(&dtype.into())?
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
            self.0.strict_cast(&PDataType::Int64)
        } else {
            self.0.cast(&PDataType::Int64)
        }?;
        Ok(my.i64().unwrap().into_iter().collect())
    }
    /// If compatible, returns a representation of this series as integers.
    #[frb]
    pub fn as_doubles(&self, #[frb(default = true)] strict: bool) -> Result<Vec<Option<f64>>> {
        let my = if strict {
            self.0.strict_cast(&PDataType::Float64)
        } else {
            self.0.cast(&PDataType::Float64)
        }?;
        Ok(my
            .cast(&PDataType::Float64)?
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
        matches!(self.0.dtype(), PDataType::Boolean)
    }
    /// Returns whether this is a series of UTF-8 strings.
    #[frb(sync)]
    pub fn is_utf8(&self) -> Result<bool> {
        Ok(matches!(self.0.dtype(), PDataType::Utf8))
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
    pub fn apply_scalar(self, op: Operator, value: f64) -> Result<Series> {
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
            Plus => self.0 .0.add(value).into_series(),
            Minus => self.0 .0.add(-value).into_series(),
            Multiply => self.0 .0.mul(value).into_series(),
            Divide => self.0 .0.div(value).into_series(),
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
