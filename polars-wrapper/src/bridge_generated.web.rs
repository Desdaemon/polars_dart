use super::*;
// Section: wire functions

#[wasm_bindgen]
pub fn wire_read_csv(
    port_: MessagePort,
    path: String,
    has_header: JsValue,
    columns: JsValue,
    delimiter: Option<String>,
    comment_char: Option<String>,
    eol_char: Option<String>,
    quote_char: Option<String>,
    skip_rows: JsValue,
    skip_rows_after_header: JsValue,
    chunk_size: JsValue,
    row_count: JsValue,
    encoding: JsValue,
    n_rows: JsValue,
    n_threads: JsValue,
    null_values: JsValue,
    projection: Option<Box<[u32]>>,
    ignore_parser_errors: bool,
    rechunk: bool,
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
    )
}

#[wasm_bindgen]
pub fn wire_iter__method__DataFrame(port_: MessagePort, that: JsValue) {
    wire_iter__method__DataFrame_impl(port_, that)
}

#[wasm_bindgen]
pub fn wire_column__method__DataFrame(that: JsValue, column: String) -> support::WireSyncReturn {
    wire_column__method__DataFrame_impl(that, column)
}

#[wasm_bindgen]
pub fn wire_columns__method__DataFrame(that: JsValue, columns: JsArray) -> support::WireSyncReturn {
    wire_columns__method__DataFrame_impl(that, columns)
}

#[wasm_bindgen]
pub fn wire_dump__method__DataFrame(port_: MessagePort, that: JsValue) {
    wire_dump__method__DataFrame_impl(port_, that)
}

#[wasm_bindgen]
pub fn wire_estimated_size__method__DataFrame(that: JsValue) -> support::WireSyncReturn {
    wire_estimated_size__method__DataFrame_impl(that)
}

#[wasm_bindgen]
pub fn wire_with_row_count__method__DataFrame(
    port_: MessagePort,
    that: JsValue,
    name: String,
    offset: JsValue,
) {
    wire_with_row_count__method__DataFrame_impl(port_, that, name, offset)
}

#[wasm_bindgen]
pub fn wire_get_column_names__method__DataFrame(that: JsValue) -> support::WireSyncReturn {
    wire_get_column_names__method__DataFrame_impl(that)
}

#[wasm_bindgen]
pub fn wire_get_columns__method__DataFrame(port_: MessagePort, that: JsValue) {
    wire_get_columns__method__DataFrame_impl(port_, that)
}

#[wasm_bindgen]
pub fn wire_width__method__DataFrame(that: JsValue) -> support::WireSyncReturn {
    wire_width__method__DataFrame_impl(that)
}

#[wasm_bindgen]
pub fn wire_height__method__DataFrame(that: JsValue) -> support::WireSyncReturn {
    wire_height__method__DataFrame_impl(that)
}

#[wasm_bindgen]
pub fn wire_is_empty__method__DataFrame(that: JsValue) -> support::WireSyncReturn {
    wire_is_empty__method__DataFrame_impl(that)
}

#[wasm_bindgen]
pub fn wire_sample__method__DataFrame(
    port_: MessagePort,
    that: JsValue,
    n: usize,
    with_replacement: bool,
    shuffle: bool,
    seed: JsValue,
) {
    wire_sample__method__DataFrame_impl(port_, that, n, with_replacement, shuffle, seed)
}

#[wasm_bindgen]
pub fn wire_select__method__DataFrame(that: JsValue, columns: JsArray) -> support::WireSyncReturn {
    wire_select__method__DataFrame_impl(that, columns)
}

#[wasm_bindgen]
pub fn wire_head__method__DataFrame(that: JsValue, length: JsValue) -> support::WireSyncReturn {
    wire_head__method__DataFrame_impl(that, length)
}

#[wasm_bindgen]
pub fn wire_tail__method__DataFrame(that: JsValue, length: JsValue) -> support::WireSyncReturn {
    wire_tail__method__DataFrame_impl(that, length)
}

#[wasm_bindgen]
pub fn wire_describe__method__DataFrame(
    port_: MessagePort,
    that: JsValue,
    percentiles: Option<Box<[f64]>>,
) {
    wire_describe__method__DataFrame_impl(port_, that, percentiles)
}

#[wasm_bindgen]
pub fn wire_drop__method__DataFrame(that: JsValue, column: String) -> support::WireSyncReturn {
    wire_drop__method__DataFrame_impl(that, column)
}

#[wasm_bindgen]
pub fn wire_drop_in_place__method__DataFrame(
    that: JsValue,
    column: String,
) -> support::WireSyncReturn {
    wire_drop_in_place__method__DataFrame_impl(that, column)
}

#[wasm_bindgen]
pub fn wire_reverse__method__DataFrame(that: JsValue) -> support::WireSyncReturn {
    wire_reverse__method__DataFrame_impl(that)
}

#[wasm_bindgen]
pub fn wire_shape__method__DataFrame(that: JsValue) -> support::WireSyncReturn {
    wire_shape__method__DataFrame_impl(that)
}

#[wasm_bindgen]
pub fn wire_max__method__DataFrame(port_: MessagePort, that: JsValue) {
    wire_max__method__DataFrame_impl(port_, that)
}

#[wasm_bindgen]
pub fn wire_get_row__method__DataFrame(port_: MessagePort, that: JsValue, index: usize) {
    wire_get_row__method__DataFrame_impl(port_, that, index)
}

#[wasm_bindgen]
pub fn wire_lazy__method__take_self__DataFrame(
    that: JsValue,
    allow_copy: bool,
    projection_pushdown: JsValue,
    predicate_pushdown: JsValue,
    type_coercion: JsValue,
    simplify_expressions: JsValue,
    slice_pushdown: JsValue,
    streaming: JsValue,
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

#[wasm_bindgen]
pub fn wire_select__method__take_self__LazyFrame(
    that: JsValue,
    exprs: JsArray,
) -> support::WireSyncReturn {
    wire_select__method__take_self__LazyFrame_impl(that, exprs)
}

#[wasm_bindgen]
pub fn wire_filter__method__take_self__LazyFrame(
    that: JsValue,
    pred: JsValue,
) -> support::WireSyncReturn {
    wire_filter__method__take_self__LazyFrame_impl(that, pred)
}

#[wasm_bindgen]
pub fn wire_group_by__method__take_self__LazyFrame(
    that: JsValue,
    exprs: JsArray,
    stable: bool,
) -> support::WireSyncReturn {
    wire_group_by__method__take_self__LazyFrame_impl(that, exprs, stable)
}

#[wasm_bindgen]
pub fn wire_reverse__method__take_self__LazyFrame(that: JsValue) -> support::WireSyncReturn {
    wire_reverse__method__take_self__LazyFrame_impl(that)
}

#[wasm_bindgen]
pub fn wire_with_column__method__take_self__LazyFrame(
    that: JsValue,
    expr: JsValue,
) -> support::WireSyncReturn {
    wire_with_column__method__take_self__LazyFrame_impl(that, expr)
}

#[wasm_bindgen]
pub fn wire_with_columns__method__take_self__LazyFrame(
    that: JsValue,
    expr: JsArray,
) -> support::WireSyncReturn {
    wire_with_columns__method__take_self__LazyFrame_impl(that, expr)
}

#[wasm_bindgen]
pub fn wire_collect__method__take_self__LazyFrame(port_: MessagePort, that: JsValue) {
    wire_collect__method__take_self__LazyFrame_impl(port_, that)
}

#[wasm_bindgen]
pub fn wire_of_strings__static_method__Series(
    name: String,
    values: JsValue,
) -> support::WireSyncReturn {
    wire_of_strings__static_method__Series_impl(name, values)
}

#[wasm_bindgen]
pub fn wire_of_i32__static_method__Series(
    name: String,
    values: Option<Box<[i32]>>,
) -> support::WireSyncReturn {
    wire_of_i32__static_method__Series_impl(name, values)
}

#[wasm_bindgen]
pub fn wire_of_i64__static_method__Series(
    name: String,
    values: Option<Box<[i64]>>,
) -> support::WireSyncReturn {
    wire_of_i64__static_method__Series_impl(name, values)
}

#[wasm_bindgen]
pub fn wire_of_durations__static_method__Series(
    name: String,
    values: Option<Box<[i64]>>,
    unit: i32,
) -> support::WireSyncReturn {
    wire_of_durations__static_method__Series_impl(name, values, unit)
}

#[wasm_bindgen]
pub fn wire_of_f64__static_method__Series(
    name: String,
    values: Option<Box<[f64]>>,
) -> support::WireSyncReturn {
    wire_of_f64__static_method__Series_impl(name, values)
}

#[wasm_bindgen]
pub fn wire_append__method__Series(port_: MessagePort, that: JsValue, other: JsValue) {
    wire_append__method__Series_impl(port_, that, other)
}

#[wasm_bindgen]
pub fn wire_as_strings__method__Series(port_: MessagePort, that: JsValue) {
    wire_as_strings__method__Series_impl(port_, that)
}

#[wasm_bindgen]
pub fn wire_as_i32__method__Series(port_: MessagePort, that: JsValue) {
    wire_as_i32__method__Series_impl(port_, that)
}

#[wasm_bindgen]
pub fn wire_as_f64__method__Series(port_: MessagePort, that: JsValue) {
    wire_as_f64__method__Series_impl(port_, that)
}

#[wasm_bindgen]
pub fn wire_as_durations__method__Series(port_: MessagePort, that: JsValue) {
    wire_as_durations__method__Series_impl(port_, that)
}

#[wasm_bindgen]
pub fn wire_as_naive_datetime__method__Series(port_: MessagePort, that: JsValue) {
    wire_as_naive_datetime__method__Series_impl(port_, that)
}

#[wasm_bindgen]
pub fn wire_as_utc_datetime__method__Series(port_: MessagePort, that: JsValue) {
    wire_as_utc_datetime__method__Series_impl(port_, that)
}

#[wasm_bindgen]
pub fn wire_as_local_datetime__method__Series(port_: MessagePort, that: JsValue) {
    wire_as_local_datetime__method__Series_impl(port_, that)
}

#[wasm_bindgen]
pub fn wire_abs__method__Series(port_: MessagePort, that: JsValue) {
    wire_abs__method__Series_impl(port_, that)
}

#[wasm_bindgen]
pub fn wire_sort__method__Series(port_: MessagePort, that: JsValue, reverse: bool) {
    wire_sort__method__Series_impl(port_, that, reverse)
}

#[wasm_bindgen]
pub fn wire_shuffle__method__Series(port_: MessagePort, that: JsValue, seed: JsValue) {
    wire_shuffle__method__Series_impl(port_, that, seed)
}

#[wasm_bindgen]
pub fn wire_sum__method__Series(port_: MessagePort, that: JsValue) {
    wire_sum__method__Series_impl(port_, that)
}

#[wasm_bindgen]
pub fn wire_sum_as_series__method__Series(port_: MessagePort, that: JsValue) {
    wire_sum_as_series__method__Series_impl(port_, that)
}

#[wasm_bindgen]
pub fn wire_min__method__Series(port_: MessagePort, that: JsValue) {
    wire_min__method__Series_impl(port_, that)
}

#[wasm_bindgen]
pub fn wire_max__method__Series(port_: MessagePort, that: JsValue) {
    wire_max__method__Series_impl(port_, that)
}

#[wasm_bindgen]
pub fn wire_explode__method__Series(port_: MessagePort, that: JsValue) {
    wire_explode__method__Series_impl(port_, that)
}

#[wasm_bindgen]
pub fn wire_explode_by_offsets__method__Series(
    port_: MessagePort,
    that: JsValue,
    offsets: Box<[i64]>,
) {
    wire_explode_by_offsets__method__Series_impl(port_, that, offsets)
}

#[wasm_bindgen]
pub fn wire_cummax__method__Series(port_: MessagePort, that: JsValue, reverse: bool) {
    wire_cummax__method__Series_impl(port_, that, reverse)
}

#[wasm_bindgen]
pub fn wire_cummin__method__Series(port_: MessagePort, that: JsValue, reverse: bool) {
    wire_cummin__method__Series_impl(port_, that, reverse)
}

#[wasm_bindgen]
pub fn wire_cumprod__method__Series(port_: MessagePort, that: JsValue, reverse: bool) {
    wire_cumprod__method__Series_impl(port_, that, reverse)
}

#[wasm_bindgen]
pub fn wire_cumsum__method__Series(port_: MessagePort, that: JsValue, reverse: bool) {
    wire_cumsum__method__Series_impl(port_, that, reverse)
}

#[wasm_bindgen]
pub fn wire_product__method__Series(port_: MessagePort, that: JsValue) {
    wire_product__method__Series_impl(port_, that)
}

#[wasm_bindgen]
pub fn wire_get_string__method__Series(that: JsValue, index: usize) -> support::WireSyncReturn {
    wire_get_string__method__Series_impl(that, index)
}

#[wasm_bindgen]
pub fn wire_get__method__Series(that: JsValue, index: usize) -> support::WireSyncReturn {
    wire_get__method__Series_impl(that, index)
}

#[wasm_bindgen]
pub fn wire_head__method__Series(that: JsValue, length: JsValue) -> support::WireSyncReturn {
    wire_head__method__Series_impl(that, length)
}

#[wasm_bindgen]
pub fn wire_tail__method__Series(that: JsValue, length: JsValue) -> support::WireSyncReturn {
    wire_tail__method__Series_impl(that, length)
}

#[wasm_bindgen]
pub fn wire_mean__method__Series(port_: MessagePort, that: JsValue) {
    wire_mean__method__Series_impl(port_, that)
}

#[wasm_bindgen]
pub fn wire_median__method__Series(port_: MessagePort, that: JsValue) {
    wire_median__method__Series_impl(port_, that)
}

#[wasm_bindgen]
pub fn wire_mean_as_series__method__Series(port_: MessagePort, that: JsValue) {
    wire_mean_as_series__method__Series_impl(port_, that)
}

#[wasm_bindgen]
pub fn wire_median_as_series__method__Series(port_: MessagePort, that: JsValue) {
    wire_median_as_series__method__Series_impl(port_, that)
}

#[wasm_bindgen]
pub fn wire_estimated_size__method__Series(that: JsValue) -> support::WireSyncReturn {
    wire_estimated_size__method__Series_impl(that)
}

#[wasm_bindgen]
pub fn wire_add_to__method__Series(that: JsValue, other: JsValue) -> support::WireSyncReturn {
    wire_add_to__method__Series_impl(that, other)
}

#[wasm_bindgen]
pub fn wire_subtract__method__Series(that: JsValue, other: JsValue) -> support::WireSyncReturn {
    wire_subtract__method__Series_impl(that, other)
}

#[wasm_bindgen]
pub fn wire_multiply__method__Series(that: JsValue, other: JsValue) -> support::WireSyncReturn {
    wire_multiply__method__Series_impl(that, other)
}

#[wasm_bindgen]
pub fn wire_divide__method__Series(that: JsValue, other: JsValue) -> support::WireSyncReturn {
    wire_divide__method__Series_impl(that, other)
}

#[wasm_bindgen]
pub fn wire_remainder__method__Series(that: JsValue, other: JsValue) -> support::WireSyncReturn {
    wire_remainder__method__Series_impl(that, other)
}

#[wasm_bindgen]
pub fn wire_is_bool__method__Series(that: JsValue) -> support::WireSyncReturn {
    wire_is_bool__method__Series_impl(that)
}

#[wasm_bindgen]
pub fn wire_is_utf8__method__Series(that: JsValue) -> support::WireSyncReturn {
    wire_is_utf8__method__Series_impl(that)
}

#[wasm_bindgen]
pub fn wire_is_numeric__method__Series(that: JsValue) -> support::WireSyncReturn {
    wire_is_numeric__method__Series_impl(that)
}

#[wasm_bindgen]
pub fn wire_is_temporal__method__Series(that: JsValue) -> support::WireSyncReturn {
    wire_is_temporal__method__Series_impl(that)
}

#[wasm_bindgen]
pub fn wire_dump__method__Series(port_: MessagePort, that: JsValue) {
    wire_dump__method__Series_impl(port_, that)
}

#[wasm_bindgen]
pub fn wire_rename__method__Series(that: JsValue, name: String) -> support::WireSyncReturn {
    wire_rename__method__Series_impl(that, name)
}

#[wasm_bindgen]
pub fn wire_unique__method__Series(port_: MessagePort, that: JsValue, stable: bool) {
    wire_unique__method__Series_impl(port_, that, stable)
}

#[wasm_bindgen]
pub fn wire_equal__method__Series(
    port_: MessagePort,
    that: JsValue,
    other: JsValue,
    ignore_null: bool,
) {
    wire_equal__method__Series_impl(port_, that, other, ignore_null)
}

#[wasm_bindgen]
pub fn wire_reshape__method__Series(port_: MessagePort, that: JsValue, dims: Box<[i64]>) {
    wire_reshape__method__Series_impl(port_, that, dims)
}

#[wasm_bindgen]
pub fn wire_std_as_series__method__Series(port_: MessagePort, that: JsValue, ddof: u8) {
    wire_std_as_series__method__Series_impl(port_, that, ddof)
}

// Section: allocate functions

// Section: related functions

#[wasm_bindgen]
pub fn drop_opaque_RwLockPDataFrame(ptr: *const c_void) {
    unsafe {
        Arc::<RwLock<PDataFrame>>::decrement_strong_count(ptr as _);
    }
}

#[wasm_bindgen]
pub fn share_opaque_RwLockPDataFrame(ptr: *const c_void) -> *const c_void {
    unsafe {
        Arc::<RwLock<PDataFrame>>::increment_strong_count(ptr as _);
        ptr
    }
}

#[wasm_bindgen]
pub fn drop_opaque_RwLockPLazyFrame(ptr: *const c_void) {
    unsafe {
        Arc::<RwLock<PLazyFrame>>::decrement_strong_count(ptr as _);
    }
}

#[wasm_bindgen]
pub fn share_opaque_RwLockPLazyFrame(ptr: *const c_void) -> *const c_void {
    unsafe {
        Arc::<RwLock<PLazyFrame>>::increment_strong_count(ptr as _);
        ptr
    }
}

#[wasm_bindgen]
pub fn drop_opaque_RwLockPLazyGroupBy(ptr: *const c_void) {
    unsafe {
        Arc::<RwLock<PLazyGroupBy>>::decrement_strong_count(ptr as _);
    }
}

#[wasm_bindgen]
pub fn share_opaque_RwLockPLazyGroupBy(ptr: *const c_void) -> *const c_void {
    unsafe {
        Arc::<RwLock<PLazyGroupBy>>::increment_strong_count(ptr as _);
        ptr
    }
}

#[wasm_bindgen]
pub fn drop_opaque_RwLockPSeries(ptr: *const c_void) {
    unsafe {
        Arc::<RwLock<PSeries>>::decrement_strong_count(ptr as _);
    }
}

#[wasm_bindgen]
pub fn share_opaque_RwLockPSeries(ptr: *const c_void) -> *const c_void {
    unsafe {
        Arc::<RwLock<PSeries>>::increment_strong_count(ptr as _);
        ptr
    }
}

// Section: impl Wire2Api

impl Wire2Api<Arc<str>> for String {
    fn wire2api(self) -> Arc<str> {
        let string: String = self.wire2api();
        <Arc<str>>::from(string)
    }
}
impl Wire2Api<chrono::Duration> for i64 {
    fn wire2api(self) -> chrono::Duration {
        chrono::Duration::milliseconds(self)
    }
}
impl Wire2Api<Vec<chrono::Duration>> for Box<[i64]> {
    fn wire2api(self) -> Vec<chrono::Duration> {
        let vec: Vec<i64> = self.wire2api();
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}

impl Wire2Api<String> for String {
    fn wire2api(self) -> String {
        self
    }
}
impl Wire2Api<Vec<String>> for JsArray {
    fn wire2api(self) -> Vec<String> {
        self.iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<AggExpr> for JsValue {
    fn wire2api(self) -> AggExpr {
        let self_ = self.unchecked_into::<JsArray>();
        match self_.get(0).unchecked_into_f64() as _ {
            0 => AggExpr::Min {
                input: self_.get(1).wire2api(),
                propagate_nans: self_.get(2).wire2api(),
            },
            1 => AggExpr::Max {
                input: self_.get(1).wire2api(),
                propagate_nans: self_.get(2).wire2api(),
            },
            2 => AggExpr::Median(self_.get(1).wire2api()),
            3 => AggExpr::NUnique(self_.get(1).wire2api()),
            4 => AggExpr::First(self_.get(1).wire2api()),
            5 => AggExpr::Last(self_.get(1).wire2api()),
            6 => AggExpr::Mean(self_.get(1).wire2api()),
            7 => AggExpr::List(self_.get(1).wire2api()),
            8 => AggExpr::Count(self_.get(1).wire2api()),
            9 => AggExpr::Quantile {
                expr: self_.get(1).wire2api(),
                quantile: self_.get(2).wire2api(),
                interpol: self_.get(3).wire2api(),
            },
            10 => AggExpr::Sum(self_.get(1).wire2api()),
            11 => AggExpr::AggGroups(self_.get(1).wire2api()),
            12 => AggExpr::Std(self_.get(1).wire2api(), self_.get(2).wire2api()),
            _ => unreachable!(),
        }
    }
}

impl Wire2Api<DataFrame> for JsValue {
    fn wire2api(self) -> DataFrame {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        DataFrame(self_.get(0).wire2api())
    }
}
impl Wire2Api<DataType> for JsValue {
    fn wire2api(self) -> DataType {
        let self_ = self.unchecked_into::<JsArray>();
        match self_.get(0).unchecked_into_f64() as _ {
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
            14 => DataType::Datetime(self_.get(1).wire2api(), self_.get(2).wire2api()),
            15 => DataType::Duration(self_.get(1).wire2api()),
            16 => DataType::Time,
            17 => DataType::List(self_.get(1).wire2api()),
            18 => DataType::Struct(self_.get(1).wire2api()),
            19 => DataType::Unknown,
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<Expr> for JsValue {
    fn wire2api(self) -> Expr {
        let self_ = self.unchecked_into::<JsArray>();
        match self_.get(0).unchecked_into_f64() as _ {
            0 => Expr::Alias(self_.get(1).wire2api(), self_.get(2).wire2api()),
            1 => Expr::Column(self_.get(1).wire2api()),
            2 => Expr::Columns(self_.get(1).wire2api()),
            3 => Expr::DtypeColumn(self_.get(1).wire2api()),
            4 => Expr::Literal(self_.get(1).wire2api()),
            5 => Expr::BinaryExpr {
                left: self_.get(1).wire2api(),
                op: self_.get(2).wire2api(),
                right: self_.get(3).wire2api(),
            },
            6 => Expr::Cast {
                expr: self_.get(1).wire2api(),
                data_type: self_.get(2).wire2api(),
                strict: self_.get(3).wire2api(),
            },
            7 => Expr::Sort {
                expr: self_.get(1).wire2api(),
                options: self_.get(2).wire2api(),
            },
            8 => Expr::Take {
                expr: self_.get(1).wire2api(),
                idx: self_.get(2).wire2api(),
            },
            9 => Expr::Agg(self_.get(1).wire2api()),
            10 => Expr::Ternary {
                predicate: self_.get(1).wire2api(),
                truthy: self_.get(2).wire2api(),
                falsy: self_.get(3).wire2api(),
            },
            11 => Expr::Explode(self_.get(1).wire2api()),
            12 => Expr::Filter {
                input: self_.get(1).wire2api(),
                by: self_.get(2).wire2api(),
            },
            13 => Expr::Wildcard,
            14 => Expr::Slice {
                input: self_.get(1).wire2api(),
                offset: self_.get(2).wire2api(),
                length: self_.get(3).wire2api(),
            },
            15 => Expr::KeepName(self_.get(1).wire2api()),
            16 => Expr::Count,
            17 => Expr::Nth(self_.get(1).wire2api()),
            _ => unreachable!(),
        }
    }
}

impl Wire2Api<Field> for JsValue {
    fn wire2api(self) -> Field {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            2,
            "Expected 2 elements, got {}",
            self_.length()
        );
        Field {
            name: self_.get(0).wire2api(),
            dtype: self_.get(1).wire2api(),
        }
    }
}
impl Wire2Api<Vec<f64>> for Box<[f64]> {
    fn wire2api(self) -> Vec<f64> {
        self.into_vec()
    }
}

impl Wire2Api<Vec<i32>> for Box<[i32]> {
    fn wire2api(self) -> Vec<i32> {
        self.into_vec()
    }
}
impl Wire2Api<Vec<i64>> for Box<[i64]> {
    fn wire2api(self) -> Vec<i64> {
        self.into_vec()
    }
}
impl Wire2Api<LazyFrame> for JsValue {
    fn wire2api(self) -> LazyFrame {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        LazyFrame(self_.get(0).wire2api())
    }
}
impl Wire2Api<Vec<DataType>> for JsArray {
    fn wire2api(self) -> Vec<DataType> {
        self.iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<Vec<Expr>> for JsArray {
    fn wire2api(self) -> Vec<Expr> {
        self.iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<Vec<Field>> for JsArray {
    fn wire2api(self) -> Vec<Field> {
        self.iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<LiteralValue> for JsValue {
    fn wire2api(self) -> LiteralValue {
        let self_ = self.unchecked_into::<JsArray>();
        match self_.get(0).unchecked_into_f64() as _ {
            0 => LiteralValue::Boolean(self_.get(1).wire2api()),
            1 => LiteralValue::Utf8(self_.get(1).wire2api()),
            2 => LiteralValue::Binary(self_.get(1).wire2api()),
            3 => LiteralValue::UInt8(self_.get(1).wire2api()),
            4 => LiteralValue::UInt16(self_.get(1).wire2api()),
            5 => LiteralValue::UInt32(self_.get(1).wire2api()),
            6 => LiteralValue::UInt64(self_.get(1).wire2api()),
            7 => LiteralValue::Int8(self_.get(1).wire2api()),
            8 => LiteralValue::Int16(self_.get(1).wire2api()),
            9 => LiteralValue::Int32(self_.get(1).wire2api()),
            10 => LiteralValue::Int64(self_.get(1).wire2api()),
            11 => LiteralValue::Float32(self_.get(1).wire2api()),
            12 => LiteralValue::Float64(self_.get(1).wire2api()),
            13 => LiteralValue::Range {
                low: self_.get(1).wire2api(),
                high: self_.get(2).wire2api(),
                data_type: self_.get(3).wire2api(),
            },
            14 => LiteralValue::DateTime(self_.get(1).wire2api(), self_.get(2).wire2api()),
            15 => LiteralValue::Duration(self_.get(1).wire2api(), self_.get(2).wire2api()),
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<NullValues> for JsValue {
    fn wire2api(self) -> NullValues {
        let self_ = self.unchecked_into::<JsArray>();
        match self_.get(0).unchecked_into_f64() as _ {
            0 => NullValues::AllColumnsSingle(self_.get(1).wire2api()),
            1 => NullValues::AllColumns(self_.get(1).wire2api()),
            _ => unreachable!(),
        }
    }
}

impl Wire2Api<Option<Vec<chrono::Duration>>> for Option<Box<[i64]>> {
    fn wire2api(self) -> Option<Vec<chrono::Duration>> {
        self.map(Wire2Api::wire2api)
    }
}
impl Wire2Api<Option<String>> for Option<String> {
    fn wire2api(self) -> Option<String> {
        self.map(Wire2Api::wire2api)
    }
}
impl Wire2Api<Option<Vec<String>>> for JsValue {
    fn wire2api(self) -> Option<Vec<String>> {
        (!self.is_undefined() && !self.is_null()).then(|| self.wire2api())
    }
}
impl Wire2Api<Option<bool>> for JsValue {
    fn wire2api(self) -> Option<bool> {
        (!self.is_undefined() && !self.is_null()).then(|| self.wire2api())
    }
}
impl Wire2Api<Option<CsvEncoding>> for JsValue {
    fn wire2api(self) -> Option<CsvEncoding> {
        (!self.is_undefined() && !self.is_null()).then(|| self.wire2api())
    }
}
impl Wire2Api<Option<Vec<f64>>> for Option<Box<[f64]>> {
    fn wire2api(self) -> Option<Vec<f64>> {
        self.map(Wire2Api::wire2api)
    }
}
impl Wire2Api<Option<Vec<i32>>> for Option<Box<[i32]>> {
    fn wire2api(self) -> Option<Vec<i32>> {
        self.map(Wire2Api::wire2api)
    }
}
impl Wire2Api<Option<Vec<i64>>> for Option<Box<[i64]>> {
    fn wire2api(self) -> Option<Vec<i64>> {
        self.map(Wire2Api::wire2api)
    }
}
impl Wire2Api<Option<NullValues>> for JsValue {
    fn wire2api(self) -> Option<NullValues> {
        (!self.is_undefined() && !self.is_null()).then(|| self.wire2api())
    }
}
impl Wire2Api<Option<RowCount>> for JsValue {
    fn wire2api(self) -> Option<RowCount> {
        (!self.is_undefined() && !self.is_null()).then(|| self.wire2api())
    }
}
impl Wire2Api<Option<u32>> for JsValue {
    fn wire2api(self) -> Option<u32> {
        (!self.is_undefined() && !self.is_null()).then(|| self.wire2api())
    }
}
impl Wire2Api<Option<u64>> for JsValue {
    fn wire2api(self) -> Option<u64> {
        (!self.is_undefined() && !self.is_null()).then(|| self.wire2api())
    }
}
impl Wire2Api<Option<Vec<u32>>> for Option<Box<[u32]>> {
    fn wire2api(self) -> Option<Vec<u32>> {
        self.map(Wire2Api::wire2api)
    }
}
impl Wire2Api<Option<usize>> for JsValue {
    fn wire2api(self) -> Option<usize> {
        (!self.is_undefined() && !self.is_null()).then(|| self.wire2api())
    }
}

impl Wire2Api<RowCount> for JsValue {
    fn wire2api(self) -> RowCount {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            2,
            "Expected 2 elements, got {}",
            self_.length()
        );
        RowCount {
            name: self_.get(0).wire2api(),
            offset: self_.get(1).wire2api(),
        }
    }
}
impl Wire2Api<Series> for JsValue {
    fn wire2api(self) -> Series {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        Series(self_.get(0).wire2api())
    }
}
impl Wire2Api<SortOptions> for JsValue {
    fn wire2api(self) -> SortOptions {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            2,
            "Expected 2 elements, got {}",
            self_.length()
        );
        SortOptions {
            descending: self_.get(0).wire2api(),
            nulls_last: self_.get(1).wire2api(),
        }
    }
}

impl Wire2Api<Vec<u32>> for Box<[u32]> {
    fn wire2api(self) -> Vec<u32> {
        self.into_vec()
    }
}
impl Wire2Api<Vec<u8>> for Box<[u8]> {
    fn wire2api(self) -> Vec<u8> {
        self.into_vec()
    }
}

// Section: impl Wire2Api for JsValue

impl Wire2Api<Arc<str>> for JsValue {
    fn wire2api(self) -> Arc<str> {
        self.as_string().unwrap().into()
    }
}
impl Wire2Api<chrono::Duration> for JsValue {
    fn wire2api(self) -> chrono::Duration {
        Wire2Api::<i64>::wire2api(self).wire2api()
    }
}
impl Wire2Api<Vec<chrono::Duration>> for JsValue {
    fn wire2api(self) -> Vec<chrono::Duration> {
        self.unchecked_into::<js_sys::BigInt64Array>()
            .to_vec()
            .into_iter()
            .map(Wire2Api::wire2api)
            .collect()
    }
}
impl Wire2Api<chrono::NaiveDateTime> for JsValue {
    fn wire2api(self) -> chrono::NaiveDateTime {
        Wire2Api::<i64>::wire2api(self).wire2api()
    }
}
impl Wire2Api<RustOpaque<RwLock<PDataFrame>>> for JsValue {
    fn wire2api(self) -> RustOpaque<RwLock<PDataFrame>> {
        #[cfg(target_pointer_width = "64")]
        {
            compile_error!("64-bit pointers are not supported.");
        }

        unsafe { support::opaque_from_dart((self.as_f64().unwrap() as usize) as _) }
    }
}
impl Wire2Api<RustOpaque<RwLock<PLazyFrame>>> for JsValue {
    fn wire2api(self) -> RustOpaque<RwLock<PLazyFrame>> {
        #[cfg(target_pointer_width = "64")]
        {
            compile_error!("64-bit pointers are not supported.");
        }

        unsafe { support::opaque_from_dart((self.as_f64().unwrap() as usize) as _) }
    }
}
impl Wire2Api<RustOpaque<RwLock<PSeries>>> for JsValue {
    fn wire2api(self) -> RustOpaque<RwLock<PSeries>> {
        #[cfg(target_pointer_width = "64")]
        {
            compile_error!("64-bit pointers are not supported.");
        }

        unsafe { support::opaque_from_dart((self.as_f64().unwrap() as usize) as _) }
    }
}
impl Wire2Api<String> for JsValue {
    fn wire2api(self) -> String {
        self.as_string().expect("non-UTF-8 string, or not a string")
    }
}
impl Wire2Api<Vec<String>> for JsValue {
    fn wire2api(self) -> Vec<String> {
        self.dyn_into::<JsArray>().unwrap().wire2api()
    }
}
impl Wire2Api<bool> for JsValue {
    fn wire2api(self) -> bool {
        self.is_truthy()
    }
}
impl Wire2Api<Box<DataType>> for JsValue {
    fn wire2api(self) -> Box<DataType> {
        Box::new(self.wire2api())
    }
}
impl Wire2Api<Box<Expr>> for JsValue {
    fn wire2api(self) -> Box<Expr> {
        Box::new(self.wire2api())
    }
}
impl Wire2Api<CsvEncoding> for JsValue {
    fn wire2api(self) -> CsvEncoding {
        (self.unchecked_into_f64() as i32).wire2api()
    }
}
impl Wire2Api<f32> for JsValue {
    fn wire2api(self) -> f32 {
        self.unchecked_into_f64() as _
    }
}
impl Wire2Api<f64> for JsValue {
    fn wire2api(self) -> f64 {
        self.unchecked_into_f64() as _
    }
}
impl Wire2Api<Vec<f64>> for JsValue {
    fn wire2api(self) -> Vec<f64> {
        self.unchecked_into::<js_sys::Float64Array>()
            .to_vec()
            .into()
    }
}
impl Wire2Api<i16> for JsValue {
    fn wire2api(self) -> i16 {
        self.unchecked_into_f64() as _
    }
}
impl Wire2Api<i32> for JsValue {
    fn wire2api(self) -> i32 {
        self.unchecked_into_f64() as _
    }
}
impl Wire2Api<i64> for JsValue {
    fn wire2api(self) -> i64 {
        ::std::convert::TryInto::try_into(self.dyn_into::<js_sys::BigInt>().unwrap()).unwrap()
    }
}
impl Wire2Api<i8> for JsValue {
    fn wire2api(self) -> i8 {
        self.unchecked_into_f64() as _
    }
}
impl Wire2Api<Vec<i32>> for JsValue {
    fn wire2api(self) -> Vec<i32> {
        self.unchecked_into::<js_sys::Int32Array>().to_vec().into()
    }
}
impl Wire2Api<Vec<i64>> for JsValue {
    fn wire2api(self) -> Vec<i64> {
        let buf = self.dyn_into::<js_sys::BigInt64Array>().unwrap();
        let buf = js_sys::Uint8Array::new(&buf.buffer());
        support::slice_from_byte_buffer(buf.to_vec()).into()
    }
}
impl Wire2Api<Vec<DataType>> for JsValue {
    fn wire2api(self) -> Vec<DataType> {
        let arr = self.dyn_into::<JsArray>().unwrap();
        arr.iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<Vec<Expr>> for JsValue {
    fn wire2api(self) -> Vec<Expr> {
        let arr = self.dyn_into::<JsArray>().unwrap();
        arr.iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<Vec<Field>> for JsValue {
    fn wire2api(self) -> Vec<Field> {
        let arr = self.dyn_into::<JsArray>().unwrap();
        arr.iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<Operator> for JsValue {
    fn wire2api(self) -> Operator {
        (self.unchecked_into_f64() as i32).wire2api()
    }
}
impl Wire2Api<Option<Vec<chrono::Duration>>> for JsValue {
    fn wire2api(self) -> Option<Vec<chrono::Duration>> {
        (!self.is_undefined() && !self.is_null()).then(|| self.wire2api())
    }
}
impl Wire2Api<Option<String>> for JsValue {
    fn wire2api(self) -> Option<String> {
        (!self.is_undefined() && !self.is_null()).then(|| self.wire2api())
    }
}
impl Wire2Api<Option<Vec<f64>>> for JsValue {
    fn wire2api(self) -> Option<Vec<f64>> {
        (!self.is_undefined() && !self.is_null()).then(|| self.wire2api())
    }
}
impl Wire2Api<Option<Vec<i32>>> for JsValue {
    fn wire2api(self) -> Option<Vec<i32>> {
        (!self.is_undefined() && !self.is_null()).then(|| self.wire2api())
    }
}
impl Wire2Api<Option<Vec<i64>>> for JsValue {
    fn wire2api(self) -> Option<Vec<i64>> {
        (!self.is_undefined() && !self.is_null()).then(|| self.wire2api())
    }
}
impl Wire2Api<Option<Vec<u32>>> for JsValue {
    fn wire2api(self) -> Option<Vec<u32>> {
        (!self.is_undefined() && !self.is_null()).then(|| self.wire2api())
    }
}
impl Wire2Api<QuantileInterpolOptions> for JsValue {
    fn wire2api(self) -> QuantileInterpolOptions {
        (self.unchecked_into_f64() as i32).wire2api()
    }
}
impl Wire2Api<TimeUnit> for JsValue {
    fn wire2api(self) -> TimeUnit {
        (self.unchecked_into_f64() as i32).wire2api()
    }
}
impl Wire2Api<u16> for JsValue {
    fn wire2api(self) -> u16 {
        self.unchecked_into_f64() as _
    }
}
impl Wire2Api<u32> for JsValue {
    fn wire2api(self) -> u32 {
        self.unchecked_into_f64() as _
    }
}
impl Wire2Api<u64> for JsValue {
    fn wire2api(self) -> u64 {
        ::std::convert::TryInto::try_into(self.dyn_into::<js_sys::BigInt>().unwrap()).unwrap()
    }
}
impl Wire2Api<u8> for JsValue {
    fn wire2api(self) -> u8 {
        self.unchecked_into_f64() as _
    }
}
impl Wire2Api<Vec<u32>> for JsValue {
    fn wire2api(self) -> Vec<u32> {
        self.unchecked_into::<js_sys::Uint32Array>().to_vec().into()
    }
}
impl Wire2Api<Vec<u8>> for JsValue {
    fn wire2api(self) -> Vec<u8> {
        self.unchecked_into::<js_sys::Uint8Array>().to_vec().into()
    }
}
impl Wire2Api<usize> for JsValue {
    fn wire2api(self) -> usize {
        self.unchecked_into_f64() as _
    }
}
