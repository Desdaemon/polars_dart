// AUTO GENERATED FILE, DO NOT EDIT.
// Generated by `flutter_rust_bridge`@ 1.59.0.
// ignore_for_file: non_constant_identifier_names, unused_element, duplicate_ignore, directives_ordering, curly_braces_in_flow_control_structures, unnecessary_lambdas, slash_for_doc_comments, prefer_const_literals_to_create_immutables, implicit_dynamic_list_literal, duplicate_import, unused_import, prefer_single_quotes, prefer_const_constructors, use_super_parameters, always_use_package_imports, annotate_overrides, invalid_use_of_protected_member, constant_identifier_names, invalid_use_of_internal_member

import 'dart:convert';
import 'dart:async';
import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'wrapper.dart';
export 'wrapper.dart';
import 'package:meta/meta.dart';

class PolarsWrapperPlatform extends FlutterRustBridgeBase<PolarsWrapperWire>
    with FlutterRustBridgeSetupMixin {
  PolarsWrapperPlatform(FutureOr<WasmModule> dylib)
      : super(PolarsWrapperWire(dylib)) {
    setupMixinConstructor();
  }
  Future<void> setup() => inner.init;

// Section: api2wire

  @protected
  Object api2wire_RwLockPDataFrame(RwLockPDataFrame raw) {
    return raw.shareOrMove();
  }

  @protected
  Object api2wire_RwLockPSeries(RwLockPSeries raw) {
    return raw.shareOrMove();
  }

  @protected
  String api2wire_String(String raw) {
    return raw;
  }

  @protected
  List<String> api2wire_StringList(List<String> raw) {
    return raw;
  }

  @protected
  int /* *bool */ api2wire_box_autoadd_bool(bool raw) {
    return inner.new_box_autoadd_bool_0(api2wire_bool(raw));
  }

  @protected
  List<dynamic> api2wire_box_autoadd_data_frame(DataFrame raw) {
    return api2wire_data_frame(raw);
  }

  @protected
  List<dynamic> api2wire_box_autoadd_series(Series raw) {
    return api2wire_series(raw);
  }

  @protected
  int /* *u64 */ api2wire_box_autoadd_u64(int raw) {
    return inner.new_box_autoadd_u64_0(api2wire_u64(raw));
  }

  @protected
  int /* *u8 */ api2wire_box_autoadd_u8(int raw) {
    return inner.new_box_autoadd_u8_0(api2wire_u8(raw));
  }

  @protected
  List<dynamic> api2wire_data_frame(DataFrame raw) {
    return [api2wire_RwLockPDataFrame(raw.field0)];
  }

  @protected
  Float64List api2wire_float_64_list(Float64List raw) {
    return raw;
  }

  @protected
  Object api2wire_i64(int raw) {
    return castNativeBigInt(raw);
  }

  @protected
  Int32List api2wire_int_32_list(Int32List raw) {
    return raw;
  }

  @protected
  Int64List api2wire_int_64_list(Int64List raw) {
    return raw;
  }

  @protected
  List<String>? api2wire_opt_StringList(List<String>? raw) {
    return raw == null ? null : api2wire_StringList(raw);
  }

  @protected
  int /* *bool */ ? api2wire_opt_box_autoadd_bool(bool? raw) {
    return raw == null ? 0 : api2wire_box_autoadd_bool(raw);
  }

  @protected
  int /* *u64 */ ? api2wire_opt_box_autoadd_u64(int? raw) {
    return raw == null ? 0 : api2wire_box_autoadd_u64(raw);
  }

  @protected
  int /* *u8 */ ? api2wire_opt_box_autoadd_u8(int? raw) {
    return raw == null ? 0 : api2wire_box_autoadd_u8(raw);
  }

  @protected
  Float64List? api2wire_opt_float_64_list(Float64List? raw) {
    return raw == null ? null : api2wire_float_64_list(raw);
  }

  @protected
  Int32List? api2wire_opt_int_32_list(Int32List? raw) {
    return raw == null ? null : api2wire_int_32_list(raw);
  }

  @protected
  List<dynamic> api2wire_series(Series raw) {
    return [api2wire_RwLockPSeries(raw.field0)];
  }

  @protected
  Object api2wire_u64(int raw) {
    return castNativeBigInt(raw);
  }

  @protected
  Uint8List api2wire_uint_8_list(Uint8List raw) {
    return raw;
  }

// Section: finalizer

  late final Finalizer<PlatformPointer> _RwLockPDataFrameFinalizer =
      Finalizer<PlatformPointer>(inner.drop_opaque_RwLockPDataFrame);
  Finalizer<PlatformPointer> get RwLockPDataFrameFinalizer =>
      _RwLockPDataFrameFinalizer;
  late final Finalizer<PlatformPointer> _RwLockPSeriesFinalizer =
      Finalizer<PlatformPointer>(inner.drop_opaque_RwLockPSeries);
  Finalizer<PlatformPointer> get RwLockPSeriesFinalizer =>
      _RwLockPSeriesFinalizer;
}

// Section: WASM wire module

@JS('wasm_bindgen')
external PolarsWrapperWasmModule get wasmModule;

@JS()
@anonymous
class PolarsWrapperWasmModule implements WasmModule {
  external Object /* Promise */ call([String? moduleName]);
  external PolarsWrapperWasmModule bind(dynamic thisArg, String moduleName);
  external dynamic /* void */ wire_read_csv(
      NativePortType port_,
      String path,
      int /* *bool */ ? has_header,
      List<String>? columns,
      int /* *u8 */ ? delimiter);

  external dynamic /* void */ wire_read_json(NativePortType port_, String path);

  external dynamic /* List<dynamic> */ wire_column__method__DataFrame(
      List<dynamic> that, String column);

  external dynamic /* List<dynamic> */ wire_columns__method__DataFrame(
      List<dynamic> that, List<String> columns);

  external dynamic /* String */ wire_dump__method__DataFrame(
      List<dynamic> that);

  external dynamic /* List<dynamic> */ wire_of_strings__static_method__Series(
      String name, List<String>? values);

  external dynamic /* List<dynamic> */ wire_of_i32__static_method__Series(
      String name, Int32List? values);

  external dynamic /* List<dynamic> */ wire_of_f64__static_method__Series(
      String name, Float64List? values);

  external dynamic /* void */ wire_append__method__Series(
      NativePortType port_, List<dynamic> that, List<dynamic> other);

  external dynamic /* void */ wire_as_strings__method__Series(
      NativePortType port_, List<dynamic> that);

  external dynamic /* void */ wire_as_i32__method__Series(
      NativePortType port_, List<dynamic> that);

  external dynamic /* void */ wire_as_f64__method__Series(
      NativePortType port_, List<dynamic> that);

  external dynamic /* void */ wire_abs__method__Series(
      NativePortType port_, List<dynamic> that);

  external dynamic /* void */ wire_sort__method__Series(
      NativePortType port_, List<dynamic> that, bool reverse);

  external dynamic /* void */ wire_shuffle__method__Series(
      NativePortType port_, List<dynamic> that, int /* *u64 */ ? seed);

  external dynamic /* void */ wire_sum__method__Series(
      NativePortType port_, List<dynamic> that);

  external dynamic /* void */ wire_min__method__Series(
      NativePortType port_, List<dynamic> that);

  external dynamic /* void */ wire_max__method__Series(
      NativePortType port_, List<dynamic> that);

  external dynamic /* void */ wire_explode__method__Series(
      NativePortType port_, List<dynamic> that);

  external dynamic /* void */ wire_explode_by_offsets__method__Series(
      NativePortType port_, List<dynamic> that, Int64List offsets);

  external dynamic /* void */ wire_cummax__method__Series(
      NativePortType port_, List<dynamic> that, bool reverse);

  external dynamic /* void */ wire_cummin__method__Series(
      NativePortType port_, List<dynamic> that, bool reverse);

  external dynamic /* void */ wire_cumprod__method__Series(
      NativePortType port_, List<dynamic> that, bool reverse);

  external dynamic /* void */ wire_cumsum__method__Series(
      NativePortType port_, List<dynamic> that, bool reverse);

  external dynamic /* void */ wire_product__method__Series(
      NativePortType port_, List<dynamic> that);

  external dynamic /* String? */ wire_get_string__method__Series(
      List<dynamic> that, int index);

  external dynamic /* int /* *f64 */? */ wire_get__method__Series(
      List<dynamic> that, int index);

  external dynamic /* void */ wire_mean__method__Series(
      NativePortType port_, List<dynamic> that);

  external dynamic /* void */ wire_median__method__Series(
      NativePortType port_, List<dynamic> that);

  external dynamic /* void */ wire_mean_as_series__method__Series(
      NativePortType port_, List<dynamic> that);

  external dynamic /* void */ wire_median_as_series__method__Series(
      NativePortType port_, List<dynamic> that);

  external dynamic /* int */ wire_estimated_size__method__Series(
      List<dynamic> that);

  external dynamic /* List<dynamic> */ wire_add_to__method__Series(
      List<dynamic> that, List<dynamic> other);

  external dynamic /* List<dynamic> */ wire_subtract__method__Series(
      List<dynamic> that, List<dynamic> other);

  external dynamic /* List<dynamic> */ wire_multiply__method__Series(
      List<dynamic> that, List<dynamic> other);

  external dynamic /* List<dynamic> */ wire_divide__method__Series(
      List<dynamic> that, List<dynamic> other);

  external dynamic /* List<dynamic> */ wire_remainder__method__Series(
      List<dynamic> that, List<dynamic> other);

  external dynamic /* String */ wire_dump__method__Series(List<dynamic> that);

  external int /* *mut bool */ new_box_autoadd_bool_0(bool value);

  external int /* *mut u64 */ new_box_autoadd_u64_0(Object value);

  external int /* *mut u8 */ new_box_autoadd_u8_0(int value);

  external dynamic /*  */ drop_opaque_RwLockPDataFrame(ptr);

  external int /* *const c_void */ share_opaque_RwLockPDataFrame(ptr);

  external dynamic /*  */ drop_opaque_RwLockPSeries(ptr);

  external int /* *const c_void */ share_opaque_RwLockPSeries(ptr);
}

// Section: WASM wire connector

class PolarsWrapperWire
    extends FlutterRustBridgeWasmWireBase<PolarsWrapperWasmModule> {
  PolarsWrapperWire(FutureOr<WasmModule> module)
      : super(WasmModule.cast<PolarsWrapperWasmModule>(module));

  void wire_read_csv(
          NativePortType port_,
          String path,
          int /* *bool */ ? has_header,
          List<String>? columns,
          int /* *u8 */ ? delimiter) =>
      wasmModule.wire_read_csv(port_, path, has_header, columns, delimiter);

  void wire_read_json(NativePortType port_, String path) =>
      wasmModule.wire_read_json(port_, path);

  dynamic /* List<dynamic> */ wire_column__method__DataFrame(
          List<dynamic> that, String column) =>
      wasmModule.wire_column__method__DataFrame(that, column);

  dynamic /* List<dynamic> */ wire_columns__method__DataFrame(
          List<dynamic> that, List<String> columns) =>
      wasmModule.wire_columns__method__DataFrame(that, columns);

  dynamic /* String */ wire_dump__method__DataFrame(List<dynamic> that) =>
      wasmModule.wire_dump__method__DataFrame(that);

  dynamic /* List<dynamic> */ wire_of_strings__static_method__Series(
          String name, List<String>? values) =>
      wasmModule.wire_of_strings__static_method__Series(name, values);

  dynamic /* List<dynamic> */ wire_of_i32__static_method__Series(
          String name, Int32List? values) =>
      wasmModule.wire_of_i32__static_method__Series(name, values);

  dynamic /* List<dynamic> */ wire_of_f64__static_method__Series(
          String name, Float64List? values) =>
      wasmModule.wire_of_f64__static_method__Series(name, values);

  void wire_append__method__Series(
          NativePortType port_, List<dynamic> that, List<dynamic> other) =>
      wasmModule.wire_append__method__Series(port_, that, other);

  void wire_as_strings__method__Series(
          NativePortType port_, List<dynamic> that) =>
      wasmModule.wire_as_strings__method__Series(port_, that);

  void wire_as_i32__method__Series(NativePortType port_, List<dynamic> that) =>
      wasmModule.wire_as_i32__method__Series(port_, that);

  void wire_as_f64__method__Series(NativePortType port_, List<dynamic> that) =>
      wasmModule.wire_as_f64__method__Series(port_, that);

  void wire_abs__method__Series(NativePortType port_, List<dynamic> that) =>
      wasmModule.wire_abs__method__Series(port_, that);

  void wire_sort__method__Series(
          NativePortType port_, List<dynamic> that, bool reverse) =>
      wasmModule.wire_sort__method__Series(port_, that, reverse);

  void wire_shuffle__method__Series(
          NativePortType port_, List<dynamic> that, int /* *u64 */ ? seed) =>
      wasmModule.wire_shuffle__method__Series(port_, that, seed);

  void wire_sum__method__Series(NativePortType port_, List<dynamic> that) =>
      wasmModule.wire_sum__method__Series(port_, that);

  void wire_min__method__Series(NativePortType port_, List<dynamic> that) =>
      wasmModule.wire_min__method__Series(port_, that);

  void wire_max__method__Series(NativePortType port_, List<dynamic> that) =>
      wasmModule.wire_max__method__Series(port_, that);

  void wire_explode__method__Series(NativePortType port_, List<dynamic> that) =>
      wasmModule.wire_explode__method__Series(port_, that);

  void wire_explode_by_offsets__method__Series(
          NativePortType port_, List<dynamic> that, Int64List offsets) =>
      wasmModule.wire_explode_by_offsets__method__Series(port_, that, offsets);

  void wire_cummax__method__Series(
          NativePortType port_, List<dynamic> that, bool reverse) =>
      wasmModule.wire_cummax__method__Series(port_, that, reverse);

  void wire_cummin__method__Series(
          NativePortType port_, List<dynamic> that, bool reverse) =>
      wasmModule.wire_cummin__method__Series(port_, that, reverse);

  void wire_cumprod__method__Series(
          NativePortType port_, List<dynamic> that, bool reverse) =>
      wasmModule.wire_cumprod__method__Series(port_, that, reverse);

  void wire_cumsum__method__Series(
          NativePortType port_, List<dynamic> that, bool reverse) =>
      wasmModule.wire_cumsum__method__Series(port_, that, reverse);

  void wire_product__method__Series(NativePortType port_, List<dynamic> that) =>
      wasmModule.wire_product__method__Series(port_, that);

  dynamic /* String? */ wire_get_string__method__Series(
          List<dynamic> that, int index) =>
      wasmModule.wire_get_string__method__Series(that, index);

  dynamic /* int /* *f64 */? */ wire_get__method__Series(
          List<dynamic> that, int index) =>
      wasmModule.wire_get__method__Series(that, index);

  void wire_mean__method__Series(NativePortType port_, List<dynamic> that) =>
      wasmModule.wire_mean__method__Series(port_, that);

  void wire_median__method__Series(NativePortType port_, List<dynamic> that) =>
      wasmModule.wire_median__method__Series(port_, that);

  void wire_mean_as_series__method__Series(
          NativePortType port_, List<dynamic> that) =>
      wasmModule.wire_mean_as_series__method__Series(port_, that);

  void wire_median_as_series__method__Series(
          NativePortType port_, List<dynamic> that) =>
      wasmModule.wire_median_as_series__method__Series(port_, that);

  dynamic /* int */ wire_estimated_size__method__Series(List<dynamic> that) =>
      wasmModule.wire_estimated_size__method__Series(that);

  dynamic /* List<dynamic> */ wire_add_to__method__Series(
          List<dynamic> that, List<dynamic> other) =>
      wasmModule.wire_add_to__method__Series(that, other);

  dynamic /* List<dynamic> */ wire_subtract__method__Series(
          List<dynamic> that, List<dynamic> other) =>
      wasmModule.wire_subtract__method__Series(that, other);

  dynamic /* List<dynamic> */ wire_multiply__method__Series(
          List<dynamic> that, List<dynamic> other) =>
      wasmModule.wire_multiply__method__Series(that, other);

  dynamic /* List<dynamic> */ wire_divide__method__Series(
          List<dynamic> that, List<dynamic> other) =>
      wasmModule.wire_divide__method__Series(that, other);

  dynamic /* List<dynamic> */ wire_remainder__method__Series(
          List<dynamic> that, List<dynamic> other) =>
      wasmModule.wire_remainder__method__Series(that, other);

  dynamic /* String */ wire_dump__method__Series(List<dynamic> that) =>
      wasmModule.wire_dump__method__Series(that);

  int /* *mut bool */ new_box_autoadd_bool_0(bool value) =>
      wasmModule.new_box_autoadd_bool_0(value);

  int /* *mut u64 */ new_box_autoadd_u64_0(Object value) =>
      wasmModule.new_box_autoadd_u64_0(value);

  int /* *mut u8 */ new_box_autoadd_u8_0(int value) =>
      wasmModule.new_box_autoadd_u8_0(value);

  dynamic /*  */ drop_opaque_RwLockPDataFrame(ptr) =>
      wasmModule.drop_opaque_RwLockPDataFrame(ptr);

  int /* *const c_void */ share_opaque_RwLockPDataFrame(ptr) =>
      wasmModule.share_opaque_RwLockPDataFrame(ptr);

  dynamic /*  */ drop_opaque_RwLockPSeries(ptr) =>
      wasmModule.drop_opaque_RwLockPSeries(ptr);

  int /* *const c_void */ share_opaque_RwLockPSeries(ptr) =>
      wasmModule.share_opaque_RwLockPSeries(ptr);
}
