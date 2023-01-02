use super::*;
// Section: wire functions

#[wasm_bindgen]
pub fn wire_read_csv(
    port_: MessagePort,
    path: String,
    has_header: *mut bool,
    columns: Option<JsValue>,
    delimiter: *mut u8,
) {
    wire_read_csv_impl(port_, path, has_header, columns, delimiter)
}

#[wasm_bindgen]
pub fn wire_read_json(port_: MessagePort, path: String) {
    wire_read_json_impl(port_, path)
}

#[wasm_bindgen]
pub fn wire_column__method__DataFrame(port_: MessagePort, that: JsValue, column: String) {
    wire_column__method__DataFrame_impl(port_, that, column)
}

#[wasm_bindgen]
pub fn wire_columns__method__DataFrame(port_: MessagePort, that: JsValue, columns: JsValue) {
    wire_columns__method__DataFrame_impl(port_, that, columns)
}

#[wasm_bindgen]
pub fn wire_dump__method__DataFrame(port_: MessagePort, that: JsValue) {
    wire_dump__method__DataFrame_impl(port_, that)
}

#[wasm_bindgen]
pub fn wire_of_strings__static_method__Series(
    port_: MessagePort,
    name: String,
    values: Option<JsValue>,
) {
    wire_of_strings__static_method__Series_impl(port_, name, values)
}

#[wasm_bindgen]
pub fn wire_of_i32__static_method__Series(
    port_: MessagePort,
    name: String,
    values: Option<Box<[i32]>>,
) {
    wire_of_i32__static_method__Series_impl(port_, name, values)
}

#[wasm_bindgen]
pub fn wire_of_f64__static_method__Series(
    port_: MessagePort,
    name: String,
    values: Option<Box<[f64]>>,
) {
    wire_of_f64__static_method__Series_impl(port_, name, values)
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

// Section: allocate functions

#[wasm_bindgen]
pub fn new_box_autoadd_bool_0(value: bool) -> *mut bool {
    support::new_leak_box_ptr(value)
}

#[wasm_bindgen]
pub fn new_box_autoadd_u8_0(value: u8) -> *mut u8 {
    support::new_leak_box_ptr(value)
}

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
impl Wire2Api<Vec<i32>> for JsValue {
    fn wire2api(self) -> Vec<i32> {
        self.unchecked_into::<js_sys::Int32Array>().to_vec().into()
    }
}
impl Wire2Api<Option<Vec<String>>> for JsValue {
    fn wire2api(self) -> Option<Vec<String>> {
        (!self.is_undefined() && !self.is_null()).then(|| self.wire2api())
    }
}
impl Wire2Api<Option<bool>> for JsValue {
    fn wire2api(self) -> Option<bool> {
        (self != 0).then(|| *Wire2Api::<Box<bool>>::wire2api(self))
    }
}
impl Wire2Api<Option<u8>> for JsValue {
    fn wire2api(self) -> Option<u8> {
        (self != 0).then(|| *Wire2Api::<Box<u8>>::wire2api(self))
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
