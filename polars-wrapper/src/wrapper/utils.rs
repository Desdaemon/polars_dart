use chrono::prelude::*;
use flutter_rust_bridge::*;
use polars::{frame::row::Row, prelude::*};

pub(crate) fn any_value_to_dart(any: AnyValue) -> DartAbi {
    match any {
        AnyValue::Null => ().into_dart(),
        AnyValue::Boolean(val) => val.into_dart(),
        AnyValue::Utf8(val) => val.into_dart(),
        AnyValue::Utf8Owned(val) => val.as_str().into_dart(),
        AnyValue::UInt8(val) => val.into_dart(),
        AnyValue::UInt16(val) => val.into_dart(),
        AnyValue::UInt32(val) => val.into_dart(),
        AnyValue::UInt64(val) => val.into_dart(),
        AnyValue::Int8(val) => val.into_dart(),
        AnyValue::Int16(val) => val.into_dart(),
        AnyValue::Int32(val) => val.into_dart(),
        AnyValue::Int64(val) => val.into_dart(),
        AnyValue::Float32(val) => val.into_dart(),
        AnyValue::Float64(val) => val.into_dart(),
        AnyValue::Date(val) => val.into_dart(),
        AnyValue::Time(val) => val.into_dart(),
        AnyValue::Binary(val) => val.to_owned().into_dart(),
        AnyValue::BinaryOwned(val) => val.into_dart(),
        AnyValue::Duration(ts, unit) => match unit {
            TimeUnit::Nanoseconds => chrono::Duration::nanoseconds(ts),
            TimeUnit::Microseconds => chrono::Duration::microseconds(ts),
            TimeUnit::Milliseconds => chrono::Duration::milliseconds(ts),
        }
        .into_dart(),
        AnyValue::Datetime(ts, unit, tz) => timestamp_to_naive(ts, unit, tz.as_deref()).into_dart(),
        AnyValue::List(series) => {
            panic!("don't know how to serialize AnyValue::List:\n{series}")
        }
        AnyValue::Struct(..) | AnyValue::StructOwned(..) => {
            panic!("don't know how to serialize AnyValue::Struct:\n{any}")
        }
    }
}

fn timestamp_to_naive(ts: i64, unit: TimeUnit, tz: Option<&str>) -> Option<NaiveDateTime> {
    let naive = match unit {
        TimeUnit::Milliseconds => chrono::NaiveDateTime::from_timestamp_millis(ts),
        TimeUnit::Microseconds => {
            let s = ts.div_euclid(1_000_000);
            let ns = ts.rem_euclid(1_000_000) * 1000;
            chrono::NaiveDateTime::from_timestamp_opt(s, ns as _)
        }
        TimeUnit::Nanoseconds => {
            let s = ts.div_euclid(1_000_000_000);
            let ns = ts.rem_euclid(1_000_000_000);
            chrono::NaiveDateTime::from_timestamp_opt(s, ns as _)
        }
    }?;

    if let Some(tz) = tz {
        let tz = tz
            .parse::<chrono_tz::Tz>()
            .map_err(|err| -> ! { panic!("invalid timezone ({err})") })
            .unwrap();

        Some(
            naive
                .and_local_timezone(tz)
                .single()?
                .with_timezone(&Local)
                .naive_local(),
        )
    } else {
        // assume local timestamp
        Some(naive)
    }
}
#[inline]
pub(crate) fn make_row<'any>(width: usize) -> Row<'any> {
    Row::new(vec![AnyValue::Null; width])
}

macro_rules! get {
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
