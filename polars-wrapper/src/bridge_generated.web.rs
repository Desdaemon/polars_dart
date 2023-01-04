use super::*;
// Section: wire functions

#[wasm_bindgen]
pub fn wire_read_csv(
    port_: MessagePort,
    path: String,
    has_header: JsValue,
    columns: Option<JsValue>,
    delimiter: JsValue,
) {
    wire_read_csv_impl(port_, path, has_header, columns, delimiter)
}

#[wasm_bindgen]
pub fn wire_read_json(port_: MessagePort, path: String) {
    wire_read_json_impl(port_, path)
}

#[wasm_bindgen]
pub fn wire_column__method__DataFrame(that: JsValue, column: String) -> support::WireSyncReturn {
    wire_column__method__DataFrame_impl(that, column)
}

#[wasm_bindgen]
pub fn wire_columns__method__DataFrame(that: JsValue, columns: JsValue) -> support::WireSyncReturn {
    wire_columns__method__DataFrame_impl(that, columns)
}

#[wasm_bindgen]
pub fn wire_dump__method__DataFrame(that: JsValue) -> support::WireSyncReturn {
    wire_dump__method__DataFrame_impl(that)
}

#[wasm_bindgen]
pub fn wire_of_strings__static_method__Series(
    name: String,
    values: Option<JsValue>,
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
    unit: JsValue,
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
pub fn wire_dump__method__Series(that: JsValue) -> support::WireSyncReturn {
    wire_dump__method__Series_impl(that)
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
impl Wire2Api<Vec<String>> for JsValue {
    fn wire2api(self) -> Vec<String> {
        self.dyn_into::<JsArray>()
            .unwrap()
            .iter()
            .map(Wire2Api::wire2api)
            .collect()
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
impl Wire2Api<Option<Vec<chrono::Duration>>> for Option<Box<[i64]>> {
    fn wire2api(self) -> Option<Vec<chrono::Duration>> {
        self.map(Wire2Api::wire2api)
    }
}
impl Wire2Api<Option<Vec<String>>> for Option<JsValue> {
    fn wire2api(self) -> Option<Vec<String>> {
        self.map(Wire2Api::wire2api)
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

impl Wire2Api<Vec<u8>> for Box<[u8]> {
    fn wire2api(self) -> Vec<u8> {
        self.into_vec()
    }
}

// Section: impl Wire2Api for JsValue

impl Wire2Api<chrono::Duration> for JsValue {
    fn wire2api(self) -> chrono::Duration {
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
impl Wire2Api<bool> for JsValue {
    fn wire2api(self) -> bool {
        self.is_truthy()
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
impl Wire2Api<Option<Vec<chrono::Duration>>> for JsValue {
    fn wire2api(self) -> Option<Vec<chrono::Duration>> {
        (!self.is_undefined() && !self.is_null()).then(|| self.wire2api())
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
impl Wire2Api<Option<TimeUnit>> for JsValue {
    fn wire2api(self) -> Option<TimeUnit> {
        (!self.is_undefined() && !self.is_null()).then(|| self.wire2api())
    }
}
impl Wire2Api<Option<u64>> for JsValue {
    fn wire2api(self) -> Option<u64> {
        (!self.is_undefined() && !self.is_null()).then(|| self.wire2api())
    }
}
impl Wire2Api<Option<u8>> for JsValue {
    fn wire2api(self) -> Option<u8> {
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
impl Wire2Api<TimeUnit> for JsValue {
    fn wire2api(self) -> TimeUnit {
        (self.unchecked_into_f64() as i32).wire2api()
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
