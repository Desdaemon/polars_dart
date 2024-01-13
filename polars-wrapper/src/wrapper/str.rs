use flutter_rust_bridge::frb;
use polars::{datatypes::TimeUnit, lazy::dsl::StrptimeOptions};

use super::{
    df::DataType,
    expr::{Ambiguous, Expr},
};

macro_rules! delegate_str {
    ($( $(#[$attribute:meta])* $fn:ident(self $(,)? $($param:ident : $(#[$conv:ident])? $ty:ty $(= $default:expr)? ),*) -> $output:ty; )*) => {paste::paste!{$(
        $(#[$attribute])*
        #[doc = concat!(" TODO: Docs for ", stringify!($fn))]
        pub fn [<str_ $fn>](&self, $($(#[frb(default = $default)])? $param : $ty),*) -> $output {
            <$output>::from(self.into_internal().str().$fn($($param $(.$conv())?),*))
        }
    )*}};
}
impl Expr {
    delegate_str! {
        #[frb(sync)] contains_literal(self, pat: #[into] Expr) -> Expr;
        #[frb(sync)] contains(self, pat: #[into] Expr, strict: bool = true) -> Expr;
        #[frb(sync)] ends_with(self, pat: #[into] Expr) -> Expr;
        #[frb(sync)] extract(self, pat: #[as_str] String, group_index: usize) -> Expr;
        #[frb(sync)] extract_all(self, pat: #[into] Expr) -> Expr;
        #[frb(sync)] count_matches(self, pat: #[into] Expr, literal: bool = false) -> Expr;
        #[frb(sync)] concat(self, delimiter: #[as_str] String, ignore_nulls: bool = true) -> Expr;
        #[frb(sync)] splitn(self, by: #[into] Expr, n: usize) -> Expr;
        #[frb(sync)] replace(self, pat: #[into] Expr, val: #[into] Expr, literal: bool = false) -> Expr;
        #[frb(sync)] replace_n(self, pat: #[into] Expr, val: #[into] Expr, literal: bool = false, n: i64) -> Expr;
        #[frb(sync)] replace_all(self, pat: #[into] Expr, val: #[into] Expr, literal: bool = false) -> Expr;
        #[frb(sync)] starts_with(self, pat: #[into] Expr) -> Expr;
        #[frb(sync)] strip_chars(self, matches: #[into] Expr) -> Expr;
        #[frb(sync)] strip_chars_start(self, matches: #[into] Expr) -> Expr;
        #[frb(sync)] strip_chars_end(self, matches: #[into] Expr) -> Expr;
        #[frb(sync)] strip_prefix(self, prefix: #[into] Expr) -> Expr;
        #[frb(sync)] strip_suffix(self, suffix: #[into] Expr) -> Expr;
        #[frb(sync, getter)] to_lowercase(self) -> Expr;
        #[frb(sync, getter)] to_uppercase(self) -> Expr;
        #[frb(sync)] to_integer(self, base: u32, strict: bool = true) -> Expr;
        #[frb(sync, getter)] len_bytes(self) -> Expr;
        #[frb(sync, getter)] len_chars(self) -> Expr;
        #[frb(sync)] slice(self, start: i64, length: Option<u64>) -> Expr;
        #[frb(sync, getter)] explode(self) -> Expr;
    }
    /// - `dtype` A temporal data type, i.e. Date, DateTime, or Time.
    #[frb(sync)]
    pub fn strptime(
        &self,
        dtype: DataType,
        format: Option<String>,
        #[frb(default = true)] strict: bool,
        #[frb(default = true)] exact: bool,
        #[frb(default = true)] cache: bool,
        #[frb(default = "Ambiguous.raise")] ambiguous: Ambiguous,
    ) -> Expr {
        Expr::from(self.into_internal().str().strptime(
            dtype.into(),
            StrptimeOptions {
                format,
                strict,
                exact,
                cache,
            },
            ambiguous.into_expr(),
        ))
    }
    #[frb(sync)]
    pub fn str_to_date(
        &self,
        format: Option<String>,
        #[frb(default = true)] strict: bool,
        #[frb(default = true)] exact: bool,
        #[frb(default = true)] cache: bool,
    ) -> Expr {
        Expr::from(self.into_internal().str().to_date(StrptimeOptions {
            format,
            strict,
            exact,
            cache,
        }))
    }
    #[frb(sync)]
    pub fn str_to_datetime(
        &self,
        time_unit: Option<TimeUnit>,
        time_zone: Option<String>,
        format: Option<String>,
        #[frb(default = true)] strict: bool,
        #[frb(default = true)] exact: bool,
        #[frb(default = true)] cache: bool,
        #[frb(default = "Ambiguous.raise")] ambiguous: Ambiguous,
    ) -> Expr {
        Expr::from(self.into_internal().str().to_datetime(
            time_unit,
            time_zone,
            StrptimeOptions {
                format,
                strict,
                exact,
                cache,
            },
            ambiguous.into_expr(),
        ))
    }
    #[frb(sync)]
    pub fn str_to_time(
        &self,
        format: Option<String>,
        #[frb(default = true)] strict: bool,
        #[frb(default = true)] exact: bool,
        #[frb(default = true)] cache: bool,
    ) -> Expr {
        Expr::from(self.into_internal().str().to_time(StrptimeOptions {
            format,
            strict,
            exact,
            cache,
        }))
    }
    #[frb(sync)]
    pub fn str_split(&self, by: Expr, #[frb(default = false)] inclusive: bool) -> Expr {
        let str = self.into_internal().str();
        if inclusive {
            Expr::from(str.split_inclusive(by.into()))
        } else {
            Expr::from(str.split(by.into()))
        }
    }
    #[frb(sync)]
    pub fn str_split_exact(
        &self,
        by: Expr,
        n: usize,
        #[frb(default = false)] inclusive: bool,
    ) -> Expr {
        let str = self.into_internal().str();
        if inclusive {
            Expr::from(str.split_exact_inclusive(by.into(), n))
        } else {
            Expr::from(str.split_exact(by.into(), n))
        }
    }
}
