use super::*;
// Section: wire functions

#[no_mangle]
pub extern "C" fn wire_read_csv(
    port_: i64,
    path: *mut wire_uint_8_list,
    has_header: *mut bool,
    columns: *mut wire_StringList,
    delimiter: *mut u8,
) {
    wire_read_csv_impl(port_, path, has_header, columns, delimiter)
}

#[no_mangle]
pub extern "C" fn wire_read_json(port_: i64, path: *mut wire_uint_8_list) {
    wire_read_json_impl(port_, path)
}

#[no_mangle]
pub extern "C" fn wire_column__method__DataFrame(
    port_: i64,
    that: *mut wire_DataFrame,
    column: *mut wire_uint_8_list,
) {
    wire_column__method__DataFrame_impl(port_, that, column)
}

#[no_mangle]
pub extern "C" fn wire_columns__method__DataFrame(
    port_: i64,
    that: *mut wire_DataFrame,
    columns: *mut wire_StringList,
) {
    wire_columns__method__DataFrame_impl(port_, that, columns)
}

#[no_mangle]
pub extern "C" fn wire_dump__method__DataFrame(port_: i64, that: *mut wire_DataFrame) {
    wire_dump__method__DataFrame_impl(port_, that)
}

#[no_mangle]
pub extern "C" fn wire_of_strings__static_method__Series(
    port_: i64,
    name: *mut wire_uint_8_list,
    values: *mut wire_StringList,
) {
    wire_of_strings__static_method__Series_impl(port_, name, values)
}

#[no_mangle]
pub extern "C" fn wire_of_i32__static_method__Series(
    port_: i64,
    name: *mut wire_uint_8_list,
    values: *mut wire_int_32_list,
) {
    wire_of_i32__static_method__Series_impl(port_, name, values)
}

#[no_mangle]
pub extern "C" fn wire_of_f64__static_method__Series(
    port_: i64,
    name: *mut wire_uint_8_list,
    values: *mut wire_float_64_list,
) {
    wire_of_f64__static_method__Series_impl(port_, name, values)
}

#[no_mangle]
pub extern "C" fn wire_append__method__Series(
    port_: i64,
    that: *mut wire_Series,
    other: *mut wire_Series,
) {
    wire_append__method__Series_impl(port_, that, other)
}

#[no_mangle]
pub extern "C" fn wire_as_strings__method__Series(port_: i64, that: *mut wire_Series) {
    wire_as_strings__method__Series_impl(port_, that)
}

#[no_mangle]
pub extern "C" fn wire_as_i32__method__Series(port_: i64, that: *mut wire_Series) {
    wire_as_i32__method__Series_impl(port_, that)
}

#[no_mangle]
pub extern "C" fn wire_as_f64__method__Series(port_: i64, that: *mut wire_Series) {
    wire_as_f64__method__Series_impl(port_, that)
}

// Section: allocate functions

#[no_mangle]
pub extern "C" fn new_RwLockPDataFrame() -> wire_RwLockPDataFrame {
    wire_RwLockPDataFrame::new_with_null_ptr()
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
pub extern "C" fn new_box_autoadd_bool_0(value: bool) -> *mut bool {
    support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_data_frame_0() -> *mut wire_DataFrame {
    support::new_leak_box_ptr(wire_DataFrame::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_series_0() -> *mut wire_Series {
    support::new_leak_box_ptr(wire_Series::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_u8_0(value: u8) -> *mut u8 {
    support::new_leak_box_ptr(value)
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

impl Wire2Api<RustOpaque<RwLock<PDataFrame>>> for wire_RwLockPDataFrame {
    fn wire2api(self) -> RustOpaque<RwLock<PDataFrame>> {
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

impl Wire2Api<DataFrame> for *mut wire_DataFrame {
    fn wire2api(self) -> DataFrame {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<DataFrame>::wire2api(*wrap).into()
    }
}
impl Wire2Api<Series> for *mut wire_Series {
    fn wire2api(self) -> Series {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<Series>::wire2api(*wrap).into()
    }
}

impl Wire2Api<DataFrame> for wire_DataFrame {
    fn wire2api(self) -> DataFrame {
        DataFrame(self.field0.wire2api())
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

impl Wire2Api<Series> for wire_Series {
    fn wire2api(self) -> Series {
        Series(self.field0.wire2api())
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
pub struct wire_Series {
    field0: wire_RwLockPSeries,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_uint_8_list {
    ptr: *mut u8,
    len: i32,
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
impl NewWithNullPtr for wire_RwLockPSeries {
    fn new_with_null_ptr() -> Self {
        Self {
            ptr: core::ptr::null(),
        }
    }
}

impl NewWithNullPtr for wire_DataFrame {
    fn new_with_null_ptr() -> Self {
        Self {
            field0: wire_RwLockPDataFrame::new_with_null_ptr(),
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

// Section: sync execution mode utility

#[no_mangle]
pub extern "C" fn free_WireSyncReturn(ptr: support::WireSyncReturn) {
    unsafe {
        let _ = support::box_from_leak_ptr(ptr);
    };
}
