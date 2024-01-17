#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
// EXTRA BEGIN
typedef struct DartCObject *WireSyncRust2DartDco;
typedef struct WireSyncRust2DartSse {
  uint8_t *ptr;
  int32_t len;
} WireSyncRust2DartSse;
// EXTRA END
typedef struct _Dart_Handle* Dart_Handle;

typedef struct wire_cst_list_prim_u_8_strict {
  uint8_t *ptr;
  int32_t len;
} wire_cst_list_prim_u_8_strict;

typedef struct wire_cst_list_String {
  struct wire_cst_list_prim_u_8_strict **ptr;
  int32_t len;
} wire_cst_list_String;

typedef struct wire_cst_list_prim_f_64_strict {
  double *ptr;
  int32_t len;
} wire_cst_list_prim_f_64_strict;

typedef struct wire_cst_list_prim_i_64_strict {
  int64_t *ptr;
  int32_t len;
} wire_cst_list_prim_i_64_strict;

typedef struct wire_cst_Literals_Int64 {
  struct wire_cst_list_prim_i_64_strict *field0;
} wire_cst_Literals_Int64;

typedef struct wire_cst_list_opt_box_autoadd_i_64 {
  int64_t **ptr;
  int32_t len;
} wire_cst_list_opt_box_autoadd_i_64;

typedef struct wire_cst_Literals_NullInt64 {
  struct wire_cst_list_opt_box_autoadd_i_64 *field0;
} wire_cst_Literals_NullInt64;

typedef struct wire_cst_Literals_Float64 {
  struct wire_cst_list_prim_f_64_strict *field0;
} wire_cst_Literals_Float64;

typedef struct wire_cst_list_opt_box_autoadd_f_64 {
  double **ptr;
  int32_t len;
} wire_cst_list_opt_box_autoadd_f_64;

typedef struct wire_cst_Literals_NullFloat64 {
  struct wire_cst_list_opt_box_autoadd_f_64 *field0;
} wire_cst_Literals_NullFloat64;

typedef struct wire_cst_list_bool {
  bool *ptr;
  int32_t len;
} wire_cst_list_bool;

typedef struct wire_cst_Literals_Boolean {
  struct wire_cst_list_bool *field0;
} wire_cst_Literals_Boolean;

typedef struct wire_cst_list_Chrono_Duration {
  int64_t *ptr;
  int32_t len;
} wire_cst_list_Chrono_Duration;

typedef struct wire_cst_Literals_Duration {
  struct wire_cst_list_Chrono_Duration *field0;
} wire_cst_Literals_Duration;

typedef struct wire_cst_list_opt_box_autoadd_Chrono_Duration {
  int64_t **ptr;
  int32_t len;
} wire_cst_list_opt_box_autoadd_Chrono_Duration;

typedef struct wire_cst_Literals_NullDuration {
  struct wire_cst_list_opt_box_autoadd_Chrono_Duration *field0;
} wire_cst_Literals_NullDuration;

typedef struct wire_cst_DataType_Datetime {
  int32_t field0;
  struct wire_cst_list_prim_u_8_strict *field1;
} wire_cst_DataType_Datetime;

typedef struct wire_cst_DataType_Duration {
  int32_t field0;
} wire_cst_DataType_Duration;

typedef struct wire_cst_DataType_List {
  struct wire_cst_data_type *field0;
} wire_cst_DataType_List;

typedef struct wire_cst_field {
  struct wire_cst_list_prim_u_8_strict *name;
  struct wire_cst_data_type *dtype;
} wire_cst_field;

typedef struct wire_cst_list_field {
  struct wire_cst_field *ptr;
  int32_t len;
} wire_cst_list_field;

typedef struct wire_cst_DataType_Struct {
  struct wire_cst_list_field *field0;
} wire_cst_DataType_Struct;

typedef union DataTypeKind {
  struct wire_cst_DataType_Datetime Datetime;
  struct wire_cst_DataType_Duration Duration;
  struct wire_cst_DataType_List List;
  struct wire_cst_DataType_Struct Struct;
} DataTypeKind;

typedef struct wire_cst_data_type {
  int32_t tag;
  union DataTypeKind kind;
} wire_cst_data_type;

typedef struct wire_cst_Literals_StringLike {
  struct wire_cst_list_String *field0;
  struct wire_cst_data_type *field1;
} wire_cst_Literals_StringLike;

typedef struct wire_cst_list_opt_String {
  struct wire_cst_list_prim_u_8_strict **ptr;
  int32_t len;
} wire_cst_list_opt_String;

typedef struct wire_cst_Literals_NullStringLike {
  struct wire_cst_list_opt_String *field0;
  struct wire_cst_data_type *field1;
} wire_cst_Literals_NullStringLike;

typedef struct wire_cst_Literals_Series {
  uintptr_t field0;
} wire_cst_Literals_Series;

typedef union LiteralsKind {
  struct wire_cst_Literals_Int64 Int64;
  struct wire_cst_Literals_NullInt64 NullInt64;
  struct wire_cst_Literals_Float64 Float64;
  struct wire_cst_Literals_NullFloat64 NullFloat64;
  struct wire_cst_Literals_Boolean Boolean;
  struct wire_cst_Literals_Duration Duration;
  struct wire_cst_Literals_NullDuration NullDuration;
  struct wire_cst_Literals_StringLike StringLike;
  struct wire_cst_Literals_NullStringLike NullStringLike;
  struct wire_cst_Literals_Series Series;
} LiteralsKind;

typedef struct wire_cst_literals {
  int32_t tag;
  union LiteralsKind kind;
} wire_cst_literals;

typedef struct wire_cst_record_string_literals {
  struct wire_cst_list_prim_u_8_strict *field0;
  struct wire_cst_literals field1;
} wire_cst_record_string_literals;

typedef struct wire_cst_list_record_string_literals {
  struct wire_cst_record_string_literals *ptr;
  int32_t len;
} wire_cst_list_record_string_literals;

typedef struct wire_cst_Expr_Alias {
  struct wire_cst_expr *field0;
  struct wire_cst_list_prim_u_8_strict *field1;
} wire_cst_Expr_Alias;

typedef struct wire_cst_Expr_Column {
  struct wire_cst_list_prim_u_8_strict *field0;
} wire_cst_Expr_Column;

typedef struct wire_cst_Expr_Columns {
  struct wire_cst_list_String *field0;
} wire_cst_Expr_Columns;

typedef struct wire_cst_list_data_type {
  struct wire_cst_data_type *ptr;
  int32_t len;
} wire_cst_list_data_type;

typedef struct wire_cst_Expr_DtypeColumn {
  struct wire_cst_list_data_type *field0;
} wire_cst_Expr_DtypeColumn;

typedef struct wire_cst_LiteralValue_Boolean {
  bool field0;
} wire_cst_LiteralValue_Boolean;

typedef struct wire_cst_LiteralValue_Utf8 {
  struct wire_cst_list_prim_u_8_strict *field0;
} wire_cst_LiteralValue_Utf8;

typedef struct wire_cst_LiteralValue_Binary {
  struct wire_cst_list_prim_u_8_strict *field0;
} wire_cst_LiteralValue_Binary;

typedef struct wire_cst_LiteralValue_Uint32 {
  uint32_t field0;
} wire_cst_LiteralValue_Uint32;

typedef struct wire_cst_LiteralValue_Uint64 {
  uint64_t field0;
} wire_cst_LiteralValue_Uint64;

typedef struct wire_cst_LiteralValue_Int32 {
  int32_t field0;
} wire_cst_LiteralValue_Int32;

typedef struct wire_cst_LiteralValue_Int64 {
  int64_t field0;
} wire_cst_LiteralValue_Int64;

typedef struct wire_cst_LiteralValue_Float32 {
  float field0;
} wire_cst_LiteralValue_Float32;

typedef struct wire_cst_LiteralValue_Float64 {
  double field0;
} wire_cst_LiteralValue_Float64;

typedef struct wire_cst_LiteralValue_Range {
  int64_t low;
  int64_t high;
  struct wire_cst_data_type *data_type;
} wire_cst_LiteralValue_Range;

typedef struct wire_cst_LiteralValue_DateTime {
  int64_t field0;
  int32_t field1;
  struct wire_cst_list_prim_u_8_strict *field2;
} wire_cst_LiteralValue_DateTime;

typedef struct wire_cst_LiteralValue_Duration {
  int64_t field0;
  int32_t field1;
} wire_cst_LiteralValue_Duration;

typedef struct wire_cst_LiteralValue_Series {
  uintptr_t field0;
} wire_cst_LiteralValue_Series;

typedef struct wire_cst_LiteralValue_Date {
  int32_t field0;
} wire_cst_LiteralValue_Date;

typedef struct wire_cst_LiteralValue_Time {
  int64_t field0;
} wire_cst_LiteralValue_Time;

typedef union LiteralValueKind {
  struct wire_cst_LiteralValue_Boolean Boolean;
  struct wire_cst_LiteralValue_Utf8 Utf8;
  struct wire_cst_LiteralValue_Binary Binary;
  struct wire_cst_LiteralValue_Uint32 Uint32;
  struct wire_cst_LiteralValue_Uint64 Uint64;
  struct wire_cst_LiteralValue_Int32 Int32;
  struct wire_cst_LiteralValue_Int64 Int64;
  struct wire_cst_LiteralValue_Float32 Float32;
  struct wire_cst_LiteralValue_Float64 Float64;
  struct wire_cst_LiteralValue_Range Range;
  struct wire_cst_LiteralValue_DateTime DateTime;
  struct wire_cst_LiteralValue_Duration Duration;
  struct wire_cst_LiteralValue_Series Series;
  struct wire_cst_LiteralValue_Date Date;
  struct wire_cst_LiteralValue_Time Time;
} LiteralValueKind;

typedef struct wire_cst_literal_value {
  int32_t tag;
  union LiteralValueKind kind;
} wire_cst_literal_value;

typedef struct wire_cst_Expr_Literal {
  struct wire_cst_literal_value *field0;
} wire_cst_Expr_Literal;

typedef struct wire_cst_Expr_BinaryExpr {
  struct wire_cst_expr *left;
  int32_t op;
  struct wire_cst_expr *right;
} wire_cst_Expr_BinaryExpr;

typedef struct wire_cst_Expr_Cast {
  struct wire_cst_expr *expr;
  struct wire_cst_data_type *data_type;
  bool strict;
} wire_cst_Expr_Cast;

typedef struct wire_cst_sort_options {
  bool descending;
  bool nulls_last;
  bool multithreaded;
  bool maintain_order;
} wire_cst_sort_options;

typedef struct wire_cst_Expr_Sort {
  struct wire_cst_expr *expr;
  struct wire_cst_sort_options *options;
} wire_cst_Expr_Sort;

typedef struct wire_cst_Expr_Gather {
  struct wire_cst_expr *expr;
  struct wire_cst_expr *idx;
  bool returns_scalar;
} wire_cst_Expr_Gather;

typedef struct wire_cst_Expr_SortBy {
  struct wire_cst_expr *expr;
  struct wire_cst_list_expr *by;
  struct wire_cst_list_bool *descending;
} wire_cst_Expr_SortBy;

typedef struct wire_cst_AggExpr_Min {
  struct wire_cst_expr *input;
  bool propagate_nans;
} wire_cst_AggExpr_Min;

typedef struct wire_cst_AggExpr_Max {
  struct wire_cst_expr *input;
  bool propagate_nans;
} wire_cst_AggExpr_Max;

typedef struct wire_cst_AggExpr_Median {
  struct wire_cst_expr *field0;
} wire_cst_AggExpr_Median;

typedef struct wire_cst_AggExpr_NUnique {
  struct wire_cst_expr *field0;
} wire_cst_AggExpr_NUnique;

typedef struct wire_cst_AggExpr_First {
  struct wire_cst_expr *field0;
} wire_cst_AggExpr_First;

typedef struct wire_cst_AggExpr_Last {
  struct wire_cst_expr *field0;
} wire_cst_AggExpr_Last;

typedef struct wire_cst_AggExpr_Mean {
  struct wire_cst_expr *field0;
} wire_cst_AggExpr_Mean;

typedef struct wire_cst_AggExpr_Implode {
  struct wire_cst_expr *field0;
} wire_cst_AggExpr_Implode;

typedef struct wire_cst_AggExpr_Count {
  struct wire_cst_expr *field0;
} wire_cst_AggExpr_Count;

typedef struct wire_cst_AggExpr_Quantile {
  struct wire_cst_expr *expr;
  struct wire_cst_expr *quantile;
  int32_t interpol;
} wire_cst_AggExpr_Quantile;

typedef struct wire_cst_AggExpr_Sum {
  struct wire_cst_expr *field0;
} wire_cst_AggExpr_Sum;

typedef struct wire_cst_AggExpr_AggGroups {
  struct wire_cst_expr *field0;
} wire_cst_AggExpr_AggGroups;

typedef struct wire_cst_AggExpr_Std {
  struct wire_cst_expr *field0;
  uint8_t field1;
} wire_cst_AggExpr_Std;

typedef struct wire_cst_AggExpr_Var {
  struct wire_cst_expr *field0;
  uint8_t field1;
} wire_cst_AggExpr_Var;

typedef union AggExprKind {
  struct wire_cst_AggExpr_Min Min;
  struct wire_cst_AggExpr_Max Max;
  struct wire_cst_AggExpr_Median Median;
  struct wire_cst_AggExpr_NUnique NUnique;
  struct wire_cst_AggExpr_First First;
  struct wire_cst_AggExpr_Last Last;
  struct wire_cst_AggExpr_Mean Mean;
  struct wire_cst_AggExpr_Implode Implode;
  struct wire_cst_AggExpr_Count Count;
  struct wire_cst_AggExpr_Quantile Quantile;
  struct wire_cst_AggExpr_Sum Sum;
  struct wire_cst_AggExpr_AggGroups AggGroups;
  struct wire_cst_AggExpr_Std Std;
  struct wire_cst_AggExpr_Var Var;
} AggExprKind;

typedef struct wire_cst_agg_expr {
  int32_t tag;
  union AggExprKind kind;
} wire_cst_agg_expr;

typedef struct wire_cst_Expr_Agg {
  struct wire_cst_agg_expr *field0;
} wire_cst_Expr_Agg;

typedef struct wire_cst_Expr_Ternary {
  struct wire_cst_expr *predicate;
  struct wire_cst_expr *truthy;
  struct wire_cst_expr *falsy;
} wire_cst_Expr_Ternary;

typedef struct wire_cst_Expr_Explode {
  struct wire_cst_expr *field0;
} wire_cst_Expr_Explode;

typedef struct wire_cst_Expr_Filter {
  struct wire_cst_expr *input;
  struct wire_cst_expr *by;
} wire_cst_Expr_Filter;

typedef struct wire_cst_WindowType_Over {
  int32_t field0;
} wire_cst_WindowType_Over;

typedef union WindowTypeKind {
  struct wire_cst_WindowType_Over Over;
} WindowTypeKind;

typedef struct wire_cst_window_type {
  int32_t tag;
  union WindowTypeKind kind;
} wire_cst_window_type;

typedef struct wire_cst_Expr_Window {
  struct wire_cst_expr *function;
  struct wire_cst_list_expr *partition_by;
  struct wire_cst_window_type *options;
} wire_cst_Expr_Window;

typedef struct wire_cst_Expr_Slice {
  struct wire_cst_expr *input;
  struct wire_cst_expr *offset;
  struct wire_cst_expr *length;
} wire_cst_Expr_Slice;

typedef struct wire_cst_Excluded_Name {
  struct wire_cst_list_prim_u_8_strict *field0;
} wire_cst_Excluded_Name;

typedef struct wire_cst_Excluded_Dtype {
  struct wire_cst_data_type *field0;
} wire_cst_Excluded_Dtype;

typedef union ExcludedKind {
  struct wire_cst_Excluded_Name Name;
  struct wire_cst_Excluded_Dtype Dtype;
} ExcludedKind;

typedef struct wire_cst_excluded {
  int32_t tag;
  union ExcludedKind kind;
} wire_cst_excluded;

typedef struct wire_cst_list_excluded {
  struct wire_cst_excluded *ptr;
  int32_t len;
} wire_cst_list_excluded;

typedef struct wire_cst_Expr_Exclude {
  struct wire_cst_expr *field0;
  struct wire_cst_list_excluded *field1;
} wire_cst_Expr_Exclude;

typedef struct wire_cst_Expr_KeepName {
  struct wire_cst_expr *field0;
} wire_cst_Expr_KeepName;

typedef struct wire_cst_Expr_Nth {
  int64_t field0;
} wire_cst_Expr_Nth;

typedef struct wire_cst_Expr_Internal {
  uintptr_t field0;
} wire_cst_Expr_Internal;

typedef union ExprKind {
  struct wire_cst_Expr_Alias Alias;
  struct wire_cst_Expr_Column Column;
  struct wire_cst_Expr_Columns Columns;
  struct wire_cst_Expr_DtypeColumn DtypeColumn;
  struct wire_cst_Expr_Literal Literal;
  struct wire_cst_Expr_BinaryExpr BinaryExpr;
  struct wire_cst_Expr_Cast Cast;
  struct wire_cst_Expr_Sort Sort;
  struct wire_cst_Expr_Gather Gather;
  struct wire_cst_Expr_SortBy SortBy;
  struct wire_cst_Expr_Agg Agg;
  struct wire_cst_Expr_Ternary Ternary;
  struct wire_cst_Expr_Explode Explode;
  struct wire_cst_Expr_Filter Filter;
  struct wire_cst_Expr_Window Window;
  struct wire_cst_Expr_Slice Slice;
  struct wire_cst_Expr_Exclude Exclude;
  struct wire_cst_Expr_KeepName KeepName;
  struct wire_cst_Expr_Nth Nth;
  struct wire_cst_Expr_Internal Internal;
} ExprKind;

typedef struct wire_cst_expr {
  int32_t tag;
  union ExprKind kind;
} wire_cst_expr;

typedef struct wire_cst_list_expr {
  struct wire_cst_expr *ptr;
  int32_t len;
} wire_cst_list_expr;

typedef struct wire_cst_row_count {
  struct wire_cst_list_prim_u_8_strict *name;
  uint32_t offset;
} wire_cst_row_count;

typedef struct wire_cst_NullValues_AllColumnsSingle {
  struct wire_cst_list_prim_u_8_strict *field0;
} wire_cst_NullValues_AllColumnsSingle;

typedef struct wire_cst_NullValues_AllColumns {
  struct wire_cst_list_String *field0;
} wire_cst_NullValues_AllColumns;

typedef struct wire_cst_record_string_string {
  struct wire_cst_list_prim_u_8_strict *field0;
  struct wire_cst_list_prim_u_8_strict *field1;
} wire_cst_record_string_string;

typedef struct wire_cst_list_record_string_string {
  struct wire_cst_record_string_string *ptr;
  int32_t len;
} wire_cst_list_record_string_string;

typedef struct wire_cst_NullValues_Named {
  struct wire_cst_list_record_string_string *field0;
} wire_cst_NullValues_Named;

typedef union NullValuesKind {
  struct wire_cst_NullValues_AllColumnsSingle AllColumnsSingle;
  struct wire_cst_NullValues_AllColumns AllColumns;
  struct wire_cst_NullValues_Named Named;
} NullValuesKind;

typedef struct wire_cst_null_values {
  int32_t tag;
  union NullValuesKind kind;
} wire_cst_null_values;

typedef struct wire_cst_list_prim_u_32_strict {
  uint32_t *ptr;
  int32_t len;
} wire_cst_list_prim_u_32_strict;

void frbgen_polars_dart_fn_deliver_output(int32_t call_id,
                                          uint8_t *ptr_,
                                          int32_t rust_vec_len_,
                                          int32_t data_len_);

WireSyncRust2DartDco frbgen_polars_wire_DataFrame_clone(uintptr_t that);

WireSyncRust2DartDco frbgen_polars_wire_DataFrame_column(uintptr_t that,
                                                         struct wire_cst_list_prim_u_8_strict *column);

WireSyncRust2DartDco frbgen_polars_wire_DataFrame_column_at(uintptr_t that, uintptr_t index);

WireSyncRust2DartDco frbgen_polars_wire_DataFrame_column_names(uintptr_t that);

WireSyncRust2DartDco frbgen_polars_wire_DataFrame_columns(uintptr_t that,
                                                          struct wire_cst_list_String *columns);

void frbgen_polars_wire_DataFrame_describe(int64_t port_,
                                           uintptr_t that,
                                           struct wire_cst_list_prim_f_64_strict *percentiles);

WireSyncRust2DartDco frbgen_polars_wire_DataFrame_drop(uintptr_t that,
                                                       struct wire_cst_list_prim_u_8_strict *column);

WireSyncRust2DartDco frbgen_polars_wire_DataFrame_drop_in_place(uintptr_t that,
                                                                struct wire_cst_list_prim_u_8_strict *column);

WireSyncRust2DartDco frbgen_polars_wire_DataFrame_dtypes(uintptr_t that);

WireSyncRust2DartDco frbgen_polars_wire_DataFrame_dump(uintptr_t that);

WireSyncRust2DartDco frbgen_polars_wire_DataFrame_estimated_size(uintptr_t that);

WireSyncRust2DartDco frbgen_polars_wire_DataFrame_get_columns(uintptr_t that);

WireSyncRust2DartDco frbgen_polars_wire_DataFrame_get_row(uintptr_t that, uintptr_t index);

WireSyncRust2DartDco frbgen_polars_wire_DataFrame_head(uintptr_t that, uintptr_t *length);

WireSyncRust2DartDco frbgen_polars_wire_DataFrame_height(uintptr_t that);

WireSyncRust2DartDco frbgen_polars_wire_DataFrame_is_empty(uintptr_t that);

void frbgen_polars_wire_DataFrame_iter(int64_t port_, uintptr_t that);

WireSyncRust2DartDco frbgen_polars_wire_DataFrame_lazy(uintptr_t that,
                                                       bool *projection_pushdown,
                                                       bool *predicate_pushdown,
                                                       bool *type_coercion,
                                                       bool *simplify_expressions,
                                                       bool *slice_pushdown,
                                                       bool *streaming);

WireSyncRust2DartDco frbgen_polars_wire_DataFrame_max(uintptr_t that);

WireSyncRust2DartDco frbgen_polars_wire_DataFrame_of_lits(struct wire_cst_list_record_string_literals *series);

WireSyncRust2DartDco frbgen_polars_wire_DataFrame_reverse(uintptr_t that);

WireSyncRust2DartDco frbgen_polars_wire_DataFrame_sample(uintptr_t that,
                                                         uintptr_t n,
                                                         bool with_replacement,
                                                         bool shuffle,
                                                         uint64_t *seed);

WireSyncRust2DartDco frbgen_polars_wire_DataFrame_schema(uintptr_t that);

WireSyncRust2DartDco frbgen_polars_wire_DataFrame_select(uintptr_t that,
                                                         struct wire_cst_list_String *columns);

WireSyncRust2DartDco frbgen_polars_wire_DataFrame_shape(uintptr_t that);

WireSyncRust2DartDco frbgen_polars_wire_DataFrame_sort_in_place(uintptr_t that,
                                                                struct wire_cst_list_String *by_column,
                                                                struct wire_cst_list_bool *descending,
                                                                bool maintain_order);

WireSyncRust2DartDco frbgen_polars_wire_DataFrame_tail(uintptr_t that, uintptr_t *length);

WireSyncRust2DartDco frbgen_polars_wire_DataFrame_width(uintptr_t that);

WireSyncRust2DartDco frbgen_polars_wire_DataFrame_with_row_count(uintptr_t that,
                                                                 struct wire_cst_list_prim_u_8_strict *name,
                                                                 uint32_t *offset);

void frbgen_polars_wire_DataFrame_write_csv(int64_t port_,
                                            uintptr_t that,
                                            struct wire_cst_list_prim_u_8_strict *path,
                                            bool include_bom,
                                            bool include_header,
                                            bool append,
                                            bool create_new,
                                            struct wire_cst_list_prim_u_8_strict *null_value);

WireSyncRust2DartDco frbgen_polars_wire_LazyFrame_cache(uintptr_t that);

void frbgen_polars_wire_LazyFrame_collect(int64_t port_, uintptr_t that, bool streaming);

WireSyncRust2DartDco frbgen_polars_wire_LazyFrame_cross_join(uintptr_t that, uintptr_t other);

WireSyncRust2DartDco frbgen_polars_wire_LazyFrame_drop_nulls(uintptr_t that,
                                                             struct wire_cst_list_expr *subset);

WireSyncRust2DartDco frbgen_polars_wire_LazyFrame_explode(uintptr_t that,
                                                          struct wire_cst_list_expr *columns);

void frbgen_polars_wire_LazyFrame_fetch(int64_t port_, uintptr_t that, uintptr_t n_rows);

WireSyncRust2DartDco frbgen_polars_wire_LazyFrame_filter(uintptr_t that,
                                                         struct wire_cst_expr *pred);

WireSyncRust2DartDco frbgen_polars_wire_LazyFrame_first(uintptr_t that);

WireSyncRust2DartDco frbgen_polars_wire_LazyFrame_group_by(uintptr_t that,
                                                           struct wire_cst_list_expr *exprs,
                                                           bool maintain_order);

WireSyncRust2DartDco frbgen_polars_wire_LazyFrame_inner_join(uintptr_t that,
                                                             uintptr_t other,
                                                             struct wire_cst_expr *left_on,
                                                             struct wire_cst_expr *right_on);

WireSyncRust2DartDco frbgen_polars_wire_LazyFrame_join(uintptr_t that,
                                                       uintptr_t other,
                                                       struct wire_cst_list_expr *on,
                                                       struct wire_cst_list_expr *left_on,
                                                       struct wire_cst_list_expr *right_on,
                                                       struct wire_cst_list_prim_u_8_strict *suffix,
                                                       int32_t how,
                                                       bool allow_parallel,
                                                       bool force_parallel);

WireSyncRust2DartDco frbgen_polars_wire_LazyFrame_last(uintptr_t that);

WireSyncRust2DartDco frbgen_polars_wire_LazyFrame_left_join(uintptr_t that,
                                                            uintptr_t other,
                                                            struct wire_cst_expr *left_on,
                                                            struct wire_cst_expr *right_on);

WireSyncRust2DartDco frbgen_polars_wire_LazyFrame_limit(uintptr_t that, uint32_t n);

WireSyncRust2DartDco frbgen_polars_wire_LazyFrame_max(uintptr_t that);

WireSyncRust2DartDco frbgen_polars_wire_LazyFrame_mean(uintptr_t that);

WireSyncRust2DartDco frbgen_polars_wire_LazyFrame_median(uintptr_t that);

WireSyncRust2DartDco frbgen_polars_wire_LazyFrame_melt(uintptr_t that,
                                                       struct wire_cst_list_String *id_vars,
                                                       struct wire_cst_list_String *value_vars,
                                                       struct wire_cst_list_prim_u_8_strict *variable_name,
                                                       struct wire_cst_list_prim_u_8_strict *value_name,
                                                       bool streamable);

WireSyncRust2DartDco frbgen_polars_wire_LazyFrame_min(uintptr_t that);

WireSyncRust2DartDco frbgen_polars_wire_LazyFrame_null_count(uintptr_t that);

WireSyncRust2DartDco frbgen_polars_wire_LazyFrame_outer_join(uintptr_t that,
                                                             uintptr_t other,
                                                             struct wire_cst_expr *left_on,
                                                             struct wire_cst_expr *right_on);

WireSyncRust2DartDco frbgen_polars_wire_LazyFrame_quantile(uintptr_t that,
                                                           struct wire_cst_expr *quantile,
                                                           int32_t interpol);

WireSyncRust2DartDco frbgen_polars_wire_LazyFrame_reverse(uintptr_t that);

WireSyncRust2DartDco frbgen_polars_wire_LazyFrame_select(uintptr_t that,
                                                         struct wire_cst_list_expr *exprs);

WireSyncRust2DartDco frbgen_polars_wire_LazyFrame_slice(uintptr_t that,
                                                        int64_t offset,
                                                        uint32_t len);

WireSyncRust2DartDco frbgen_polars_wire_LazyFrame_sort(uintptr_t that,
                                                       struct wire_cst_list_prim_u_8_strict *by_column,
                                                       bool descending,
                                                       bool nulls_last,
                                                       bool multithreaded,
                                                       bool maintain_order);

WireSyncRust2DartDco frbgen_polars_wire_LazyFrame_std(uintptr_t that, uint8_t ddof);

WireSyncRust2DartDco frbgen_polars_wire_LazyFrame_sum(uintptr_t that);

WireSyncRust2DartDco frbgen_polars_wire_LazyFrame_tail(uintptr_t that, uint32_t n);

WireSyncRust2DartDco frbgen_polars_wire_LazyFrame_unique(uintptr_t that,
                                                         struct wire_cst_list_String *subset,
                                                         int32_t keep_strategy);

WireSyncRust2DartDco frbgen_polars_wire_LazyFrame_variance(uintptr_t that, uint8_t ddof);

WireSyncRust2DartDco frbgen_polars_wire_LazyFrame_with_column(uintptr_t that,
                                                              struct wire_cst_expr *expr);

WireSyncRust2DartDco frbgen_polars_wire_LazyFrame_with_columns(uintptr_t that,
                                                               struct wire_cst_list_expr *exprs);

WireSyncRust2DartDco frbgen_polars_wire_LazyFrame_with_row_count(uintptr_t that,
                                                                 struct wire_cst_list_prim_u_8_strict *name,
                                                                 uint32_t *offset);

void frbgen_polars_wire_read_csv(int64_t port_,
                                 struct wire_cst_list_prim_u_8_strict *path,
                                 struct wire_cst_list_data_type *dtypes_slice,
                                 bool *has_header,
                                 struct wire_cst_list_String *columns,
                                 struct wire_cst_list_prim_u_8_strict *comment_char,
                                 struct wire_cst_list_prim_u_8_strict *eol_char,
                                 uintptr_t *chunk_size,
                                 uintptr_t *sample_size,
                                 struct wire_cst_row_count *row_count,
                                 int32_t *encoding,
                                 uintptr_t *n_rows,
                                 uintptr_t *n_threads,
                                 struct wire_cst_null_values *null_values,
                                 struct wire_cst_list_prim_u_32_strict *projection,
                                 struct wire_cst_list_prim_u_8_strict *quote_char,
                                 uintptr_t skip_rows,
                                 uintptr_t skip_rows_after_header,
                                 bool ignore_errors,
                                 bool rechunk,
                                 bool try_parse_dates,
                                 bool low_memory);

void frbgen_polars_wire_read_json(int64_t port_,
                                  struct wire_cst_list_prim_u_8_strict *path,
                                  uintptr_t *batch_size,
                                  struct wire_cst_list_String *projection);

void frbgen_polars_wire_scan_csv(int64_t port_,
                                 struct wire_cst_list_prim_u_8_strict *path,
                                 uintptr_t dtype_overwrite,
                                 bool *has_header,
                                 struct wire_cst_list_prim_u_8_strict *comment_char,
                                 struct wire_cst_list_prim_u_8_strict *eol_char,
                                 struct wire_cst_list_prim_u_8_strict *quote_char,
                                 uintptr_t skip_rows,
                                 uintptr_t skip_rows_after_header,
                                 struct wire_cst_row_count *row_count,
                                 int32_t *encoding,
                                 uintptr_t *n_rows,
                                 struct wire_cst_null_values *null_values,
                                 bool ignore_errors,
                                 bool rechunk,
                                 bool try_parse_dates,
                                 uintptr_t *infer_schema_length,
                                 bool cache);

WireSyncRust2DartDco frbgen_polars_wire_Expr_abs(struct wire_cst_expr *that);

WireSyncRust2DartDco frbgen_polars_wire_Expr_all(struct wire_cst_expr *that, bool ignore_nulls);

WireSyncRust2DartDco frbgen_polars_wire_Expr_any(struct wire_cst_expr *that, bool ignore_nulls);

WireSyncRust2DartDco frbgen_polars_wire_Expr_append(struct wire_cst_expr *that,
                                                    struct wire_cst_expr *other,
                                                    bool upcast);

WireSyncRust2DartDco frbgen_polars_wire_Expr_arccos(struct wire_cst_expr *that);

WireSyncRust2DartDco frbgen_polars_wire_Expr_arccosh(struct wire_cst_expr *that);

WireSyncRust2DartDco frbgen_polars_wire_Expr_arcsin(struct wire_cst_expr *that);

WireSyncRust2DartDco frbgen_polars_wire_Expr_arcsinh(struct wire_cst_expr *that);

WireSyncRust2DartDco frbgen_polars_wire_Expr_arctan(struct wire_cst_expr *that);

WireSyncRust2DartDco frbgen_polars_wire_Expr_arctan2(struct wire_cst_expr *that,
                                                     struct wire_cst_expr *x);

WireSyncRust2DartDco frbgen_polars_wire_Expr_arctanh(struct wire_cst_expr *that);

WireSyncRust2DartDco frbgen_polars_wire_Expr_arg_max(struct wire_cst_expr *that);

WireSyncRust2DartDco frbgen_polars_wire_Expr_arg_min(struct wire_cst_expr *that);

WireSyncRust2DartDco frbgen_polars_wire_Expr_arg_sort(struct wire_cst_expr *that,
                                                      bool descending,
                                                      bool nulls_last,
                                                      bool multithreaded,
                                                      bool maintain_order);

WireSyncRust2DartDco frbgen_polars_wire_Expr_arg_unique(struct wire_cst_expr *that);

WireSyncRust2DartDco frbgen_polars_wire_Expr_backward_fill(struct wire_cst_expr *that,
                                                           uint32_t *limit);

WireSyncRust2DartDco frbgen_polars_wire_Expr_cbrt(struct wire_cst_expr *that);

WireSyncRust2DartDco frbgen_polars_wire_Expr_ceil(struct wire_cst_expr *that);

WireSyncRust2DartDco frbgen_polars_wire_Expr_clip(struct wire_cst_expr *that,
                                                  struct wire_cst_expr *min,
                                                  struct wire_cst_expr *max);

WireSyncRust2DartDco frbgen_polars_wire_Expr_clip_max(struct wire_cst_expr *that,
                                                      struct wire_cst_expr *max);

WireSyncRust2DartDco frbgen_polars_wire_Expr_clip_min(struct wire_cst_expr *that,
                                                      struct wire_cst_expr *min);

WireSyncRust2DartDco frbgen_polars_wire_Expr_cos(struct wire_cst_expr *that);

WireSyncRust2DartDco frbgen_polars_wire_Expr_cosh(struct wire_cst_expr *that);

WireSyncRust2DartDco frbgen_polars_wire_Expr_cot(struct wire_cst_expr *that);

WireSyncRust2DartDco frbgen_polars_wire_Expr_count(struct wire_cst_expr *that);

WireSyncRust2DartDco frbgen_polars_wire_Expr_cum_count(struct wire_cst_expr *that, bool reverse);

WireSyncRust2DartDco frbgen_polars_wire_Expr_cum_max(struct wire_cst_expr *that, bool reverse);

WireSyncRust2DartDco frbgen_polars_wire_Expr_cum_min(struct wire_cst_expr *that, bool reverse);

WireSyncRust2DartDco frbgen_polars_wire_Expr_cum_prod(struct wire_cst_expr *that, bool reverse);

WireSyncRust2DartDco frbgen_polars_wire_Expr_cum_sum(struct wire_cst_expr *that, bool reverse);

WireSyncRust2DartDco frbgen_polars_wire_Expr_degrees(struct wire_cst_expr *that);

WireSyncRust2DartDco frbgen_polars_wire_Expr_div(struct wire_cst_expr *that,
                                                 struct wire_cst_expr *other);

WireSyncRust2DartDco frbgen_polars_wire_Expr_dot(struct wire_cst_expr *that,
                                                 struct wire_cst_expr *other);

WireSyncRust2DartDco frbgen_polars_wire_Expr_drop_nans(struct wire_cst_expr *that);

WireSyncRust2DartDco frbgen_polars_wire_Expr_drop_nulls(struct wire_cst_expr *that);

WireSyncRust2DartDco frbgen_polars_wire_Expr_entropy(struct wire_cst_expr *that,
                                                     double base,
                                                     bool normalize);

WireSyncRust2DartDco frbgen_polars_wire_Expr_exp(struct wire_cst_expr *that);

WireSyncRust2DartDco frbgen_polars_wire_Expr_fill_nan(struct wire_cst_expr *that,
                                                      struct wire_cst_expr *value);

WireSyncRust2DartDco frbgen_polars_wire_Expr_fill_null(struct wire_cst_expr *that,
                                                       struct wire_cst_expr *value);

WireSyncRust2DartDco frbgen_polars_wire_Expr_floor(struct wire_cst_expr *that);

WireSyncRust2DartDco frbgen_polars_wire_Expr_forward_fill(struct wire_cst_expr *that,
                                                          uint32_t *limit);

WireSyncRust2DartDco frbgen_polars_wire_Expr_is_finite(struct wire_cst_expr *that);

WireSyncRust2DartDco frbgen_polars_wire_Expr_is_in(struct wire_cst_expr *that,
                                                   struct wire_cst_expr *other);

WireSyncRust2DartDco frbgen_polars_wire_Expr_is_nan(struct wire_cst_expr *that);

WireSyncRust2DartDco frbgen_polars_wire_Expr_is_not_nan(struct wire_cst_expr *that);

WireSyncRust2DartDco frbgen_polars_wire_Expr_is_not_null(struct wire_cst_expr *that);

WireSyncRust2DartDco frbgen_polars_wire_Expr_is_null(struct wire_cst_expr *that);

WireSyncRust2DartDco frbgen_polars_wire_Expr_log(struct wire_cst_expr *that, double base);

WireSyncRust2DartDco frbgen_polars_wire_Expr_log1p(struct wire_cst_expr *that);

WireSyncRust2DartDco frbgen_polars_wire_Expr_lower_bound(struct wire_cst_expr *that);

WireSyncRust2DartDco frbgen_polars_wire_Expr_not(struct wire_cst_expr *that);

WireSyncRust2DartDco frbgen_polars_wire_Expr_null_count(struct wire_cst_expr *that);

WireSyncRust2DartDco frbgen_polars_wire_Expr_pow(struct wire_cst_expr *that, double exponent);

WireSyncRust2DartDco frbgen_polars_wire_Expr_product(struct wire_cst_expr *that);

WireSyncRust2DartDco frbgen_polars_wire_Expr_radians(struct wire_cst_expr *that);

WireSyncRust2DartDco frbgen_polars_wire_Expr_reshape(struct wire_cst_expr *that,
                                                     struct wire_cst_list_prim_i_64_strict *dims);

WireSyncRust2DartDco frbgen_polars_wire_Expr_reverse(struct wire_cst_expr *that);

WireSyncRust2DartDco frbgen_polars_wire_Expr_rolling_max(struct wire_cst_expr *that,
                                                         int64_t *window_size,
                                                         uintptr_t min_periods,
                                                         struct wire_cst_list_prim_f_64_strict *weights,
                                                         bool center,
                                                         struct wire_cst_list_prim_u_8_strict *by,
                                                         int32_t *closed_window);

WireSyncRust2DartDco frbgen_polars_wire_Expr_rolling_mean(struct wire_cst_expr *that,
                                                          int64_t *window_size,
                                                          uintptr_t min_periods,
                                                          struct wire_cst_list_prim_f_64_strict *weights,
                                                          bool center,
                                                          struct wire_cst_list_prim_u_8_strict *by,
                                                          int32_t *closed_window);

WireSyncRust2DartDco frbgen_polars_wire_Expr_rolling_median(struct wire_cst_expr *that,
                                                            int64_t *window_size,
                                                            uintptr_t min_periods,
                                                            struct wire_cst_list_prim_f_64_strict *weights,
                                                            bool center,
                                                            struct wire_cst_list_prim_u_8_strict *by,
                                                            int32_t *closed_window);

WireSyncRust2DartDco frbgen_polars_wire_Expr_rolling_min(struct wire_cst_expr *that,
                                                         int64_t *window_size,
                                                         uintptr_t min_periods,
                                                         struct wire_cst_list_prim_f_64_strict *weights,
                                                         bool center,
                                                         struct wire_cst_list_prim_u_8_strict *by,
                                                         int32_t *closed_window);

WireSyncRust2DartDco frbgen_polars_wire_Expr_rolling_quantile(struct wire_cst_expr *that,
                                                              int64_t *window_size,
                                                              uintptr_t min_periods,
                                                              struct wire_cst_list_prim_f_64_strict *weights,
                                                              bool center,
                                                              struct wire_cst_list_prim_u_8_strict *by,
                                                              int32_t *closed_window);

WireSyncRust2DartDco frbgen_polars_wire_Expr_rolling_std(struct wire_cst_expr *that,
                                                         int64_t *window_size,
                                                         uintptr_t min_periods,
                                                         struct wire_cst_list_prim_f_64_strict *weights,
                                                         bool center,
                                                         struct wire_cst_list_prim_u_8_strict *by,
                                                         int32_t *closed_window);

WireSyncRust2DartDco frbgen_polars_wire_Expr_rolling_sum(struct wire_cst_expr *that,
                                                         int64_t *window_size,
                                                         uintptr_t min_periods,
                                                         struct wire_cst_list_prim_f_64_strict *weights,
                                                         bool center,
                                                         struct wire_cst_list_prim_u_8_strict *by,
                                                         int32_t *closed_window);

WireSyncRust2DartDco frbgen_polars_wire_Expr_rolling_var(struct wire_cst_expr *that,
                                                         int64_t *window_size,
                                                         uintptr_t min_periods,
                                                         struct wire_cst_list_prim_f_64_strict *weights,
                                                         bool center,
                                                         struct wire_cst_list_prim_u_8_strict *by,
                                                         int32_t *closed_window);

WireSyncRust2DartDco frbgen_polars_wire_Expr_round(struct wire_cst_expr *that, uint32_t decimals);

WireSyncRust2DartDco frbgen_polars_wire_Expr_round_sig_figs(struct wire_cst_expr *that,
                                                            int32_t digits);

WireSyncRust2DartDco frbgen_polars_wire_Expr_set_sorted_flag(struct wire_cst_expr *that,
                                                             int32_t sorted);

WireSyncRust2DartDco frbgen_polars_wire_Expr_shift(struct wire_cst_expr *that,
                                                   struct wire_cst_expr *n);

WireSyncRust2DartDco frbgen_polars_wire_Expr_shift_and_fill(struct wire_cst_expr *that,
                                                            struct wire_cst_expr *n,
                                                            struct wire_cst_expr *fill_value);

WireSyncRust2DartDco frbgen_polars_wire_Expr_shrink_dtype(struct wire_cst_expr *that);

WireSyncRust2DartDco frbgen_polars_wire_Expr_sin(struct wire_cst_expr *that);

WireSyncRust2DartDco frbgen_polars_wire_Expr_sinh(struct wire_cst_expr *that);

WireSyncRust2DartDco frbgen_polars_wire_Expr_sqrt(struct wire_cst_expr *that);

WireSyncRust2DartDco frbgen_polars_wire_Expr_tan(struct wire_cst_expr *that);

WireSyncRust2DartDco frbgen_polars_wire_Expr_tanh(struct wire_cst_expr *that);

WireSyncRust2DartDco frbgen_polars_wire_Expr_to_dot(struct wire_cst_expr *that);

WireSyncRust2DartDco frbgen_polars_wire_Expr_to_physical(struct wire_cst_expr *that);

WireSyncRust2DartDco frbgen_polars_wire_Expr_unique(struct wire_cst_expr *that);

WireSyncRust2DartDco frbgen_polars_wire_Expr_unique_stable(struct wire_cst_expr *that);

WireSyncRust2DartDco frbgen_polars_wire_Expr_upper_bound(struct wire_cst_expr *that);

WireSyncRust2DartDco frbgen_polars_wire_Expr_value_counts(struct wire_cst_expr *that,
                                                          bool sort,
                                                          bool parallel);

WireSyncRust2DartDco frbgen_polars_wire_LiteralValue_from_series(uintptr_t series);

WireSyncRust2DartDco frbgen_polars_wire_Expr_list_arg_max(struct wire_cst_expr *that);

WireSyncRust2DartDco frbgen_polars_wire_Expr_list_arg_min(struct wire_cst_expr *that);

WireSyncRust2DartDco frbgen_polars_wire_Expr_list_contains(struct wire_cst_expr *that,
                                                           struct wire_cst_expr *other);

WireSyncRust2DartDco frbgen_polars_wire_Expr_list_first(struct wire_cst_expr *that);

WireSyncRust2DartDco frbgen_polars_wire_Expr_list_get(struct wire_cst_expr *that,
                                                      struct wire_cst_expr *index);

WireSyncRust2DartDco frbgen_polars_wire_Expr_list_head(struct wire_cst_expr *that,
                                                       struct wire_cst_expr *n);

WireSyncRust2DartDco frbgen_polars_wire_Expr_list_join(struct wire_cst_expr *that,
                                                       struct wire_cst_expr *separator);

WireSyncRust2DartDco frbgen_polars_wire_Expr_list_last(struct wire_cst_expr *that);

WireSyncRust2DartDco frbgen_polars_wire_Expr_list_len(struct wire_cst_expr *that);

WireSyncRust2DartDco frbgen_polars_wire_Expr_list_max(struct wire_cst_expr *that);

WireSyncRust2DartDco frbgen_polars_wire_Expr_list_mean(struct wire_cst_expr *that);

WireSyncRust2DartDco frbgen_polars_wire_Expr_list_min(struct wire_cst_expr *that);

WireSyncRust2DartDco frbgen_polars_wire_Expr_list_reverse(struct wire_cst_expr *that);

WireSyncRust2DartDco frbgen_polars_wire_Expr_list_shift(struct wire_cst_expr *that,
                                                        struct wire_cst_expr *periods);

WireSyncRust2DartDco frbgen_polars_wire_Expr_list_slice(struct wire_cst_expr *that,
                                                        struct wire_cst_expr *offset,
                                                        struct wire_cst_expr *length);

WireSyncRust2DartDco frbgen_polars_wire_Expr_list_sum(struct wire_cst_expr *that);

WireSyncRust2DartDco frbgen_polars_wire_Expr_list_tail(struct wire_cst_expr *that,
                                                       struct wire_cst_expr *n);

WireSyncRust2DartDco frbgen_polars_wire_Expr_list_unique(struct wire_cst_expr *that,
                                                         bool maintain_order);

WireSyncRust2DartDco frbgen_polars_wire_LazyGroupBy_agg(uintptr_t that,
                                                        struct wire_cst_list_expr *exprs);

WireSyncRust2DartDco frbgen_polars_wire_LazyGroupBy_head(uintptr_t that, uintptr_t *n);

WireSyncRust2DartDco frbgen_polars_wire_LazyGroupBy_tail(uintptr_t that, uintptr_t *n);

WireSyncRust2DartDco frbgen_polars_wire_Schema_of(struct wire_cst_list_field *fields);

WireSyncRust2DartDco frbgen_polars_wire_Series_add_to(uintptr_t that, uintptr_t other);

WireSyncRust2DartDco frbgen_polars_wire_Series_append(uintptr_t that, uintptr_t other);

WireSyncRust2DartDco frbgen_polars_wire_Series_apply_scalar(uintptr_t that,
                                                            int32_t op,
                                                            double value);

WireSyncRust2DartDco frbgen_polars_wire_Series_as_doubles(uintptr_t that, bool strict);

WireSyncRust2DartDco frbgen_polars_wire_Series_as_durations(uintptr_t that);

WireSyncRust2DartDco frbgen_polars_wire_Series_as_ints(uintptr_t that, bool strict);

WireSyncRust2DartDco frbgen_polars_wire_Series_as_local_datetime(uintptr_t that);

WireSyncRust2DartDco frbgen_polars_wire_Series_as_naive_datetime(uintptr_t that);

WireSyncRust2DartDco frbgen_polars_wire_Series_as_strings(uintptr_t that);

WireSyncRust2DartDco frbgen_polars_wire_Series_as_utc_datetime(uintptr_t that);

WireSyncRust2DartDco frbgen_polars_wire_Series_cast(uintptr_t that,
                                                    struct wire_cst_data_type *dtype,
                                                    bool strict);

WireSyncRust2DartDco frbgen_polars_wire_Series_divide(uintptr_t that, uintptr_t other);

WireSyncRust2DartDco frbgen_polars_wire_Series_dump(uintptr_t that);

WireSyncRust2DartDco frbgen_polars_wire_Series_equal(uintptr_t that,
                                                     uintptr_t other,
                                                     bool ignore_null);

WireSyncRust2DartDco frbgen_polars_wire_Series_estimated_size(uintptr_t that);

WireSyncRust2DartDco frbgen_polars_wire_Series_explode(uintptr_t that);

WireSyncRust2DartDco frbgen_polars_wire_Series_explode_by_offsets(uintptr_t that,
                                                                  struct wire_cst_list_prim_i_64_strict *offsets);

WireSyncRust2DartDco frbgen_polars_wire_Series_get(uintptr_t that, uintptr_t index);

WireSyncRust2DartDco frbgen_polars_wire_Series_get_string(uintptr_t that, uintptr_t index);

WireSyncRust2DartDco frbgen_polars_wire_Series_head(uintptr_t that, uintptr_t *length);

WireSyncRust2DartDco frbgen_polars_wire_Series_into_frame(uintptr_t that);

WireSyncRust2DartDco frbgen_polars_wire_Series_into_literal(uintptr_t that);

WireSyncRust2DartDco frbgen_polars_wire_Series_into_literals(uintptr_t that);

WireSyncRust2DartDco frbgen_polars_wire_Series_is_bool(uintptr_t that);

WireSyncRust2DartDco frbgen_polars_wire_Series_is_numeric(uintptr_t that);

WireSyncRust2DartDco frbgen_polars_wire_Series_is_temporal(uintptr_t that);

WireSyncRust2DartDco frbgen_polars_wire_Series_is_utf8(uintptr_t that);

void frbgen_polars_wire_Series_iter(int64_t port_, uintptr_t that);

WireSyncRust2DartDco frbgen_polars_wire_Series_max(uintptr_t that);

WireSyncRust2DartDco frbgen_polars_wire_Series_mean(uintptr_t that);

WireSyncRust2DartDco frbgen_polars_wire_Series_mean_as_series(uintptr_t that);

WireSyncRust2DartDco frbgen_polars_wire_Series_median(uintptr_t that);

WireSyncRust2DartDco frbgen_polars_wire_Series_median_as_series(uintptr_t that);

WireSyncRust2DartDco frbgen_polars_wire_Series_min(uintptr_t that);

WireSyncRust2DartDco frbgen_polars_wire_Series_multiply(uintptr_t that, uintptr_t other);

WireSyncRust2DartDco frbgen_polars_wire_Series_of_lits(struct wire_cst_list_prim_u_8_strict *name,
                                                       struct wire_cst_literals *values);

WireSyncRust2DartDco frbgen_polars_wire_Series_product(uintptr_t that);

WireSyncRust2DartDco frbgen_polars_wire_Series_remainder(uintptr_t that, uintptr_t other);

WireSyncRust2DartDco frbgen_polars_wire_Series_rename(uintptr_t that,
                                                      struct wire_cst_list_prim_u_8_strict *name);

WireSyncRust2DartDco frbgen_polars_wire_Series_reshape(uintptr_t that,
                                                       struct wire_cst_list_prim_i_64_strict *dims);

WireSyncRust2DartDco frbgen_polars_wire_Series_rolling_max(uintptr_t that,
                                                           int64_t *window_size,
                                                           uintptr_t min_periods,
                                                           struct wire_cst_list_prim_f_64_strict *weights,
                                                           bool center,
                                                           struct wire_cst_list_prim_i_64_strict *by,
                                                           int32_t *closed_window,
                                                           int32_t *time_unit,
                                                           struct wire_cst_list_prim_u_8_strict *timezone);

WireSyncRust2DartDco frbgen_polars_wire_Series_rolling_mean(uintptr_t that,
                                                            int64_t *window_size,
                                                            uintptr_t min_periods,
                                                            struct wire_cst_list_prim_f_64_strict *weights,
                                                            bool center,
                                                            struct wire_cst_list_prim_i_64_strict *by,
                                                            int32_t *closed_window,
                                                            int32_t *time_unit,
                                                            struct wire_cst_list_prim_u_8_strict *timezone);

WireSyncRust2DartDco frbgen_polars_wire_Series_rolling_median(uintptr_t that,
                                                              int64_t *window_size,
                                                              uintptr_t min_periods,
                                                              struct wire_cst_list_prim_f_64_strict *weights,
                                                              bool center,
                                                              struct wire_cst_list_prim_i_64_strict *by,
                                                              int32_t *closed_window,
                                                              int32_t *time_unit,
                                                              struct wire_cst_list_prim_u_8_strict *timezone);

WireSyncRust2DartDco frbgen_polars_wire_Series_rolling_min(uintptr_t that,
                                                           int64_t *window_size,
                                                           uintptr_t min_periods,
                                                           struct wire_cst_list_prim_f_64_strict *weights,
                                                           bool center,
                                                           struct wire_cst_list_prim_i_64_strict *by,
                                                           int32_t *closed_window,
                                                           int32_t *time_unit,
                                                           struct wire_cst_list_prim_u_8_strict *timezone);

WireSyncRust2DartDco frbgen_polars_wire_Series_rolling_quantile(uintptr_t that,
                                                                int64_t *window_size,
                                                                uintptr_t min_periods,
                                                                struct wire_cst_list_prim_f_64_strict *weights,
                                                                bool center,
                                                                struct wire_cst_list_prim_i_64_strict *by,
                                                                int32_t *closed_window,
                                                                int32_t *time_unit,
                                                                struct wire_cst_list_prim_u_8_strict *timezone);

WireSyncRust2DartDco frbgen_polars_wire_Series_rolling_std(uintptr_t that,
                                                           int64_t *window_size,
                                                           uintptr_t min_periods,
                                                           struct wire_cst_list_prim_f_64_strict *weights,
                                                           bool center,
                                                           struct wire_cst_list_prim_i_64_strict *by,
                                                           int32_t *closed_window,
                                                           int32_t *time_unit,
                                                           struct wire_cst_list_prim_u_8_strict *timezone);

WireSyncRust2DartDco frbgen_polars_wire_Series_rolling_sum(uintptr_t that,
                                                           int64_t *window_size,
                                                           uintptr_t min_periods,
                                                           struct wire_cst_list_prim_f_64_strict *weights,
                                                           bool center,
                                                           struct wire_cst_list_prim_i_64_strict *by,
                                                           int32_t *closed_window,
                                                           int32_t *time_unit,
                                                           struct wire_cst_list_prim_u_8_strict *timezone);

WireSyncRust2DartDco frbgen_polars_wire_Series_rolling_var(uintptr_t that,
                                                           int64_t *window_size,
                                                           uintptr_t min_periods,
                                                           struct wire_cst_list_prim_f_64_strict *weights,
                                                           bool center,
                                                           struct wire_cst_list_prim_i_64_strict *by,
                                                           int32_t *closed_window,
                                                           int32_t *time_unit,
                                                           struct wire_cst_list_prim_u_8_strict *timezone);

WireSyncRust2DartDco frbgen_polars_wire_Series_shuffle(uintptr_t that, uint64_t *seed);

WireSyncRust2DartDco frbgen_polars_wire_Series_sort(uintptr_t that, bool reverse);

WireSyncRust2DartDco frbgen_polars_wire_Series_std_as_series(uintptr_t that, uint8_t ddof);

WireSyncRust2DartDco frbgen_polars_wire_Series_subtract(uintptr_t that, uintptr_t other);

WireSyncRust2DartDco frbgen_polars_wire_Series_sum(uintptr_t that);

WireSyncRust2DartDco frbgen_polars_wire_Series_sum_as_series(uintptr_t that);

WireSyncRust2DartDco frbgen_polars_wire_Series_tail(uintptr_t that, uintptr_t *length);

WireSyncRust2DartDco frbgen_polars_wire_Series_to_list(uintptr_t that);

WireSyncRust2DartDco frbgen_polars_wire_Series_unique(uintptr_t that, bool maintain_order);

WireSyncRust2DartDco frbgen_polars_wire_Series_var_as_series(uintptr_t that, uint8_t ddof);

WireSyncRust2DartDco frbgen_polars_wire_Expr_str_concat(struct wire_cst_expr *that,
                                                        struct wire_cst_list_prim_u_8_strict *delimiter,
                                                        bool ignore_nulls);

WireSyncRust2DartDco frbgen_polars_wire_Expr_str_contains(struct wire_cst_expr *that,
                                                          struct wire_cst_expr *pat,
                                                          bool strict);

WireSyncRust2DartDco frbgen_polars_wire_Expr_str_contains_literal(struct wire_cst_expr *that,
                                                                  struct wire_cst_expr *pat);

WireSyncRust2DartDco frbgen_polars_wire_Expr_str_count_matches(struct wire_cst_expr *that,
                                                               struct wire_cst_expr *pat,
                                                               bool literal);

WireSyncRust2DartDco frbgen_polars_wire_Expr_str_ends_with(struct wire_cst_expr *that,
                                                           struct wire_cst_expr *pat);

WireSyncRust2DartDco frbgen_polars_wire_Expr_str_explode(struct wire_cst_expr *that);

WireSyncRust2DartDco frbgen_polars_wire_Expr_str_extract(struct wire_cst_expr *that,
                                                         struct wire_cst_list_prim_u_8_strict *pat,
                                                         uintptr_t group_index);

WireSyncRust2DartDco frbgen_polars_wire_Expr_str_extract_all(struct wire_cst_expr *that,
                                                             struct wire_cst_expr *pat);

WireSyncRust2DartDco frbgen_polars_wire_Expr_str_len_bytes(struct wire_cst_expr *that);

WireSyncRust2DartDco frbgen_polars_wire_Expr_str_len_chars(struct wire_cst_expr *that);

WireSyncRust2DartDco frbgen_polars_wire_Expr_str_replace(struct wire_cst_expr *that,
                                                         struct wire_cst_expr *pat,
                                                         struct wire_cst_expr *val,
                                                         bool literal);

WireSyncRust2DartDco frbgen_polars_wire_Expr_str_replace_all(struct wire_cst_expr *that,
                                                             struct wire_cst_expr *pat,
                                                             struct wire_cst_expr *val,
                                                             bool literal);

WireSyncRust2DartDco frbgen_polars_wire_Expr_str_replace_n(struct wire_cst_expr *that,
                                                           struct wire_cst_expr *pat,
                                                           struct wire_cst_expr *val,
                                                           bool literal,
                                                           int64_t n);

WireSyncRust2DartDco frbgen_polars_wire_Expr_str_slice(struct wire_cst_expr *that,
                                                       int64_t start,
                                                       uint64_t *length);

WireSyncRust2DartDco frbgen_polars_wire_Expr_str_split(struct wire_cst_expr *that,
                                                       struct wire_cst_expr *by,
                                                       bool inclusive);

WireSyncRust2DartDco frbgen_polars_wire_Expr_str_split_exact(struct wire_cst_expr *that,
                                                             struct wire_cst_expr *by,
                                                             uintptr_t n,
                                                             bool inclusive);

WireSyncRust2DartDco frbgen_polars_wire_Expr_str_splitn(struct wire_cst_expr *that,
                                                        struct wire_cst_expr *by,
                                                        uintptr_t n);

WireSyncRust2DartDco frbgen_polars_wire_Expr_str_starts_with(struct wire_cst_expr *that,
                                                             struct wire_cst_expr *pat);

WireSyncRust2DartDco frbgen_polars_wire_Expr_str_strip_chars(struct wire_cst_expr *that,
                                                             struct wire_cst_expr *matches);

WireSyncRust2DartDco frbgen_polars_wire_Expr_str_strip_chars_end(struct wire_cst_expr *that,
                                                                 struct wire_cst_expr *matches);

WireSyncRust2DartDco frbgen_polars_wire_Expr_str_strip_chars_start(struct wire_cst_expr *that,
                                                                   struct wire_cst_expr *matches);

WireSyncRust2DartDco frbgen_polars_wire_Expr_str_strip_prefix(struct wire_cst_expr *that,
                                                              struct wire_cst_expr *prefix);

WireSyncRust2DartDco frbgen_polars_wire_Expr_str_strip_suffix(struct wire_cst_expr *that,
                                                              struct wire_cst_expr *suffix);

WireSyncRust2DartDco frbgen_polars_wire_Expr_str_to_date(struct wire_cst_expr *that,
                                                         struct wire_cst_list_prim_u_8_strict *format,
                                                         bool strict,
                                                         bool exact,
                                                         bool cache);

WireSyncRust2DartDco frbgen_polars_wire_Expr_str_to_datetime(struct wire_cst_expr *that,
                                                             int32_t *time_unit,
                                                             struct wire_cst_list_prim_u_8_strict *time_zone,
                                                             struct wire_cst_list_prim_u_8_strict *format,
                                                             bool strict,
                                                             bool exact,
                                                             bool cache,
                                                             int32_t ambiguous);

WireSyncRust2DartDco frbgen_polars_wire_Expr_str_to_integer(struct wire_cst_expr *that,
                                                            uint32_t base,
                                                            bool strict);

WireSyncRust2DartDco frbgen_polars_wire_Expr_str_to_lowercase(struct wire_cst_expr *that);

WireSyncRust2DartDco frbgen_polars_wire_Expr_str_to_time(struct wire_cst_expr *that,
                                                         struct wire_cst_list_prim_u_8_strict *format,
                                                         bool strict,
                                                         bool exact,
                                                         bool cache);

WireSyncRust2DartDco frbgen_polars_wire_Expr_str_to_uppercase(struct wire_cst_expr *that);

WireSyncRust2DartDco frbgen_polars_wire_Expr_strptime(struct wire_cst_expr *that,
                                                      struct wire_cst_data_type *dtype,
                                                      struct wire_cst_list_prim_u_8_strict *format,
                                                      bool strict,
                                                      bool exact,
                                                      bool cache,
                                                      int32_t ambiguous);

void frbgen_polars_rust_arc_increment_strong_count_RustOpaque_AssertUnwindSafePExpr(const void *ptr);

void frbgen_polars_rust_arc_decrement_strong_count_RustOpaque_AssertUnwindSafePExpr(const void *ptr);

void frbgen_polars_rust_arc_increment_strong_count_RustOpaque_AssertUnwindSafePSeries(const void *ptr);

void frbgen_polars_rust_arc_decrement_strong_count_RustOpaque_AssertUnwindSafePSeries(const void *ptr);

void frbgen_polars_rust_arc_increment_strong_count_RustOpaque_AssertUnwindSafeSpecialEqPSeries(const void *ptr);

void frbgen_polars_rust_arc_decrement_strong_count_RustOpaque_AssertUnwindSafeSpecialEqPSeries(const void *ptr);

void frbgen_polars_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockDataFrame(const void *ptr);

void frbgen_polars_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockDataFrame(const void *ptr);

void frbgen_polars_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockLazyFrame(const void *ptr);

void frbgen_polars_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockLazyFrame(const void *ptr);

void frbgen_polars_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockLazyGroupBy(const void *ptr);

void frbgen_polars_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockLazyGroupBy(const void *ptr);

void frbgen_polars_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockOptionSchema(const void *ptr);

void frbgen_polars_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockOptionSchema(const void *ptr);

void frbgen_polars_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockSchema(const void *ptr);

void frbgen_polars_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockSchema(const void *ptr);

void frbgen_polars_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockSeries(const void *ptr);

void frbgen_polars_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockSeries(const void *ptr);

void frbgen_polars_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockVecSeries(const void *ptr);

void frbgen_polars_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockVecSeries(const void *ptr);

int64_t *frbgen_polars_cst_new_box_autoadd_Chrono_Duration(int64_t value);

struct wire_cst_agg_expr *frbgen_polars_cst_new_box_autoadd_agg_expr(void);

bool *frbgen_polars_cst_new_box_autoadd_bool(bool value);

int32_t *frbgen_polars_cst_new_box_autoadd_closed_window(int32_t value);

int32_t *frbgen_polars_cst_new_box_autoadd_csv_encoding(int32_t value);

struct wire_cst_data_type *frbgen_polars_cst_new_box_autoadd_data_type(void);

struct wire_cst_expr *frbgen_polars_cst_new_box_autoadd_expr(void);

double *frbgen_polars_cst_new_box_autoadd_f_64(double value);

int64_t *frbgen_polars_cst_new_box_autoadd_i_64(int64_t value);

struct wire_cst_literal_value *frbgen_polars_cst_new_box_autoadd_literal_value(void);

struct wire_cst_literals *frbgen_polars_cst_new_box_autoadd_literals(void);

struct wire_cst_null_values *frbgen_polars_cst_new_box_autoadd_null_values(void);

struct wire_cst_row_count *frbgen_polars_cst_new_box_autoadd_row_count(void);

struct wire_cst_sort_options *frbgen_polars_cst_new_box_autoadd_sort_options(void);

int32_t *frbgen_polars_cst_new_box_autoadd_time_unit(int32_t value);

uint32_t *frbgen_polars_cst_new_box_autoadd_u_32(uint32_t value);

uint64_t *frbgen_polars_cst_new_box_autoadd_u_64(uint64_t value);

uintptr_t *frbgen_polars_cst_new_box_autoadd_usize(uintptr_t value);

struct wire_cst_window_type *frbgen_polars_cst_new_box_autoadd_window_type(void);

struct wire_cst_data_type *frbgen_polars_cst_new_box_data_type(void);

struct wire_cst_expr *frbgen_polars_cst_new_box_expr(void);

struct wire_cst_list_Chrono_Duration *frbgen_polars_cst_new_list_Chrono_Duration(int32_t len);

struct wire_cst_list_String *frbgen_polars_cst_new_list_String(int32_t len);

struct wire_cst_list_bool *frbgen_polars_cst_new_list_bool(int32_t len);

struct wire_cst_list_data_type *frbgen_polars_cst_new_list_data_type(int32_t len);

struct wire_cst_list_excluded *frbgen_polars_cst_new_list_excluded(int32_t len);

struct wire_cst_list_expr *frbgen_polars_cst_new_list_expr(int32_t len);

struct wire_cst_list_field *frbgen_polars_cst_new_list_field(int32_t len);

struct wire_cst_list_opt_String *frbgen_polars_cst_new_list_opt_String(int32_t len);

struct wire_cst_list_opt_box_autoadd_Chrono_Duration *frbgen_polars_cst_new_list_opt_box_autoadd_Chrono_Duration(int32_t len);

struct wire_cst_list_opt_box_autoadd_f_64 *frbgen_polars_cst_new_list_opt_box_autoadd_f_64(int32_t len);

struct wire_cst_list_opt_box_autoadd_i_64 *frbgen_polars_cst_new_list_opt_box_autoadd_i_64(int32_t len);

struct wire_cst_list_prim_f_64_strict *frbgen_polars_cst_new_list_prim_f_64_strict(int32_t len);

struct wire_cst_list_prim_i_64_strict *frbgen_polars_cst_new_list_prim_i_64_strict(int32_t len);

struct wire_cst_list_prim_u_32_strict *frbgen_polars_cst_new_list_prim_u_32_strict(int32_t len);

struct wire_cst_list_prim_u_8_strict *frbgen_polars_cst_new_list_prim_u_8_strict(int32_t len);

struct wire_cst_list_record_string_literals *frbgen_polars_cst_new_list_record_string_literals(int32_t len);

struct wire_cst_list_record_string_string *frbgen_polars_cst_new_list_record_string_string(int32_t len);
static int64_t dummy_method_to_enforce_bundling(void) {
    int64_t dummy_var = 0;
    dummy_var ^= ((int64_t) (void*) drop_dart_object);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_cst_new_box_autoadd_Chrono_Duration);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_cst_new_box_autoadd_agg_expr);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_cst_new_box_autoadd_bool);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_cst_new_box_autoadd_closed_window);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_cst_new_box_autoadd_csv_encoding);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_cst_new_box_autoadd_data_type);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_cst_new_box_autoadd_expr);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_cst_new_box_autoadd_f_64);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_cst_new_box_autoadd_i_64);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_cst_new_box_autoadd_literal_value);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_cst_new_box_autoadd_literals);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_cst_new_box_autoadd_null_values);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_cst_new_box_autoadd_row_count);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_cst_new_box_autoadd_sort_options);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_cst_new_box_autoadd_time_unit);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_cst_new_box_autoadd_u_32);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_cst_new_box_autoadd_u_64);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_cst_new_box_autoadd_usize);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_cst_new_box_autoadd_window_type);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_cst_new_box_data_type);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_cst_new_box_expr);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_cst_new_list_Chrono_Duration);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_cst_new_list_String);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_cst_new_list_bool);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_cst_new_list_data_type);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_cst_new_list_excluded);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_cst_new_list_expr);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_cst_new_list_field);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_cst_new_list_opt_String);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_cst_new_list_opt_box_autoadd_Chrono_Duration);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_cst_new_list_opt_box_autoadd_f_64);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_cst_new_list_opt_box_autoadd_i_64);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_cst_new_list_prim_f_64_strict);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_cst_new_list_prim_i_64_strict);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_cst_new_list_prim_u_32_strict);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_cst_new_list_prim_u_8_strict);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_cst_new_list_record_string_literals);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_cst_new_list_record_string_string);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_dart_fn_deliver_output);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_rust_arc_decrement_strong_count_RustOpaque_AssertUnwindSafePExpr);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_rust_arc_decrement_strong_count_RustOpaque_AssertUnwindSafePSeries);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_rust_arc_decrement_strong_count_RustOpaque_AssertUnwindSafeSpecialEqPSeries);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockDataFrame);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockLazyFrame);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockLazyGroupBy);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockOptionSchema);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockSchema);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockSeries);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockVecSeries);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_rust_arc_increment_strong_count_RustOpaque_AssertUnwindSafePExpr);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_rust_arc_increment_strong_count_RustOpaque_AssertUnwindSafePSeries);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_rust_arc_increment_strong_count_RustOpaque_AssertUnwindSafeSpecialEqPSeries);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockDataFrame);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockLazyFrame);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockLazyGroupBy);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockOptionSchema);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockSchema);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockSeries);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockVecSeries);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_DataFrame_clone);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_DataFrame_column);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_DataFrame_column_at);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_DataFrame_column_names);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_DataFrame_columns);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_DataFrame_describe);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_DataFrame_drop);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_DataFrame_drop_in_place);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_DataFrame_dtypes);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_DataFrame_dump);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_DataFrame_estimated_size);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_DataFrame_get_columns);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_DataFrame_get_row);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_DataFrame_head);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_DataFrame_height);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_DataFrame_is_empty);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_DataFrame_iter);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_DataFrame_lazy);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_DataFrame_max);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_DataFrame_of_lits);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_DataFrame_reverse);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_DataFrame_sample);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_DataFrame_schema);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_DataFrame_select);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_DataFrame_shape);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_DataFrame_sort_in_place);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_DataFrame_tail);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_DataFrame_width);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_DataFrame_with_row_count);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_DataFrame_write_csv);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_abs);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_all);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_any);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_append);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_arccos);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_arccosh);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_arcsin);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_arcsinh);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_arctan);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_arctan2);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_arctanh);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_arg_max);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_arg_min);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_arg_sort);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_arg_unique);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_backward_fill);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_cbrt);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_ceil);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_clip);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_clip_max);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_clip_min);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_cos);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_cosh);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_cot);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_count);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_cum_count);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_cum_max);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_cum_min);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_cum_prod);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_cum_sum);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_degrees);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_div);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_dot);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_drop_nans);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_drop_nulls);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_entropy);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_exp);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_fill_nan);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_fill_null);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_floor);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_forward_fill);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_is_finite);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_is_in);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_is_nan);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_is_not_nan);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_is_not_null);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_is_null);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_list_arg_max);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_list_arg_min);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_list_contains);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_list_first);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_list_get);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_list_head);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_list_join);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_list_last);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_list_len);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_list_max);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_list_mean);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_list_min);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_list_reverse);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_list_shift);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_list_slice);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_list_sum);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_list_tail);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_list_unique);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_log);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_log1p);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_lower_bound);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_not);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_null_count);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_pow);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_product);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_radians);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_reshape);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_reverse);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_rolling_max);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_rolling_mean);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_rolling_median);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_rolling_min);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_rolling_quantile);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_rolling_std);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_rolling_sum);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_rolling_var);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_round);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_round_sig_figs);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_set_sorted_flag);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_shift);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_shift_and_fill);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_shrink_dtype);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_sin);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_sinh);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_sqrt);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_str_concat);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_str_contains);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_str_contains_literal);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_str_count_matches);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_str_ends_with);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_str_explode);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_str_extract);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_str_extract_all);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_str_len_bytes);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_str_len_chars);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_str_replace);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_str_replace_all);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_str_replace_n);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_str_slice);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_str_split);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_str_split_exact);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_str_splitn);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_str_starts_with);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_str_strip_chars);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_str_strip_chars_end);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_str_strip_chars_start);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_str_strip_prefix);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_str_strip_suffix);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_str_to_date);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_str_to_datetime);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_str_to_integer);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_str_to_lowercase);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_str_to_time);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_str_to_uppercase);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_strptime);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_tan);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_tanh);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_to_dot);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_to_physical);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_unique);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_unique_stable);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_upper_bound);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Expr_value_counts);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_LazyFrame_cache);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_LazyFrame_collect);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_LazyFrame_cross_join);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_LazyFrame_drop_nulls);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_LazyFrame_explode);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_LazyFrame_fetch);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_LazyFrame_filter);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_LazyFrame_first);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_LazyFrame_group_by);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_LazyFrame_inner_join);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_LazyFrame_join);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_LazyFrame_last);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_LazyFrame_left_join);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_LazyFrame_limit);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_LazyFrame_max);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_LazyFrame_mean);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_LazyFrame_median);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_LazyFrame_melt);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_LazyFrame_min);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_LazyFrame_null_count);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_LazyFrame_outer_join);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_LazyFrame_quantile);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_LazyFrame_reverse);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_LazyFrame_select);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_LazyFrame_slice);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_LazyFrame_sort);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_LazyFrame_std);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_LazyFrame_sum);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_LazyFrame_tail);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_LazyFrame_unique);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_LazyFrame_variance);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_LazyFrame_with_column);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_LazyFrame_with_columns);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_LazyFrame_with_row_count);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_LazyGroupBy_agg);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_LazyGroupBy_head);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_LazyGroupBy_tail);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_LiteralValue_from_series);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Schema_of);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Series_add_to);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Series_append);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Series_apply_scalar);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Series_as_doubles);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Series_as_durations);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Series_as_ints);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Series_as_local_datetime);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Series_as_naive_datetime);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Series_as_strings);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Series_as_utc_datetime);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Series_cast);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Series_divide);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Series_dump);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Series_equal);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Series_estimated_size);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Series_explode);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Series_explode_by_offsets);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Series_get);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Series_get_string);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Series_head);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Series_into_frame);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Series_into_literal);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Series_into_literals);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Series_is_bool);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Series_is_numeric);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Series_is_temporal);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Series_is_utf8);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Series_iter);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Series_max);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Series_mean);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Series_mean_as_series);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Series_median);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Series_median_as_series);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Series_min);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Series_multiply);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Series_of_lits);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Series_product);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Series_remainder);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Series_rename);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Series_reshape);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Series_rolling_max);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Series_rolling_mean);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Series_rolling_median);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Series_rolling_min);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Series_rolling_quantile);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Series_rolling_std);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Series_rolling_sum);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Series_rolling_var);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Series_shuffle);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Series_sort);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Series_std_as_series);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Series_subtract);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Series_sum);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Series_sum_as_series);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Series_tail);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Series_to_list);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Series_unique);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_Series_var_as_series);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_read_csv);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_read_json);
    dummy_var ^= ((int64_t) (void*) frbgen_polars_wire_scan_csv);
    dummy_var ^= ((int64_t) (void*) get_dart_object);
    dummy_var ^= ((int64_t) (void*) new_dart_opaque);
    dummy_var ^= ((int64_t) (void*) store_dart_post_cobject);
    return dummy_var;
}
