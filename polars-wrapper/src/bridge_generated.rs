#![allow(
    non_camel_case_types,
    unused,
    clippy::redundant_closure,
    clippy::useless_conversion,
    clippy::unit_arg,
    clippy::double_parens,
    non_snake_case,
    clippy::too_many_arguments
)]
// AUTO GENERATED FILE, DO NOT EDIT.
// Generated by `flutter_rust_bridge`@ 1.59.0.

use crate::wrapper::*;
use core::panic::UnwindSafe;
use flutter_rust_bridge::*;
use std::ffi::c_void;
use std::sync::Arc;

// Section: imports

// Section: wire functions

fn wire_read_csv_impl(
    port_: MessagePort,
    path: impl Wire2Api<String> + UnwindSafe,
    has_header: impl Wire2Api<Option<bool>> + UnwindSafe,
    delimiter: impl Wire2Api<Option<u8>> + UnwindSafe,
    skip_rows: impl Wire2Api<Option<usize>> + UnwindSafe,
    skip_rows_after_header: impl Wire2Api<Option<usize>> + UnwindSafe,
    chunk_size: impl Wire2Api<Option<usize>> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "read_csv",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_path = path.wire2api();
            let api_has_header = has_header.wire2api();
            let api_delimiter = delimiter.wire2api();
            let api_skip_rows = skip_rows.wire2api();
            let api_skip_rows_after_header = skip_rows_after_header.wire2api();
            let api_chunk_size = chunk_size.wire2api();
            move |task_callback| {
                read_csv(
                    api_path,
                    api_has_header,
                    api_delimiter,
                    api_skip_rows,
                    api_skip_rows_after_header,
                    api_chunk_size,
                )
            }
        },
    )
}
fn wire_column__method__DataFrame_impl(
    that: impl Wire2Api<DataFrame> + UnwindSafe,
    column: impl Wire2Api<String> + UnwindSafe,
) -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "column__method__DataFrame",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_that = that.wire2api();
            let api_column = column.wire2api();
            DataFrame::column(&api_that, api_column)
        },
    )
}
fn wire_columns__method__DataFrame_impl(
    that: impl Wire2Api<DataFrame> + UnwindSafe,
    columns: impl Wire2Api<Vec<String>> + UnwindSafe,
) -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "columns__method__DataFrame",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_that = that.wire2api();
            let api_columns = columns.wire2api();
            DataFrame::columns(&api_that, api_columns)
        },
    )
}
fn wire_dump__method__DataFrame_impl(
    that: impl Wire2Api<DataFrame> + UnwindSafe,
) -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "dump__method__DataFrame",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_that = that.wire2api();
            DataFrame::dump(&api_that)
        },
    )
}
fn wire_of_i32__static_method__Series_impl(
    name: impl Wire2Api<String> + UnwindSafe,
    values: impl Wire2Api<Option<Vec<i32>>> + UnwindSafe,
) -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "of_i32__static_method__Series",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_name = name.wire2api();
            let api_values = values.wire2api();
            Ok(Series::of_i32(api_name, api_values))
        },
    )
}
fn wire_of_i64__static_method__Series_impl(
    name: impl Wire2Api<String> + UnwindSafe,
    values: impl Wire2Api<Option<Vec<i64>>> + UnwindSafe,
) -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "of_i64__static_method__Series",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_name = name.wire2api();
            let api_values = values.wire2api();
            Ok(Series::of_i64(api_name, api_values))
        },
    )
}
fn wire_of_f64__static_method__Series_impl(
    name: impl Wire2Api<String> + UnwindSafe,
    values: impl Wire2Api<Option<Vec<f64>>> + UnwindSafe,
) -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "of_f64__static_method__Series",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_name = name.wire2api();
            let api_values = values.wire2api();
            Ok(Series::of_f64(api_name, api_values))
        },
    )
}
fn wire_append__method__Series_impl(
    port_: MessagePort,
    that: impl Wire2Api<Series> + UnwindSafe,
    other: impl Wire2Api<Series> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "append__method__Series",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            let api_other = other.wire2api();
            move |task_callback| Series::append(&api_that, api_other)
        },
    )
}
fn wire_as_strings__method__Series_impl(
    port_: MessagePort,
    that: impl Wire2Api<Series> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "as_strings__method__Series",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            move |task_callback| Series::as_strings(&api_that)
        },
    )
}
fn wire_as_i32__method__Series_impl(port_: MessagePort, that: impl Wire2Api<Series> + UnwindSafe) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "as_i32__method__Series",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            move |task_callback| Series::as_i32(&api_that)
        },
    )
}
fn wire_as_f64__method__Series_impl(port_: MessagePort, that: impl Wire2Api<Series> + UnwindSafe) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "as_f64__method__Series",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            move |task_callback| Series::as_f64(&api_that)
        },
    )
}
fn wire_as_durations__method__Series_impl(
    port_: MessagePort,
    that: impl Wire2Api<Series> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "as_durations__method__Series",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            move |task_callback| Series::as_durations(&api_that)
        },
    )
}
fn wire_as_naive_datetime__method__Series_impl(
    port_: MessagePort,
    that: impl Wire2Api<Series> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "as_naive_datetime__method__Series",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            move |task_callback| Series::as_naive_datetime(&api_that)
        },
    )
}
fn wire_as_utc_datetime__method__Series_impl(
    port_: MessagePort,
    that: impl Wire2Api<Series> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "as_utc_datetime__method__Series",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            move |task_callback| Series::as_utc_datetime(&api_that)
        },
    )
}
fn wire_as_local_datetime__method__Series_impl(
    port_: MessagePort,
    that: impl Wire2Api<Series> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "as_local_datetime__method__Series",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            move |task_callback| Series::as_local_datetime(&api_that)
        },
    )
}
fn wire_abs__method__Series_impl(port_: MessagePort, that: impl Wire2Api<Series> + UnwindSafe) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "abs__method__Series",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            move |task_callback| Series::abs(&api_that)
        },
    )
}
fn wire_sort__method__Series_impl(
    port_: MessagePort,
    that: impl Wire2Api<Series> + UnwindSafe,
    reverse: impl Wire2Api<bool> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "sort__method__Series",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            let api_reverse = reverse.wire2api();
            move |task_callback| Series::sort(&api_that, api_reverse)
        },
    )
}
fn wire_shuffle__method__Series_impl(
    port_: MessagePort,
    that: impl Wire2Api<Series> + UnwindSafe,
    seed: impl Wire2Api<Option<u64>> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "shuffle__method__Series",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            let api_seed = seed.wire2api();
            move |task_callback| Series::shuffle(&api_that, api_seed)
        },
    )
}
fn wire_sum__method__Series_impl(port_: MessagePort, that: impl Wire2Api<Series> + UnwindSafe) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "sum__method__Series",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            move |task_callback| Series::sum(&api_that)
        },
    )
}
fn wire_min__method__Series_impl(port_: MessagePort, that: impl Wire2Api<Series> + UnwindSafe) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "min__method__Series",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            move |task_callback| Series::min(&api_that)
        },
    )
}
fn wire_max__method__Series_impl(port_: MessagePort, that: impl Wire2Api<Series> + UnwindSafe) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "max__method__Series",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            move |task_callback| Series::max(&api_that)
        },
    )
}
fn wire_explode__method__Series_impl(port_: MessagePort, that: impl Wire2Api<Series> + UnwindSafe) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "explode__method__Series",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            move |task_callback| Series::explode(&api_that)
        },
    )
}
fn wire_explode_by_offsets__method__Series_impl(
    port_: MessagePort,
    that: impl Wire2Api<Series> + UnwindSafe,
    offsets: impl Wire2Api<Vec<i64>> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "explode_by_offsets__method__Series",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            let api_offsets = offsets.wire2api();
            move |task_callback| Series::explode_by_offsets(&api_that, api_offsets)
        },
    )
}
fn wire_cummax__method__Series_impl(
    port_: MessagePort,
    that: impl Wire2Api<Series> + UnwindSafe,
    reverse: impl Wire2Api<bool> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "cummax__method__Series",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            let api_reverse = reverse.wire2api();
            move |task_callback| Series::cummax(&api_that, api_reverse)
        },
    )
}
fn wire_cummin__method__Series_impl(
    port_: MessagePort,
    that: impl Wire2Api<Series> + UnwindSafe,
    reverse: impl Wire2Api<bool> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "cummin__method__Series",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            let api_reverse = reverse.wire2api();
            move |task_callback| Series::cummin(&api_that, api_reverse)
        },
    )
}
fn wire_cumprod__method__Series_impl(
    port_: MessagePort,
    that: impl Wire2Api<Series> + UnwindSafe,
    reverse: impl Wire2Api<bool> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "cumprod__method__Series",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            let api_reverse = reverse.wire2api();
            move |task_callback| Series::cumprod(&api_that, api_reverse)
        },
    )
}
fn wire_cumsum__method__Series_impl(
    port_: MessagePort,
    that: impl Wire2Api<Series> + UnwindSafe,
    reverse: impl Wire2Api<bool> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "cumsum__method__Series",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            let api_reverse = reverse.wire2api();
            move |task_callback| Series::cumsum(&api_that, api_reverse)
        },
    )
}
fn wire_product__method__Series_impl(port_: MessagePort, that: impl Wire2Api<Series> + UnwindSafe) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "product__method__Series",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            move |task_callback| Series::product(&api_that)
        },
    )
}
fn wire_get_string__method__Series_impl(
    that: impl Wire2Api<Series> + UnwindSafe,
    index: impl Wire2Api<usize> + UnwindSafe,
) -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "get_string__method__Series",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_that = that.wire2api();
            let api_index = index.wire2api();
            Series::get_string(&api_that, api_index)
        },
    )
}
fn wire_get__method__Series_impl(
    that: impl Wire2Api<Series> + UnwindSafe,
    index: impl Wire2Api<usize> + UnwindSafe,
) -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "get__method__Series",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_that = that.wire2api();
            let api_index = index.wire2api();
            Series::get(&api_that, api_index)
        },
    )
}
fn wire_head__method__Series_impl(
    that: impl Wire2Api<Series> + UnwindSafe,
    length: impl Wire2Api<Option<usize>> + UnwindSafe,
) -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "head__method__Series",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_that = that.wire2api();
            let api_length = length.wire2api();
            Series::head(&api_that, api_length)
        },
    )
}
fn wire_tail__method__Series_impl(
    that: impl Wire2Api<Series> + UnwindSafe,
    length: impl Wire2Api<Option<usize>> + UnwindSafe,
) -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "tail__method__Series",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_that = that.wire2api();
            let api_length = length.wire2api();
            Series::tail(&api_that, api_length)
        },
    )
}
fn wire_mean__method__Series_impl(port_: MessagePort, that: impl Wire2Api<Series> + UnwindSafe) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "mean__method__Series",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            move |task_callback| Series::mean(&api_that)
        },
    )
}
fn wire_median__method__Series_impl(port_: MessagePort, that: impl Wire2Api<Series> + UnwindSafe) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "median__method__Series",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            move |task_callback| Series::median(&api_that)
        },
    )
}
fn wire_mean_as_series__method__Series_impl(
    port_: MessagePort,
    that: impl Wire2Api<Series> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "mean_as_series__method__Series",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            move |task_callback| Series::mean_as_series(&api_that)
        },
    )
}
fn wire_median_as_series__method__Series_impl(
    port_: MessagePort,
    that: impl Wire2Api<Series> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "median_as_series__method__Series",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            move |task_callback| Series::median_as_series(&api_that)
        },
    )
}
fn wire_estimated_size__method__Series_impl(
    that: impl Wire2Api<Series> + UnwindSafe,
) -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "estimated_size__method__Series",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_that = that.wire2api();
            Series::estimated_size(&api_that)
        },
    )
}
fn wire_add_to__method__Series_impl(
    that: impl Wire2Api<Series> + UnwindSafe,
    other: impl Wire2Api<Series> + UnwindSafe,
) -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "add_to__method__Series",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_that = that.wire2api();
            let api_other = other.wire2api();
            Series::add_to(&api_that, api_other)
        },
    )
}
fn wire_subtract__method__Series_impl(
    that: impl Wire2Api<Series> + UnwindSafe,
    other: impl Wire2Api<Series> + UnwindSafe,
) -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "subtract__method__Series",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_that = that.wire2api();
            let api_other = other.wire2api();
            Series::subtract(&api_that, api_other)
        },
    )
}
fn wire_multiply__method__Series_impl(
    that: impl Wire2Api<Series> + UnwindSafe,
    other: impl Wire2Api<Series> + UnwindSafe,
) -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "multiply__method__Series",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_that = that.wire2api();
            let api_other = other.wire2api();
            Series::multiply(&api_that, api_other)
        },
    )
}
fn wire_divide__method__Series_impl(
    that: impl Wire2Api<Series> + UnwindSafe,
    other: impl Wire2Api<Series> + UnwindSafe,
) -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "divide__method__Series",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_that = that.wire2api();
            let api_other = other.wire2api();
            Series::divide(&api_that, api_other)
        },
    )
}
fn wire_remainder__method__Series_impl(
    that: impl Wire2Api<Series> + UnwindSafe,
    other: impl Wire2Api<Series> + UnwindSafe,
) -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "remainder__method__Series",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_that = that.wire2api();
            let api_other = other.wire2api();
            Series::remainder(&api_that, api_other)
        },
    )
}
fn wire_is_bool__method__Series_impl(
    that: impl Wire2Api<Series> + UnwindSafe,
) -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "is_bool__method__Series",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_that = that.wire2api();
            Series::is_bool(&api_that)
        },
    )
}
fn wire_is_utf8__method__Series_impl(
    that: impl Wire2Api<Series> + UnwindSafe,
) -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "is_utf8__method__Series",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_that = that.wire2api();
            Series::is_utf8(&api_that)
        },
    )
}
fn wire_is_numeric__method__Series_impl(
    that: impl Wire2Api<Series> + UnwindSafe,
) -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "is_numeric__method__Series",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_that = that.wire2api();
            Series::is_numeric(&api_that)
        },
    )
}
fn wire_is_temporal__method__Series_impl(
    that: impl Wire2Api<Series> + UnwindSafe,
) -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "is_temporal__method__Series",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_that = that.wire2api();
            Series::is_temporal(&api_that)
        },
    )
}
fn wire_dump__method__Series_impl(
    that: impl Wire2Api<Series> + UnwindSafe,
) -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "dump__method__Series",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_that = that.wire2api();
            Series::dump(&api_that)
        },
    )
}
fn wire_rename__method__Series_impl(
    that: impl Wire2Api<Series> + UnwindSafe,
    name: impl Wire2Api<String> + UnwindSafe,
) -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "rename__method__Series",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_that = that.wire2api();
            let api_name = name.wire2api();
            Series::rename(&api_that, api_name)
        },
    )
}
// Section: wrapper structs

// Section: static checks

// Section: allocate functions

// Section: related functions

// Section: impl Wire2Api

pub trait Wire2Api<T> {
    fn wire2api(self) -> T;
}

impl<T, S> Wire2Api<Option<T>> for *mut S
where
    *mut S: Wire2Api<T>,
{
    fn wire2api(self) -> Option<T> {
        (!self.is_null()).then(|| self.wire2api())
    }
}

impl Wire2Api<bool> for bool {
    fn wire2api(self) -> bool {
        self
    }
}

impl Wire2Api<f64> for f64 {
    fn wire2api(self) -> f64 {
        self
    }
}

impl Wire2Api<i32> for i32 {
    fn wire2api(self) -> i32 {
        self
    }
}
impl Wire2Api<i64> for i64 {
    fn wire2api(self) -> i64 {
        self
    }
}

impl Wire2Api<u64> for u64 {
    fn wire2api(self) -> u64 {
        self
    }
}
impl Wire2Api<u8> for u8 {
    fn wire2api(self) -> u8 {
        self
    }
}

impl Wire2Api<usize> for usize {
    fn wire2api(self) -> usize {
        self
    }
}
// Section: impl IntoDart

impl support::IntoDart for DataFrame {
    fn into_dart(self) -> support::DartAbi {
        vec![self.0.into_dart()].into_dart()
    }
}
impl support::IntoDartExceptPrimitive for DataFrame {}

impl support::IntoDart for Series {
    fn into_dart(self) -> support::DartAbi {
        vec![self.0.into_dart()].into_dart()
    }
}
impl support::IntoDartExceptPrimitive for Series {}

// Section: executor

support::lazy_static! {
    pub static ref FLUTTER_RUST_BRIDGE_HANDLER: support::DefaultHandler = Default::default();
}

/// cbindgen:ignore
#[cfg(target_family = "wasm")]
#[path = "bridge_generated.web.rs"]
mod web;
#[cfg(target_family = "wasm")]
pub use web::*;

#[cfg(not(target_family = "wasm"))]
#[path = "bridge_generated.io.rs"]
mod io;
#[cfg(not(target_family = "wasm"))]
pub use io::*;
