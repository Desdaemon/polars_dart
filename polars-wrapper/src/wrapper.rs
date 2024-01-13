#![warn(missing_docs)]
#![allow(clippy::too_many_arguments)]

pub(crate) mod expr;
pub(crate) mod series;

pub(crate) mod df;
pub(crate) mod entry;
pub(crate) mod list;
pub(crate) mod str;
mod util;

pub(crate) mod prelude {
    pub use polars::prelude::*;
}
