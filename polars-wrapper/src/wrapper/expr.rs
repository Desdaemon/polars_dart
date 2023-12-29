use anyhow::Result;
use flutter_rust_bridge::*;
pub use polars::lazy::dsl::SpecialEq;
use std::ops::Rem;

use std::ops::{Add, Div, Mul, Sub};
pub use std::panic::AssertUnwindSafe;

use super::prelude::*;
pub(crate) use super::prelude::{ClosedWindow, Operator};
use super::series::PSeries;
pub use chrono::Duration;
pub(crate) use polars::lazy::dsl::WindowMapping;
pub use polars::series::IsSorted;

/// Expressions for use in query and aggregration operations.
#[frb(opaque)]
#[repr(transparent)]
pub struct Expr(pub(crate) AssertUnwindSafe<PExpr>);
pub(crate) type PExpr = polars::lazy::dsl::Expr;

pub(crate) fn cast_exprs(exprs: Vec<Expr>) -> Vec<PExpr> {
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
        #[doc = concat!("TODO: Docs for ", stringify!($fn))]
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
            Expr::new(self.0 .0 .$fn(rolling_options(
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
    Expr::new(PExpr::DtypeColumn(
        types.into_iter().map(Into::into).collect(),
    ))
}

#[frb(sync)]
pub fn nth(idx: i64) -> Expr {
    Expr::new(PExpr::Nth(idx))
}

#[frb(sync)]
pub fn count() -> Expr {
    Expr::new(PExpr::Count)
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
        #[frb(sync)] cast(self, data_type: #[into] DataType) -> Expr;
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
        // TODO: #[frb(sync)] exclude_dtype(self, dtypes: Vec<DataType>) -> Expr;
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
        #[frb(sync)] sqrt(self) -> Expr;
        #[frb(sync)] sub(self, other: #[erase] Expr) -> Expr;
        #[frb(sync)] std(self, ddof: u8) -> Expr;
        #[frb(sync)] strict_cast(self, data_type: #[into] DataType) -> Expr;
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
    fn what(self) {
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
        Expr::new(
            self.0
                 .0
                .quantile(quantile.0 .0, interpol.unwrap_or_default()),
        )
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
        Expr::new(self.0 .0.var(ddof))
    }
    #[frb(sync)]
    pub fn then(self, value: Expr, otherwise: Expr) -> Expr {
        Expr::new(PExpr::Ternary {
            predicate: Box::new(self.erase()),
            truthy: Box::new(value.erase()),
            falsy: Box::new(otherwise.erase()),
        })
    }
    #[frb(sync)]
    pub fn literal(value: LiteralValue) -> Expr {
        Expr::new(PExpr::Literal(value.into()))
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
        options.window_size = match window_size.num_nanoseconds() {
            Some(ns) => polars::prelude::Duration::new(ns),
            None => polars::prelude::Duration::parse(&format!("{}s", window_size.num_seconds())),
        }
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

pub type PDataType = polars::datatypes::DataType;

/// Supported datatypes in a [DataFrame].
pub enum DataType {
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
    /// in the given timeunit (64 bits), with optional timezone.
    Datetime(TimeUnit, Option<String>),
    /// 64-bit integer representing difference between times in milliseconds or nanoseconds
    Duration(TimeUnit),
    /// A 64-bit time representing the elapsed time since midnight in nanoseconds
    Time,
    /// A typed list.
    List(Box<DataType>),
    Struct(Vec<Field>),
    /// Null value.
    Null,
    /// Some logical types we cannot know statically, e.g. Datetime
    Unknown,
}

impl From<DataType> for PDataType {
    fn from(value: DataType) -> Self {
        match value {
            DataType::Boolean => polars::prelude::DataType::Boolean,
            DataType::UInt8 => polars::prelude::DataType::UInt8,
            DataType::UInt16 => polars::prelude::DataType::UInt16,
            DataType::UInt32 => polars::prelude::DataType::UInt32,
            DataType::UInt64 => polars::prelude::DataType::UInt64,
            DataType::Int8 => polars::prelude::DataType::Int8,
            DataType::Int16 => polars::prelude::DataType::Int16,
            DataType::Int32 => polars::prelude::DataType::Int32,
            DataType::Int64 => polars::prelude::DataType::Int64,
            DataType::Float32 => polars::prelude::DataType::Float32,
            DataType::Float64 => polars::prelude::DataType::Float64,
            DataType::Utf8 => polars::prelude::DataType::Utf8,
            DataType::Binary => polars::prelude::DataType::Binary,
            DataType::Date => polars::prelude::DataType::Date,
            DataType::Datetime(time_unit, timezone) => {
                polars::prelude::DataType::Datetime(time_unit.into(), timezone)
            }
            DataType::Duration(time_unit) => polars::prelude::DataType::Duration(time_unit.into()),
            DataType::Time => polars::prelude::DataType::Time,
            DataType::List(data_type) => {
                polars::prelude::DataType::List(Box::new((*data_type).into()))
            }
            DataType::Struct(fields) => {
                polars::prelude::DataType::Struct(fields.into_iter().map(Into::into).collect())
            }
            DataType::Null => polars::prelude::DataType::Null,
            DataType::Unknown => polars::prelude::DataType::Null,
        }
    }
}

impl From<PDataType> for DataType {
    fn from(value: PDataType) -> Self {
        match value {
            polars::prelude::DataType::Boolean => DataType::Boolean,
            polars::prelude::DataType::UInt8 => DataType::UInt8,
            polars::prelude::DataType::UInt16 => DataType::UInt16,
            polars::prelude::DataType::UInt32 => DataType::UInt32,
            polars::prelude::DataType::UInt64 => DataType::UInt64,
            polars::prelude::DataType::Int8 => DataType::Int8,
            polars::prelude::DataType::Int16 => DataType::Int16,
            polars::prelude::DataType::Int32 => DataType::Int32,
            polars::prelude::DataType::Int64 => DataType::Int64,
            polars::prelude::DataType::Float32 => DataType::Float32,
            polars::prelude::DataType::Float64 => DataType::Float64,
            polars::prelude::DataType::Utf8 => DataType::Utf8,
            polars::prelude::DataType::Binary => DataType::Binary,
            polars::prelude::DataType::Date => DataType::Date,
            polars::prelude::DataType::Datetime(time_unit, timezone) => {
                DataType::Datetime(time_unit.into(), timezone)
            }
            polars::prelude::DataType::Duration(time_unit) => DataType::Duration(time_unit.into()),
            polars::prelude::DataType::Time => DataType::Time,
            polars::prelude::DataType::List(data_type) => {
                DataType::List(Box::new((*data_type).into()))
            }
            polars::prelude::DataType::Struct(fields) => {
                DataType::Struct(fields.into_iter().map(Into::into).collect())
            }
            polars::prelude::DataType::Null => DataType::Null,
            PDataType::Unknown => DataType::Unknown,
        }
    }
}

pub struct Field {
    pub name: String,
    pub dtype: DataType,
}

impl From<Field> for polars::datatypes::Field {
    fn from(value: Field) -> Self {
        polars::datatypes::Field::new(&value.name, value.dtype.into())
    }
}

impl From<polars::datatypes::Field> for Field {
    fn from(value: polars::datatypes::Field) -> Self {
        Field {
            name: value.name().to_string(),
            dtype: value.data_type().clone().into(),
        }
    }
}

/// Literal values for use in [Expr]essions.
pub enum LiteralValue {
    /// Null value.
    Null,
    /// A binary true or false.
    Boolean(bool),
    /// A UTF8 encoded string type.
    Utf8(String),
    /// A raw binary array
    Binary(Vec<u8>),
    /// An unsigned 32-bit integer number.
    Uint32(u32),
    /// An unsigned 64-bit integer number.
    Uint64(u64),
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
    /// Datetimes, with optional timezone.
    DateTime(i64, TimeUnit, Option<String>),
    /// Durations.
    Duration(i64, TimeUnit),
    Series(RustOpaque<AssertUnwindSafe<SpecialEq<PSeries>>>),
    Date(i32),
    /// Nanoseconds elapsed since midnight.
    Time(i64),
}

impl From<polars::prelude::LiteralValue> for LiteralValue {
    fn from(value: polars::prelude::LiteralValue) -> Self {
        match value {
            polars::prelude::LiteralValue::Null => LiteralValue::Null,
            polars::prelude::LiteralValue::Boolean(value) => LiteralValue::Boolean(value),
            polars::prelude::LiteralValue::Utf8(value) => LiteralValue::Utf8(value),
            polars::prelude::LiteralValue::Binary(value) => LiteralValue::Binary(value),
            polars::prelude::LiteralValue::UInt32(value) => LiteralValue::Uint32(value),
            polars::prelude::LiteralValue::UInt64(value) => LiteralValue::Uint64(value),
            polars::prelude::LiteralValue::Int32(value) => LiteralValue::Int32(value),
            polars::prelude::LiteralValue::Int64(value) => LiteralValue::Int64(value),
            polars::prelude::LiteralValue::Float32(value) => LiteralValue::Float32(value),
            polars::prelude::LiteralValue::Float64(value) => LiteralValue::Float64(value),
            polars::prelude::LiteralValue::Range {
                low,
                high,
                data_type,
            } => LiteralValue::Range {
                low,
                high,
                data_type: data_type.into(),
            },
            polars::prelude::LiteralValue::Date(value) => LiteralValue::Date(value),
            polars::prelude::LiteralValue::Time(value) => LiteralValue::Time(value),
            polars::prelude::LiteralValue::Duration(value, time_unit) => {
                LiteralValue::Duration(value, time_unit.into())
            }
            polars::prelude::LiteralValue::DateTime(value, time_unit, timezone) => {
                LiteralValue::DateTime(value, time_unit.into(), timezone)
            }
            polars::prelude::LiteralValue::Series(series) => {
                LiteralValue::Series(RustOpaque::new(AssertUnwindSafe(series)))
            }
        }
    }
}

impl From<LiteralValue> for polars::prelude::LiteralValue {
    fn from(value: LiteralValue) -> Self {
        match value {
            LiteralValue::Null => polars::prelude::LiteralValue::Null,
            LiteralValue::Boolean(value) => polars::prelude::LiteralValue::Boolean(value),
            LiteralValue::Utf8(value) => polars::prelude::LiteralValue::Utf8(value),
            LiteralValue::Binary(value) => polars::prelude::LiteralValue::Binary(value),
            LiteralValue::Uint32(value) => polars::prelude::LiteralValue::UInt32(value),
            LiteralValue::Uint64(value) => polars::prelude::LiteralValue::UInt64(value),
            LiteralValue::Int32(value) => polars::prelude::LiteralValue::Int32(value),
            LiteralValue::Int64(value) => polars::prelude::LiteralValue::Int64(value),
            LiteralValue::Float32(value) => polars::prelude::LiteralValue::Float32(value),
            LiteralValue::Float64(value) => polars::prelude::LiteralValue::Float64(value),
            LiteralValue::Range {
                low,
                high,
                data_type,
            } => polars::prelude::LiteralValue::Range {
                low,
                high,
                data_type: data_type.into(),
            },
            LiteralValue::Date(value) => polars::prelude::LiteralValue::Date(value),
            LiteralValue::Time(value) => polars::prelude::LiteralValue::Time(value),
            LiteralValue::Duration(value, time_unit) => {
                polars::prelude::LiteralValue::Duration(value, time_unit.into())
            }
            LiteralValue::DateTime(value, time_unit, timezone) => {
                polars::prelude::LiteralValue::DateTime(value, time_unit.into(), timezone)
            }
            LiteralValue::Series(series) => match series.into_inner() {
                Some(inner) => polars::prelude::LiteralValue::Series(inner.0),
                None => polars::prelude::LiteralValue::Series(SpecialEq::new(PSeries::new_empty(
                    series.name(),
                    series.dtype(),
                ))),
            },
        }
    }
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
