use flutter_rust_bridge::frb;

use super::expr::Expr;

macro_rules! delegate_str {
    ($( $(#[$attribute:meta])* $fn:ident(self $(,)? $($param:ident : $(#[$conv:ident])? $ty:ty $(= $default:expr)? ),*) -> $output:ty; )*) => {paste::paste!{$(
        $(#[$attribute])*
        #[doc = concat!(" TODO: Docs for ", stringify!($fn))]
        pub fn [<list_ $fn>](&self, $($(#[frb(default = $default)])? $param : $ty),*) -> $output {
            <$output>::from(self.into_internal().list().$fn($($param $(.$conv())?),*))
        }
    )*}};
}

impl Expr {
    delegate_str! {
        #[frb(sync, getter)] len(self) -> Expr;
        #[frb(sync, getter)] max(self) -> Expr;
        #[frb(sync, getter)] min(self) -> Expr;
        #[frb(sync, getter)] sum(self) -> Expr;
        #[frb(sync, getter)] mean(self) -> Expr;
        // #[frb(sync, getter)] sort(self) -> Expr;
        #[frb(sync, getter)] reverse(self) -> Expr;
        #[frb(sync)] get(self, index: #[into] Expr) -> Expr;
        #[frb(sync, getter)] first(self) -> Expr;
        #[frb(sync, getter)] last(self) -> Expr;
        #[frb(sync)] join(self, separator: #[into] Expr) -> Expr;
        #[frb(sync, getter)] arg_min(self) -> Expr;
        #[frb(sync, getter)] arg_max(self) -> Expr;
        #[frb(sync)] shift(self, periods: #[into] Expr) -> Expr;
        #[frb(sync)] slice(self, offset: #[into] Expr, length: #[into] Expr) -> Expr;
        #[frb(sync)] head(self, n: #[into] Expr) -> Expr;
        #[frb(sync)] tail(self, n: #[into] Expr) -> Expr;
        #[frb(sync)] contains(self, other: Expr) -> Expr;
    }
    #[frb(sync)]
    pub fn list_unique(self, #[frb(default = false)] maintain_order: bool) -> Expr {
        if maintain_order {
            self.into_internal().list().unique_stable().into()
        } else {
            self.into_internal().list().unique().into()
        }
    }
}
