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
  Object api2wire_Chrono_Duration(Duration raw) {
    return api2wire_i64(raw.inMilliseconds);
  }

  @protected
  Object api2wire_Chrono_Naive(DateTime raw) {
    return api2wire_i64(raw.millisecondsSinceEpoch);
  }

  @protected
  Object api2wire_RwLockPDataFrame(RwLockPDataFrame raw) {
    return raw.shareOrMove();
  }

  @protected
  Object api2wire_RwLockPLazyFrame(RwLockPLazyFrame raw) {
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
  List<dynamic> api2wire_agg_expr(AggExpr raw) {
    if (raw is AggExpr_Min) {
      return [
        0,
        api2wire_box_expr(raw.input),
        api2wire_bool(raw.propagateNans)
      ];
    }
    if (raw is AggExpr_Max) {
      return [
        1,
        api2wire_box_expr(raw.input),
        api2wire_bool(raw.propagateNans)
      ];
    }
    if (raw is AggExpr_Median) {
      return [2, api2wire_box_expr(raw.field0)];
    }
    if (raw is AggExpr_NUnique) {
      return [3, api2wire_box_expr(raw.field0)];
    }
    if (raw is AggExpr_First) {
      return [4, api2wire_box_expr(raw.field0)];
    }
    if (raw is AggExpr_Last) {
      return [5, api2wire_box_expr(raw.field0)];
    }
    if (raw is AggExpr_Mean) {
      return [6, api2wire_box_expr(raw.field0)];
    }
    if (raw is AggExpr_List) {
      return [7, api2wire_box_expr(raw.field0)];
    }
    if (raw is AggExpr_Count) {
      return [8, api2wire_box_expr(raw.field0)];
    }
    if (raw is AggExpr_Sum) {
      return [9, api2wire_box_expr(raw.field0)];
    }
    if (raw is AggExpr_AggGroups) {
      return [10, api2wire_box_expr(raw.field0)];
    }
    if (raw is AggExpr_Std) {
      return [11, api2wire_box_expr(raw.field0), api2wire_u8(raw.field1)];
    }

    throw Exception('unreachable');
  }

  @protected
  List<dynamic> api2wire_box_autoadd_agg_expr(AggExpr raw) {
    return api2wire_agg_expr(raw);
  }

  @protected
  bool api2wire_box_autoadd_bool(bool raw) {
    return api2wire_bool(raw);
  }

  @protected
  List<dynamic> api2wire_box_autoadd_data_frame(DataFrame raw) {
    return api2wire_data_frame(raw);
  }

  @protected
  List<dynamic> api2wire_box_autoadd_data_type(DataType raw) {
    return api2wire_data_type(raw);
  }

  @protected
  List<dynamic> api2wire_box_autoadd_expr(Expr raw) {
    return api2wire_expr(raw);
  }

  @protected
  List<dynamic> api2wire_box_autoadd_lazy_frame(LazyFrame raw) {
    return api2wire_lazy_frame(raw);
  }

  @protected
  List<dynamic> api2wire_box_autoadd_literal_value(LiteralValue raw) {
    return api2wire_literal_value(raw);
  }

  @protected
  List<dynamic> api2wire_box_autoadd_series(Series raw) {
    return api2wire_series(raw);
  }

  @protected
  List<dynamic> api2wire_box_autoadd_sort_options(SortOptions raw) {
    return api2wire_sort_options(raw);
  }

  @protected
  int api2wire_box_autoadd_u32(int raw) {
    return api2wire_u32(raw);
  }

  @protected
  Object api2wire_box_autoadd_u64(int raw) {
    return api2wire_u64(raw);
  }

  @protected
  int api2wire_box_autoadd_u8(int raw) {
    return api2wire_u8(raw);
  }

  @protected
  int api2wire_box_autoadd_usize(int raw) {
    return api2wire_usize(raw);
  }

  @protected
  List<dynamic> api2wire_box_data_type(DataType raw) {
    return api2wire_data_type(raw);
  }

  @protected
  List<dynamic> api2wire_box_expr(Expr raw) {
    return api2wire_expr(raw);
  }

  @protected
  List<dynamic> api2wire_data_frame(DataFrame raw) {
    return [api2wire_RwLockPDataFrame(raw.field0)];
  }

  @protected
  List<dynamic> api2wire_data_type(DataType raw) {
    if (raw is DataType_Boolean) {
      return [0];
    }
    if (raw is DataType_UInt8) {
      return [1];
    }
    if (raw is DataType_UInt16) {
      return [2];
    }
    if (raw is DataType_UInt32) {
      return [3];
    }
    if (raw is DataType_UInt64) {
      return [4];
    }
    if (raw is DataType_Int8) {
      return [5];
    }
    if (raw is DataType_Int16) {
      return [6];
    }
    if (raw is DataType_Int32) {
      return [7];
    }
    if (raw is DataType_Int64) {
      return [8];
    }
    if (raw is DataType_Float32) {
      return [9];
    }
    if (raw is DataType_Float64) {
      return [10];
    }
    if (raw is DataType_Utf8) {
      return [11];
    }
    if (raw is DataType_Binary) {
      return [12];
    }
    if (raw is DataType_Date) {
      return [13];
    }
    if (raw is DataType_Datetime) {
      return [
        14,
        api2wire_time_unit(raw.field0),
        api2wire_opt_String(raw.field1)
      ];
    }
    if (raw is DataType_Duration) {
      return [15, api2wire_time_unit(raw.field0)];
    }
    if (raw is DataType_Time) {
      return [16];
    }
    if (raw is DataType_List) {
      return [17, api2wire_box_data_type(raw.field0)];
    }
    if (raw is DataType_Unknown) {
      return [18];
    }

    throw Exception('unreachable');
  }

  @protected
  List<dynamic> api2wire_expr(Expr raw) {
    if (raw is Expr_Columns) {
      return [0, api2wire_StringList(raw.field0)];
    }
    if (raw is Expr_DtypeColumn) {
      return [1, api2wire_list_data_type(raw.field0)];
    }
    if (raw is Expr_Literal) {
      return [2, api2wire_box_autoadd_literal_value(raw.field0)];
    }
    if (raw is Expr_BinaryExpr) {
      return [
        3,
        api2wire_box_expr(raw.left),
        api2wire_operator(raw.op),
        api2wire_box_expr(raw.right)
      ];
    }
    if (raw is Expr_Cast) {
      return [
        4,
        api2wire_box_expr(raw.expr),
        api2wire_box_autoadd_data_type(raw.dataType),
        api2wire_bool(raw.strict)
      ];
    }
    if (raw is Expr_Sort) {
      return [
        5,
        api2wire_box_expr(raw.expr),
        api2wire_box_autoadd_sort_options(raw.options)
      ];
    }
    if (raw is Expr_Take) {
      return [6, api2wire_box_expr(raw.expr), api2wire_box_expr(raw.idx)];
    }
    if (raw is Expr_Agg) {
      return [7, api2wire_box_autoadd_agg_expr(raw.field0)];
    }
    if (raw is Expr_Ternary) {
      return [
        8,
        api2wire_box_expr(raw.predicate),
        api2wire_box_expr(raw.truthy),
        api2wire_box_expr(raw.falsy)
      ];
    }
    if (raw is Expr_Explode) {
      return [9, api2wire_box_expr(raw.field0)];
    }
    if (raw is Expr_Filter) {
      return [10, api2wire_box_expr(raw.input), api2wire_box_expr(raw.by)];
    }
    if (raw is Expr_Wildcard) {
      return [11];
    }
    if (raw is Expr_Slice) {
      return [
        12,
        api2wire_box_expr(raw.input),
        api2wire_box_expr(raw.offset),
        api2wire_box_expr(raw.length)
      ];
    }
    if (raw is Expr_KeepName) {
      return [13, api2wire_box_expr(raw.field0)];
    }
    if (raw is Expr_Count) {
      return [14];
    }
    if (raw is Expr_Nth) {
      return [15, api2wire_i64(raw.field0)];
    }

    throw Exception('unreachable');
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
  Object /* BigInt64Array */ api2wire_int_64_list(Int64List raw) {
    return raw.inner;
  }

  @protected
  List<dynamic> api2wire_lazy_frame(LazyFrame raw) {
    return [api2wire_RwLockPLazyFrame(raw.field0)];
  }

  @protected
  List<dynamic> api2wire_list_data_type(List<DataType> raw) {
    return raw.map(api2wire_data_type).toList();
  }

  @protected
  List<dynamic> api2wire_literal_value(LiteralValue raw) {
    if (raw is LiteralValue_Boolean) {
      return [0, api2wire_bool(raw.field0)];
    }
    if (raw is LiteralValue_Utf8) {
      return [1, api2wire_String(raw.field0)];
    }
    if (raw is LiteralValue_Binary) {
      return [2, api2wire_uint_8_list(raw.field0)];
    }
    if (raw is LiteralValue_UInt8) {
      return [3, api2wire_u8(raw.field0)];
    }
    if (raw is LiteralValue_UInt16) {
      return [4, api2wire_u16(raw.field0)];
    }
    if (raw is LiteralValue_UInt32) {
      return [5, api2wire_u32(raw.field0)];
    }
    if (raw is LiteralValue_UInt64) {
      return [6, api2wire_u64(raw.field0)];
    }
    if (raw is LiteralValue_Int8) {
      return [7, api2wire_i8(raw.field0)];
    }
    if (raw is LiteralValue_Int16) {
      return [8, api2wire_i16(raw.field0)];
    }
    if (raw is LiteralValue_Int32) {
      return [9, api2wire_i32(raw.field0)];
    }
    if (raw is LiteralValue_Int64) {
      return [10, api2wire_i64(raw.field0)];
    }
    if (raw is LiteralValue_Float32) {
      return [11, api2wire_f32(raw.field0)];
    }
    if (raw is LiteralValue_Float64) {
      return [12, api2wire_f64(raw.field0)];
    }
    if (raw is LiteralValue_Range) {
      return [
        13,
        api2wire_i64(raw.low),
        api2wire_i64(raw.high),
        api2wire_box_autoadd_data_type(raw.dataType)
      ];
    }
    if (raw is LiteralValue_DateTime) {
      return [
        14,
        api2wire_Chrono_Naive(raw.field0),
        api2wire_time_unit(raw.field1)
      ];
    }
    if (raw is LiteralValue_Duration) {
      return [
        15,
        api2wire_Chrono_Duration(raw.field0),
        api2wire_time_unit(raw.field1)
      ];
    }

    throw Exception('unreachable');
  }

  @protected
  String? api2wire_opt_String(String? raw) {
    return raw == null ? null : api2wire_String(raw);
  }

  @protected
  bool? api2wire_opt_box_autoadd_bool(bool? raw) {
    return raw == null ? null : api2wire_box_autoadd_bool(raw);
  }

  @protected
  int? api2wire_opt_box_autoadd_u32(int? raw) {
    return raw == null ? null : api2wire_box_autoadd_u32(raw);
  }

  @protected
  Object? api2wire_opt_box_autoadd_u64(int? raw) {
    return raw == null ? null : api2wire_box_autoadd_u64(raw);
  }

  @protected
  int? api2wire_opt_box_autoadd_u8(int? raw) {
    return raw == null ? null : api2wire_box_autoadd_u8(raw);
  }

  @protected
  int? api2wire_opt_box_autoadd_usize(int? raw) {
    return raw == null ? null : api2wire_box_autoadd_usize(raw);
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
  Object /* BigInt64Array */ ? api2wire_opt_int_64_list(Int64List? raw) {
    return raw == null ? null : api2wire_int_64_list(raw);
  }

  @protected
  List<dynamic> api2wire_series(Series raw) {
    return [api2wire_RwLockPSeries(raw.field0)];
  }

  @protected
  List<dynamic> api2wire_sort_options(SortOptions raw) {
    return [api2wire_bool(raw.descending), api2wire_bool(raw.nullsLast)];
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
  late final Finalizer<PlatformPointer> _RwLockPLazyFrameFinalizer =
      Finalizer<PlatformPointer>(inner.drop_opaque_RwLockPLazyFrame);
  Finalizer<PlatformPointer> get RwLockPLazyFrameFinalizer =>
      _RwLockPLazyFrameFinalizer;
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
      bool? has_header,
      int? delimiter,
      int? skip_rows,
      int? skip_rows_after_header,
      int? chunk_size);

  external dynamic /* void */ wire_iter__method__DataFrame(
      NativePortType port_, List<dynamic> that);

  external dynamic /* List<dynamic> */ wire_column__method__DataFrame(
      List<dynamic> that, String column);

  external dynamic /* List<dynamic> */ wire_columns__method__DataFrame(
      List<dynamic> that, List<String> columns);

  external dynamic /* void */ wire_dump__method__DataFrame(
      NativePortType port_, List<dynamic> that);

  external dynamic /* int */ wire_estimated_size__method__DataFrame(
      List<dynamic> that);

  external dynamic /* void */ wire_with_row_count__method__DataFrame(
      NativePortType port_, List<dynamic> that, String name, int? offset);

  external dynamic /* List<String> */ wire_get_column_names__method__DataFrame(
      List<dynamic> that);

  external dynamic /* void */ wire_get_columns__method__DataFrame(
      NativePortType port_, List<dynamic> that);

  external dynamic /* int */ wire_width__method__DataFrame(List<dynamic> that);

  external dynamic /* int */ wire_height__method__DataFrame(List<dynamic> that);

  external dynamic /* bool */ wire_is_empty__method__DataFrame(
      List<dynamic> that);

  external dynamic /* void */ wire_sample__method__DataFrame(
      NativePortType port_,
      List<dynamic> that,
      int n,
      bool with_replacement,
      bool shuffle,
      Object? seed);

  external dynamic /* List<dynamic> */ wire_select__method__DataFrame(
      List<dynamic> that, List<String> columns);

  external dynamic /* List<dynamic> */ wire_head__method__DataFrame(
      List<dynamic> that, int? length);

  external dynamic /* List<dynamic> */ wire_tail__method__DataFrame(
      List<dynamic> that, int? length);

  external dynamic /* void */ wire_describe__method__DataFrame(
      NativePortType port_, List<dynamic> that, Float64List? percentiles);

  external dynamic /* List<dynamic> */ wire_drop__method__DataFrame(
      List<dynamic> that, String column);

  external dynamic /* List<dynamic> */ wire_drop_in_place__method__DataFrame(
      List<dynamic> that, String column);

  external dynamic /* List<dynamic> */ wire_reverse__method__DataFrame(
      List<dynamic> that);

  external dynamic /* List<dynamic> */ wire_shape__method__DataFrame(
      List<dynamic> that);

  external dynamic /* void */ wire_max__method__DataFrame(
      NativePortType port_, List<dynamic> that);

  external dynamic /* void */ wire_get_row__method__DataFrame(
      NativePortType port_, List<dynamic> that, int index);

  external dynamic /* List<dynamic> */ wire_lazy__method__DataFrame(
      List<dynamic> that,
      bool allow_copy,
      bool? projection_pushdown,
      bool? predicate_pushdown,
      bool? type_coercion,
      bool? simplify_expressions,
      bool? slice_pushdown,
      bool? streaming);

  external dynamic /* List<dynamic> */ wire_with_column__method__LazyFrame(
      List<dynamic> that, List<dynamic> expr);

  external dynamic /* List<dynamic> */ wire_of_i32__static_method__Series(
      String name, Int32List? values);

  external dynamic /* List<dynamic> */ wire_of_i64__static_method__Series(
      String name, Object /* BigInt64Array */ ? values);

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

  external dynamic /* void */ wire_as_durations__method__Series(
      NativePortType port_, List<dynamic> that);

  external dynamic /* void */ wire_as_naive_datetime__method__Series(
      NativePortType port_, List<dynamic> that);

  external dynamic /* void */ wire_as_utc_datetime__method__Series(
      NativePortType port_, List<dynamic> that);

  external dynamic /* void */ wire_as_local_datetime__method__Series(
      NativePortType port_, List<dynamic> that);

  external dynamic /* void */ wire_abs__method__Series(
      NativePortType port_, List<dynamic> that);

  external dynamic /* void */ wire_sort__method__Series(
      NativePortType port_, List<dynamic> that, bool reverse);

  external dynamic /* void */ wire_shuffle__method__Series(
      NativePortType port_, List<dynamic> that, Object? seed);

  external dynamic /* void */ wire_sum__method__Series(
      NativePortType port_, List<dynamic> that);

  external dynamic /* void */ wire_sum_as_series__method__Series(
      NativePortType port_, List<dynamic> that);

  external dynamic /* void */ wire_min__method__Series(
      NativePortType port_, List<dynamic> that);

  external dynamic /* void */ wire_max__method__Series(
      NativePortType port_, List<dynamic> that);

  external dynamic /* void */ wire_explode__method__Series(
      NativePortType port_, List<dynamic> that);

  external dynamic /* void */ wire_explode_by_offsets__method__Series(
      NativePortType port_,
      List<dynamic> that,
      Object /* BigInt64Array */ offsets);

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

  external dynamic /* double? */ wire_get__method__Series(
      List<dynamic> that, int index);

  external dynamic /* List<dynamic> */ wire_head__method__Series(
      List<dynamic> that, int? length);

  external dynamic /* List<dynamic> */ wire_tail__method__Series(
      List<dynamic> that, int? length);

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

  external dynamic /* bool */ wire_is_bool__method__Series(List<dynamic> that);

  external dynamic /* bool */ wire_is_utf8__method__Series(List<dynamic> that);

  external dynamic /* bool */ wire_is_numeric__method__Series(
      List<dynamic> that);

  external dynamic /* bool */ wire_is_temporal__method__Series(
      List<dynamic> that);

  external dynamic /* void */ wire_dump__method__Series(
      NativePortType port_, List<dynamic> that);

  external dynamic /* void */ wire_rename__method__Series(
      List<dynamic> that, String name);

  external dynamic /* void */ wire_unique__method__Series(
      NativePortType port_, List<dynamic> that, bool stable);

  external dynamic /* void */ wire_equal__method__Series(NativePortType port_,
      List<dynamic> that, List<dynamic> other, bool ignore_null);

  external dynamic /* void */ wire_reshape__method__Series(NativePortType port_,
      List<dynamic> that, Object /* BigInt64Array */ dims);

  external dynamic /* void */ wire_std_as_series__method__Series(
      NativePortType port_, List<dynamic> that, int ddof);

  external dynamic /*  */ drop_opaque_RwLockPDataFrame(ptr);

  external int /* *const c_void */ share_opaque_RwLockPDataFrame(ptr);

  external dynamic /*  */ drop_opaque_RwLockPLazyFrame(ptr);

  external int /* *const c_void */ share_opaque_RwLockPLazyFrame(ptr);

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
          bool? has_header,
          int? delimiter,
          int? skip_rows,
          int? skip_rows_after_header,
          int? chunk_size) =>
      wasmModule.wire_read_csv(port_, path, has_header, delimiter, skip_rows,
          skip_rows_after_header, chunk_size);

  void wire_iter__method__DataFrame(NativePortType port_, List<dynamic> that) =>
      wasmModule.wire_iter__method__DataFrame(port_, that);

  dynamic /* List<dynamic> */ wire_column__method__DataFrame(
          List<dynamic> that, String column) =>
      wasmModule.wire_column__method__DataFrame(that, column);

  dynamic /* List<dynamic> */ wire_columns__method__DataFrame(
          List<dynamic> that, List<String> columns) =>
      wasmModule.wire_columns__method__DataFrame(that, columns);

  void wire_dump__method__DataFrame(NativePortType port_, List<dynamic> that) =>
      wasmModule.wire_dump__method__DataFrame(port_, that);

  dynamic /* int */ wire_estimated_size__method__DataFrame(
          List<dynamic> that) =>
      wasmModule.wire_estimated_size__method__DataFrame(that);

  void wire_with_row_count__method__DataFrame(
          NativePortType port_, List<dynamic> that, String name, int? offset) =>
      wasmModule.wire_with_row_count__method__DataFrame(
          port_, that, name, offset);

  dynamic /* List<String> */ wire_get_column_names__method__DataFrame(
          List<dynamic> that) =>
      wasmModule.wire_get_column_names__method__DataFrame(that);

  void wire_get_columns__method__DataFrame(
          NativePortType port_, List<dynamic> that) =>
      wasmModule.wire_get_columns__method__DataFrame(port_, that);

  dynamic /* int */ wire_width__method__DataFrame(List<dynamic> that) =>
      wasmModule.wire_width__method__DataFrame(that);

  dynamic /* int */ wire_height__method__DataFrame(List<dynamic> that) =>
      wasmModule.wire_height__method__DataFrame(that);

  dynamic /* bool */ wire_is_empty__method__DataFrame(List<dynamic> that) =>
      wasmModule.wire_is_empty__method__DataFrame(that);

  void wire_sample__method__DataFrame(NativePortType port_, List<dynamic> that,
          int n, bool with_replacement, bool shuffle, Object? seed) =>
      wasmModule.wire_sample__method__DataFrame(
          port_, that, n, with_replacement, shuffle, seed);

  dynamic /* List<dynamic> */ wire_select__method__DataFrame(
          List<dynamic> that, List<String> columns) =>
      wasmModule.wire_select__method__DataFrame(that, columns);

  dynamic /* List<dynamic> */ wire_head__method__DataFrame(
          List<dynamic> that, int? length) =>
      wasmModule.wire_head__method__DataFrame(that, length);

  dynamic /* List<dynamic> */ wire_tail__method__DataFrame(
          List<dynamic> that, int? length) =>
      wasmModule.wire_tail__method__DataFrame(that, length);

  void wire_describe__method__DataFrame(
          NativePortType port_, List<dynamic> that, Float64List? percentiles) =>
      wasmModule.wire_describe__method__DataFrame(port_, that, percentiles);

  dynamic /* List<dynamic> */ wire_drop__method__DataFrame(
          List<dynamic> that, String column) =>
      wasmModule.wire_drop__method__DataFrame(that, column);

  dynamic /* List<dynamic> */ wire_drop_in_place__method__DataFrame(
          List<dynamic> that, String column) =>
      wasmModule.wire_drop_in_place__method__DataFrame(that, column);

  dynamic /* List<dynamic> */ wire_reverse__method__DataFrame(
          List<dynamic> that) =>
      wasmModule.wire_reverse__method__DataFrame(that);

  dynamic /* List<dynamic> */ wire_shape__method__DataFrame(
          List<dynamic> that) =>
      wasmModule.wire_shape__method__DataFrame(that);

  void wire_max__method__DataFrame(NativePortType port_, List<dynamic> that) =>
      wasmModule.wire_max__method__DataFrame(port_, that);

  void wire_get_row__method__DataFrame(
          NativePortType port_, List<dynamic> that, int index) =>
      wasmModule.wire_get_row__method__DataFrame(port_, that, index);

  dynamic /* List<dynamic> */ wire_lazy__method__DataFrame(
          List<dynamic> that,
          bool allow_copy,
          bool? projection_pushdown,
          bool? predicate_pushdown,
          bool? type_coercion,
          bool? simplify_expressions,
          bool? slice_pushdown,
          bool? streaming) =>
      wasmModule.wire_lazy__method__DataFrame(
          that,
          allow_copy,
          projection_pushdown,
          predicate_pushdown,
          type_coercion,
          simplify_expressions,
          slice_pushdown,
          streaming);

  dynamic /* List<dynamic> */ wire_with_column__method__LazyFrame(
          List<dynamic> that, List<dynamic> expr) =>
      wasmModule.wire_with_column__method__LazyFrame(that, expr);

  dynamic /* List<dynamic> */ wire_of_i32__static_method__Series(
          String name, Int32List? values) =>
      wasmModule.wire_of_i32__static_method__Series(name, values);

  dynamic /* List<dynamic> */ wire_of_i64__static_method__Series(
          String name, Object /* BigInt64Array */ ? values) =>
      wasmModule.wire_of_i64__static_method__Series(name, values);

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

  void wire_as_durations__method__Series(
          NativePortType port_, List<dynamic> that) =>
      wasmModule.wire_as_durations__method__Series(port_, that);

  void wire_as_naive_datetime__method__Series(
          NativePortType port_, List<dynamic> that) =>
      wasmModule.wire_as_naive_datetime__method__Series(port_, that);

  void wire_as_utc_datetime__method__Series(
          NativePortType port_, List<dynamic> that) =>
      wasmModule.wire_as_utc_datetime__method__Series(port_, that);

  void wire_as_local_datetime__method__Series(
          NativePortType port_, List<dynamic> that) =>
      wasmModule.wire_as_local_datetime__method__Series(port_, that);

  void wire_abs__method__Series(NativePortType port_, List<dynamic> that) =>
      wasmModule.wire_abs__method__Series(port_, that);

  void wire_sort__method__Series(
          NativePortType port_, List<dynamic> that, bool reverse) =>
      wasmModule.wire_sort__method__Series(port_, that, reverse);

  void wire_shuffle__method__Series(
          NativePortType port_, List<dynamic> that, Object? seed) =>
      wasmModule.wire_shuffle__method__Series(port_, that, seed);

  void wire_sum__method__Series(NativePortType port_, List<dynamic> that) =>
      wasmModule.wire_sum__method__Series(port_, that);

  void wire_sum_as_series__method__Series(
          NativePortType port_, List<dynamic> that) =>
      wasmModule.wire_sum_as_series__method__Series(port_, that);

  void wire_min__method__Series(NativePortType port_, List<dynamic> that) =>
      wasmModule.wire_min__method__Series(port_, that);

  void wire_max__method__Series(NativePortType port_, List<dynamic> that) =>
      wasmModule.wire_max__method__Series(port_, that);

  void wire_explode__method__Series(NativePortType port_, List<dynamic> that) =>
      wasmModule.wire_explode__method__Series(port_, that);

  void wire_explode_by_offsets__method__Series(NativePortType port_,
          List<dynamic> that, Object /* BigInt64Array */ offsets) =>
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

  dynamic /* double? */ wire_get__method__Series(
          List<dynamic> that, int index) =>
      wasmModule.wire_get__method__Series(that, index);

  dynamic /* List<dynamic> */ wire_head__method__Series(
          List<dynamic> that, int? length) =>
      wasmModule.wire_head__method__Series(that, length);

  dynamic /* List<dynamic> */ wire_tail__method__Series(
          List<dynamic> that, int? length) =>
      wasmModule.wire_tail__method__Series(that, length);

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

  dynamic /* bool */ wire_is_bool__method__Series(List<dynamic> that) =>
      wasmModule.wire_is_bool__method__Series(that);

  dynamic /* bool */ wire_is_utf8__method__Series(List<dynamic> that) =>
      wasmModule.wire_is_utf8__method__Series(that);

  dynamic /* bool */ wire_is_numeric__method__Series(List<dynamic> that) =>
      wasmModule.wire_is_numeric__method__Series(that);

  dynamic /* bool */ wire_is_temporal__method__Series(List<dynamic> that) =>
      wasmModule.wire_is_temporal__method__Series(that);

  void wire_dump__method__Series(NativePortType port_, List<dynamic> that) =>
      wasmModule.wire_dump__method__Series(port_, that);

  dynamic /* void */ wire_rename__method__Series(
          List<dynamic> that, String name) =>
      wasmModule.wire_rename__method__Series(that, name);

  void wire_unique__method__Series(
          NativePortType port_, List<dynamic> that, bool stable) =>
      wasmModule.wire_unique__method__Series(port_, that, stable);

  void wire_equal__method__Series(NativePortType port_, List<dynamic> that,
          List<dynamic> other, bool ignore_null) =>
      wasmModule.wire_equal__method__Series(port_, that, other, ignore_null);

  void wire_reshape__method__Series(NativePortType port_, List<dynamic> that,
          Object /* BigInt64Array */ dims) =>
      wasmModule.wire_reshape__method__Series(port_, that, dims);

  void wire_std_as_series__method__Series(
          NativePortType port_, List<dynamic> that, int ddof) =>
      wasmModule.wire_std_as_series__method__Series(port_, that, ddof);

  dynamic /*  */ drop_opaque_RwLockPDataFrame(ptr) =>
      wasmModule.drop_opaque_RwLockPDataFrame(ptr);

  int /* *const c_void */ share_opaque_RwLockPDataFrame(ptr) =>
      wasmModule.share_opaque_RwLockPDataFrame(ptr);

  dynamic /*  */ drop_opaque_RwLockPLazyFrame(ptr) =>
      wasmModule.drop_opaque_RwLockPLazyFrame(ptr);

  int /* *const c_void */ share_opaque_RwLockPLazyFrame(ptr) =>
      wasmModule.share_opaque_RwLockPLazyFrame(ptr);

  dynamic /*  */ drop_opaque_RwLockPSeries(ptr) =>
      wasmModule.drop_opaque_RwLockPSeries(ptr);

  int /* *const c_void */ share_opaque_RwLockPSeries(ptr) =>
      wasmModule.share_opaque_RwLockPSeries(ptr);
}
