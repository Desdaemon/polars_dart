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

typedef struct wire_cst_list_prim_u_8 {
  uint8_t *ptr;
  int32_t len;
} wire_cst_list_prim_u_8;

typedef struct wire_cst_list_String {
  struct wire_cst_list_prim_u_8 **ptr;
  int32_t len;
} wire_cst_list_String;

typedef struct wire_cst_list_prim_f_64 {
  double *ptr;
  int32_t len;
} wire_cst_list_prim_f_64;

typedef struct wire_cst_list_bool {
  bool *ptr;
  int32_t len;
} wire_cst_list_bool;

typedef struct wire_cst_Expr_Alias {
  struct wire_cst_expr *field0;
  struct wire_cst_list_prim_u_8 *field1;
} wire_cst_Expr_Alias;

typedef struct wire_cst_Expr_Column {
  struct wire_cst_list_prim_u_8 *field0;
} wire_cst_Expr_Column;

typedef struct wire_cst_Expr_Columns {
  struct wire_cst_list_String *field0;
} wire_cst_Expr_Columns;

typedef struct wire_cst_DataType_Boolean {

} wire_cst_DataType_Boolean;

typedef struct wire_cst_DataType_UInt8 {

} wire_cst_DataType_UInt8;

typedef struct wire_cst_DataType_UInt16 {

} wire_cst_DataType_UInt16;

typedef struct wire_cst_DataType_UInt32 {

} wire_cst_DataType_UInt32;

typedef struct wire_cst_DataType_UInt64 {

} wire_cst_DataType_UInt64;

typedef struct wire_cst_DataType_Int8 {

} wire_cst_DataType_Int8;

typedef struct wire_cst_DataType_Int16 {

} wire_cst_DataType_Int16;

typedef struct wire_cst_DataType_Int32 {

} wire_cst_DataType_Int32;

typedef struct wire_cst_DataType_Int64 {

} wire_cst_DataType_Int64;

typedef struct wire_cst_DataType_Float32 {

} wire_cst_DataType_Float32;

typedef struct wire_cst_DataType_Float64 {

} wire_cst_DataType_Float64;

typedef struct wire_cst_DataType_Utf8 {

} wire_cst_DataType_Utf8;

typedef struct wire_cst_DataType_Binary {

} wire_cst_DataType_Binary;

typedef struct wire_cst_DataType_Date {

} wire_cst_DataType_Date;

typedef struct wire_cst_DataType_Datetime {
  int32_t field0;
  struct wire_cst_list_prim_u_8 *field1;
} wire_cst_DataType_Datetime;

typedef struct wire_cst_DataType_Duration {
  int32_t field0;
} wire_cst_DataType_Duration;

typedef struct wire_cst_DataType_Time {

} wire_cst_DataType_Time;

typedef struct wire_cst_DataType_List {
  struct wire_cst_data_type *field0;
} wire_cst_DataType_List;

typedef struct wire_cst_DataType_Null {

} wire_cst_DataType_Null;

typedef struct wire_cst_field {
  struct wire_cst_list_prim_u_8 *name;
  struct wire_cst_data_type dtype;
} wire_cst_field;

typedef struct wire_cst_list_field {
  struct wire_cst_field *ptr;
  int32_t len;
} wire_cst_list_field;

typedef struct wire_cst_DataType_Struct {
  struct wire_cst_list_field *field0;
} wire_cst_DataType_Struct;

typedef struct wire_cst_DataType_Unknown {

} wire_cst_DataType_Unknown;

typedef union DataTypeKind {
  struct wire_cst_DataType_Boolean *Boolean;
  struct wire_cst_DataType_UInt8 *UInt8;
  struct wire_cst_DataType_UInt16 *UInt16;
  struct wire_cst_DataType_UInt32 *UInt32;
  struct wire_cst_DataType_UInt64 *UInt64;
  struct wire_cst_DataType_Int8 *Int8;
  struct wire_cst_DataType_Int16 *Int16;
  struct wire_cst_DataType_Int32 *Int32;
  struct wire_cst_DataType_Int64 *Int64;
  struct wire_cst_DataType_Float32 *Float32;
  struct wire_cst_DataType_Float64 *Float64;
  struct wire_cst_DataType_Utf8 *Utf8;
  struct wire_cst_DataType_Binary *Binary;
  struct wire_cst_DataType_Date *Date;
  struct wire_cst_DataType_Datetime *Datetime;
  struct wire_cst_DataType_Duration *Duration;
  struct wire_cst_DataType_Time *Time;
  struct wire_cst_DataType_List *List;
  struct wire_cst_DataType_Null *Null;
  struct wire_cst_DataType_Struct *Struct;
  struct wire_cst_DataType_Unknown *Unknown;
} DataTypeKind;

typedef struct wire_cst_data_type {
  int32_t tag;
  union DataTypeKind *kind;
} wire_cst_data_type;

typedef struct wire_cst_list_data_type {
  struct wire_cst_data_type *ptr;
  int32_t len;
} wire_cst_list_data_type;

typedef struct wire_cst_Expr_DtypeColumn {
  struct wire_cst_list_data_type *field0;
} wire_cst_Expr_DtypeColumn;

typedef struct wire_cst_LiteralValue_Null {

} wire_cst_LiteralValue_Null;

typedef struct wire_cst_LiteralValue_Boolean {
  bool field0;
} wire_cst_LiteralValue_Boolean;

typedef struct wire_cst_LiteralValue_Utf8 {
  struct wire_cst_list_prim_u_8 *field0;
} wire_cst_LiteralValue_Utf8;

typedef struct wire_cst_LiteralValue_Binary {
  struct wire_cst_list_prim_u_8 *field0;
} wire_cst_LiteralValue_Binary;

typedef struct wire_cst_LiteralValue_UInt8 {
  uint8_t field0;
} wire_cst_LiteralValue_UInt8;

typedef struct wire_cst_LiteralValue_UInt16 {
  uint16_t field0;
} wire_cst_LiteralValue_UInt16;

typedef struct wire_cst_LiteralValue_UInt32 {
  uint32_t field0;
} wire_cst_LiteralValue_UInt32;

typedef struct wire_cst_LiteralValue_UInt64 {
  uint64_t field0;
} wire_cst_LiteralValue_UInt64;

typedef struct wire_cst_LiteralValue_Int8 {
  int8_t field0;
} wire_cst_LiteralValue_Int8;

typedef struct wire_cst_LiteralValue_Int16 {
  int16_t field0;
} wire_cst_LiteralValue_Int16;

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
  struct wire_cst_list_prim_u_8 *field2;
} wire_cst_LiteralValue_DateTime;

typedef struct wire_cst_LiteralValue_Duration {
  int64_t field0;
  int32_t field1;
} wire_cst_LiteralValue_Duration;

typedef struct wire_cst_LiteralValue_Date {
  int32_t field0;
} wire_cst_LiteralValue_Date;

typedef struct wire_cst_LiteralValue_Time {
  int64_t field0;
} wire_cst_LiteralValue_Time;

typedef union LiteralValueKind {
  struct wire_cst_LiteralValue_Null *Null;
  struct wire_cst_LiteralValue_Boolean *Boolean;
  struct wire_cst_LiteralValue_Utf8 *Utf8;
  struct wire_cst_LiteralValue_Binary *Binary;
  struct wire_cst_LiteralValue_UInt8 *UInt8;
  struct wire_cst_LiteralValue_UInt16 *UInt16;
  struct wire_cst_LiteralValue_UInt32 *UInt32;
  struct wire_cst_LiteralValue_UInt64 *UInt64;
  struct wire_cst_LiteralValue_Int8 *Int8;
  struct wire_cst_LiteralValue_Int16 *Int16;
  struct wire_cst_LiteralValue_Int32 *Int32;
  struct wire_cst_LiteralValue_Int64 *Int64;
  struct wire_cst_LiteralValue_Float32 *Float32;
  struct wire_cst_LiteralValue_Float64 *Float64;
  struct wire_cst_LiteralValue_Range *Range;
  struct wire_cst_LiteralValue_DateTime *DateTime;
  struct wire_cst_LiteralValue_Duration *Duration;
  struct wire_cst_LiteralValue_Date *Date;
  struct wire_cst_LiteralValue_Time *Time;
} LiteralValueKind;

typedef struct wire_cst_literal_value {
  int32_t tag;
  union LiteralValueKind *kind;
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
  struct wire_cst_AggExpr_Min *Min;
  struct wire_cst_AggExpr_Max *Max;
  struct wire_cst_AggExpr_Median *Median;
  struct wire_cst_AggExpr_NUnique *NUnique;
  struct wire_cst_AggExpr_First *First;
  struct wire_cst_AggExpr_Last *Last;
  struct wire_cst_AggExpr_Mean *Mean;
  struct wire_cst_AggExpr_Count *Count;
  struct wire_cst_AggExpr_Quantile *Quantile;
  struct wire_cst_AggExpr_Sum *Sum;
  struct wire_cst_AggExpr_AggGroups *AggGroups;
  struct wire_cst_AggExpr_Std *Std;
  struct wire_cst_AggExpr_Var *Var;
} AggExprKind;

typedef struct wire_cst_agg_expr {
  int32_t tag;
  union AggExprKind *kind;
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
  struct wire_cst_WindowType_Over *Over;
} WindowTypeKind;

typedef struct wire_cst_window_type {
  int32_t tag;
  union WindowTypeKind *kind;
} wire_cst_window_type;

typedef struct wire_cst_Expr_Window {
  struct wire_cst_expr *function;
  struct wire_cst_list_expr *partition_by;
  struct wire_cst_window_type *options;
} wire_cst_Expr_Window;

typedef struct wire_cst_Expr_Wildcard {

} wire_cst_Expr_Wildcard;

typedef struct wire_cst_Expr_Slice {
  struct wire_cst_expr *input;
  struct wire_cst_expr *offset;
  struct wire_cst_expr *length;
} wire_cst_Expr_Slice;

typedef struct wire_cst_Excluded_Name {
  struct wire_cst_list_prim_u_8 *field0;
} wire_cst_Excluded_Name;

typedef struct wire_cst_Excluded_Dtype {
  struct wire_cst_data_type *field0;
} wire_cst_Excluded_Dtype;

typedef union ExcludedKind {
  struct wire_cst_Excluded_Name *Name;
  struct wire_cst_Excluded_Dtype *Dtype;
} ExcludedKind;

typedef struct wire_cst_excluded {
  int32_t tag;
  union ExcludedKind *kind;
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

typedef struct wire_cst_Expr_Count {

} wire_cst_Expr_Count;

typedef struct wire_cst_Expr_Nth {
  int64_t field0;
} wire_cst_Expr_Nth;

typedef union ExprKind {
  struct wire_cst_Expr_Alias *Alias;
  struct wire_cst_Expr_Column *Column;
  struct wire_cst_Expr_Columns *Columns;
  struct wire_cst_Expr_DtypeColumn *DtypeColumn;
  struct wire_cst_Expr_Literal *Literal;
  struct wire_cst_Expr_BinaryExpr *BinaryExpr;
  struct wire_cst_Expr_Cast *Cast;
  struct wire_cst_Expr_Sort *Sort;
  struct wire_cst_Expr_Gather *Gather;
  struct wire_cst_Expr_SortBy *SortBy;
  struct wire_cst_Expr_Agg *Agg;
  struct wire_cst_Expr_Ternary *Ternary;
  struct wire_cst_Expr_Explode *Explode;
  struct wire_cst_Expr_Filter *Filter;
  struct wire_cst_Expr_Window *Window;
  struct wire_cst_Expr_Wildcard *Wildcard;
  struct wire_cst_Expr_Slice *Slice;
  struct wire_cst_Expr_Exclude *Exclude;
  struct wire_cst_Expr_KeepName *KeepName;
  struct wire_cst_Expr_Count *Count;
  struct wire_cst_Expr_Nth *Nth;
} ExprKind;

typedef struct wire_cst_expr {
  int32_t tag;
  union ExprKind *kind;
} wire_cst_expr;

typedef struct wire_cst_list_expr {
  struct wire_cst_expr *ptr;
  int32_t len;
} wire_cst_list_expr;

typedef struct wire_cst_list_prim_i_64 {
  int64_t *ptr;
  int32_t len;
} wire_cst_list_prim_i_64;

typedef struct wire_cst_list_opt_box_autoadd_f_64 {
  double **ptr;
  int32_t len;
} wire_cst_list_opt_box_autoadd_f_64;

typedef struct wire_cst_list_opt_box_autoadd_Chrono_Duration {
  int64_t **ptr;
  int32_t len;
} wire_cst_list_opt_box_autoadd_Chrono_Duration;

typedef struct wire_cst_list_opt_box_autoadd_i_32 {
  int32_t **ptr;
  int32_t len;
} wire_cst_list_opt_box_autoadd_i_32;

typedef struct wire_cst_list_opt_box_autoadd_i_64 {
  int64_t **ptr;
  int32_t len;
} wire_cst_list_opt_box_autoadd_i_64;

typedef struct wire_cst_list_opt_String {
  struct wire_cst_list_prim_u_8 **ptr;
  int32_t len;
} wire_cst_list_opt_String;

typedef struct wire_cst_row_count {
  struct wire_cst_list_prim_u_8 *name;
  uint32_t offset;
} wire_cst_row_count;

typedef struct wire_cst_NullValues_AllColumnsSingle {
  struct wire_cst_list_prim_u_8 *field0;
} wire_cst_NullValues_AllColumnsSingle;

typedef struct wire_cst_NullValues_AllColumns {
  struct wire_cst_list_String *field0;
} wire_cst_NullValues_AllColumns;

typedef struct wire_cst_record_string_string {
  struct wire_cst_list_prim_u_8 *field0;
  struct wire_cst_list_prim_u_8 *field1;
} wire_cst_record_string_string;

typedef struct wire_cst_list_record_string_string {
  struct wire_cst_record_string_string *ptr;
  int32_t len;
} wire_cst_list_record_string_string;

typedef struct wire_cst_NullValues_Named {
  struct wire_cst_list_record_string_string *field0;
} wire_cst_NullValues_Named;

typedef union NullValuesKind {
  struct wire_cst_NullValues_AllColumnsSingle *AllColumnsSingle;
  struct wire_cst_NullValues_AllColumns *AllColumns;
  struct wire_cst_NullValues_Named *Named;
} NullValuesKind;

typedef struct wire_cst_null_values {
  int32_t tag;
  union NullValuesKind *kind;
} wire_cst_null_values;

typedef struct wire_cst_list_prim_u_32 {
  uint32_t *ptr;
  int32_t len;
} wire_cst_list_prim_u_32;

typedef struct wire_cst_list_dartabi {
  UNREACHABLE_RUST_WIRE_TYPE *ptr;
  int32_t len;
} wire_cst_list_dartabi;

typedef struct wire_cst_list_opt_box_autoadd_Chrono_Local {
  int64_t **ptr;
  int32_t len;
} wire_cst_list_opt_box_autoadd_Chrono_Local;

typedef struct wire_cst_list_opt_box_autoadd_Chrono_Naive {
  int64_t **ptr;
  int32_t len;
} wire_cst_list_opt_box_autoadd_Chrono_Naive;

typedef struct wire_cst_list_opt_box_autoadd_Chrono_Utc {
  int64_t **ptr;
  int32_t len;
} wire_cst_list_opt_box_autoadd_Chrono_Utc;

typedef struct wire_cst_shape {
  uintptr_t height;
  uintptr_t width;
} wire_cst_shape;

void dart_fn_deliver_output(int32_t call_id,
                            uint8_t *ptr_,
                            int32_t rust_vec_len_,
                            int32_t data_len_);

WireSyncRust2DartDco wire_DataFrame_column(const void *that, struct wire_cst_list_prim_u_8 *column);

WireSyncRust2DartDco wire_DataFrame_column_at(const void *that, uintptr_t index);

WireSyncRust2DartDco wire_DataFrame_columns(const void *that, struct wire_cst_list_String *columns);

void wire_DataFrame_describe(int64_t port_,
                             const void *that,
                             struct wire_cst_list_prim_f_64 *percentiles);

WireSyncRust2DartDco wire_DataFrame_drop(const void *that, struct wire_cst_list_prim_u_8 *column);

WireSyncRust2DartDco wire_DataFrame_drop_in_place(const void *that,
                                                  struct wire_cst_list_prim_u_8 *column);

WireSyncRust2DartDco wire_DataFrame_dtypes(const void *that);

WireSyncRust2DartDco wire_DataFrame_dump(const void *that);

WireSyncRust2DartDco wire_DataFrame_estimated_size(const void *that);

WireSyncRust2DartDco wire_DataFrame_get_column_names(const void *that);

WireSyncRust2DartDco wire_DataFrame_get_columns(const void *that);

void wire_DataFrame_get_row(int64_t port_, const void *that, uintptr_t index);

WireSyncRust2DartDco wire_DataFrame_head(const void *that, uintptr_t *length);

WireSyncRust2DartDco wire_DataFrame_height(const void *that);

WireSyncRust2DartDco wire_DataFrame_is_empty(const void *that);

void wire_DataFrame_lazy(int64_t port_,
                         const void *that,
                         bool *projection_pushdown,
                         bool *predicate_pushdown,
                         bool *type_coercion,
                         bool *simplify_expressions,
                         bool *slice_pushdown,
                         bool *streaming);

void wire_DataFrame_max(int64_t port_, const void *that);

WireSyncRust2DartDco wire_DataFrame_of(const void *series);

WireSyncRust2DartDco wire_DataFrame_reverse(const void *that);

void wire_DataFrame_sample(int64_t port_,
                           const void *that,
                           const void *n,
                           bool with_replacement,
                           bool shuffle,
                           uint64_t *seed);

WireSyncRust2DartDco wire_DataFrame_schema(const void *that);

WireSyncRust2DartDco wire_DataFrame_select(const void *that, struct wire_cst_list_String *columns);

WireSyncRust2DartDco wire_DataFrame_shape(const void *that);

void wire_DataFrame_sort_in_place(int64_t port_,
                                  const void *that,
                                  struct wire_cst_list_String *by_column,
                                  struct wire_cst_list_bool *descending,
                                  bool maintain_order);

WireSyncRust2DartDco wire_DataFrame_tail(const void *that, uintptr_t *length);

WireSyncRust2DartDco wire_DataFrame_width(const void *that);

WireSyncRust2DartDco wire_DataFrame_with_row_count(const void *that,
                                                   struct wire_cst_list_prim_u_8 *name,
                                                   uint32_t *offset);

WireSyncRust2DartDco wire_LazyFrame_cache(const void *that);

void wire_LazyFrame_collect(int64_t port_, const void *that);

WireSyncRust2DartDco wire_LazyFrame_cross_join(const void *that, const void *other);

WireSyncRust2DartDco wire_LazyFrame_drop_nulls(const void *that, struct wire_cst_list_expr *subset);

WireSyncRust2DartDco wire_LazyFrame_explode(const void *that, struct wire_cst_list_expr *columns);

void wire_LazyFrame_fetch(int64_t port_, const void *that, uintptr_t n_rows);

WireSyncRust2DartDco wire_LazyFrame_filter(const void *that, struct wire_cst_expr *pred);

WireSyncRust2DartDco wire_LazyFrame_first(const void *that);

WireSyncRust2DartDco wire_LazyFrame_group_by(const void *that,
                                             struct wire_cst_list_expr *exprs,
                                             bool maintain_order);

WireSyncRust2DartDco wire_LazyFrame_inner_join(const void *that,
                                               const void *other,
                                               struct wire_cst_expr *left_on,
                                               struct wire_cst_expr *right_on);

void wire_LazyFrame_join(int64_t port_,
                         const void *that,
                         const void *other,
                         struct wire_cst_list_expr *on,
                         struct wire_cst_list_expr *left_on,
                         struct wire_cst_list_expr *right_on,
                         struct wire_cst_list_prim_u_8 *suffix,
                         int32_t how,
                         bool allow_parallel,
                         bool force_parallel);

WireSyncRust2DartDco wire_LazyFrame_last(const void *that);

WireSyncRust2DartDco wire_LazyFrame_left_join(const void *that,
                                              const void *other,
                                              struct wire_cst_expr *left_on,
                                              struct wire_cst_expr *right_on);

WireSyncRust2DartDco wire_LazyFrame_limit(const void *that, uint32_t n);

WireSyncRust2DartDco wire_LazyFrame_max(const void *that);

WireSyncRust2DartDco wire_LazyFrame_mean(const void *that);

WireSyncRust2DartDco wire_LazyFrame_median(const void *that);

WireSyncRust2DartDco wire_LazyFrame_melt(const void *that,
                                         struct wire_cst_list_String *id_vars,
                                         struct wire_cst_list_String *value_vars,
                                         struct wire_cst_list_prim_u_8 *variable_name,
                                         struct wire_cst_list_prim_u_8 *value_name,
                                         bool streamable);

void wire_LazyFrame_min(int64_t port_, const void *that);

WireSyncRust2DartDco wire_LazyFrame_outer_join(const void *that,
                                               const void *other,
                                               struct wire_cst_expr *left_on,
                                               struct wire_cst_expr *right_on);

WireSyncRust2DartDco wire_LazyFrame_quantile(const void *that,
                                             struct wire_cst_expr *quantile,
                                             int32_t interpol);

WireSyncRust2DartDco wire_LazyFrame_reverse(const void *that);

WireSyncRust2DartDco wire_LazyFrame_slice(const void *that, int64_t offset, uint32_t len);

WireSyncRust2DartDco wire_LazyFrame_std(const void *that, uint8_t ddof);

WireSyncRust2DartDco wire_LazyFrame_sum(const void *that);

WireSyncRust2DartDco wire_LazyFrame_tail(const void *that, uint32_t n);

WireSyncRust2DartDco wire_LazyFrame_unique(const void *that,
                                           struct wire_cst_list_String *subset,
                                           int32_t keep_strategy);

WireSyncRust2DartDco wire_LazyFrame_variance(const void *that, uint8_t ddof);

WireSyncRust2DartDco wire_LazyFrame_with_column(const void *that, struct wire_cst_expr *expr);

WireSyncRust2DartDco wire_LazyFrame_with_row_count(const void *that,
                                                   struct wire_cst_list_prim_u_8 *name,
                                                   uint32_t *offset);

WireSyncRust2DartDco wire_LazyGroupBy_head(const void *that, uintptr_t *n);

WireSyncRust2DartDco wire_LazyGroupBy_tail(const void *that, uintptr_t *n);

WireSyncRust2DartDco wire_Schema_of(struct wire_cst_list_field *fields);

WireSyncRust2DartDco wire_Series_add_to(const void *that, const void *other);

void wire_Series_append(int64_t port_, const void *that, const void *other);

void wire_Series_apply_scalar(int64_t port_, const void *that, int32_t op, double value);

void wire_Series_as_doubles(int64_t port_, const void *that, bool strict);

void wire_Series_as_durations(int64_t port_, const void *that);

void wire_Series_as_ints(int64_t port_, const void *that, bool strict);

void wire_Series_as_local_datetime(int64_t port_, const void *that);

void wire_Series_as_naive_datetime(int64_t port_, const void *that);

void wire_Series_as_strings(int64_t port_, const void *that);

void wire_Series_as_utc_datetime(int64_t port_, const void *that);

void wire_Series_cast(int64_t port_,
                      const void *that,
                      struct wire_cst_data_type *dtype,
                      bool strict);

WireSyncRust2DartDco wire_Series_divide(const void *that, const void *other);

void wire_Series_dump(int64_t port_, const void *that);

void wire_Series_equal(int64_t port_, const void *that, const void *other, bool ignore_null);

WireSyncRust2DartDco wire_Series_estimated_size(const void *that);

void wire_Series_explode(int64_t port_, const void *that);

void wire_Series_explode_by_offsets(int64_t port_,
                                    const void *that,
                                    struct wire_cst_list_prim_i_64 *offsets);

WireSyncRust2DartDco wire_Series_get(const void *that, uintptr_t index);

WireSyncRust2DartDco wire_Series_get_string(const void *that, uintptr_t index);

WireSyncRust2DartDco wire_Series_head(const void *that, uintptr_t *length);

WireSyncRust2DartDco wire_Series_into_frame(const void *that);

WireSyncRust2DartDco wire_Series_is_bool(const void *that);

WireSyncRust2DartDco wire_Series_is_numeric(const void *that);

WireSyncRust2DartDco wire_Series_is_temporal(const void *that);

WireSyncRust2DartDco wire_Series_is_utf8(const void *that);

void wire_Series_max(int64_t port_, const void *that);

void wire_Series_mean(int64_t port_, const void *that);

void wire_Series_mean_as_series(int64_t port_, const void *that);

void wire_Series_median(int64_t port_, const void *that);

void wire_Series_median_as_series(int64_t port_, const void *that);

void wire_Series_min(int64_t port_, const void *that);

WireSyncRust2DartDco wire_Series_multiply(const void *that, const void *other);

WireSyncRust2DartDco wire_Series_of_bools(struct wire_cst_list_prim_u_8 *name,
                                          struct wire_cst_list_bool *values);

WireSyncRust2DartDco wire_Series_of_doubles(struct wire_cst_list_prim_u_8 *name,
                                            struct wire_cst_list_opt_box_autoadd_f_64 *values);

WireSyncRust2DartDco wire_Series_of_durations(struct wire_cst_list_prim_u_8 *name,
                                              struct wire_cst_list_opt_box_autoadd_Chrono_Duration *values,
                                              int32_t unit);

WireSyncRust2DartDco wire_Series_of_i32(struct wire_cst_list_prim_u_8 *name,
                                        struct wire_cst_list_opt_box_autoadd_i_32 *values);

WireSyncRust2DartDco wire_Series_of_ints(struct wire_cst_list_prim_u_8 *name,
                                         struct wire_cst_list_opt_box_autoadd_i_64 *values);

WireSyncRust2DartDco wire_Series_of_strings(struct wire_cst_list_prim_u_8 *name,
                                            struct wire_cst_list_opt_String *values);

void wire_Series_product(int64_t port_, const void *that);

WireSyncRust2DartDco wire_Series_remainder(const void *that, const void *other);

WireSyncRust2DartDco wire_Series_rename(const void *that, struct wire_cst_list_prim_u_8 *name);

void wire_Series_reshape(int64_t port_, const void *that, struct wire_cst_list_prim_i_64 *dims);

void wire_Series_shuffle(int64_t port_, const void *that, uint64_t *seed);

void wire_Series_sort(int64_t port_, const void *that, bool reverse);

void wire_Series_std_as_series(int64_t port_, const void *that, uint8_t ddof);

WireSyncRust2DartDco wire_Series_subtract(const void *that, const void *other);

void wire_Series_sum(int64_t port_, const void *that);

void wire_Series_sum_as_series(int64_t port_, const void *that);

WireSyncRust2DartDco wire_Series_tail(const void *that, uintptr_t *length);

void wire_Series_to_list(int64_t port_, const void *that);

void wire_Series_unique(int64_t port_, const void *that, bool stable);

void wire_Series_var_as_series(int64_t port_, const void *that, uint8_t ddof);

void wire_read_csv(int64_t port_,
                   struct wire_cst_list_prim_u_8 *path,
                   struct wire_cst_list_data_type *dtypes_slice,
                   bool *has_header,
                   struct wire_cst_list_String *columns,
                   struct wire_cst_list_prim_u_8 *comment_char,
                   struct wire_cst_list_prim_u_8 *eol_char,
                   uintptr_t *chunk_size,
                   uintptr_t *sample_size,
                   struct wire_cst_row_count *row_count,
                   int32_t *encoding,
                   uintptr_t *n_rows,
                   uintptr_t *n_threads,
                   struct wire_cst_null_values *null_values,
                   struct wire_cst_list_prim_u_32 *projection,
                   struct wire_cst_list_prim_u_8 *quote_char,
                   uintptr_t skip_rows,
                   uintptr_t skip_rows_after_header,
                   bool ignore_errors,
                   bool rechunk,
                   bool try_parse_dates,
                   bool low_memory);

void wire_read_json(int64_t port_,
                    struct wire_cst_list_prim_u_8 *path,
                    uintptr_t *batch_size,
                    struct wire_cst_list_String *projection);

void wire_scan_csv(int64_t port_,
                   struct wire_cst_list_prim_u_8 *path,
                   const void *dtype_overwrite,
                   bool *has_header,
                   struct wire_cst_list_prim_u_8 *comment_char,
                   struct wire_cst_list_prim_u_8 *eol_char,
                   struct wire_cst_list_prim_u_8 *quote_char,
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

void rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockOptionVeccratewrapperSeries(const void *ptr);

void rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockOptionVeccratewrapperSeries(const void *ptr);

void rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockOptioncratewrapperSchema(const void *ptr);

void rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockOptioncratewrapperSchema(const void *ptr);

void rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockVeccratewrapperSeries(const void *ptr);

void rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockVeccratewrapperSeries(const void *ptr);

void rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockcratewrapperDataFrame(const void *ptr);

void rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockcratewrapperDataFrame(const void *ptr);

void rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockcratewrapperLazyFrame(const void *ptr);

void rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockcratewrapperLazyFrame(const void *ptr);

void rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockcratewrapperLazyGroupBy(const void *ptr);

void rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockcratewrapperLazyGroupBy(const void *ptr);

void rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockcratewrapperSchema(const void *ptr);

void rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockcratewrapperSchema(const void *ptr);

void rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockcratewrapperSeries(const void *ptr);

void rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockcratewrapperSeries(const void *ptr);

int64_t *cst_new_box_autoadd_Chrono_Duration(int64_t value);

int64_t *cst_new_box_autoadd_Chrono_Local(int64_t value);

int64_t *cst_new_box_autoadd_Chrono_Naive(int64_t value);

int64_t *cst_new_box_autoadd_Chrono_Utc(int64_t value);

struct wire_cst_agg_expr *cst_new_box_autoadd_agg_expr(void);

bool *cst_new_box_autoadd_bool(bool value);

int32_t *cst_new_box_autoadd_csv_encoding(int32_t value);

struct wire_cst_data_type *cst_new_box_autoadd_data_type(void);

struct wire_cst_expr *cst_new_box_autoadd_expr(void);

double *cst_new_box_autoadd_f_64(double value);

int32_t *cst_new_box_autoadd_i_32(int32_t value);

int64_t *cst_new_box_autoadd_i_64(int64_t value);

struct wire_cst_literal_value *cst_new_box_autoadd_literal_value(void);

struct wire_cst_null_values *cst_new_box_autoadd_null_values(void);

struct wire_cst_row_count *cst_new_box_autoadd_row_count(void);

struct wire_cst_sort_options *cst_new_box_autoadd_sort_options(void);

uint32_t *cst_new_box_autoadd_u_32(uint32_t value);

uint64_t *cst_new_box_autoadd_u_64(uint64_t value);

uintptr_t *cst_new_box_autoadd_usize(uintptr_t value);

struct wire_cst_window_type *cst_new_box_autoadd_window_type(void);

struct wire_cst_data_type *cst_new_box_data_type(void);

struct wire_cst_expr *cst_new_box_expr(void);

struct wire_cst_list_String *cst_new_list_String(int32_t len);

struct wire_cst_list_bool *cst_new_list_bool(int32_t len);

struct wire_cst_list_dartabi *cst_new_list_dartabi(int32_t len);

struct wire_cst_list_data_type *cst_new_list_data_type(int32_t len);

struct wire_cst_list_excluded *cst_new_list_excluded(int32_t len);

struct wire_cst_list_expr *cst_new_list_expr(int32_t len);

struct wire_cst_list_field *cst_new_list_field(int32_t len);

struct wire_cst_list_opt_String *cst_new_list_opt_String(int32_t len);

struct wire_cst_list_opt_box_autoadd_Chrono_Duration *cst_new_list_opt_box_autoadd_Chrono_Duration(int32_t len);

struct wire_cst_list_opt_box_autoadd_Chrono_Local *cst_new_list_opt_box_autoadd_Chrono_Local(int32_t len);

struct wire_cst_list_opt_box_autoadd_Chrono_Naive *cst_new_list_opt_box_autoadd_Chrono_Naive(int32_t len);

struct wire_cst_list_opt_box_autoadd_Chrono_Utc *cst_new_list_opt_box_autoadd_Chrono_Utc(int32_t len);

struct wire_cst_list_opt_box_autoadd_f_64 *cst_new_list_opt_box_autoadd_f_64(int32_t len);

struct wire_cst_list_opt_box_autoadd_i_32 *cst_new_list_opt_box_autoadd_i_32(int32_t len);

struct wire_cst_list_opt_box_autoadd_i_64 *cst_new_list_opt_box_autoadd_i_64(int32_t len);

struct wire_cst_list_prim_f_64 *cst_new_list_prim_f_64(int32_t len);

struct wire_cst_list_prim_i_64 *cst_new_list_prim_i_64(int32_t len);

struct wire_cst_list_prim_u_32 *cst_new_list_prim_u_32(int32_t len);

struct wire_cst_list_prim_u_8 *cst_new_list_prim_u_8(int32_t len);

struct wire_cst_list_record_string_string *cst_new_list_record_string_string(int32_t len);

union AggExprKind *cst_inflate_AggExpr_Min(void);

union AggExprKind *cst_inflate_AggExpr_Max(void);

union AggExprKind *cst_inflate_AggExpr_Median(void);

union AggExprKind *cst_inflate_AggExpr_NUnique(void);

union AggExprKind *cst_inflate_AggExpr_First(void);

union AggExprKind *cst_inflate_AggExpr_Last(void);

union AggExprKind *cst_inflate_AggExpr_Mean(void);

union AggExprKind *cst_inflate_AggExpr_Count(void);

union AggExprKind *cst_inflate_AggExpr_Quantile(void);

union AggExprKind *cst_inflate_AggExpr_Sum(void);

union AggExprKind *cst_inflate_AggExpr_AggGroups(void);

union AggExprKind *cst_inflate_AggExpr_Std(void);

union AggExprKind *cst_inflate_AggExpr_Var(void);

union DataTypeKind *cst_inflate_DataType_Datetime(void);

union DataTypeKind *cst_inflate_DataType_Duration(void);

union DataTypeKind *cst_inflate_DataType_List(void);

union DataTypeKind *cst_inflate_DataType_Struct(void);

union ExcludedKind *cst_inflate_Excluded_Name(void);

union ExcludedKind *cst_inflate_Excluded_Dtype(void);

union ExprKind *cst_inflate_Expr_Alias(void);

union ExprKind *cst_inflate_Expr_Column(void);

union ExprKind *cst_inflate_Expr_Columns(void);

union ExprKind *cst_inflate_Expr_DtypeColumn(void);

union ExprKind *cst_inflate_Expr_Literal(void);

union ExprKind *cst_inflate_Expr_BinaryExpr(void);

union ExprKind *cst_inflate_Expr_Cast(void);

union ExprKind *cst_inflate_Expr_Sort(void);

union ExprKind *cst_inflate_Expr_Gather(void);

union ExprKind *cst_inflate_Expr_SortBy(void);

union ExprKind *cst_inflate_Expr_Agg(void);

union ExprKind *cst_inflate_Expr_Ternary(void);

union ExprKind *cst_inflate_Expr_Explode(void);

union ExprKind *cst_inflate_Expr_Filter(void);

union ExprKind *cst_inflate_Expr_Window(void);

union ExprKind *cst_inflate_Expr_Slice(void);

union ExprKind *cst_inflate_Expr_Exclude(void);

union ExprKind *cst_inflate_Expr_KeepName(void);

union ExprKind *cst_inflate_Expr_Nth(void);

union LiteralValueKind *cst_inflate_LiteralValue_Boolean(void);

union LiteralValueKind *cst_inflate_LiteralValue_Utf8(void);

union LiteralValueKind *cst_inflate_LiteralValue_Binary(void);

union LiteralValueKind *cst_inflate_LiteralValue_UInt8(void);

union LiteralValueKind *cst_inflate_LiteralValue_UInt16(void);

union LiteralValueKind *cst_inflate_LiteralValue_UInt32(void);

union LiteralValueKind *cst_inflate_LiteralValue_UInt64(void);

union LiteralValueKind *cst_inflate_LiteralValue_Int8(void);

union LiteralValueKind *cst_inflate_LiteralValue_Int16(void);

union LiteralValueKind *cst_inflate_LiteralValue_Int32(void);

union LiteralValueKind *cst_inflate_LiteralValue_Int64(void);

union LiteralValueKind *cst_inflate_LiteralValue_Float32(void);

union LiteralValueKind *cst_inflate_LiteralValue_Float64(void);

union LiteralValueKind *cst_inflate_LiteralValue_Range(void);

union LiteralValueKind *cst_inflate_LiteralValue_DateTime(void);

union LiteralValueKind *cst_inflate_LiteralValue_Duration(void);

union LiteralValueKind *cst_inflate_LiteralValue_Date(void);

union LiteralValueKind *cst_inflate_LiteralValue_Time(void);

union NullValuesKind *cst_inflate_NullValues_AllColumnsSingle(void);

union NullValuesKind *cst_inflate_NullValues_AllColumns(void);

union NullValuesKind *cst_inflate_NullValues_Named(void);

union WindowTypeKind *cst_inflate_WindowType_Over(void);
static int64_t dummy_method_to_enforce_bundling(void) {
    int64_t dummy_var = 0;
    dummy_var ^= ((int64_t) (void*) cst_inflate_AggExpr_AggGroups);
    dummy_var ^= ((int64_t) (void*) cst_inflate_AggExpr_Count);
    dummy_var ^= ((int64_t) (void*) cst_inflate_AggExpr_First);
    dummy_var ^= ((int64_t) (void*) cst_inflate_AggExpr_Last);
    dummy_var ^= ((int64_t) (void*) cst_inflate_AggExpr_Max);
    dummy_var ^= ((int64_t) (void*) cst_inflate_AggExpr_Mean);
    dummy_var ^= ((int64_t) (void*) cst_inflate_AggExpr_Median);
    dummy_var ^= ((int64_t) (void*) cst_inflate_AggExpr_Min);
    dummy_var ^= ((int64_t) (void*) cst_inflate_AggExpr_NUnique);
    dummy_var ^= ((int64_t) (void*) cst_inflate_AggExpr_Quantile);
    dummy_var ^= ((int64_t) (void*) cst_inflate_AggExpr_Std);
    dummy_var ^= ((int64_t) (void*) cst_inflate_AggExpr_Sum);
    dummy_var ^= ((int64_t) (void*) cst_inflate_AggExpr_Var);
    dummy_var ^= ((int64_t) (void*) cst_inflate_DataType_Datetime);
    dummy_var ^= ((int64_t) (void*) cst_inflate_DataType_Duration);
    dummy_var ^= ((int64_t) (void*) cst_inflate_DataType_List);
    dummy_var ^= ((int64_t) (void*) cst_inflate_DataType_Struct);
    dummy_var ^= ((int64_t) (void*) cst_inflate_Excluded_Dtype);
    dummy_var ^= ((int64_t) (void*) cst_inflate_Excluded_Name);
    dummy_var ^= ((int64_t) (void*) cst_inflate_Expr_Agg);
    dummy_var ^= ((int64_t) (void*) cst_inflate_Expr_Alias);
    dummy_var ^= ((int64_t) (void*) cst_inflate_Expr_BinaryExpr);
    dummy_var ^= ((int64_t) (void*) cst_inflate_Expr_Cast);
    dummy_var ^= ((int64_t) (void*) cst_inflate_Expr_Column);
    dummy_var ^= ((int64_t) (void*) cst_inflate_Expr_Columns);
    dummy_var ^= ((int64_t) (void*) cst_inflate_Expr_DtypeColumn);
    dummy_var ^= ((int64_t) (void*) cst_inflate_Expr_Exclude);
    dummy_var ^= ((int64_t) (void*) cst_inflate_Expr_Explode);
    dummy_var ^= ((int64_t) (void*) cst_inflate_Expr_Filter);
    dummy_var ^= ((int64_t) (void*) cst_inflate_Expr_Gather);
    dummy_var ^= ((int64_t) (void*) cst_inflate_Expr_KeepName);
    dummy_var ^= ((int64_t) (void*) cst_inflate_Expr_Literal);
    dummy_var ^= ((int64_t) (void*) cst_inflate_Expr_Nth);
    dummy_var ^= ((int64_t) (void*) cst_inflate_Expr_Slice);
    dummy_var ^= ((int64_t) (void*) cst_inflate_Expr_Sort);
    dummy_var ^= ((int64_t) (void*) cst_inflate_Expr_SortBy);
    dummy_var ^= ((int64_t) (void*) cst_inflate_Expr_Ternary);
    dummy_var ^= ((int64_t) (void*) cst_inflate_Expr_Window);
    dummy_var ^= ((int64_t) (void*) cst_inflate_LiteralValue_Binary);
    dummy_var ^= ((int64_t) (void*) cst_inflate_LiteralValue_Boolean);
    dummy_var ^= ((int64_t) (void*) cst_inflate_LiteralValue_Date);
    dummy_var ^= ((int64_t) (void*) cst_inflate_LiteralValue_DateTime);
    dummy_var ^= ((int64_t) (void*) cst_inflate_LiteralValue_Duration);
    dummy_var ^= ((int64_t) (void*) cst_inflate_LiteralValue_Float32);
    dummy_var ^= ((int64_t) (void*) cst_inflate_LiteralValue_Float64);
    dummy_var ^= ((int64_t) (void*) cst_inflate_LiteralValue_Int16);
    dummy_var ^= ((int64_t) (void*) cst_inflate_LiteralValue_Int32);
    dummy_var ^= ((int64_t) (void*) cst_inflate_LiteralValue_Int64);
    dummy_var ^= ((int64_t) (void*) cst_inflate_LiteralValue_Int8);
    dummy_var ^= ((int64_t) (void*) cst_inflate_LiteralValue_Range);
    dummy_var ^= ((int64_t) (void*) cst_inflate_LiteralValue_Time);
    dummy_var ^= ((int64_t) (void*) cst_inflate_LiteralValue_UInt16);
    dummy_var ^= ((int64_t) (void*) cst_inflate_LiteralValue_UInt32);
    dummy_var ^= ((int64_t) (void*) cst_inflate_LiteralValue_UInt64);
    dummy_var ^= ((int64_t) (void*) cst_inflate_LiteralValue_UInt8);
    dummy_var ^= ((int64_t) (void*) cst_inflate_LiteralValue_Utf8);
    dummy_var ^= ((int64_t) (void*) cst_inflate_NullValues_AllColumns);
    dummy_var ^= ((int64_t) (void*) cst_inflate_NullValues_AllColumnsSingle);
    dummy_var ^= ((int64_t) (void*) cst_inflate_NullValues_Named);
    dummy_var ^= ((int64_t) (void*) cst_inflate_WindowType_Over);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_Chrono_Duration);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_Chrono_Local);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_Chrono_Naive);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_Chrono_Utc);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_agg_expr);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_bool);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_csv_encoding);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_data_type);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_expr);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_f_64);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_i_32);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_i_64);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_literal_value);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_null_values);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_row_count);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_sort_options);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_u_32);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_u_64);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_usize);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_window_type);
    dummy_var ^= ((int64_t) (void*) cst_new_box_data_type);
    dummy_var ^= ((int64_t) (void*) cst_new_box_expr);
    dummy_var ^= ((int64_t) (void*) cst_new_list_String);
    dummy_var ^= ((int64_t) (void*) cst_new_list_bool);
    dummy_var ^= ((int64_t) (void*) cst_new_list_dartabi);
    dummy_var ^= ((int64_t) (void*) cst_new_list_data_type);
    dummy_var ^= ((int64_t) (void*) cst_new_list_excluded);
    dummy_var ^= ((int64_t) (void*) cst_new_list_expr);
    dummy_var ^= ((int64_t) (void*) cst_new_list_field);
    dummy_var ^= ((int64_t) (void*) cst_new_list_opt_String);
    dummy_var ^= ((int64_t) (void*) cst_new_list_opt_box_autoadd_Chrono_Duration);
    dummy_var ^= ((int64_t) (void*) cst_new_list_opt_box_autoadd_Chrono_Local);
    dummy_var ^= ((int64_t) (void*) cst_new_list_opt_box_autoadd_Chrono_Naive);
    dummy_var ^= ((int64_t) (void*) cst_new_list_opt_box_autoadd_Chrono_Utc);
    dummy_var ^= ((int64_t) (void*) cst_new_list_opt_box_autoadd_f_64);
    dummy_var ^= ((int64_t) (void*) cst_new_list_opt_box_autoadd_i_32);
    dummy_var ^= ((int64_t) (void*) cst_new_list_opt_box_autoadd_i_64);
    dummy_var ^= ((int64_t) (void*) cst_new_list_prim_f_64);
    dummy_var ^= ((int64_t) (void*) cst_new_list_prim_i_64);
    dummy_var ^= ((int64_t) (void*) cst_new_list_prim_u_32);
    dummy_var ^= ((int64_t) (void*) cst_new_list_prim_u_8);
    dummy_var ^= ((int64_t) (void*) cst_new_list_record_string_string);
    dummy_var ^= ((int64_t) (void*) dart_fn_deliver_output);
    dummy_var ^= ((int64_t) (void*) drop_dart_object);
    dummy_var ^= ((int64_t) (void*) get_dart_object);
    dummy_var ^= ((int64_t) (void*) new_dart_opaque);
    dummy_var ^= ((int64_t) (void*) rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockOptionVeccratewrapperSeries);
    dummy_var ^= ((int64_t) (void*) rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockOptioncratewrapperSchema);
    dummy_var ^= ((int64_t) (void*) rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockVeccratewrapperSeries);
    dummy_var ^= ((int64_t) (void*) rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockcratewrapperDataFrame);
    dummy_var ^= ((int64_t) (void*) rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockcratewrapperLazyFrame);
    dummy_var ^= ((int64_t) (void*) rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockcratewrapperLazyGroupBy);
    dummy_var ^= ((int64_t) (void*) rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockcratewrapperSchema);
    dummy_var ^= ((int64_t) (void*) rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockcratewrapperSeries);
    dummy_var ^= ((int64_t) (void*) rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockOptionVeccratewrapperSeries);
    dummy_var ^= ((int64_t) (void*) rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockOptioncratewrapperSchema);
    dummy_var ^= ((int64_t) (void*) rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockVeccratewrapperSeries);
    dummy_var ^= ((int64_t) (void*) rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockcratewrapperDataFrame);
    dummy_var ^= ((int64_t) (void*) rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockcratewrapperLazyFrame);
    dummy_var ^= ((int64_t) (void*) rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockcratewrapperLazyGroupBy);
    dummy_var ^= ((int64_t) (void*) rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockcratewrapperSchema);
    dummy_var ^= ((int64_t) (void*) rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockcratewrapperSeries);
    dummy_var ^= ((int64_t) (void*) store_dart_post_cobject);
    dummy_var ^= ((int64_t) (void*) wire_DataFrame_column);
    dummy_var ^= ((int64_t) (void*) wire_DataFrame_column_at);
    dummy_var ^= ((int64_t) (void*) wire_DataFrame_columns);
    dummy_var ^= ((int64_t) (void*) wire_DataFrame_describe);
    dummy_var ^= ((int64_t) (void*) wire_DataFrame_drop);
    dummy_var ^= ((int64_t) (void*) wire_DataFrame_drop_in_place);
    dummy_var ^= ((int64_t) (void*) wire_DataFrame_dtypes);
    dummy_var ^= ((int64_t) (void*) wire_DataFrame_dump);
    dummy_var ^= ((int64_t) (void*) wire_DataFrame_estimated_size);
    dummy_var ^= ((int64_t) (void*) wire_DataFrame_get_column_names);
    dummy_var ^= ((int64_t) (void*) wire_DataFrame_get_columns);
    dummy_var ^= ((int64_t) (void*) wire_DataFrame_get_row);
    dummy_var ^= ((int64_t) (void*) wire_DataFrame_head);
    dummy_var ^= ((int64_t) (void*) wire_DataFrame_height);
    dummy_var ^= ((int64_t) (void*) wire_DataFrame_is_empty);
    dummy_var ^= ((int64_t) (void*) wire_DataFrame_lazy);
    dummy_var ^= ((int64_t) (void*) wire_DataFrame_max);
    dummy_var ^= ((int64_t) (void*) wire_DataFrame_of);
    dummy_var ^= ((int64_t) (void*) wire_DataFrame_reverse);
    dummy_var ^= ((int64_t) (void*) wire_DataFrame_sample);
    dummy_var ^= ((int64_t) (void*) wire_DataFrame_schema);
    dummy_var ^= ((int64_t) (void*) wire_DataFrame_select);
    dummy_var ^= ((int64_t) (void*) wire_DataFrame_shape);
    dummy_var ^= ((int64_t) (void*) wire_DataFrame_sort_in_place);
    dummy_var ^= ((int64_t) (void*) wire_DataFrame_tail);
    dummy_var ^= ((int64_t) (void*) wire_DataFrame_width);
    dummy_var ^= ((int64_t) (void*) wire_DataFrame_with_row_count);
    dummy_var ^= ((int64_t) (void*) wire_LazyFrame_cache);
    dummy_var ^= ((int64_t) (void*) wire_LazyFrame_collect);
    dummy_var ^= ((int64_t) (void*) wire_LazyFrame_cross_join);
    dummy_var ^= ((int64_t) (void*) wire_LazyFrame_drop_nulls);
    dummy_var ^= ((int64_t) (void*) wire_LazyFrame_explode);
    dummy_var ^= ((int64_t) (void*) wire_LazyFrame_fetch);
    dummy_var ^= ((int64_t) (void*) wire_LazyFrame_filter);
    dummy_var ^= ((int64_t) (void*) wire_LazyFrame_first);
    dummy_var ^= ((int64_t) (void*) wire_LazyFrame_group_by);
    dummy_var ^= ((int64_t) (void*) wire_LazyFrame_inner_join);
    dummy_var ^= ((int64_t) (void*) wire_LazyFrame_join);
    dummy_var ^= ((int64_t) (void*) wire_LazyFrame_last);
    dummy_var ^= ((int64_t) (void*) wire_LazyFrame_left_join);
    dummy_var ^= ((int64_t) (void*) wire_LazyFrame_limit);
    dummy_var ^= ((int64_t) (void*) wire_LazyFrame_max);
    dummy_var ^= ((int64_t) (void*) wire_LazyFrame_mean);
    dummy_var ^= ((int64_t) (void*) wire_LazyFrame_median);
    dummy_var ^= ((int64_t) (void*) wire_LazyFrame_melt);
    dummy_var ^= ((int64_t) (void*) wire_LazyFrame_min);
    dummy_var ^= ((int64_t) (void*) wire_LazyFrame_outer_join);
    dummy_var ^= ((int64_t) (void*) wire_LazyFrame_quantile);
    dummy_var ^= ((int64_t) (void*) wire_LazyFrame_reverse);
    dummy_var ^= ((int64_t) (void*) wire_LazyFrame_slice);
    dummy_var ^= ((int64_t) (void*) wire_LazyFrame_std);
    dummy_var ^= ((int64_t) (void*) wire_LazyFrame_sum);
    dummy_var ^= ((int64_t) (void*) wire_LazyFrame_tail);
    dummy_var ^= ((int64_t) (void*) wire_LazyFrame_unique);
    dummy_var ^= ((int64_t) (void*) wire_LazyFrame_variance);
    dummy_var ^= ((int64_t) (void*) wire_LazyFrame_with_column);
    dummy_var ^= ((int64_t) (void*) wire_LazyFrame_with_row_count);
    dummy_var ^= ((int64_t) (void*) wire_LazyGroupBy_head);
    dummy_var ^= ((int64_t) (void*) wire_LazyGroupBy_tail);
    dummy_var ^= ((int64_t) (void*) wire_Schema_of);
    dummy_var ^= ((int64_t) (void*) wire_Series_add_to);
    dummy_var ^= ((int64_t) (void*) wire_Series_append);
    dummy_var ^= ((int64_t) (void*) wire_Series_apply_scalar);
    dummy_var ^= ((int64_t) (void*) wire_Series_as_doubles);
    dummy_var ^= ((int64_t) (void*) wire_Series_as_durations);
    dummy_var ^= ((int64_t) (void*) wire_Series_as_ints);
    dummy_var ^= ((int64_t) (void*) wire_Series_as_local_datetime);
    dummy_var ^= ((int64_t) (void*) wire_Series_as_naive_datetime);
    dummy_var ^= ((int64_t) (void*) wire_Series_as_strings);
    dummy_var ^= ((int64_t) (void*) wire_Series_as_utc_datetime);
    dummy_var ^= ((int64_t) (void*) wire_Series_cast);
    dummy_var ^= ((int64_t) (void*) wire_Series_divide);
    dummy_var ^= ((int64_t) (void*) wire_Series_dump);
    dummy_var ^= ((int64_t) (void*) wire_Series_equal);
    dummy_var ^= ((int64_t) (void*) wire_Series_estimated_size);
    dummy_var ^= ((int64_t) (void*) wire_Series_explode);
    dummy_var ^= ((int64_t) (void*) wire_Series_explode_by_offsets);
    dummy_var ^= ((int64_t) (void*) wire_Series_get);
    dummy_var ^= ((int64_t) (void*) wire_Series_get_string);
    dummy_var ^= ((int64_t) (void*) wire_Series_head);
    dummy_var ^= ((int64_t) (void*) wire_Series_into_frame);
    dummy_var ^= ((int64_t) (void*) wire_Series_is_bool);
    dummy_var ^= ((int64_t) (void*) wire_Series_is_numeric);
    dummy_var ^= ((int64_t) (void*) wire_Series_is_temporal);
    dummy_var ^= ((int64_t) (void*) wire_Series_is_utf8);
    dummy_var ^= ((int64_t) (void*) wire_Series_max);
    dummy_var ^= ((int64_t) (void*) wire_Series_mean);
    dummy_var ^= ((int64_t) (void*) wire_Series_mean_as_series);
    dummy_var ^= ((int64_t) (void*) wire_Series_median);
    dummy_var ^= ((int64_t) (void*) wire_Series_median_as_series);
    dummy_var ^= ((int64_t) (void*) wire_Series_min);
    dummy_var ^= ((int64_t) (void*) wire_Series_multiply);
    dummy_var ^= ((int64_t) (void*) wire_Series_of_bools);
    dummy_var ^= ((int64_t) (void*) wire_Series_of_doubles);
    dummy_var ^= ((int64_t) (void*) wire_Series_of_durations);
    dummy_var ^= ((int64_t) (void*) wire_Series_of_i32);
    dummy_var ^= ((int64_t) (void*) wire_Series_of_ints);
    dummy_var ^= ((int64_t) (void*) wire_Series_of_strings);
    dummy_var ^= ((int64_t) (void*) wire_Series_product);
    dummy_var ^= ((int64_t) (void*) wire_Series_remainder);
    dummy_var ^= ((int64_t) (void*) wire_Series_rename);
    dummy_var ^= ((int64_t) (void*) wire_Series_reshape);
    dummy_var ^= ((int64_t) (void*) wire_Series_shuffle);
    dummy_var ^= ((int64_t) (void*) wire_Series_sort);
    dummy_var ^= ((int64_t) (void*) wire_Series_std_as_series);
    dummy_var ^= ((int64_t) (void*) wire_Series_subtract);
    dummy_var ^= ((int64_t) (void*) wire_Series_sum);
    dummy_var ^= ((int64_t) (void*) wire_Series_sum_as_series);
    dummy_var ^= ((int64_t) (void*) wire_Series_tail);
    dummy_var ^= ((int64_t) (void*) wire_Series_to_list);
    dummy_var ^= ((int64_t) (void*) wire_Series_unique);
    dummy_var ^= ((int64_t) (void*) wire_Series_var_as_series);
    dummy_var ^= ((int64_t) (void*) wire_read_csv);
    dummy_var ^= ((int64_t) (void*) wire_read_json);
    dummy_var ^= ((int64_t) (void*) wire_scan_csv);
    return dummy_var;
}
