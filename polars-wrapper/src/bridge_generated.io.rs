use super::*;
// Section: wire functions

#[no_mangle]
pub extern "C" fn wire_read_csv(
    port_: i64,
    path: *mut wire_uint_8_list,
    has_header: *mut bool,
    columns: *mut wire_StringList,
    delimiter: *mut u32,
    comment_char: *mut u32,
    eol_char: *mut u32,
    quote_char: *mut u32,
    skip_rows: usize,
    skip_rows_after_header: usize,
    chunk_size: *mut usize,
    row_count: *mut wire_RowCount,
    encoding: *mut i32,
    n_rows: *mut usize,
    n_threads: *mut usize,
    null_values: *mut wire_NullValues,
    projection: *mut wire_uint_32_list,
    ignore_parser_errors: bool,
    rechunk: bool,
    parse_dates: bool,
) {
    wire_read_csv_impl(
        port_,
        path,
        has_header,
        columns,
        delimiter,
        comment_char,
        eol_char,
        quote_char,
        skip_rows,
        skip_rows_after_header,
        chunk_size,
        row_count,
        encoding,
        n_rows,
        n_threads,
        null_values,
        projection,
        ignore_parser_errors,
        rechunk,
        parse_dates,
    )
}

#[no_mangle]
pub extern "C" fn wire_scan_csv(
    port_: i64,
    path: *mut wire_uint_8_list,
    has_header: *mut bool,
    delimiter: *mut u32,
    comment_char: *mut u32,
    eol_char: *mut u32,
    quote_char: *mut u32,
    skip_rows: usize,
    skip_rows_after_header: usize,
    row_count: *mut wire_RowCount,
    encoding: *mut i32,
    n_rows: *mut usize,
    null_values: *mut wire_NullValues,
    ignore_parser_errors: bool,
    rechunk: bool,
    parse_dates: bool,
    infer_schema_length: *mut usize,
    cache: bool,
) {
    wire_scan_csv_impl(
        port_,
        path,
        has_header,
        delimiter,
        comment_char,
        eol_char,
        quote_char,
        skip_rows,
        skip_rows_after_header,
        row_count,
        encoding,
        n_rows,
        null_values,
        ignore_parser_errors,
        rechunk,
        parse_dates,
        infer_schema_length,
        cache,
    )
}

#[no_mangle]
pub extern "C" fn wire_iter__method__DataFrame(port_: i64, that: wire_DataFrame) {
    wire_iter__method__DataFrame_impl(port_, that)
}

#[no_mangle]
pub extern "C" fn wire_column__method__DataFrame(
    that: wire_DataFrame,
    column: *mut wire_uint_8_list,
) -> support::WireSyncReturn {
    wire_column__method__DataFrame_impl(that, column)
}

#[no_mangle]
pub extern "C" fn wire_columns__method__DataFrame(
    that: wire_DataFrame,
    columns: *mut wire_StringList,
) -> support::WireSyncReturn {
    wire_columns__method__DataFrame_impl(that, columns)
}

#[no_mangle]
pub extern "C" fn wire_dump__method__DataFrame(port_: i64, that: wire_DataFrame) {
    wire_dump__method__DataFrame_impl(port_, that)
}

#[no_mangle]
pub extern "C" fn wire_estimated_size__method__DataFrame(
    that: wire_DataFrame,
) -> support::WireSyncReturn {
    wire_estimated_size__method__DataFrame_impl(that)
}

#[no_mangle]
pub extern "C" fn wire_with_row_count__method__DataFrame(
    port_: i64,
    that: wire_DataFrame,
    name: *mut wire_uint_8_list,
    offset: *mut u32,
) {
    wire_with_row_count__method__DataFrame_impl(port_, that, name, offset)
}

#[no_mangle]
pub extern "C" fn wire_get_column_names__method__DataFrame(
    that: wire_DataFrame,
) -> support::WireSyncReturn {
    wire_get_column_names__method__DataFrame_impl(that)
}

#[no_mangle]
pub extern "C" fn wire_get_columns__method__DataFrame(port_: i64, that: wire_DataFrame) {
    wire_get_columns__method__DataFrame_impl(port_, that)
}

#[no_mangle]
pub extern "C" fn wire_width__method__DataFrame(that: wire_DataFrame) -> support::WireSyncReturn {
    wire_width__method__DataFrame_impl(that)
}

#[no_mangle]
pub extern "C" fn wire_height__method__DataFrame(that: wire_DataFrame) -> support::WireSyncReturn {
    wire_height__method__DataFrame_impl(that)
}

#[no_mangle]
pub extern "C" fn wire_is_empty__method__DataFrame(
    that: wire_DataFrame,
) -> support::WireSyncReturn {
    wire_is_empty__method__DataFrame_impl(that)
}

#[no_mangle]
pub extern "C" fn wire_sample__method__DataFrame(
    port_: i64,
    that: wire_DataFrame,
    n: usize,
    with_replacement: bool,
    shuffle: bool,
    seed: *mut u64,
) {
    wire_sample__method__DataFrame_impl(port_, that, n, with_replacement, shuffle, seed)
}

#[no_mangle]
pub extern "C" fn wire_select__method__DataFrame(
    that: wire_DataFrame,
    columns: *mut wire_StringList,
) -> support::WireSyncReturn {
    wire_select__method__DataFrame_impl(that, columns)
}

#[no_mangle]
pub extern "C" fn wire_head__method__DataFrame(
    that: wire_DataFrame,
    length: *mut usize,
) -> support::WireSyncReturn {
    wire_head__method__DataFrame_impl(that, length)
}

#[no_mangle]
pub extern "C" fn wire_tail__method__DataFrame(
    that: wire_DataFrame,
    length: *mut usize,
) -> support::WireSyncReturn {
    wire_tail__method__DataFrame_impl(that, length)
}

#[no_mangle]
pub extern "C" fn wire_describe__method__DataFrame(
    port_: i64,
    that: wire_DataFrame,
    percentiles: *mut wire_float_64_list,
) {
    wire_describe__method__DataFrame_impl(port_, that, percentiles)
}

#[no_mangle]
pub extern "C" fn wire_drop__method__DataFrame(
    that: wire_DataFrame,
    column: *mut wire_uint_8_list,
) -> support::WireSyncReturn {
    wire_drop__method__DataFrame_impl(that, column)
}

#[no_mangle]
pub extern "C" fn wire_drop_in_place__method__DataFrame(
    that: wire_DataFrame,
    column: *mut wire_uint_8_list,
) -> support::WireSyncReturn {
    wire_drop_in_place__method__DataFrame_impl(that, column)
}

#[no_mangle]
pub extern "C" fn wire_reverse__method__DataFrame(that: wire_DataFrame) -> support::WireSyncReturn {
    wire_reverse__method__DataFrame_impl(that)
}

#[no_mangle]
pub extern "C" fn wire_shape__method__DataFrame(that: wire_DataFrame) -> support::WireSyncReturn {
    wire_shape__method__DataFrame_impl(that)
}

#[no_mangle]
pub extern "C" fn wire_max__method__DataFrame(port_: i64, that: wire_DataFrame) {
    wire_max__method__DataFrame_impl(port_, that)
}

#[no_mangle]
pub extern "C" fn wire_get_row__method__DataFrame(port_: i64, that: wire_DataFrame, index: usize) {
    wire_get_row__method__DataFrame_impl(port_, that, index)
}

#[no_mangle]
pub extern "C" fn wire_lazy__method__take_self__DataFrame(
    that: wire_DataFrame,
    allow_copy: bool,
    projection_pushdown: *mut bool,
    predicate_pushdown: *mut bool,
    type_coercion: *mut bool,
    simplify_expressions: *mut bool,
    slice_pushdown: *mut bool,
    streaming: *mut bool,
) -> support::WireSyncReturn {
    wire_lazy__method__take_self__DataFrame_impl(
        that,
        allow_copy,
        projection_pushdown,
        predicate_pushdown,
        type_coercion,
        simplify_expressions,
        slice_pushdown,
        streaming,
    )
}

#[no_mangle]
pub extern "C" fn wire_select__method__take_self__LazyFrame(
    that: wire_LazyFrame,
    exprs: *mut wire_list_expr,
) -> support::WireSyncReturn {
    wire_select__method__take_self__LazyFrame_impl(that, exprs)
}

#[no_mangle]
pub extern "C" fn wire_filter__method__take_self__LazyFrame(
    that: wire_LazyFrame,
    pred: wire_Expr,
) -> support::WireSyncReturn {
    wire_filter__method__take_self__LazyFrame_impl(that, pred)
}

#[no_mangle]
pub extern "C" fn wire_groupby__method__take_self__LazyFrame(
    that: wire_LazyFrame,
    exprs: *mut wire_list_expr,
    stable: bool,
) -> support::WireSyncReturn {
    wire_groupby__method__take_self__LazyFrame_impl(that, exprs, stable)
}

#[no_mangle]
pub extern "C" fn wire_reverse__method__take_self__LazyFrame(
    that: wire_LazyFrame,
) -> support::WireSyncReturn {
    wire_reverse__method__take_self__LazyFrame_impl(that)
}

#[no_mangle]
pub extern "C" fn wire_with_column__method__take_self__LazyFrame(
    that: wire_LazyFrame,
    expr: wire_Expr,
) -> support::WireSyncReturn {
    wire_with_column__method__take_self__LazyFrame_impl(that, expr)
}

#[no_mangle]
pub extern "C" fn wire_with_columns__method__take_self__LazyFrame(
    that: wire_LazyFrame,
    expr: *mut wire_list_expr,
) -> support::WireSyncReturn {
    wire_with_columns__method__take_self__LazyFrame_impl(that, expr)
}

#[no_mangle]
pub extern "C" fn wire_cache__method__take_self__LazyFrame(
    that: wire_LazyFrame,
) -> support::WireSyncReturn {
    wire_cache__method__take_self__LazyFrame_impl(that)
}

#[no_mangle]
pub extern "C" fn wire_collect__method__take_self__LazyFrame(port_: i64, that: wire_LazyFrame) {
    wire_collect__method__take_self__LazyFrame_impl(port_, that)
}

#[no_mangle]
pub extern "C" fn wire_cross_join__method__take_self__LazyFrame(
    that: wire_LazyFrame,
    other: wire_LazyFrame,
) -> support::WireSyncReturn {
    wire_cross_join__method__take_self__LazyFrame_impl(that, other)
}

#[no_mangle]
pub extern "C" fn wire_left_join__method__take_self__LazyFrame(
    that: wire_LazyFrame,
    other: wire_LazyFrame,
    left_on: wire_Expr,
    right_on: wire_Expr,
) -> support::WireSyncReturn {
    wire_left_join__method__take_self__LazyFrame_impl(that, other, left_on, right_on)
}

#[no_mangle]
pub extern "C" fn wire_outer_join__method__take_self__LazyFrame(
    that: wire_LazyFrame,
    other: wire_LazyFrame,
    left_on: wire_Expr,
    right_on: wire_Expr,
) -> support::WireSyncReturn {
    wire_outer_join__method__take_self__LazyFrame_impl(that, other, left_on, right_on)
}

#[no_mangle]
pub extern "C" fn wire_inner_join__method__take_self__LazyFrame(
    that: wire_LazyFrame,
    other: wire_LazyFrame,
    left_on: wire_Expr,
    right_on: wire_Expr,
) -> support::WireSyncReturn {
    wire_inner_join__method__take_self__LazyFrame_impl(that, other, left_on, right_on)
}

#[no_mangle]
pub extern "C" fn wire_join__method__take_self__LazyFrame(
    that: wire_LazyFrame,
    other: wire_LazyFrame,
    on: *mut wire_list_expr,
    left_on: *mut wire_list_expr,
    right_on: *mut wire_list_expr,
    suffix: *mut wire_uint_8_list,
    how: i32,
    allow_parallel: bool,
    force_parallel: bool,
) -> support::WireSyncReturn {
    wire_join__method__take_self__LazyFrame_impl(
        that,
        other,
        on,
        left_on,
        right_on,
        suffix,
        how,
        allow_parallel,
        force_parallel,
    )
}

#[no_mangle]
pub extern "C" fn wire_max__method__take_self__LazyFrame(
    that: wire_LazyFrame,
) -> support::WireSyncReturn {
    wire_max__method__take_self__LazyFrame_impl(that)
}

#[no_mangle]
pub extern "C" fn wire_min__method__take_self__LazyFrame(
    that: wire_LazyFrame,
) -> support::WireSyncReturn {
    wire_min__method__take_self__LazyFrame_impl(that)
}

#[no_mangle]
pub extern "C" fn wire_sum__method__take_self__LazyFrame(
    that: wire_LazyFrame,
) -> support::WireSyncReturn {
    wire_sum__method__take_self__LazyFrame_impl(that)
}

#[no_mangle]
pub extern "C" fn wire_mean__method__take_self__LazyFrame(
    that: wire_LazyFrame,
) -> support::WireSyncReturn {
    wire_mean__method__take_self__LazyFrame_impl(that)
}

#[no_mangle]
pub extern "C" fn wire_median__method__take_self__LazyFrame(
    that: wire_LazyFrame,
) -> support::WireSyncReturn {
    wire_median__method__take_self__LazyFrame_impl(that)
}

#[no_mangle]
pub extern "C" fn wire_quantile__method__take_self__LazyFrame(
    that: wire_LazyFrame,
    quantile: wire_Expr,
    interpol: i32,
) -> support::WireSyncReturn {
    wire_quantile__method__take_self__LazyFrame_impl(that, quantile, interpol)
}

#[no_mangle]
pub extern "C" fn wire_std__method__take_self__LazyFrame(
    that: wire_LazyFrame,
    ddof: u8,
) -> support::WireSyncReturn {
    wire_std__method__take_self__LazyFrame_impl(that, ddof)
}

#[no_mangle]
pub extern "C" fn wire_variance__method__take_self__LazyFrame(
    that: wire_LazyFrame,
    ddof: u8,
) -> support::WireSyncReturn {
    wire_variance__method__take_self__LazyFrame_impl(that, ddof)
}

#[no_mangle]
pub extern "C" fn wire_explode__method__take_self__LazyFrame(
    that: wire_LazyFrame,
    columns: *mut wire_list_expr,
) -> support::WireSyncReturn {
    wire_explode__method__take_self__LazyFrame_impl(that, columns)
}

#[no_mangle]
pub extern "C" fn wire_unique__method__take_self__LazyFrame(
    that: wire_LazyFrame,
    subset: *mut wire_StringList,
    keep_strategy: i32,
) -> support::WireSyncReturn {
    wire_unique__method__take_self__LazyFrame_impl(that, subset, keep_strategy)
}

#[no_mangle]
pub extern "C" fn wire_drop_nulls__method__take_self__LazyFrame(
    that: wire_LazyFrame,
    subset: *mut wire_list_expr,
) -> support::WireSyncReturn {
    wire_drop_nulls__method__take_self__LazyFrame_impl(that, subset)
}

#[no_mangle]
pub extern "C" fn wire_slice__method__take_self__LazyFrame(
    that: wire_LazyFrame,
    offset: i64,
    len: u32,
) -> support::WireSyncReturn {
    wire_slice__method__take_self__LazyFrame_impl(that, offset, len)
}

#[no_mangle]
pub extern "C" fn wire_first__method__take_self__LazyFrame(
    that: wire_LazyFrame,
) -> support::WireSyncReturn {
    wire_first__method__take_self__LazyFrame_impl(that)
}

#[no_mangle]
pub extern "C" fn wire_last__method__take_self__LazyFrame(
    that: wire_LazyFrame,
) -> support::WireSyncReturn {
    wire_last__method__take_self__LazyFrame_impl(that)
}

#[no_mangle]
pub extern "C" fn wire_tail__method__take_self__LazyFrame(
    that: wire_LazyFrame,
    n: u32,
) -> support::WireSyncReturn {
    wire_tail__method__take_self__LazyFrame_impl(that, n)
}

#[no_mangle]
pub extern "C" fn wire_melt__method__take_self__LazyFrame(
    that: wire_LazyFrame,
    id_vars: *mut wire_StringList,
    value_vars: *mut wire_StringList,
    variable_name: *mut wire_uint_8_list,
    value_name: *mut wire_uint_8_list,
) -> support::WireSyncReturn {
    wire_melt__method__take_self__LazyFrame_impl(
        that,
        id_vars,
        value_vars,
        variable_name,
        value_name,
    )
}

#[no_mangle]
pub extern "C" fn wire_limit__method__take_self__LazyFrame(
    that: wire_LazyFrame,
    n: u32,
) -> support::WireSyncReturn {
    wire_limit__method__take_self__LazyFrame_impl(that, n)
}

#[no_mangle]
pub extern "C" fn wire_fetch__method__take_self__LazyFrame(
    port_: i64,
    that: wire_LazyFrame,
    n_rows: usize,
) {
    wire_fetch__method__take_self__LazyFrame_impl(port_, that, n_rows)
}

#[no_mangle]
pub extern "C" fn wire_with_row_count__method__take_self__LazyFrame(
    that: wire_LazyFrame,
    name: *mut wire_uint_8_list,
    offset: *mut u32,
) -> support::WireSyncReturn {
    wire_with_row_count__method__take_self__LazyFrame_impl(that, name, offset)
}

#[no_mangle]
pub extern "C" fn wire_of_strings__static_method__Series(
    name: *mut wire_uint_8_list,
    values: *mut wire_StringList,
) -> support::WireSyncReturn {
    wire_of_strings__static_method__Series_impl(name, values)
}

#[no_mangle]
pub extern "C" fn wire_of_i32__static_method__Series(
    name: *mut wire_uint_8_list,
    values: *mut wire_int_32_list,
) -> support::WireSyncReturn {
    wire_of_i32__static_method__Series_impl(name, values)
}

#[no_mangle]
pub extern "C" fn wire_of_i64__static_method__Series(
    name: *mut wire_uint_8_list,
    values: *mut wire_int_64_list,
) -> support::WireSyncReturn {
    wire_of_i64__static_method__Series_impl(name, values)
}

#[no_mangle]
pub extern "C" fn wire_of_durations__static_method__Series(
    name: *mut wire_uint_8_list,
    values: *mut wire_int_64_list,
    unit: i32,
) -> support::WireSyncReturn {
    wire_of_durations__static_method__Series_impl(name, values, unit)
}

#[no_mangle]
pub extern "C" fn wire_of_f64__static_method__Series(
    name: *mut wire_uint_8_list,
    values: *mut wire_float_64_list,
) -> support::WireSyncReturn {
    wire_of_f64__static_method__Series_impl(name, values)
}

#[no_mangle]
pub extern "C" fn wire_append__method__Series(port_: i64, that: wire_Series, other: wire_Series) {
    wire_append__method__Series_impl(port_, that, other)
}

#[no_mangle]
pub extern "C" fn wire_as_strings__method__Series(port_: i64, that: wire_Series) {
    wire_as_strings__method__Series_impl(port_, that)
}

#[no_mangle]
pub extern "C" fn wire_as_i32__method__Series(port_: i64, that: wire_Series) {
    wire_as_i32__method__Series_impl(port_, that)
}

#[no_mangle]
pub extern "C" fn wire_as_f64__method__Series(port_: i64, that: wire_Series) {
    wire_as_f64__method__Series_impl(port_, that)
}

#[no_mangle]
pub extern "C" fn wire_as_durations__method__Series(port_: i64, that: wire_Series) {
    wire_as_durations__method__Series_impl(port_, that)
}

#[no_mangle]
pub extern "C" fn wire_as_naive_datetime__method__Series(port_: i64, that: wire_Series) {
    wire_as_naive_datetime__method__Series_impl(port_, that)
}

#[no_mangle]
pub extern "C" fn wire_as_utc_datetime__method__Series(port_: i64, that: wire_Series) {
    wire_as_utc_datetime__method__Series_impl(port_, that)
}

#[no_mangle]
pub extern "C" fn wire_as_local_datetime__method__Series(port_: i64, that: wire_Series) {
    wire_as_local_datetime__method__Series_impl(port_, that)
}

#[no_mangle]
pub extern "C" fn wire_abs__method__Series(port_: i64, that: wire_Series) {
    wire_abs__method__Series_impl(port_, that)
}

#[no_mangle]
pub extern "C" fn wire_sort__method__Series(port_: i64, that: wire_Series, reverse: bool) {
    wire_sort__method__Series_impl(port_, that, reverse)
}

#[no_mangle]
pub extern "C" fn wire_shuffle__method__Series(port_: i64, that: wire_Series, seed: *mut u64) {
    wire_shuffle__method__Series_impl(port_, that, seed)
}

#[no_mangle]
pub extern "C" fn wire_sum__method__Series(port_: i64, that: wire_Series) {
    wire_sum__method__Series_impl(port_, that)
}

#[no_mangle]
pub extern "C" fn wire_sum_as_series__method__Series(port_: i64, that: wire_Series) {
    wire_sum_as_series__method__Series_impl(port_, that)
}

#[no_mangle]
pub extern "C" fn wire_min__method__Series(port_: i64, that: wire_Series) {
    wire_min__method__Series_impl(port_, that)
}

#[no_mangle]
pub extern "C" fn wire_max__method__Series(port_: i64, that: wire_Series) {
    wire_max__method__Series_impl(port_, that)
}

#[no_mangle]
pub extern "C" fn wire_explode__method__Series(port_: i64, that: wire_Series) {
    wire_explode__method__Series_impl(port_, that)
}

#[no_mangle]
pub extern "C" fn wire_explode_by_offsets__method__Series(
    port_: i64,
    that: wire_Series,
    offsets: *mut wire_int_64_list,
) {
    wire_explode_by_offsets__method__Series_impl(port_, that, offsets)
}

#[no_mangle]
pub extern "C" fn wire_cummax__method__Series(port_: i64, that: wire_Series, reverse: bool) {
    wire_cummax__method__Series_impl(port_, that, reverse)
}

#[no_mangle]
pub extern "C" fn wire_cummin__method__Series(port_: i64, that: wire_Series, reverse: bool) {
    wire_cummin__method__Series_impl(port_, that, reverse)
}

#[no_mangle]
pub extern "C" fn wire_cumprod__method__Series(port_: i64, that: wire_Series, reverse: bool) {
    wire_cumprod__method__Series_impl(port_, that, reverse)
}

#[no_mangle]
pub extern "C" fn wire_cumsum__method__Series(port_: i64, that: wire_Series, reverse: bool) {
    wire_cumsum__method__Series_impl(port_, that, reverse)
}

#[no_mangle]
pub extern "C" fn wire_product__method__Series(port_: i64, that: wire_Series) {
    wire_product__method__Series_impl(port_, that)
}

#[no_mangle]
pub extern "C" fn wire_get_string__method__Series(
    that: wire_Series,
    index: usize,
) -> support::WireSyncReturn {
    wire_get_string__method__Series_impl(that, index)
}

#[no_mangle]
pub extern "C" fn wire_get__method__Series(
    that: wire_Series,
    index: usize,
) -> support::WireSyncReturn {
    wire_get__method__Series_impl(that, index)
}

#[no_mangle]
pub extern "C" fn wire_head__method__Series(
    that: wire_Series,
    length: *mut usize,
) -> support::WireSyncReturn {
    wire_head__method__Series_impl(that, length)
}

#[no_mangle]
pub extern "C" fn wire_tail__method__Series(
    that: wire_Series,
    length: *mut usize,
) -> support::WireSyncReturn {
    wire_tail__method__Series_impl(that, length)
}

#[no_mangle]
pub extern "C" fn wire_mean__method__Series(port_: i64, that: wire_Series) {
    wire_mean__method__Series_impl(port_, that)
}

#[no_mangle]
pub extern "C" fn wire_median__method__Series(port_: i64, that: wire_Series) {
    wire_median__method__Series_impl(port_, that)
}

#[no_mangle]
pub extern "C" fn wire_mean_as_series__method__Series(port_: i64, that: wire_Series) {
    wire_mean_as_series__method__Series_impl(port_, that)
}

#[no_mangle]
pub extern "C" fn wire_median_as_series__method__Series(port_: i64, that: wire_Series) {
    wire_median_as_series__method__Series_impl(port_, that)
}

#[no_mangle]
pub extern "C" fn wire_estimated_size__method__Series(
    that: wire_Series,
) -> support::WireSyncReturn {
    wire_estimated_size__method__Series_impl(that)
}

#[no_mangle]
pub extern "C" fn wire_add_to__method__Series(
    that: wire_Series,
    other: wire_Series,
) -> support::WireSyncReturn {
    wire_add_to__method__Series_impl(that, other)
}

#[no_mangle]
pub extern "C" fn wire_subtract__method__Series(
    that: wire_Series,
    other: wire_Series,
) -> support::WireSyncReturn {
    wire_subtract__method__Series_impl(that, other)
}

#[no_mangle]
pub extern "C" fn wire_multiply__method__Series(
    that: wire_Series,
    other: wire_Series,
) -> support::WireSyncReturn {
    wire_multiply__method__Series_impl(that, other)
}

#[no_mangle]
pub extern "C" fn wire_divide__method__Series(
    that: wire_Series,
    other: wire_Series,
) -> support::WireSyncReturn {
    wire_divide__method__Series_impl(that, other)
}

#[no_mangle]
pub extern "C" fn wire_remainder__method__Series(
    that: wire_Series,
    other: wire_Series,
) -> support::WireSyncReturn {
    wire_remainder__method__Series_impl(that, other)
}

#[no_mangle]
pub extern "C" fn wire_is_bool__method__Series(that: wire_Series) -> support::WireSyncReturn {
    wire_is_bool__method__Series_impl(that)
}

#[no_mangle]
pub extern "C" fn wire_is_utf8__method__Series(that: wire_Series) -> support::WireSyncReturn {
    wire_is_utf8__method__Series_impl(that)
}

#[no_mangle]
pub extern "C" fn wire_is_numeric__method__Series(that: wire_Series) -> support::WireSyncReturn {
    wire_is_numeric__method__Series_impl(that)
}

#[no_mangle]
pub extern "C" fn wire_is_temporal__method__Series(that: wire_Series) -> support::WireSyncReturn {
    wire_is_temporal__method__Series_impl(that)
}

#[no_mangle]
pub extern "C" fn wire_dump__method__Series(port_: i64, that: wire_Series) {
    wire_dump__method__Series_impl(port_, that)
}

#[no_mangle]
pub extern "C" fn wire_rename__method__Series(
    that: wire_Series,
    name: *mut wire_uint_8_list,
) -> support::WireSyncReturn {
    wire_rename__method__Series_impl(that, name)
}

#[no_mangle]
pub extern "C" fn wire_unique__method__Series(port_: i64, that: wire_Series, stable: bool) {
    wire_unique__method__Series_impl(port_, that, stable)
}

#[no_mangle]
pub extern "C" fn wire_equal__method__Series(
    port_: i64,
    that: wire_Series,
    other: wire_Series,
    ignore_null: bool,
) {
    wire_equal__method__Series_impl(port_, that, other, ignore_null)
}

#[no_mangle]
pub extern "C" fn wire_reshape__method__Series(
    port_: i64,
    that: wire_Series,
    dims: *mut wire_int_64_list,
) {
    wire_reshape__method__Series_impl(port_, that, dims)
}

#[no_mangle]
pub extern "C" fn wire_std_as_series__method__Series(port_: i64, that: wire_Series, ddof: u8) {
    wire_std_as_series__method__Series_impl(port_, that, ddof)
}

#[no_mangle]
pub extern "C" fn wire_agg__method__take_self__LazyGroupBy(
    that: wire_LazyGroupBy,
    exprs: *mut wire_list_expr,
) -> support::WireSyncReturn {
    wire_agg__method__take_self__LazyGroupBy_impl(that, exprs)
}

#[no_mangle]
pub extern "C" fn wire_head__method__take_self__LazyGroupBy(
    that: wire_LazyGroupBy,
    n: *mut usize,
) -> support::WireSyncReturn {
    wire_head__method__take_self__LazyGroupBy_impl(that, n)
}

#[no_mangle]
pub extern "C" fn wire_tail__method__take_self__LazyGroupBy(
    that: wire_LazyGroupBy,
    n: *mut usize,
) -> support::WireSyncReturn {
    wire_tail__method__take_self__LazyGroupBy_impl(that, n)
}

// Section: allocate functions

#[no_mangle]
pub extern "C" fn new_RwLockPDataFrame() -> wire_RwLockPDataFrame {
    wire_RwLockPDataFrame::new_with_null_ptr()
}

#[no_mangle]
pub extern "C" fn new_RwLockPLazyFrame() -> wire_RwLockPLazyFrame {
    wire_RwLockPLazyFrame::new_with_null_ptr()
}

#[no_mangle]
pub extern "C" fn new_RwLockPLazyGroupBy() -> wire_RwLockPLazyGroupBy {
    wire_RwLockPLazyGroupBy::new_with_null_ptr()
}

#[no_mangle]
pub extern "C" fn new_RwLockPSeries() -> wire_RwLockPSeries {
    wire_RwLockPSeries::new_with_null_ptr()
}

#[no_mangle]
pub extern "C" fn new_StringList_0(len: i32) -> *mut wire_StringList {
    let wrap = wire_StringList {
        ptr: support::new_leak_vec_ptr(<*mut wire_uint_8_list>::new_with_null_ptr(), len),
        len,
    };
    support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_agg_expr_0() -> wire_AggExpr {
    NewWithNullPtr::new_with_null_ptr()
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_agg_expr_0() -> *mut wire_AggExpr {
    support::new_leak_box_ptr(wire_AggExpr::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_bool_0(value: bool) -> *mut bool {
    support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_char_0(value: u32) -> *mut u32 {
    support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_csv_encoding_0(value: i32) -> *mut i32 {
    support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_data_type_0() -> *mut wire_DataType {
    support::new_leak_box_ptr(wire_DataType::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_literal_value_0() -> *mut wire_LiteralValue {
    support::new_leak_box_ptr(wire_LiteralValue::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_null_values_0() -> *mut wire_NullValues {
    support::new_leak_box_ptr(wire_NullValues::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_row_count_0() -> *mut wire_RowCount {
    support::new_leak_box_ptr(wire_RowCount::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_sort_options_0() -> *mut wire_SortOptions {
    support::new_leak_box_ptr(wire_SortOptions::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_u32_0(value: u32) -> *mut u32 {
    support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_u64_0(value: u64) -> *mut u64 {
    support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_usize_0(value: usize) -> *mut usize {
    support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_data_type_0() -> *mut wire_DataType {
    support::new_leak_box_ptr(wire_DataType::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_expr_0() -> *mut wire_Expr {
    support::new_leak_box_ptr(wire_Expr::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_data_frame_0() -> wire_DataFrame {
    NewWithNullPtr::new_with_null_ptr()
}

#[no_mangle]
pub extern "C" fn new_data_type_0() -> wire_DataType {
    NewWithNullPtr::new_with_null_ptr()
}

#[no_mangle]
pub extern "C" fn new_excluded_0() -> wire_Excluded {
    NewWithNullPtr::new_with_null_ptr()
}

#[no_mangle]
pub extern "C" fn new_expr_0() -> wire_Expr {
    NewWithNullPtr::new_with_null_ptr()
}

#[no_mangle]
pub extern "C" fn new_field_0() -> wire_Field {
    NewWithNullPtr::new_with_null_ptr()
}

#[no_mangle]
pub extern "C" fn new_float_64_list_0(len: i32) -> *mut wire_float_64_list {
    let ans = wire_float_64_list {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

#[no_mangle]
pub extern "C" fn new_int_32_list_0(len: i32) -> *mut wire_int_32_list {
    let ans = wire_int_32_list {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

#[no_mangle]
pub extern "C" fn new_int_64_list_0(len: i32) -> *mut wire_int_64_list {
    let ans = wire_int_64_list {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

#[no_mangle]
pub extern "C" fn new_lazy_frame_0() -> wire_LazyFrame {
    NewWithNullPtr::new_with_null_ptr()
}

#[no_mangle]
pub extern "C" fn new_lazy_group_by_0() -> wire_LazyGroupBy {
    NewWithNullPtr::new_with_null_ptr()
}

#[no_mangle]
pub extern "C" fn new_list_data_type_0(len: i32) -> *mut wire_list_data_type {
    let wrap = wire_list_data_type {
        ptr: support::new_leak_vec_ptr(<wire_DataType>::new_with_null_ptr(), len),
        len,
    };
    support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_list_excluded_0(len: i32) -> *mut wire_list_excluded {
    let wrap = wire_list_excluded {
        ptr: support::new_leak_vec_ptr(<wire_Excluded>::new_with_null_ptr(), len),
        len,
    };
    support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_list_expr_0(len: i32) -> *mut wire_list_expr {
    let wrap = wire_list_expr {
        ptr: support::new_leak_vec_ptr(<wire_Expr>::new_with_null_ptr(), len),
        len,
    };
    support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_list_field_0(len: i32) -> *mut wire_list_field {
    let wrap = wire_list_field {
        ptr: support::new_leak_vec_ptr(<wire_Field>::new_with_null_ptr(), len),
        len,
    };
    support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_literal_value_0() -> wire_LiteralValue {
    NewWithNullPtr::new_with_null_ptr()
}

#[no_mangle]
pub extern "C" fn new_null_values_0() -> wire_NullValues {
    NewWithNullPtr::new_with_null_ptr()
}

#[no_mangle]
pub extern "C" fn new_row_count_0() -> wire_RowCount {
    NewWithNullPtr::new_with_null_ptr()
}

#[no_mangle]
pub extern "C" fn new_series_0() -> wire_Series {
    NewWithNullPtr::new_with_null_ptr()
}

#[no_mangle]
pub extern "C" fn new_sort_options_0() -> wire_SortOptions {
    NewWithNullPtr::new_with_null_ptr()
}

#[no_mangle]
pub extern "C" fn new_uint_32_list_0(len: i32) -> *mut wire_uint_32_list {
    let ans = wire_uint_32_list {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

#[no_mangle]
pub extern "C" fn new_uint_8_list_0(len: i32) -> *mut wire_uint_8_list {
    let ans = wire_uint_8_list {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

// Section: related functions

#[no_mangle]
pub extern "C" fn drop_opaque_RwLockPDataFrame(ptr: *const c_void) {
    unsafe {
        Arc::<RwLock<PDataFrame>>::decrement_strong_count(ptr as _);
    }
}

#[no_mangle]
pub extern "C" fn share_opaque_RwLockPDataFrame(ptr: *const c_void) -> *const c_void {
    unsafe {
        Arc::<RwLock<PDataFrame>>::increment_strong_count(ptr as _);
        ptr
    }
}

#[no_mangle]
pub extern "C" fn drop_opaque_RwLockPLazyFrame(ptr: *const c_void) {
    unsafe {
        Arc::<RwLock<PLazyFrame>>::decrement_strong_count(ptr as _);
    }
}

#[no_mangle]
pub extern "C" fn share_opaque_RwLockPLazyFrame(ptr: *const c_void) -> *const c_void {
    unsafe {
        Arc::<RwLock<PLazyFrame>>::increment_strong_count(ptr as _);
        ptr
    }
}

#[no_mangle]
pub extern "C" fn drop_opaque_RwLockPLazyGroupBy(ptr: *const c_void) {
    unsafe {
        Arc::<RwLock<PLazyGroupBy>>::decrement_strong_count(ptr as _);
    }
}

#[no_mangle]
pub extern "C" fn share_opaque_RwLockPLazyGroupBy(ptr: *const c_void) -> *const c_void {
    unsafe {
        Arc::<RwLock<PLazyGroupBy>>::increment_strong_count(ptr as _);
        ptr
    }
}

#[no_mangle]
pub extern "C" fn drop_opaque_RwLockPSeries(ptr: *const c_void) {
    unsafe {
        Arc::<RwLock<PSeries>>::decrement_strong_count(ptr as _);
    }
}

#[no_mangle]
pub extern "C" fn share_opaque_RwLockPSeries(ptr: *const c_void) -> *const c_void {
    unsafe {
        Arc::<RwLock<PSeries>>::increment_strong_count(ptr as _);
        ptr
    }
}

// Section: impl Wire2Api

impl Wire2Api<Arc<str>> for *mut wire_uint_8_list {
    fn wire2api(self) -> Arc<str> {
        let string: String = self.wire2api();
        <Arc<str>>::from(string)
    }
}
impl Wire2Api<chrono::Duration> for i64 {
    fn wire2api(self) -> chrono::Duration {
        chrono::Duration::microseconds(self)
    }
}
impl Wire2Api<Vec<chrono::Duration>> for *mut wire_int_64_list {
    fn wire2api(self) -> Vec<chrono::Duration> {
        let vec: Vec<i64> = self.wire2api();
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}

impl Wire2Api<RustOpaque<RwLock<PDataFrame>>> for wire_RwLockPDataFrame {
    fn wire2api(self) -> RustOpaque<RwLock<PDataFrame>> {
        unsafe { support::opaque_from_dart(self.ptr as _) }
    }
}
impl Wire2Api<RustOpaque<RwLock<PLazyFrame>>> for wire_RwLockPLazyFrame {
    fn wire2api(self) -> RustOpaque<RwLock<PLazyFrame>> {
        unsafe { support::opaque_from_dart(self.ptr as _) }
    }
}
impl Wire2Api<RustOpaque<RwLock<PLazyGroupBy>>> for wire_RwLockPLazyGroupBy {
    fn wire2api(self) -> RustOpaque<RwLock<PLazyGroupBy>> {
        unsafe { support::opaque_from_dart(self.ptr as _) }
    }
}
impl Wire2Api<RustOpaque<RwLock<PSeries>>> for wire_RwLockPSeries {
    fn wire2api(self) -> RustOpaque<RwLock<PSeries>> {
        unsafe { support::opaque_from_dart(self.ptr as _) }
    }
}
impl Wire2Api<String> for *mut wire_uint_8_list {
    fn wire2api(self) -> String {
        let vec: Vec<u8> = self.wire2api();
        String::from_utf8_lossy(&vec).into_owned()
    }
}
impl Wire2Api<Vec<String>> for *mut wire_StringList {
    fn wire2api(self) -> Vec<String> {
        let vec = unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<AggExpr> for wire_AggExpr {
    fn wire2api(self) -> AggExpr {
        match self.tag {
            0 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.Min);
                AggExpr::Min {
                    input: ans.input.wire2api(),
                    propagate_nans: ans.propagate_nans.wire2api(),
                }
            },
            1 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.Max);
                AggExpr::Max {
                    input: ans.input.wire2api(),
                    propagate_nans: ans.propagate_nans.wire2api(),
                }
            },
            2 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.Median);
                AggExpr::Median(ans.field0.wire2api())
            },
            3 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.NUnique);
                AggExpr::NUnique(ans.field0.wire2api())
            },
            4 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.First);
                AggExpr::First(ans.field0.wire2api())
            },
            5 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.Last);
                AggExpr::Last(ans.field0.wire2api())
            },
            6 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.Mean);
                AggExpr::Mean(ans.field0.wire2api())
            },
            7 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.List);
                AggExpr::List(ans.field0.wire2api())
            },
            8 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.Count);
                AggExpr::Count(ans.field0.wire2api())
            },
            9 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.Quantile);
                AggExpr::Quantile {
                    expr: ans.expr.wire2api(),
                    quantile: ans.quantile.wire2api(),
                    interpol: ans.interpol.wire2api(),
                }
            },
            10 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.Sum);
                AggExpr::Sum(ans.field0.wire2api())
            },
            11 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.AggGroups);
                AggExpr::AggGroups(ans.field0.wire2api())
            },
            12 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.Std);
                AggExpr::Std(ans.field0.wire2api(), ans.field1.wire2api())
            },
            _ => unreachable!(),
        }
    }
}

impl Wire2Api<AggExpr> for *mut wire_AggExpr {
    fn wire2api(self) -> AggExpr {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<AggExpr>::wire2api(*wrap).into()
    }
}
impl Wire2Api<bool> for *mut bool {
    fn wire2api(self) -> bool {
        unsafe { *support::box_from_leak_ptr(self) }
    }
}
impl Wire2Api<char> for *mut u32 {
    fn wire2api(self) -> char {
        unsafe { *support::box_from_leak_ptr(self) }.wire2api()
    }
}
impl Wire2Api<CsvEncoding> for *mut i32 {
    fn wire2api(self) -> CsvEncoding {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<CsvEncoding>::wire2api(*wrap).into()
    }
}
impl Wire2Api<DataType> for *mut wire_DataType {
    fn wire2api(self) -> DataType {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<DataType>::wire2api(*wrap).into()
    }
}
impl Wire2Api<LiteralValue> for *mut wire_LiteralValue {
    fn wire2api(self) -> LiteralValue {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<LiteralValue>::wire2api(*wrap).into()
    }
}
impl Wire2Api<NullValues> for *mut wire_NullValues {
    fn wire2api(self) -> NullValues {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<NullValues>::wire2api(*wrap).into()
    }
}
impl Wire2Api<RowCount> for *mut wire_RowCount {
    fn wire2api(self) -> RowCount {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<RowCount>::wire2api(*wrap).into()
    }
}
impl Wire2Api<SortOptions> for *mut wire_SortOptions {
    fn wire2api(self) -> SortOptions {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<SortOptions>::wire2api(*wrap).into()
    }
}
impl Wire2Api<u32> for *mut u32 {
    fn wire2api(self) -> u32 {
        unsafe { *support::box_from_leak_ptr(self) }
    }
}
impl Wire2Api<u64> for *mut u64 {
    fn wire2api(self) -> u64 {
        unsafe { *support::box_from_leak_ptr(self) }
    }
}
impl Wire2Api<usize> for *mut usize {
    fn wire2api(self) -> usize {
        unsafe { *support::box_from_leak_ptr(self) }
    }
}
impl Wire2Api<Box<DataType>> for *mut wire_DataType {
    fn wire2api(self) -> Box<DataType> {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<DataType>::wire2api(*wrap).into()
    }
}
impl Wire2Api<Box<Expr>> for *mut wire_Expr {
    fn wire2api(self) -> Box<Expr> {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<Expr>::wire2api(*wrap).into()
    }
}

impl Wire2Api<DataFrame> for wire_DataFrame {
    fn wire2api(self) -> DataFrame {
        DataFrame(self.field0.wire2api())
    }
}
impl Wire2Api<DataType> for wire_DataType {
    fn wire2api(self) -> DataType {
        match self.tag {
            0 => DataType::Boolean,
            1 => DataType::UInt8,
            2 => DataType::UInt16,
            3 => DataType::UInt32,
            4 => DataType::UInt64,
            5 => DataType::Int8,
            6 => DataType::Int16,
            7 => DataType::Int32,
            8 => DataType::Int64,
            9 => DataType::Float32,
            10 => DataType::Float64,
            11 => DataType::Utf8,
            12 => DataType::Binary,
            13 => DataType::Date,
            14 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.Datetime);
                DataType::Datetime(ans.field0.wire2api(), ans.field1.wire2api())
            },
            15 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.Duration);
                DataType::Duration(ans.field0.wire2api())
            },
            16 => DataType::Time,
            17 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.List);
                DataType::List(ans.field0.wire2api())
            },
            18 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.Struct);
                DataType::Struct(ans.field0.wire2api())
            },
            19 => DataType::Unknown,
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<Excluded> for wire_Excluded {
    fn wire2api(self) -> Excluded {
        match self.tag {
            0 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.Name);
                Excluded::Name(ans.field0.wire2api())
            },
            1 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.Dtype);
                Excluded::Dtype(ans.field0.wire2api())
            },
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<Expr> for wire_Expr {
    fn wire2api(self) -> Expr {
        match self.tag {
            0 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.Alias);
                Expr::Alias(ans.field0.wire2api(), ans.field1.wire2api())
            },
            1 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.Column);
                Expr::Column(ans.field0.wire2api())
            },
            2 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.Columns);
                Expr::Columns(ans.field0.wire2api())
            },
            3 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.DtypeColumn);
                Expr::DtypeColumn(ans.field0.wire2api())
            },
            4 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.Literal);
                Expr::Literal(ans.field0.wire2api())
            },
            5 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.BinaryExpr);
                Expr::BinaryExpr {
                    left: ans.left.wire2api(),
                    op: ans.op.wire2api(),
                    right: ans.right.wire2api(),
                }
            },
            6 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.Cast);
                Expr::Cast {
                    expr: ans.expr.wire2api(),
                    data_type: ans.data_type.wire2api(),
                    strict: ans.strict.wire2api(),
                }
            },
            7 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.Sort);
                Expr::Sort {
                    expr: ans.expr.wire2api(),
                    options: ans.options.wire2api(),
                }
            },
            8 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.Take);
                Expr::Take {
                    expr: ans.expr.wire2api(),
                    idx: ans.idx.wire2api(),
                }
            },
            9 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.Agg);
                Expr::Agg(ans.field0.wire2api())
            },
            10 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.Ternary);
                Expr::Ternary {
                    predicate: ans.predicate.wire2api(),
                    truthy: ans.truthy.wire2api(),
                    falsy: ans.falsy.wire2api(),
                }
            },
            11 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.Explode);
                Expr::Explode(ans.field0.wire2api())
            },
            12 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.Filter);
                Expr::Filter {
                    input: ans.input.wire2api(),
                    by: ans.by.wire2api(),
                }
            },
            13 => Expr::Wildcard,
            14 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.Slice);
                Expr::Slice {
                    input: ans.input.wire2api(),
                    offset: ans.offset.wire2api(),
                    length: ans.length.wire2api(),
                }
            },
            15 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.Exclude);
                Expr::Exclude(ans.field0.wire2api(), ans.field1.wire2api())
            },
            16 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.KeepName);
                Expr::KeepName(ans.field0.wire2api())
            },
            17 => Expr::Count,
            18 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.Nth);
                Expr::Nth(ans.field0.wire2api())
            },
            _ => unreachable!(),
        }
    }
}

impl Wire2Api<Field> for wire_Field {
    fn wire2api(self) -> Field {
        Field {
            name: self.name.wire2api(),
            dtype: self.dtype.wire2api(),
        }
    }
}
impl Wire2Api<Vec<f64>> for *mut wire_float_64_list {
    fn wire2api(self) -> Vec<f64> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}

impl Wire2Api<Vec<i32>> for *mut wire_int_32_list {
    fn wire2api(self) -> Vec<i32> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}
impl Wire2Api<Vec<i64>> for *mut wire_int_64_list {
    fn wire2api(self) -> Vec<i64> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}

impl Wire2Api<LazyFrame> for wire_LazyFrame {
    fn wire2api(self) -> LazyFrame {
        LazyFrame(self.field0.wire2api())
    }
}
impl Wire2Api<LazyGroupBy> for wire_LazyGroupBy {
    fn wire2api(self) -> LazyGroupBy {
        LazyGroupBy(self.field0.wire2api())
    }
}
impl Wire2Api<Vec<DataType>> for *mut wire_list_data_type {
    fn wire2api(self) -> Vec<DataType> {
        let vec = unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<Vec<Excluded>> for *mut wire_list_excluded {
    fn wire2api(self) -> Vec<Excluded> {
        let vec = unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<Vec<Expr>> for *mut wire_list_expr {
    fn wire2api(self) -> Vec<Expr> {
        let vec = unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<Vec<Field>> for *mut wire_list_field {
    fn wire2api(self) -> Vec<Field> {
        let vec = unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<LiteralValue> for wire_LiteralValue {
    fn wire2api(self) -> LiteralValue {
        match self.tag {
            0 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.Boolean);
                LiteralValue::Boolean(ans.field0.wire2api())
            },
            1 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.Utf8);
                LiteralValue::Utf8(ans.field0.wire2api())
            },
            2 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.Binary);
                LiteralValue::Binary(ans.field0.wire2api())
            },
            3 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.UInt8);
                LiteralValue::UInt8(ans.field0.wire2api())
            },
            4 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.UInt16);
                LiteralValue::UInt16(ans.field0.wire2api())
            },
            5 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.UInt32);
                LiteralValue::UInt32(ans.field0.wire2api())
            },
            6 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.UInt64);
                LiteralValue::UInt64(ans.field0.wire2api())
            },
            7 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.Int8);
                LiteralValue::Int8(ans.field0.wire2api())
            },
            8 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.Int16);
                LiteralValue::Int16(ans.field0.wire2api())
            },
            9 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.Int32);
                LiteralValue::Int32(ans.field0.wire2api())
            },
            10 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.Int64);
                LiteralValue::Int64(ans.field0.wire2api())
            },
            11 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.Float32);
                LiteralValue::Float32(ans.field0.wire2api())
            },
            12 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.Float64);
                LiteralValue::Float64(ans.field0.wire2api())
            },
            13 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.Range);
                LiteralValue::Range {
                    low: ans.low.wire2api(),
                    high: ans.high.wire2api(),
                    data_type: ans.data_type.wire2api(),
                }
            },
            14 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.DateTime);
                LiteralValue::DateTime(ans.field0.wire2api(), ans.field1.wire2api())
            },
            15 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.Duration);
                LiteralValue::Duration(ans.field0.wire2api(), ans.field1.wire2api())
            },
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<NullValues> for wire_NullValues {
    fn wire2api(self) -> NullValues {
        match self.tag {
            0 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.AllColumnsSingle);
                NullValues::AllColumnsSingle(ans.field0.wire2api())
            },
            1 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.AllColumns);
                NullValues::AllColumns(ans.field0.wire2api())
            },
            _ => unreachable!(),
        }
    }
}

impl Wire2Api<RowCount> for wire_RowCount {
    fn wire2api(self) -> RowCount {
        RowCount {
            name: self.name.wire2api(),
            offset: self.offset.wire2api(),
        }
    }
}
impl Wire2Api<Series> for wire_Series {
    fn wire2api(self) -> Series {
        Series(self.field0.wire2api())
    }
}
impl Wire2Api<SortOptions> for wire_SortOptions {
    fn wire2api(self) -> SortOptions {
        SortOptions {
            descending: self.descending.wire2api(),
            nulls_last: self.nulls_last.wire2api(),
        }
    }
}

impl Wire2Api<Vec<u32>> for *mut wire_uint_32_list {
    fn wire2api(self) -> Vec<u32> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}
impl Wire2Api<Vec<u8>> for *mut wire_uint_8_list {
    fn wire2api(self) -> Vec<u8> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}

// Section: wire structs

#[repr(C)]
#[derive(Clone)]
pub struct wire_RwLockPDataFrame {
    ptr: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_RwLockPLazyFrame {
    ptr: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_RwLockPLazyGroupBy {
    ptr: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_RwLockPSeries {
    ptr: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_StringList {
    ptr: *mut *mut wire_uint_8_list,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_DataFrame {
    field0: wire_RwLockPDataFrame,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_Field {
    name: *mut wire_uint_8_list,
    dtype: wire_DataType,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_float_64_list {
    ptr: *mut f64,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_int_32_list {
    ptr: *mut i32,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_int_64_list {
    ptr: *mut i64,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_LazyFrame {
    field0: wire_RwLockPLazyFrame,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_LazyGroupBy {
    field0: wire_RwLockPLazyGroupBy,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_data_type {
    ptr: *mut wire_DataType,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_excluded {
    ptr: *mut wire_Excluded,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_expr {
    ptr: *mut wire_Expr,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_field {
    ptr: *mut wire_Field,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_RowCount {
    name: *mut wire_uint_8_list,
    offset: u32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_Series {
    field0: wire_RwLockPSeries,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_SortOptions {
    descending: bool,
    nulls_last: bool,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_uint_32_list {
    ptr: *mut u32,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_uint_8_list {
    ptr: *mut u8,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_AggExpr {
    tag: i32,
    kind: *mut AggExprKind,
}

#[repr(C)]
pub union AggExprKind {
    Min: *mut wire_AggExpr_Min,
    Max: *mut wire_AggExpr_Max,
    Median: *mut wire_AggExpr_Median,
    NUnique: *mut wire_AggExpr_NUnique,
    First: *mut wire_AggExpr_First,
    Last: *mut wire_AggExpr_Last,
    Mean: *mut wire_AggExpr_Mean,
    List: *mut wire_AggExpr_List,
    Count: *mut wire_AggExpr_Count,
    Quantile: *mut wire_AggExpr_Quantile,
    Sum: *mut wire_AggExpr_Sum,
    AggGroups: *mut wire_AggExpr_AggGroups,
    Std: *mut wire_AggExpr_Std,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_AggExpr_Min {
    input: *mut wire_Expr,
    propagate_nans: bool,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_AggExpr_Max {
    input: *mut wire_Expr,
    propagate_nans: bool,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_AggExpr_Median {
    field0: *mut wire_Expr,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_AggExpr_NUnique {
    field0: *mut wire_Expr,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_AggExpr_First {
    field0: *mut wire_Expr,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_AggExpr_Last {
    field0: *mut wire_Expr,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_AggExpr_Mean {
    field0: *mut wire_Expr,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_AggExpr_List {
    field0: *mut wire_Expr,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_AggExpr_Count {
    field0: *mut wire_Expr,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_AggExpr_Quantile {
    expr: *mut wire_Expr,
    quantile: *mut wire_Expr,
    interpol: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_AggExpr_Sum {
    field0: *mut wire_Expr,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_AggExpr_AggGroups {
    field0: *mut wire_Expr,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_AggExpr_Std {
    field0: *mut wire_Expr,
    field1: u8,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_DataType {
    tag: i32,
    kind: *mut DataTypeKind,
}

#[repr(C)]
pub union DataTypeKind {
    Boolean: *mut wire_DataType_Boolean,
    UInt8: *mut wire_DataType_UInt8,
    UInt16: *mut wire_DataType_UInt16,
    UInt32: *mut wire_DataType_UInt32,
    UInt64: *mut wire_DataType_UInt64,
    Int8: *mut wire_DataType_Int8,
    Int16: *mut wire_DataType_Int16,
    Int32: *mut wire_DataType_Int32,
    Int64: *mut wire_DataType_Int64,
    Float32: *mut wire_DataType_Float32,
    Float64: *mut wire_DataType_Float64,
    Utf8: *mut wire_DataType_Utf8,
    Binary: *mut wire_DataType_Binary,
    Date: *mut wire_DataType_Date,
    Datetime: *mut wire_DataType_Datetime,
    Duration: *mut wire_DataType_Duration,
    Time: *mut wire_DataType_Time,
    List: *mut wire_DataType_List,
    Struct: *mut wire_DataType_Struct,
    Unknown: *mut wire_DataType_Unknown,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_DataType_Boolean {}

#[repr(C)]
#[derive(Clone)]
pub struct wire_DataType_UInt8 {}

#[repr(C)]
#[derive(Clone)]
pub struct wire_DataType_UInt16 {}

#[repr(C)]
#[derive(Clone)]
pub struct wire_DataType_UInt32 {}

#[repr(C)]
#[derive(Clone)]
pub struct wire_DataType_UInt64 {}

#[repr(C)]
#[derive(Clone)]
pub struct wire_DataType_Int8 {}

#[repr(C)]
#[derive(Clone)]
pub struct wire_DataType_Int16 {}

#[repr(C)]
#[derive(Clone)]
pub struct wire_DataType_Int32 {}

#[repr(C)]
#[derive(Clone)]
pub struct wire_DataType_Int64 {}

#[repr(C)]
#[derive(Clone)]
pub struct wire_DataType_Float32 {}

#[repr(C)]
#[derive(Clone)]
pub struct wire_DataType_Float64 {}

#[repr(C)]
#[derive(Clone)]
pub struct wire_DataType_Utf8 {}

#[repr(C)]
#[derive(Clone)]
pub struct wire_DataType_Binary {}

#[repr(C)]
#[derive(Clone)]
pub struct wire_DataType_Date {}

#[repr(C)]
#[derive(Clone)]
pub struct wire_DataType_Datetime {
    field0: i32,
    field1: *mut wire_uint_8_list,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_DataType_Duration {
    field0: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_DataType_Time {}

#[repr(C)]
#[derive(Clone)]
pub struct wire_DataType_List {
    field0: *mut wire_DataType,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_DataType_Struct {
    field0: *mut wire_list_field,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_DataType_Unknown {}
#[repr(C)]
#[derive(Clone)]
pub struct wire_Excluded {
    tag: i32,
    kind: *mut ExcludedKind,
}

#[repr(C)]
pub union ExcludedKind {
    Name: *mut wire_Excluded_Name,
    Dtype: *mut wire_Excluded_Dtype,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_Excluded_Name {
    field0: *mut wire_uint_8_list,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_Excluded_Dtype {
    field0: *mut wire_DataType,
}
#[repr(C)]
#[derive(Clone)]
pub struct wire_Expr {
    tag: i32,
    kind: *mut ExprKind,
}

#[repr(C)]
pub union ExprKind {
    Alias: *mut wire_Expr_Alias,
    Column: *mut wire_Expr_Column,
    Columns: *mut wire_Expr_Columns,
    DtypeColumn: *mut wire_Expr_DtypeColumn,
    Literal: *mut wire_Expr_Literal,
    BinaryExpr: *mut wire_Expr_BinaryExpr,
    Cast: *mut wire_Expr_Cast,
    Sort: *mut wire_Expr_Sort,
    Take: *mut wire_Expr_Take,
    Agg: *mut wire_Expr_Agg,
    Ternary: *mut wire_Expr_Ternary,
    Explode: *mut wire_Expr_Explode,
    Filter: *mut wire_Expr_Filter,
    Wildcard: *mut wire_Expr_Wildcard,
    Slice: *mut wire_Expr_Slice,
    Exclude: *mut wire_Expr_Exclude,
    KeepName: *mut wire_Expr_KeepName,
    Count: *mut wire_Expr_Count,
    Nth: *mut wire_Expr_Nth,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_Expr_Alias {
    field0: *mut wire_Expr,
    field1: *mut wire_uint_8_list,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_Expr_Column {
    field0: *mut wire_uint_8_list,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_Expr_Columns {
    field0: *mut wire_StringList,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_Expr_DtypeColumn {
    field0: *mut wire_list_data_type,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_Expr_Literal {
    field0: *mut wire_LiteralValue,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_Expr_BinaryExpr {
    left: *mut wire_Expr,
    op: i32,
    right: *mut wire_Expr,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_Expr_Cast {
    expr: *mut wire_Expr,
    data_type: *mut wire_DataType,
    strict: bool,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_Expr_Sort {
    expr: *mut wire_Expr,
    options: *mut wire_SortOptions,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_Expr_Take {
    expr: *mut wire_Expr,
    idx: *mut wire_Expr,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_Expr_Agg {
    field0: *mut wire_AggExpr,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_Expr_Ternary {
    predicate: *mut wire_Expr,
    truthy: *mut wire_Expr,
    falsy: *mut wire_Expr,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_Expr_Explode {
    field0: *mut wire_Expr,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_Expr_Filter {
    input: *mut wire_Expr,
    by: *mut wire_Expr,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_Expr_Wildcard {}

#[repr(C)]
#[derive(Clone)]
pub struct wire_Expr_Slice {
    input: *mut wire_Expr,
    offset: *mut wire_Expr,
    length: *mut wire_Expr,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_Expr_Exclude {
    field0: *mut wire_Expr,
    field1: *mut wire_list_excluded,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_Expr_KeepName {
    field0: *mut wire_Expr,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_Expr_Count {}

#[repr(C)]
#[derive(Clone)]
pub struct wire_Expr_Nth {
    field0: i64,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_LiteralValue {
    tag: i32,
    kind: *mut LiteralValueKind,
}

#[repr(C)]
pub union LiteralValueKind {
    Boolean: *mut wire_LiteralValue_Boolean,
    Utf8: *mut wire_LiteralValue_Utf8,
    Binary: *mut wire_LiteralValue_Binary,
    UInt8: *mut wire_LiteralValue_UInt8,
    UInt16: *mut wire_LiteralValue_UInt16,
    UInt32: *mut wire_LiteralValue_UInt32,
    UInt64: *mut wire_LiteralValue_UInt64,
    Int8: *mut wire_LiteralValue_Int8,
    Int16: *mut wire_LiteralValue_Int16,
    Int32: *mut wire_LiteralValue_Int32,
    Int64: *mut wire_LiteralValue_Int64,
    Float32: *mut wire_LiteralValue_Float32,
    Float64: *mut wire_LiteralValue_Float64,
    Range: *mut wire_LiteralValue_Range,
    DateTime: *mut wire_LiteralValue_DateTime,
    Duration: *mut wire_LiteralValue_Duration,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_LiteralValue_Boolean {
    field0: bool,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_LiteralValue_Utf8 {
    field0: *mut wire_uint_8_list,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_LiteralValue_Binary {
    field0: *mut wire_uint_8_list,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_LiteralValue_UInt8 {
    field0: u8,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_LiteralValue_UInt16 {
    field0: u16,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_LiteralValue_UInt32 {
    field0: u32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_LiteralValue_UInt64 {
    field0: u64,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_LiteralValue_Int8 {
    field0: i8,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_LiteralValue_Int16 {
    field0: i16,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_LiteralValue_Int32 {
    field0: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_LiteralValue_Int64 {
    field0: i64,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_LiteralValue_Float32 {
    field0: f32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_LiteralValue_Float64 {
    field0: f64,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_LiteralValue_Range {
    low: i64,
    high: i64,
    data_type: *mut wire_DataType,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_LiteralValue_DateTime {
    field0: i64,
    field1: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_LiteralValue_Duration {
    field0: i64,
    field1: i32,
}
#[repr(C)]
#[derive(Clone)]
pub struct wire_NullValues {
    tag: i32,
    kind: *mut NullValuesKind,
}

#[repr(C)]
pub union NullValuesKind {
    AllColumnsSingle: *mut wire_NullValues_AllColumnsSingle,
    AllColumns: *mut wire_NullValues_AllColumns,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_NullValues_AllColumnsSingle {
    field0: *mut wire_uint_8_list,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_NullValues_AllColumns {
    field0: *mut wire_StringList,
}

// Section: impl NewWithNullPtr

pub trait NewWithNullPtr {
    fn new_with_null_ptr() -> Self;
}

impl<T> NewWithNullPtr for *mut T {
    fn new_with_null_ptr() -> Self {
        std::ptr::null_mut()
    }
}

impl NewWithNullPtr for wire_RwLockPDataFrame {
    fn new_with_null_ptr() -> Self {
        Self {
            ptr: core::ptr::null(),
        }
    }
}
impl NewWithNullPtr for wire_RwLockPLazyFrame {
    fn new_with_null_ptr() -> Self {
        Self {
            ptr: core::ptr::null(),
        }
    }
}
impl NewWithNullPtr for wire_RwLockPLazyGroupBy {
    fn new_with_null_ptr() -> Self {
        Self {
            ptr: core::ptr::null(),
        }
    }
}
impl NewWithNullPtr for wire_RwLockPSeries {
    fn new_with_null_ptr() -> Self {
        Self {
            ptr: core::ptr::null(),
        }
    }
}

impl NewWithNullPtr for wire_AggExpr {
    fn new_with_null_ptr() -> Self {
        Self {
            tag: -1,
            kind: core::ptr::null_mut(),
        }
    }
}

#[no_mangle]
pub extern "C" fn inflate_AggExpr_Min() -> *mut AggExprKind {
    support::new_leak_box_ptr(AggExprKind {
        Min: support::new_leak_box_ptr(wire_AggExpr_Min {
            input: core::ptr::null_mut(),
            propagate_nans: Default::default(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_AggExpr_Max() -> *mut AggExprKind {
    support::new_leak_box_ptr(AggExprKind {
        Max: support::new_leak_box_ptr(wire_AggExpr_Max {
            input: core::ptr::null_mut(),
            propagate_nans: Default::default(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_AggExpr_Median() -> *mut AggExprKind {
    support::new_leak_box_ptr(AggExprKind {
        Median: support::new_leak_box_ptr(wire_AggExpr_Median {
            field0: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_AggExpr_NUnique() -> *mut AggExprKind {
    support::new_leak_box_ptr(AggExprKind {
        NUnique: support::new_leak_box_ptr(wire_AggExpr_NUnique {
            field0: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_AggExpr_First() -> *mut AggExprKind {
    support::new_leak_box_ptr(AggExprKind {
        First: support::new_leak_box_ptr(wire_AggExpr_First {
            field0: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_AggExpr_Last() -> *mut AggExprKind {
    support::new_leak_box_ptr(AggExprKind {
        Last: support::new_leak_box_ptr(wire_AggExpr_Last {
            field0: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_AggExpr_Mean() -> *mut AggExprKind {
    support::new_leak_box_ptr(AggExprKind {
        Mean: support::new_leak_box_ptr(wire_AggExpr_Mean {
            field0: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_AggExpr_List() -> *mut AggExprKind {
    support::new_leak_box_ptr(AggExprKind {
        List: support::new_leak_box_ptr(wire_AggExpr_List {
            field0: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_AggExpr_Count() -> *mut AggExprKind {
    support::new_leak_box_ptr(AggExprKind {
        Count: support::new_leak_box_ptr(wire_AggExpr_Count {
            field0: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_AggExpr_Quantile() -> *mut AggExprKind {
    support::new_leak_box_ptr(AggExprKind {
        Quantile: support::new_leak_box_ptr(wire_AggExpr_Quantile {
            expr: core::ptr::null_mut(),
            quantile: core::ptr::null_mut(),
            interpol: Default::default(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_AggExpr_Sum() -> *mut AggExprKind {
    support::new_leak_box_ptr(AggExprKind {
        Sum: support::new_leak_box_ptr(wire_AggExpr_Sum {
            field0: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_AggExpr_AggGroups() -> *mut AggExprKind {
    support::new_leak_box_ptr(AggExprKind {
        AggGroups: support::new_leak_box_ptr(wire_AggExpr_AggGroups {
            field0: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_AggExpr_Std() -> *mut AggExprKind {
    support::new_leak_box_ptr(AggExprKind {
        Std: support::new_leak_box_ptr(wire_AggExpr_Std {
            field0: core::ptr::null_mut(),
            field1: Default::default(),
        }),
    })
}

impl NewWithNullPtr for wire_DataFrame {
    fn new_with_null_ptr() -> Self {
        Self {
            field0: wire_RwLockPDataFrame::new_with_null_ptr(),
        }
    }
}

impl NewWithNullPtr for wire_DataType {
    fn new_with_null_ptr() -> Self {
        Self {
            tag: -1,
            kind: core::ptr::null_mut(),
        }
    }
}

#[no_mangle]
pub extern "C" fn inflate_DataType_Datetime() -> *mut DataTypeKind {
    support::new_leak_box_ptr(DataTypeKind {
        Datetime: support::new_leak_box_ptr(wire_DataType_Datetime {
            field0: Default::default(),
            field1: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_DataType_Duration() -> *mut DataTypeKind {
    support::new_leak_box_ptr(DataTypeKind {
        Duration: support::new_leak_box_ptr(wire_DataType_Duration {
            field0: Default::default(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_DataType_List() -> *mut DataTypeKind {
    support::new_leak_box_ptr(DataTypeKind {
        List: support::new_leak_box_ptr(wire_DataType_List {
            field0: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_DataType_Struct() -> *mut DataTypeKind {
    support::new_leak_box_ptr(DataTypeKind {
        Struct: support::new_leak_box_ptr(wire_DataType_Struct {
            field0: core::ptr::null_mut(),
        }),
    })
}

impl NewWithNullPtr for wire_Excluded {
    fn new_with_null_ptr() -> Self {
        Self {
            tag: -1,
            kind: core::ptr::null_mut(),
        }
    }
}

#[no_mangle]
pub extern "C" fn inflate_Excluded_Name() -> *mut ExcludedKind {
    support::new_leak_box_ptr(ExcludedKind {
        Name: support::new_leak_box_ptr(wire_Excluded_Name {
            field0: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_Excluded_Dtype() -> *mut ExcludedKind {
    support::new_leak_box_ptr(ExcludedKind {
        Dtype: support::new_leak_box_ptr(wire_Excluded_Dtype {
            field0: core::ptr::null_mut(),
        }),
    })
}

impl NewWithNullPtr for wire_Expr {
    fn new_with_null_ptr() -> Self {
        Self {
            tag: -1,
            kind: core::ptr::null_mut(),
        }
    }
}

#[no_mangle]
pub extern "C" fn inflate_Expr_Alias() -> *mut ExprKind {
    support::new_leak_box_ptr(ExprKind {
        Alias: support::new_leak_box_ptr(wire_Expr_Alias {
            field0: core::ptr::null_mut(),
            field1: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_Expr_Column() -> *mut ExprKind {
    support::new_leak_box_ptr(ExprKind {
        Column: support::new_leak_box_ptr(wire_Expr_Column {
            field0: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_Expr_Columns() -> *mut ExprKind {
    support::new_leak_box_ptr(ExprKind {
        Columns: support::new_leak_box_ptr(wire_Expr_Columns {
            field0: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_Expr_DtypeColumn() -> *mut ExprKind {
    support::new_leak_box_ptr(ExprKind {
        DtypeColumn: support::new_leak_box_ptr(wire_Expr_DtypeColumn {
            field0: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_Expr_Literal() -> *mut ExprKind {
    support::new_leak_box_ptr(ExprKind {
        Literal: support::new_leak_box_ptr(wire_Expr_Literal {
            field0: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_Expr_BinaryExpr() -> *mut ExprKind {
    support::new_leak_box_ptr(ExprKind {
        BinaryExpr: support::new_leak_box_ptr(wire_Expr_BinaryExpr {
            left: core::ptr::null_mut(),
            op: Default::default(),
            right: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_Expr_Cast() -> *mut ExprKind {
    support::new_leak_box_ptr(ExprKind {
        Cast: support::new_leak_box_ptr(wire_Expr_Cast {
            expr: core::ptr::null_mut(),
            data_type: core::ptr::null_mut(),
            strict: Default::default(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_Expr_Sort() -> *mut ExprKind {
    support::new_leak_box_ptr(ExprKind {
        Sort: support::new_leak_box_ptr(wire_Expr_Sort {
            expr: core::ptr::null_mut(),
            options: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_Expr_Take() -> *mut ExprKind {
    support::new_leak_box_ptr(ExprKind {
        Take: support::new_leak_box_ptr(wire_Expr_Take {
            expr: core::ptr::null_mut(),
            idx: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_Expr_Agg() -> *mut ExprKind {
    support::new_leak_box_ptr(ExprKind {
        Agg: support::new_leak_box_ptr(wire_Expr_Agg {
            field0: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_Expr_Ternary() -> *mut ExprKind {
    support::new_leak_box_ptr(ExprKind {
        Ternary: support::new_leak_box_ptr(wire_Expr_Ternary {
            predicate: core::ptr::null_mut(),
            truthy: core::ptr::null_mut(),
            falsy: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_Expr_Explode() -> *mut ExprKind {
    support::new_leak_box_ptr(ExprKind {
        Explode: support::new_leak_box_ptr(wire_Expr_Explode {
            field0: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_Expr_Filter() -> *mut ExprKind {
    support::new_leak_box_ptr(ExprKind {
        Filter: support::new_leak_box_ptr(wire_Expr_Filter {
            input: core::ptr::null_mut(),
            by: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_Expr_Slice() -> *mut ExprKind {
    support::new_leak_box_ptr(ExprKind {
        Slice: support::new_leak_box_ptr(wire_Expr_Slice {
            input: core::ptr::null_mut(),
            offset: core::ptr::null_mut(),
            length: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_Expr_Exclude() -> *mut ExprKind {
    support::new_leak_box_ptr(ExprKind {
        Exclude: support::new_leak_box_ptr(wire_Expr_Exclude {
            field0: core::ptr::null_mut(),
            field1: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_Expr_KeepName() -> *mut ExprKind {
    support::new_leak_box_ptr(ExprKind {
        KeepName: support::new_leak_box_ptr(wire_Expr_KeepName {
            field0: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_Expr_Nth() -> *mut ExprKind {
    support::new_leak_box_ptr(ExprKind {
        Nth: support::new_leak_box_ptr(wire_Expr_Nth {
            field0: Default::default(),
        }),
    })
}

impl NewWithNullPtr for wire_Field {
    fn new_with_null_ptr() -> Self {
        Self {
            name: core::ptr::null_mut(),
            dtype: wire_DataType::new_with_null_ptr(),
        }
    }
}

impl NewWithNullPtr for wire_LazyFrame {
    fn new_with_null_ptr() -> Self {
        Self {
            field0: wire_RwLockPLazyFrame::new_with_null_ptr(),
        }
    }
}

impl NewWithNullPtr for wire_LazyGroupBy {
    fn new_with_null_ptr() -> Self {
        Self {
            field0: wire_RwLockPLazyGroupBy::new_with_null_ptr(),
        }
    }
}

impl NewWithNullPtr for wire_LiteralValue {
    fn new_with_null_ptr() -> Self {
        Self {
            tag: -1,
            kind: core::ptr::null_mut(),
        }
    }
}

#[no_mangle]
pub extern "C" fn inflate_LiteralValue_Boolean() -> *mut LiteralValueKind {
    support::new_leak_box_ptr(LiteralValueKind {
        Boolean: support::new_leak_box_ptr(wire_LiteralValue_Boolean {
            field0: Default::default(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_LiteralValue_Utf8() -> *mut LiteralValueKind {
    support::new_leak_box_ptr(LiteralValueKind {
        Utf8: support::new_leak_box_ptr(wire_LiteralValue_Utf8 {
            field0: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_LiteralValue_Binary() -> *mut LiteralValueKind {
    support::new_leak_box_ptr(LiteralValueKind {
        Binary: support::new_leak_box_ptr(wire_LiteralValue_Binary {
            field0: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_LiteralValue_UInt8() -> *mut LiteralValueKind {
    support::new_leak_box_ptr(LiteralValueKind {
        UInt8: support::new_leak_box_ptr(wire_LiteralValue_UInt8 {
            field0: Default::default(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_LiteralValue_UInt16() -> *mut LiteralValueKind {
    support::new_leak_box_ptr(LiteralValueKind {
        UInt16: support::new_leak_box_ptr(wire_LiteralValue_UInt16 {
            field0: Default::default(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_LiteralValue_UInt32() -> *mut LiteralValueKind {
    support::new_leak_box_ptr(LiteralValueKind {
        UInt32: support::new_leak_box_ptr(wire_LiteralValue_UInt32 {
            field0: Default::default(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_LiteralValue_UInt64() -> *mut LiteralValueKind {
    support::new_leak_box_ptr(LiteralValueKind {
        UInt64: support::new_leak_box_ptr(wire_LiteralValue_UInt64 {
            field0: Default::default(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_LiteralValue_Int8() -> *mut LiteralValueKind {
    support::new_leak_box_ptr(LiteralValueKind {
        Int8: support::new_leak_box_ptr(wire_LiteralValue_Int8 {
            field0: Default::default(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_LiteralValue_Int16() -> *mut LiteralValueKind {
    support::new_leak_box_ptr(LiteralValueKind {
        Int16: support::new_leak_box_ptr(wire_LiteralValue_Int16 {
            field0: Default::default(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_LiteralValue_Int32() -> *mut LiteralValueKind {
    support::new_leak_box_ptr(LiteralValueKind {
        Int32: support::new_leak_box_ptr(wire_LiteralValue_Int32 {
            field0: Default::default(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_LiteralValue_Int64() -> *mut LiteralValueKind {
    support::new_leak_box_ptr(LiteralValueKind {
        Int64: support::new_leak_box_ptr(wire_LiteralValue_Int64 {
            field0: Default::default(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_LiteralValue_Float32() -> *mut LiteralValueKind {
    support::new_leak_box_ptr(LiteralValueKind {
        Float32: support::new_leak_box_ptr(wire_LiteralValue_Float32 {
            field0: Default::default(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_LiteralValue_Float64() -> *mut LiteralValueKind {
    support::new_leak_box_ptr(LiteralValueKind {
        Float64: support::new_leak_box_ptr(wire_LiteralValue_Float64 {
            field0: Default::default(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_LiteralValue_Range() -> *mut LiteralValueKind {
    support::new_leak_box_ptr(LiteralValueKind {
        Range: support::new_leak_box_ptr(wire_LiteralValue_Range {
            low: Default::default(),
            high: Default::default(),
            data_type: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_LiteralValue_DateTime() -> *mut LiteralValueKind {
    support::new_leak_box_ptr(LiteralValueKind {
        DateTime: support::new_leak_box_ptr(wire_LiteralValue_DateTime {
            field0: Default::default(),
            field1: Default::default(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_LiteralValue_Duration() -> *mut LiteralValueKind {
    support::new_leak_box_ptr(LiteralValueKind {
        Duration: support::new_leak_box_ptr(wire_LiteralValue_Duration {
            field0: Default::default(),
            field1: Default::default(),
        }),
    })
}

impl NewWithNullPtr for wire_NullValues {
    fn new_with_null_ptr() -> Self {
        Self {
            tag: -1,
            kind: core::ptr::null_mut(),
        }
    }
}

#[no_mangle]
pub extern "C" fn inflate_NullValues_AllColumnsSingle() -> *mut NullValuesKind {
    support::new_leak_box_ptr(NullValuesKind {
        AllColumnsSingle: support::new_leak_box_ptr(wire_NullValues_AllColumnsSingle {
            field0: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_NullValues_AllColumns() -> *mut NullValuesKind {
    support::new_leak_box_ptr(NullValuesKind {
        AllColumns: support::new_leak_box_ptr(wire_NullValues_AllColumns {
            field0: core::ptr::null_mut(),
        }),
    })
}

impl NewWithNullPtr for wire_RowCount {
    fn new_with_null_ptr() -> Self {
        Self {
            name: core::ptr::null_mut(),
            offset: Default::default(),
        }
    }
}

impl NewWithNullPtr for wire_Series {
    fn new_with_null_ptr() -> Self {
        Self {
            field0: wire_RwLockPSeries::new_with_null_ptr(),
        }
    }
}

impl NewWithNullPtr for wire_SortOptions {
    fn new_with_null_ptr() -> Self {
        Self {
            descending: Default::default(),
            nulls_last: Default::default(),
        }
    }
}

// Section: sync execution mode utility

#[no_mangle]
pub extern "C" fn free_WireSyncReturn(ptr: support::WireSyncReturn) {
    unsafe {
        let _ = support::box_from_leak_ptr(ptr);
    };
}
