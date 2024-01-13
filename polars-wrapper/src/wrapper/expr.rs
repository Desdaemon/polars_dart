use anyhow::Result;
use flutter_rust_bridge::*;
pub use polars::lazy::dsl::SpecialEq;

use std::ops::Div;
pub use std::panic::AssertUnwindSafe;

use super::prelude::*;
pub(crate) use super::prelude::{ClosedWindow, Operator, SortOptions, WindowType};
use super::series::{PSeries, Series};
use super::util::chrono_to_polars_duration;
pub use chrono::Duration;
pub(crate) use polars::lazy::dsl::WindowMapping;
pub use polars::series::IsSorted;

/// Expressions for use in query and aggregration operations.
#[frb]
pub enum Expr {
    Alias(Box<Expr>, String),
    Column(String),
    Columns(Vec<String>),
    DtypeColumn(Vec<DataType>),
    Literal(LiteralValue),
    BinaryExpr {
        left: Box<Expr>,
        op: Operator,
        right: Box<Expr>,
    },
    Cast {
        expr: Box<Expr>,
        data_type: DataType,
        strict: bool,
    },
    Sort {
        expr: Box<Expr>,
        #[frb(default = "const SortOptions()")]
        options: SortOptions,
    },
    Gather {
        expr: Box<Expr>,
        idx: Box<Expr>,
        returns_scalar: bool,
    },
    SortBy {
        expr: Box<Expr>,
        #[frb(default = [])]
        by: Vec<Expr>,
        #[frb(default = [])]
        descending: Vec<bool>,
    },
    Agg(AggExpr),
    Ternary {
        predicate: Box<Expr>,
        truthy: Box<Expr>,
        falsy: Box<Expr>,
    },
    // Function
    Explode(Box<Expr>),
    Filter {
        input: Box<Expr>,
        by: Box<Expr>,
    },
    Wildcard,
    Window {
        function: Box<Expr>,
        partition_by: Vec<Expr>,
        options: WindowType,
    },
    Slice {
        input: Box<Expr>,
        offset: Box<Expr>,
        length: Box<Expr>,
    },
    Exclude(Box<Expr>, Vec<Excluded>),
    KeepName(Box<Expr>),
    Count,
    Nth(i64),
    Internal(RustOpaque<AssertUnwindSafe<PExpr>>),
}

pub enum AggExpr {
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
    Implode(Box<Expr>),
    Count(Box<Expr>),
    Quantile {
        expr: Box<Expr>,
        quantile: Box<Expr>,
        interpol: QuantileInterpolOptions,
    },
    Sum(Box<Expr>),
    AggGroups(Box<Expr>),
    Std(Box<Expr>, u8),
    Var(Box<Expr>, u8),
}

pub(crate) type PExpr = polars::lazy::dsl::Expr;
pub(crate) type PAggExpr = polars::lazy::dsl::AggExpr;

pub(crate) fn into_vec<T, U>(exprs: Vec<T>) -> Vec<U>
where
    U: From<T>,
{
    exprs.into_iter().map(Into::into).collect()
}

impl From<Box<Expr>> for Box<PExpr> {
    #[inline]
    fn from(value: Box<Expr>) -> Self {
        Box::new((*value).into())
    }
}

impl From<Box<PExpr>> for Box<Expr> {
    #[inline]
    fn from(value: Box<PExpr>) -> Self {
        Box::new((*value).into())
    }
}

impl From<Expr> for PExpr {
    fn from(value: Expr) -> Self {
        match value {
            Expr::Alias(expr, name) => PExpr::Alias(expr.into(), name.into()),
            Expr::Column(name) => PExpr::Column(name.into()),
            Expr::Columns(names) => PExpr::Columns(names),
            Expr::DtypeColumn(types) => PExpr::DtypeColumn(into_vec(types)),
            Expr::Literal(value) => PExpr::Literal(value.into()),
            Expr::BinaryExpr { left, op, right } => PExpr::BinaryExpr {
                left: left.into(),
                op,
                right: right.into(),
            },
            Expr::Cast {
                expr,
                data_type,
                strict,
            } => PExpr::Cast {
                expr: expr.into(),
                data_type: data_type.into(),
                strict,
            },
            Expr::Sort { expr, options } => PExpr::Sort {
                expr: expr.into(),
                options,
            },
            Expr::Gather {
                expr,
                idx,
                returns_scalar,
            } => PExpr::Gather {
                expr: expr.into(),
                idx: idx.into(),
                returns_scalar,
            },
            Expr::SortBy {
                expr,
                by,
                descending,
            } => PExpr::SortBy {
                expr: expr.into(),
                by: into_vec(by),
                descending,
            },
            Expr::Agg(agg) => PExpr::Agg(match agg {
                AggExpr::Min {
                    input,
                    propagate_nans,
                } => PAggExpr::Min {
                    input: input.into(),
                    propagate_nans,
                },
                AggExpr::Max {
                    input,
                    propagate_nans,
                } => PAggExpr::Max {
                    input: input.into(),
                    propagate_nans,
                },
                AggExpr::Median(expr) => PAggExpr::Median(expr.into()),
                AggExpr::NUnique(expr) => PAggExpr::NUnique(expr.into()),
                AggExpr::First(expr) => PAggExpr::First(expr.into()),
                AggExpr::Last(expr) => PAggExpr::Last(expr.into()),
                AggExpr::Mean(expr) => PAggExpr::Mean(expr.into()),
                AggExpr::Implode(expr) => PAggExpr::Implode(expr.into()),
                AggExpr::Count(expr) => PAggExpr::Count(expr.into()),
                AggExpr::Quantile {
                    expr,
                    quantile,
                    interpol,
                } => PAggExpr::Quantile {
                    expr: expr.into(),
                    quantile: quantile.into(),
                    interpol,
                },
                AggExpr::Sum(expr) => PAggExpr::Sum(expr.into()),
                AggExpr::AggGroups(expr) => PAggExpr::AggGroups(expr.into()),
                AggExpr::Std(expr, ddof) => PAggExpr::Std(expr.into(), ddof),
                AggExpr::Var(expr, ddof) => PAggExpr::Var(expr.into(), ddof),
            }),
            Expr::Ternary {
                predicate,
                truthy,
                falsy,
            } => PExpr::Ternary {
                predicate: predicate.into(),
                truthy: truthy.into(),
                falsy: falsy.into(),
            },
            Expr::Explode(expr) => PExpr::Explode(expr.into()),
            Expr::Filter { input, by } => PExpr::Filter {
                input: input.into(),
                by: by.into(),
            },
            Expr::Window {
                function,
                partition_by,
                options,
            } => PExpr::Window {
                function: function.into(),
                partition_by: into_vec(partition_by),
                options,
            },
            Expr::Slice {
                input,
                offset,
                length,
            } => PExpr::Slice {
                input: input.into(),
                offset: offset.into(),
                length: length.into(),
            },
            Expr::Exclude(expr, excludes) => PExpr::Exclude(expr.into(), into_vec(excludes)),
            Expr::KeepName(expr) => PExpr::KeepName(expr.into()),
            Expr::Count => PExpr::Count,
            Expr::Nth(idx) => PExpr::Nth(idx),
            Expr::Internal(expr) => expr.into_inner().unwrap().0,
            Expr::Wildcard => PExpr::Wildcard,
        }
    }
}

impl From<PExpr> for Expr {
    fn from(value: PExpr) -> Self {
        match value {
            PExpr::Alias(expr, name) => Expr::Alias(expr.into(), name.to_string()),
            PExpr::Column(name) => Expr::Column(name.to_string()),
            PExpr::Columns(names) => Expr::Columns(names),
            PExpr::DtypeColumn(types) => Expr::DtypeColumn(
                types
                    .into_iter()
                    .map(|t| t.into())
                    .collect::<Vec<DataType>>(),
            ),
            PExpr::Literal(value) => Expr::Literal(value.into()),
            PExpr::BinaryExpr { left, op, right } => Expr::BinaryExpr {
                left: left.into(),
                op,
                right: right.into(),
            },
            PExpr::Cast {
                expr,
                data_type,
                strict,
            } => Expr::Cast {
                expr: expr.into(),
                data_type: data_type.into(),
                strict,
            },
            PExpr::Sort { expr, options } => Expr::Sort {
                expr: expr.into(),
                options,
            },
            PExpr::Gather {
                expr,
                idx,
                returns_scalar,
            } => Expr::Gather {
                expr: expr.into(),
                idx: idx.into(),
                returns_scalar,
            },
            PExpr::SortBy {
                expr,
                by,
                descending,
            } => Expr::SortBy {
                expr: expr.into(),
                by: by.into_iter().map(Into::into).collect(),
                descending,
            },
            PExpr::Agg(agg) => Expr::Agg(match agg {
                PAggExpr::Min {
                    input,
                    propagate_nans,
                } => AggExpr::Min {
                    input: input.into(),
                    propagate_nans,
                },
                PAggExpr::Max {
                    input,
                    propagate_nans,
                } => AggExpr::Max {
                    input: input.into(),
                    propagate_nans,
                },
                PAggExpr::Median(expr) => AggExpr::Median(expr.into()),
                PAggExpr::NUnique(expr) => AggExpr::NUnique(expr.into()),
                PAggExpr::First(expr) => AggExpr::First(expr.into()),
                PAggExpr::Last(expr) => AggExpr::Last(expr.into()),
                PAggExpr::Mean(expr) => AggExpr::Mean(expr.into()),
                PAggExpr::Implode(expr) => AggExpr::Implode(expr.into()),
                PAggExpr::Count(expr) => AggExpr::Count(expr.into()),
                PAggExpr::Quantile {
                    expr,
                    quantile,
                    interpol,
                } => AggExpr::Quantile {
                    expr: expr.into(),
                    quantile: quantile.into(),
                    interpol,
                },
                PAggExpr::Sum(expr) => AggExpr::Sum(expr.into()),
                PAggExpr::AggGroups(expr) => AggExpr::AggGroups(expr.into()),
                PAggExpr::Std(expr, ddof) => AggExpr::Std(expr.into(), ddof),
                PAggExpr::Var(expr, ddof) => AggExpr::Var(expr.into(), ddof),
            }),
            PExpr::Ternary {
                predicate,
                truthy,
                falsy,
            } => Expr::Ternary {
                predicate: predicate.into(),
                truthy: truthy.into(),
                falsy: falsy.into(),
            },
            PExpr::Explode(expr) => Expr::Explode(expr.into()),
            PExpr::Filter { input, by } => Expr::Filter {
                input: input.into(),
                by: by.into(),
            },
            PExpr::Window {
                function,
                partition_by,
                options,
            } => Expr::Window {
                function: function.into(),
                partition_by: into_vec(partition_by),
                options,
            },
            PExpr::Wildcard => Expr::Wildcard,
            PExpr::Slice {
                input,
                offset,
                length,
            } => Expr::Slice {
                input: input.into(),
                offset: offset.into(),
                length: length.into(),
            },
            PExpr::Exclude(expr, excludes) => {
                Expr::Exclude(expr.into(), excludes.into_iter().map(Into::into).collect())
            }
            PExpr::KeepName(expr) => Expr::KeepName(expr.into()),
            PExpr::Count => Expr::Count,
            PExpr::Nth(idx) => Expr::Nth(idx),
            PExpr::AnonymousFunction { .. }
            | PExpr::Function { .. }
            | PExpr::RenameAlias { .. }
            | PExpr::SubPlan(..)
            | PExpr::Selector(..) => Expr::Internal(RustOpaque::new(AssertUnwindSafe(value))),
        }
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
        pub fn $fn(&self, $($(#[frb(default = $default)])? $param : $ty),*) -> $output {
            <$output>::from(self.into_internal().$fn($($param $(.$conv())?),*))
        }
    )*};
}

macro_rules! rolling_series {
    ($($fn:ident;)*) => {$(
        #[doc = concat!(" TODO: Docs for ", stringify!($fn))]
        #[frb(sync)]
        pub fn $fn(
            &self,
            window_size: Option<Duration>,
            #[frb(default = 1)] min_periods: usize,
            weights: Option<Vec<f64>>,
            #[frb(default = false)] center: bool,
            by: Option<String>,
            closed_window: Option<ClosedWindow>,
        ) -> Expr {
            Expr::from(self.into_internal().$fn(rolling_options(
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

impl Expr {
    #[inline]
    pub(crate) fn into_internal(self) -> PExpr {
        match self {
            Expr::Internal(expr) => expr.into_inner().unwrap().0,
            _ => self.into(),
        }
    }
    delegate! {
        // #[frb(sync)] add(self, other: #[into] Expr) -> Expr;
        // #[frb(sync)] alias(self, name: #[as_str] String) -> Expr;
        // #[frb(sync)] agg_groups(self) -> Expr;
        // #[frb(sync)] and(self, expr: Expr) -> Expr;
        // #[frb(sync)] cast(self, data_type: #[into] DataType) -> Expr;
        // #[frb(sync)] eq(self, other: Expr) -> Expr;
        // #[frb(sync)] eq_missing(self, other: Expr) -> Expr;
        // #[frb(sync)] exclude(self, columns: Vec<String>) -> Expr;
        // TODO: #[frb(sync)] exclude_dtype(self, dtypes: Vec<DataType>) -> Expr;
        // #[frb(sync)] explode(self) -> Expr;
        #[frb(sync)] fill_nan(self, value: Expr) -> Expr;
        // TODO: Disallow col('*')
        // #[frb(sync)] filter(self, cond: Expr) -> Expr;
        // #[frb(sync)] first(self) -> Expr;
        // TODO: alias for explode
        // #[frb(sync)] flatten(self) -> Expr;
        // #[frb(sync)] floor_div(self, rhs: #[into] Expr) -> Expr;
        // #[frb(sync)] gather(self, idx: Expr) -> Expr;
        /// Similar to [gather] but allows for scalars.
        // #[frb(sync)] get(self, idx: Expr) -> Expr;
        // #[frb(sync)] gt(self, other: Expr) -> Expr;
        // #[frb(sync)] gt_eq(self, other: Expr) -> Expr;
        // alias for slice
        // #[frb(sync)] head(self, length: Option<usize>) -> Expr;
        // #[frb(sync)] implode(self) -> Expr;
        // #[frb(sync)] last(self) -> Expr;
        // #[frb(sync)] lt(self, other: Expr) -> Expr;
        // #[frb(sync)] lt_eq(self, other: Expr) -> Expr;
        // #[frb(sync)] mul(self, other: Expr) -> Expr;
        // #[frb(sync)] n_unique(self) -> Expr;
        // #[frb(sync)] nan_max(self) -> Expr;
        // #[frb(sync)] nan_min(self) -> Expr;
        // #[frb(sync)] neq(self, other: Expr) -> Expr;
        // #[frb(sync)] neq_missing(self, other: Expr) -> Expr;
        // #[frb(sync)] or(self, expr: Expr) -> Expr;
        // #[frb(sync)] rem(self, other: #[into] Expr) -> Expr;
        // #[frb(sync)] slice(self, offset: Expr, length: Expr) -> Expr;
        // #[frb(sync)] sub(self, other: Expr) -> Expr;
        // #[frb(sync)] std(self, ddof: u8) -> Expr;
        // #[frb(sync)] strict_cast(self, data_type: #[into] DataType) -> Expr;
        // #[frb(sync)] sum(self) -> Expr;
        // TODO: Alias to slice
        // #[frb(sync)] tail(self, length: Option<usize>) -> Expr;
        // #[frb(sync)] xor(self, expr: Expr) -> Expr;
        #[frb(sync, getter)] abs(self) -> Expr;
        #[frb(sync, getter)] arccos(self) -> Expr;
        #[frb(sync, getter)] arccosh(self) -> Expr;
        #[frb(sync, getter)] arcsin(self) -> Expr;
        #[frb(sync, getter)] arcsinh(self) -> Expr;
        #[frb(sync, getter)] arctan(self) -> Expr;
        #[frb(sync)] arctan2(self, x: #[into] Expr) -> Expr;
        #[frb(sync, getter)] arctanh(self) -> Expr;
        #[frb(sync, getter)] arg_max(self) -> Expr;
        #[frb(sync, getter)] arg_min(self) -> Expr;
        #[frb(sync, getter)] arg_unique(self) -> Expr;
        #[frb(sync)] all(self, ignore_nulls: bool = false) -> Expr;
        #[frb(sync)] any(self, ignore_nulls: bool = false) -> Expr;
        #[frb(sync)] append(self, other: Expr, upcast: bool = true) -> Expr;
        #[frb(sync)] backward_fill(self, limit: Option<u32>) -> Expr;
        #[frb(sync, getter)] cbrt(self) -> Expr;
        #[frb(sync, getter)] ceil(self) -> Expr;
        #[frb(sync, getter)] clip(self, min: #[into] Expr, max: #[into] Expr) -> Expr;
        #[frb(sync, getter)] cos(self) -> Expr;
        #[frb(sync, getter)] cosh(self) -> Expr;
        /// Calculate the cotangent of this expression.
        #[frb(sync, getter)] cot(self) -> Expr;
        #[frb(sync, getter)] count(self) -> Expr;
        #[frb(sync)] clip_max(self, max: #[into] Expr) -> Expr;
        #[frb(sync)] clip_min(self, min: #[into] Expr) -> Expr;
        #[frb(sync)] cum_count(self, reverse: bool = false) -> Expr;
        #[frb(sync)] cum_max(self, reverse: bool = false) -> Expr;
        #[frb(sync)] cum_min(self, reverse: bool = false) -> Expr;
        #[frb(sync)] cum_prod(self, reverse: bool = false) -> Expr;
        #[frb(sync)] cum_sum(self, reverse: bool = false) -> Expr;
        #[frb(sync)] div(self, other: #[into] Expr) -> Expr;
        #[frb(sync, getter)] degrees(self) -> Expr;
        #[frb(sync)] dot(self, other: Expr) -> Expr;
        #[frb(sync, getter)] drop_nans(self) -> Expr;
        #[frb(sync, getter)] drop_nulls(self) -> Expr;
        #[frb(sync)] entropy(self, base: f64, normalize: bool = false) -> Expr;
        #[frb(sync, getter)] exp(self) -> Expr;
        #[frb(sync)] fill_null(self, value: Expr) -> Expr;
        #[frb(sync, getter)] floor(self) -> Expr;
        #[frb(sync)] forward_fill(self, limit: Option<u32>) -> Expr;
        #[frb(sync, getter)] is_finite(self) -> Expr;
        #[frb(sync, getter)] is_in(self, other: Expr) -> Expr;
        #[frb(sync, getter)] is_nan(self) -> Expr;
        #[frb(sync, getter)] is_not_nan(self) -> Expr;
        #[frb(sync, getter)] is_not_null(self) -> Expr;
        #[frb(sync, getter)] is_null(self) -> Expr;
        #[frb(sync)] log(self, base: f64) -> Expr;
        #[frb(sync, getter)] log1p(self) -> Expr;
        #[frb(sync, getter)] lower_bound(self) -> Expr;
        #[frb(sync, getter)] not(self) -> Expr;
        #[frb(sync, getter)] null_count(self) -> Expr;
        #[frb(sync)] pow(self, exponent: f64) -> Expr;
        #[frb(sync, getter)] product(self) -> Expr;
        #[frb(sync, getter)] radians(self) -> Expr;
        #[frb(sync)] reshape(self, dims: #[as_slice] Vec<i64>) -> Expr;
        #[frb(sync, getter)] reverse(self) -> Expr;
        #[frb(sync)] round(self, decimals: u32) -> Expr;
        #[frb(sync)] round_sig_figs(self, digits: i32) -> Expr;
        #[frb(sync)] set_sorted_flag(self, sorted: IsSorted) -> Expr;
        #[frb(sync)] shift(self, n: #[into] Expr) -> Expr;
        #[frb(sync)] shift_and_fill(self, n: Expr, fill_value: Expr) -> Expr;
        #[frb(sync, getter)] shrink_dtype(self) -> Expr;
        #[frb(sync, getter)] sin(self) -> Expr;
        #[frb(sync, getter)] sinh(self) -> Expr;
        #[frb(sync, getter)] sqrt(self) -> Expr;
        #[frb(sync, getter)] tan(self) -> Expr;
        #[frb(sync, getter)] tanh(self) -> Expr;
        #[frb(sync, getter)] to_physical(self) -> Expr;
        #[frb(sync, getter)] unique(self) -> Expr;
        #[frb(sync, getter)] unique_stable(self) -> Expr;
        #[frb(sync, getter)] upper_bound(self) -> Expr;
        #[frb(sync)] value_counts(self, sort: bool = false, parallel: bool = true) -> Expr;
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
        &self,
        #[frb(default = false)] descending: bool,
        #[frb(default = false)] nulls_last: bool,
        #[frb(default = true)] multithreaded: bool,
        #[frb(default = false)] maintain_order: bool,
    ) -> Expr {
        Expr::from(self.into_internal().arg_sort(SortOptions {
            descending,
            nulls_last,
            multithreaded,
            maintain_order,
        }))
    }
    // TODO: alias to Window
    // #[frb(sync)]
    // pub fn over(self, partiion_by: Vec<Expr>, kind: Option<WindowMapping>) -> Expr {
    //     Expr::from(
    //         self.into_internal()
    //             .over_with_options(into_vec(partiion_by), kind.unwrap_or_default()),
    //     )
    // }
    /// Returns a dot representation of this expression.
    #[frb(sync)]
    pub fn to_dot(&self) -> Result<String> {
        Ok(self.into_internal().to_dot()?)
    }
}

fn rolling_options(
    window_size: Option<Duration>,
    min_periods: usize,
    weights: Option<Vec<f64>>,
    center: bool,
    by: Option<String>,
    closed_window: Option<ClosedWindow>,
) -> RollingOptions {
    let mut opts = RollingOptions {
        weights,
        center,
        by,
        closed_window,
        min_periods,
        ..Default::default()
    };
    if let Some(window_size) = window_size {
        opts.window_size = chrono_to_polars_duration(window_size);
    }
    opts
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
    Uint8,
    /// Unsigned 16-bit integer
    Uint16,
    /// Unsigned 32-bit integer
    Uint32,
    /// Unsigned 64-bit integer
    Uint64,
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
            DataType::Uint8 => polars::prelude::DataType::UInt8,
            DataType::Uint16 => polars::prelude::DataType::UInt16,
            DataType::Uint32 => polars::prelude::DataType::UInt32,
            DataType::Uint64 => polars::prelude::DataType::UInt64,
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
            DataType::Struct(fields) => polars::prelude::DataType::Struct(into_vec(fields)),
            DataType::Null => polars::prelude::DataType::Null,
            DataType::Unknown => polars::prelude::DataType::Null,
        }
    }
}

impl From<PDataType> for DataType {
    fn from(value: PDataType) -> Self {
        match value {
            polars::prelude::DataType::Boolean => DataType::Boolean,
            polars::prelude::DataType::UInt8 => DataType::Uint8,
            polars::prelude::DataType::UInt16 => DataType::Uint16,
            polars::prelude::DataType::UInt32 => DataType::Uint32,
            polars::prelude::DataType::UInt64 => DataType::Uint64,
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
            polars::prelude::DataType::Struct(fields) => DataType::Struct(into_vec(fields)),
            polars::prelude::DataType::Null => DataType::Null,
            PDataType::Unknown => DataType::Unknown,
        }
    }
}

pub struct Field {
    pub name: String,
    pub dtype: Box<DataType>,
}

impl From<Field> for polars::datatypes::Field {
    fn from(value: Field) -> Self {
        polars::datatypes::Field::new(&value.name, (*value.dtype).into())
    }
}

impl From<polars::datatypes::Field> for Field {
    fn from(value: polars::datatypes::Field) -> Self {
        Field {
            name: value.name().to_string(),
            dtype: Box::new(value.data_type().clone().into()),
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
            LiteralValue::Series(series) => match series.try_unwrap() {
                Ok(inner) => polars::prelude::LiteralValue::Series(inner.0),
                Err(series) => polars::prelude::LiteralValue::Series(SpecialEq::new(
                    PSeries::new_empty(series.name(), series.dtype()),
                )),
            },
        }
    }
}

impl LiteralValue {
    #[frb(sync)]
    pub fn from_series(series: Series) -> LiteralValue {
        LiteralValue::Series(RustOpaque::new(AssertUnwindSafe(SpecialEq::new(
            series.0 .0,
        ))))
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

#[frb(mirror(QuantileInterpolOptions))]
pub enum _QuantileInterpolOptions {
    Nearest,
    Lower,
    Higher,
    Midpoint,
    Linear,
}

#[frb(mirror(SortOptions))]
pub struct _SortOptions {
    #[frb(default = false)]
    pub descending: bool,
    #[frb(default = false)]
    pub nulls_last: bool,
    #[frb(default = true)]
    pub multithreaded: bool,
    #[frb(default = false)]
    pub maintain_order: bool,
}

#[frb(mirror(Operator))]
pub enum _Operator {
    Eq,
    EqValidity,
    NotEq,
    NotEqValidity,
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

#[frb(mirror(WindowType))]
pub enum _WindowType {
    /// Explode the aggregated list and just do a hstack instead of a join
    /// this requires the groups to be sorted to make any sense
    Over(WindowMapping),
    // #[cfg(feature = "dynamic_group_by")]
    // Rolling(RollingGroupOptions),
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

pub enum Excluded {
    Name(String),
    Dtype(DataType),
}

impl From<Excluded> for polars::lazy::dsl::Excluded {
    fn from(value: Excluded) -> Self {
        match value {
            Excluded::Name(name) => polars::lazy::dsl::Excluded::Name(name.into()),
            Excluded::Dtype(dtype) => polars::lazy::dsl::Excluded::Dtype(dtype.into()),
        }
    }
}

impl From<polars::lazy::dsl::Excluded> for Excluded {
    fn from(value: polars::lazy::dsl::Excluded) -> Self {
        match value {
            polars::lazy::dsl::Excluded::Name(name) => Excluded::Name(name.to_string()),
            polars::lazy::dsl::Excluded::Dtype(dtype) => Excluded::Dtype(dtype.into()),
        }
    }
}

pub enum Ambiguous {
    Raise,
    Earliest,
    Latest,
}

impl Ambiguous {
    pub(crate) fn into_expr(self) -> PExpr {
        match self {
            Self::Raise => PExpr::Literal(polars::prelude::LiteralValue::Utf8("raise".into())),
            Self::Earliest => {
                PExpr::Literal(polars::prelude::LiteralValue::Utf8("earliest".into()))
            }
            Self::Latest => PExpr::Literal(polars::prelude::LiteralValue::Utf8("latest".into())),
        }
    }
}
