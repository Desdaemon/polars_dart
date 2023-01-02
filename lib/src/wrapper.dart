// AUTO GENERATED FILE, DO NOT EDIT.
// Generated by `flutter_rust_bridge`@ 1.59.0.
// ignore_for_file: non_constant_identifier_names, unused_element, duplicate_ignore, directives_ordering, curly_braces_in_flow_control_structures, unnecessary_lambdas, slash_for_doc_comments, prefer_const_literals_to_create_immutables, implicit_dynamic_list_literal, duplicate_import, unused_import, prefer_single_quotes, prefer_const_constructors, use_super_parameters, always_use_package_imports, annotate_overrides, invalid_use_of_protected_member, constant_identifier_names, invalid_use_of_internal_member

import 'dart:convert';
import 'dart:async';
import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';

import 'dart:convert';
import 'dart:async';
import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'wrapper.io.dart' if (dart.library.html) 'wrapper.web.dart';

import 'package:meta/meta.dart';

abstract class PolarsWrapper {
  Future<DataFrame> readCsv(
      {required String path,
      bool? hasHeader,
      List<String>? columns,
      int? delimiter,
      dynamic hint});

  FlutterRustBridgeTaskConstMeta get kReadCsvConstMeta;

  Future<DataFrame> readJson({required String path, dynamic hint});

  FlutterRustBridgeTaskConstMeta get kReadJsonConstMeta;

  Future<Series> columnMethodDataFrame(
      {required DataFrame that, required String column, dynamic hint});

  FlutterRustBridgeTaskConstMeta get kColumnMethodDataFrameConstMeta;

  Future<List<Series>> columnsMethodDataFrame(
      {required DataFrame that, required List<String> columns, dynamic hint});

  FlutterRustBridgeTaskConstMeta get kColumnsMethodDataFrameConstMeta;

  Future<String> dumpMethodDataFrame({required DataFrame that, dynamic hint});

  FlutterRustBridgeTaskConstMeta get kDumpMethodDataFrameConstMeta;

  Future<Series> ofStringsStaticMethodSeries(
      {required String name, List<String>? values, dynamic hint});

  FlutterRustBridgeTaskConstMeta get kOfStringsStaticMethodSeriesConstMeta;

  Future<Series> ofI32StaticMethodSeries(
      {required String name, Int32List? values, dynamic hint});

  FlutterRustBridgeTaskConstMeta get kOfI32StaticMethodSeriesConstMeta;

  Future<Series> ofF64StaticMethodSeries(
      {required String name, Float64List? values, dynamic hint});

  FlutterRustBridgeTaskConstMeta get kOfF64StaticMethodSeriesConstMeta;

  Future<void> appendMethodSeries(
      {required Series that, required Series other, dynamic hint});

  FlutterRustBridgeTaskConstMeta get kAppendMethodSeriesConstMeta;

  Future<List<String?>> asStringsMethodSeries(
      {required Series that, dynamic hint});

  FlutterRustBridgeTaskConstMeta get kAsStringsMethodSeriesConstMeta;

  Future<List<int?>> asI32MethodSeries({required Series that, dynamic hint});

  FlutterRustBridgeTaskConstMeta get kAsI32MethodSeriesConstMeta;

  Future<List<double?>> asF64MethodSeries({required Series that, dynamic hint});

  FlutterRustBridgeTaskConstMeta get kAsF64MethodSeriesConstMeta;

  DropFnType get dropOpaqueRwLockPDataFrame;
  ShareFnType get shareOpaqueRwLockPDataFrame;
  OpaqueTypeFinalizer get RwLockPDataFrameFinalizer;

  DropFnType get dropOpaqueRwLockPSeries;
  ShareFnType get shareOpaqueRwLockPSeries;
  OpaqueTypeFinalizer get RwLockPSeriesFinalizer;
}

@sealed
class RwLockPDataFrame extends FrbOpaque {
  final PolarsWrapper bridge;
  RwLockPDataFrame.fromRaw(int ptr, int size, this.bridge)
      : super.unsafe(ptr, size);
  @override
  DropFnType get dropFn => bridge.dropOpaqueRwLockPDataFrame;

  @override
  ShareFnType get shareFn => bridge.shareOpaqueRwLockPDataFrame;

  @override
  OpaqueTypeFinalizer get staticFinalizer => bridge.RwLockPDataFrameFinalizer;
}

@sealed
class RwLockPSeries extends FrbOpaque {
  final PolarsWrapper bridge;
  RwLockPSeries.fromRaw(int ptr, int size, this.bridge)
      : super.unsafe(ptr, size);
  @override
  DropFnType get dropFn => bridge.dropOpaqueRwLockPSeries;

  @override
  ShareFnType get shareFn => bridge.shareOpaqueRwLockPSeries;

  @override
  OpaqueTypeFinalizer get staticFinalizer => bridge.RwLockPSeriesFinalizer;
}

class DataFrame {
  final PolarsWrapper bridge;
  final RwLockPDataFrame field0;

  DataFrame({
    required this.bridge,
    required this.field0,
  });

  Future<Series> column({required String column, dynamic hint}) =>
      bridge.columnMethodDataFrame(
        that: this,
        column: column,
      );

  Future<List<Series>> columns({required List<String> columns, dynamic hint}) =>
      bridge.columnsMethodDataFrame(
        that: this,
        columns: columns,
      );

  Future<String> dump({dynamic hint}) => bridge.dumpMethodDataFrame(
        that: this,
      );
}

class Series {
  final PolarsWrapper bridge;
  final RwLockPSeries field0;

  Series({
    required this.bridge,
    required this.field0,
  });

  static Future<Series> ofStrings(
          {required PolarsWrapper bridge,
          required String name,
          List<String>? values,
          dynamic hint}) =>
      bridge.ofStringsStaticMethodSeries(
          name: name, values: values, hint: hint);

  static Future<Series> ofI32(
          {required PolarsWrapper bridge,
          required String name,
          Int32List? values,
          dynamic hint}) =>
      bridge.ofI32StaticMethodSeries(name: name, values: values, hint: hint);

  static Future<Series> ofF64(
          {required PolarsWrapper bridge,
          required String name,
          Float64List? values,
          dynamic hint}) =>
      bridge.ofF64StaticMethodSeries(name: name, values: values, hint: hint);

  Future<void> append({required Series other, dynamic hint}) =>
      bridge.appendMethodSeries(
        that: this,
        other: other,
      );

  Future<List<String?>> asStrings({dynamic hint}) =>
      bridge.asStringsMethodSeries(
        that: this,
      );

  Future<List<int?>> asI32({dynamic hint}) => bridge.asI32MethodSeries(
        that: this,
      );

  Future<List<double?>> asF64({dynamic hint}) => bridge.asF64MethodSeries(
        that: this,
      );
}

class PolarsWrapperImpl implements PolarsWrapper {
  final PolarsWrapperPlatform _platform;
  factory PolarsWrapperImpl(ExternalLibrary dylib) =>
      PolarsWrapperImpl.raw(PolarsWrapperPlatform(dylib));

  /// Only valid on web/WASM platforms.
  factory PolarsWrapperImpl.wasm(FutureOr<WasmModule> module) =>
      PolarsWrapperImpl(module as ExternalLibrary);
  PolarsWrapperImpl.raw(this._platform);
  Future<DataFrame> readCsv(
      {required String path,
      bool? hasHeader,
      List<String>? columns,
      int? delimiter,
      dynamic hint}) {
    var arg0 = _platform.api2wire_String(path);
    var arg1 = _platform.api2wire_opt_box_autoadd_bool(hasHeader);
    var arg2 = _platform.api2wire_opt_StringList(columns);
    var arg3 = _platform.api2wire_opt_box_autoadd_u8(delimiter);
    return _platform.executeNormal(FlutterRustBridgeTask(
      callFfi: (port_) =>
          _platform.inner.wire_read_csv(port_, arg0, arg1, arg2, arg3),
      parseSuccessData: (d) => _wire2api_data_frame(d),
      constMeta: kReadCsvConstMeta,
      argValues: [path, hasHeader, columns, delimiter],
      hint: hint,
    ));
  }

  FlutterRustBridgeTaskConstMeta get kReadCsvConstMeta =>
      const FlutterRustBridgeTaskConstMeta(
        debugName: "read_csv",
        argNames: ["path", "hasHeader", "columns", "delimiter"],
      );

  Future<DataFrame> readJson({required String path, dynamic hint}) {
    var arg0 = _platform.api2wire_String(path);
    return _platform.executeNormal(FlutterRustBridgeTask(
      callFfi: (port_) => _platform.inner.wire_read_json(port_, arg0),
      parseSuccessData: (d) => _wire2api_data_frame(d),
      constMeta: kReadJsonConstMeta,
      argValues: [path],
      hint: hint,
    ));
  }

  FlutterRustBridgeTaskConstMeta get kReadJsonConstMeta =>
      const FlutterRustBridgeTaskConstMeta(
        debugName: "read_json",
        argNames: ["path"],
      );

  Future<Series> columnMethodDataFrame(
      {required DataFrame that, required String column, dynamic hint}) {
    var arg0 = _platform.api2wire_box_autoadd_data_frame(that);
    var arg1 = _platform.api2wire_String(column);
    return _platform.executeNormal(FlutterRustBridgeTask(
      callFfi: (port_) =>
          _platform.inner.wire_column__method__DataFrame(port_, arg0, arg1),
      parseSuccessData: (d) => _wire2api_series(d),
      constMeta: kColumnMethodDataFrameConstMeta,
      argValues: [that, column],
      hint: hint,
    ));
  }

  FlutterRustBridgeTaskConstMeta get kColumnMethodDataFrameConstMeta =>
      const FlutterRustBridgeTaskConstMeta(
        debugName: "column__method__DataFrame",
        argNames: ["that", "column"],
      );

  Future<List<Series>> columnsMethodDataFrame(
      {required DataFrame that, required List<String> columns, dynamic hint}) {
    var arg0 = _platform.api2wire_box_autoadd_data_frame(that);
    var arg1 = _platform.api2wire_StringList(columns);
    return _platform.executeNormal(FlutterRustBridgeTask(
      callFfi: (port_) =>
          _platform.inner.wire_columns__method__DataFrame(port_, arg0, arg1),
      parseSuccessData: _wire2api_list_series,
      constMeta: kColumnsMethodDataFrameConstMeta,
      argValues: [that, columns],
      hint: hint,
    ));
  }

  FlutterRustBridgeTaskConstMeta get kColumnsMethodDataFrameConstMeta =>
      const FlutterRustBridgeTaskConstMeta(
        debugName: "columns__method__DataFrame",
        argNames: ["that", "columns"],
      );

  Future<String> dumpMethodDataFrame({required DataFrame that, dynamic hint}) {
    var arg0 = _platform.api2wire_box_autoadd_data_frame(that);
    return _platform.executeNormal(FlutterRustBridgeTask(
      callFfi: (port_) =>
          _platform.inner.wire_dump__method__DataFrame(port_, arg0),
      parseSuccessData: _wire2api_String,
      constMeta: kDumpMethodDataFrameConstMeta,
      argValues: [that],
      hint: hint,
    ));
  }

  FlutterRustBridgeTaskConstMeta get kDumpMethodDataFrameConstMeta =>
      const FlutterRustBridgeTaskConstMeta(
        debugName: "dump__method__DataFrame",
        argNames: ["that"],
      );

  Future<Series> ofStringsStaticMethodSeries(
      {required String name, List<String>? values, dynamic hint}) {
    var arg0 = _platform.api2wire_String(name);
    var arg1 = _platform.api2wire_opt_StringList(values);
    return _platform.executeNormal(FlutterRustBridgeTask(
      callFfi: (port_) => _platform.inner
          .wire_of_strings__static_method__Series(port_, arg0, arg1),
      parseSuccessData: (d) => _wire2api_series(d),
      constMeta: kOfStringsStaticMethodSeriesConstMeta,
      argValues: [name, values],
      hint: hint,
    ));
  }

  FlutterRustBridgeTaskConstMeta get kOfStringsStaticMethodSeriesConstMeta =>
      const FlutterRustBridgeTaskConstMeta(
        debugName: "of_strings__static_method__Series",
        argNames: ["name", "values"],
      );

  Future<Series> ofI32StaticMethodSeries(
      {required String name, Int32List? values, dynamic hint}) {
    var arg0 = _platform.api2wire_String(name);
    var arg1 = _platform.api2wire_opt_int_32_list(values);
    return _platform.executeNormal(FlutterRustBridgeTask(
      callFfi: (port_) =>
          _platform.inner.wire_of_i32__static_method__Series(port_, arg0, arg1),
      parseSuccessData: (d) => _wire2api_series(d),
      constMeta: kOfI32StaticMethodSeriesConstMeta,
      argValues: [name, values],
      hint: hint,
    ));
  }

  FlutterRustBridgeTaskConstMeta get kOfI32StaticMethodSeriesConstMeta =>
      const FlutterRustBridgeTaskConstMeta(
        debugName: "of_i32__static_method__Series",
        argNames: ["name", "values"],
      );

  Future<Series> ofF64StaticMethodSeries(
      {required String name, Float64List? values, dynamic hint}) {
    var arg0 = _platform.api2wire_String(name);
    var arg1 = _platform.api2wire_opt_float_64_list(values);
    return _platform.executeNormal(FlutterRustBridgeTask(
      callFfi: (port_) =>
          _platform.inner.wire_of_f64__static_method__Series(port_, arg0, arg1),
      parseSuccessData: (d) => _wire2api_series(d),
      constMeta: kOfF64StaticMethodSeriesConstMeta,
      argValues: [name, values],
      hint: hint,
    ));
  }

  FlutterRustBridgeTaskConstMeta get kOfF64StaticMethodSeriesConstMeta =>
      const FlutterRustBridgeTaskConstMeta(
        debugName: "of_f64__static_method__Series",
        argNames: ["name", "values"],
      );

  Future<void> appendMethodSeries(
      {required Series that, required Series other, dynamic hint}) {
    var arg0 = _platform.api2wire_box_autoadd_series(that);
    var arg1 = _platform.api2wire_box_autoadd_series(other);
    return _platform.executeNormal(FlutterRustBridgeTask(
      callFfi: (port_) =>
          _platform.inner.wire_append__method__Series(port_, arg0, arg1),
      parseSuccessData: _wire2api_unit,
      constMeta: kAppendMethodSeriesConstMeta,
      argValues: [that, other],
      hint: hint,
    ));
  }

  FlutterRustBridgeTaskConstMeta get kAppendMethodSeriesConstMeta =>
      const FlutterRustBridgeTaskConstMeta(
        debugName: "append__method__Series",
        argNames: ["that", "other"],
      );

  Future<List<String?>> asStringsMethodSeries(
      {required Series that, dynamic hint}) {
    var arg0 = _platform.api2wire_box_autoadd_series(that);
    return _platform.executeNormal(FlutterRustBridgeTask(
      callFfi: (port_) =>
          _platform.inner.wire_as_strings__method__Series(port_, arg0),
      parseSuccessData: _wire2api_list_opt_String,
      constMeta: kAsStringsMethodSeriesConstMeta,
      argValues: [that],
      hint: hint,
    ));
  }

  FlutterRustBridgeTaskConstMeta get kAsStringsMethodSeriesConstMeta =>
      const FlutterRustBridgeTaskConstMeta(
        debugName: "as_strings__method__Series",
        argNames: ["that"],
      );

  Future<List<int?>> asI32MethodSeries({required Series that, dynamic hint}) {
    var arg0 = _platform.api2wire_box_autoadd_series(that);
    return _platform.executeNormal(FlutterRustBridgeTask(
      callFfi: (port_) =>
          _platform.inner.wire_as_i32__method__Series(port_, arg0),
      parseSuccessData: _wire2api_list_opt_box_autoadd_i32,
      constMeta: kAsI32MethodSeriesConstMeta,
      argValues: [that],
      hint: hint,
    ));
  }

  FlutterRustBridgeTaskConstMeta get kAsI32MethodSeriesConstMeta =>
      const FlutterRustBridgeTaskConstMeta(
        debugName: "as_i32__method__Series",
        argNames: ["that"],
      );

  Future<List<double?>> asF64MethodSeries(
      {required Series that, dynamic hint}) {
    var arg0 = _platform.api2wire_box_autoadd_series(that);
    return _platform.executeNormal(FlutterRustBridgeTask(
      callFfi: (port_) =>
          _platform.inner.wire_as_f64__method__Series(port_, arg0),
      parseSuccessData: _wire2api_list_opt_box_autoadd_f64,
      constMeta: kAsF64MethodSeriesConstMeta,
      argValues: [that],
      hint: hint,
    ));
  }

  FlutterRustBridgeTaskConstMeta get kAsF64MethodSeriesConstMeta =>
      const FlutterRustBridgeTaskConstMeta(
        debugName: "as_f64__method__Series",
        argNames: ["that"],
      );

  DropFnType get dropOpaqueRwLockPDataFrame =>
      _platform.inner.drop_opaque_RwLockPDataFrame;
  ShareFnType get shareOpaqueRwLockPDataFrame =>
      _platform.inner.share_opaque_RwLockPDataFrame;
  OpaqueTypeFinalizer get RwLockPDataFrameFinalizer =>
      _platform.RwLockPDataFrameFinalizer;

  DropFnType get dropOpaqueRwLockPSeries =>
      _platform.inner.drop_opaque_RwLockPSeries;
  ShareFnType get shareOpaqueRwLockPSeries =>
      _platform.inner.share_opaque_RwLockPSeries;
  OpaqueTypeFinalizer get RwLockPSeriesFinalizer =>
      _platform.RwLockPSeriesFinalizer;

  void dispose() {
    _platform.dispose();
  }
// Section: wire2api

  RwLockPDataFrame _wire2api_RwLockPDataFrame(dynamic raw) {
    return RwLockPDataFrame.fromRaw(raw[0], raw[1], this);
  }

  RwLockPSeries _wire2api_RwLockPSeries(dynamic raw) {
    return RwLockPSeries.fromRaw(raw[0], raw[1], this);
  }

  String _wire2api_String(dynamic raw) {
    return raw as String;
  }

  double _wire2api_box_autoadd_f64(dynamic raw) {
    return raw as double;
  }

  int _wire2api_box_autoadd_i32(dynamic raw) {
    return raw as int;
  }

  DataFrame _wire2api_data_frame(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 1)
      throw Exception('unexpected arr length: expect 1 but see ${arr.length}');
    return DataFrame(
      bridge: this,
      field0: _wire2api_RwLockPDataFrame(arr[0]),
    );
  }

  double _wire2api_f64(dynamic raw) {
    return raw as double;
  }

  int _wire2api_i32(dynamic raw) {
    return raw as int;
  }

  List<String?> _wire2api_list_opt_String(dynamic raw) {
    return (raw as List<dynamic>).map(_wire2api_opt_String).toList();
  }

  List<double?> _wire2api_list_opt_box_autoadd_f64(dynamic raw) {
    return (raw as List<dynamic>).map(_wire2api_opt_box_autoadd_f64).toList();
  }

  List<int?> _wire2api_list_opt_box_autoadd_i32(dynamic raw) {
    return (raw as List<dynamic>).map(_wire2api_opt_box_autoadd_i32).toList();
  }

  List<Series> _wire2api_list_series(dynamic raw) {
    return (raw as List<dynamic>).map(_wire2api_series).toList();
  }

  String? _wire2api_opt_String(dynamic raw) {
    return raw == null ? null : _wire2api_String(raw);
  }

  double? _wire2api_opt_box_autoadd_f64(dynamic raw) {
    return raw == null ? null : _wire2api_box_autoadd_f64(raw);
  }

  int? _wire2api_opt_box_autoadd_i32(dynamic raw) {
    return raw == null ? null : _wire2api_box_autoadd_i32(raw);
  }

  Series _wire2api_series(dynamic raw) {
    final arr = raw as List<dynamic>;
    if (arr.length != 1)
      throw Exception('unexpected arr length: expect 1 but see ${arr.length}');
    return Series(
      bridge: this,
      field0: _wire2api_RwLockPSeries(arr[0]),
    );
  }

  int _wire2api_u8(dynamic raw) {
    return raw as int;
  }

  Uint8List _wire2api_uint_8_list(dynamic raw) {
    return raw as Uint8List;
  }

  void _wire2api_unit(dynamic raw) {
    return;
  }
}

// Section: api2wire

@protected
bool api2wire_bool(bool raw) {
  return raw;
}

@protected
double api2wire_f64(double raw) {
  return raw;
}

@protected
int api2wire_i32(int raw) {
  return raw;
}

@protected
int api2wire_u8(int raw) {
  return raw;
}

// Section: finalizer
