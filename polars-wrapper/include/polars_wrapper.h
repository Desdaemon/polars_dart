#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
typedef struct _Dart_Handle* Dart_Handle;

typedef struct DartCObject DartCObject;

typedef int64_t DartPort;

typedef bool (*DartPostCObjectFnType)(DartPort port_id, void *message);

typedef struct wire_uint_8_list {
  uint8_t *ptr;
  int32_t len;
} wire_uint_8_list;

typedef struct wire_RwLockPSchema {
  const void *ptr;
} wire_RwLockPSchema;

typedef struct wire_Schema {
  struct wire_RwLockPSchema field0;
} wire_Schema;

typedef struct wire_DataType_Boolean {

} wire_DataType_Boolean;

typedef struct wire_DataType_UInt8 {

} wire_DataType_UInt8;

typedef struct wire_DataType_UInt16 {

} wire_DataType_UInt16;

typedef struct wire_DataType_UInt32 {

} wire_DataType_UInt32;

typedef struct wire_DataType_UInt64 {

} wire_DataType_UInt64;

typedef struct wire_DataType_Int8 {

} wire_DataType_Int8;

typedef struct wire_DataType_Int16 {

} wire_DataType_Int16;

typedef struct wire_DataType_Int32 {

} wire_DataType_Int32;

typedef struct wire_DataType_Int64 {

} wire_DataType_Int64;

typedef struct wire_DataType_Float32 {

} wire_DataType_Float32;

typedef struct wire_DataType_Float64 {

} wire_DataType_Float64;

typedef struct wire_DataType_Utf8 {

} wire_DataType_Utf8;

typedef struct wire_DataType_Binary {

} wire_DataType_Binary;

typedef struct wire_DataType_Date {

} wire_DataType_Date;

typedef struct wire_DataType_Datetime {
  int32_t field0;
  struct wire_uint_8_list *field1;
} wire_DataType_Datetime;

typedef struct wire_DataType_Duration {
  int32_t field0;
} wire_DataType_Duration;

typedef struct wire_DataType_Time {

} wire_DataType_Time;

typedef struct wire_DataType_List {
  struct wire_DataType *field0;
} wire_DataType_List;

typedef struct wire_Field {
  struct wire_uint_8_list *name;
  struct wire_DataType dtype;
} wire_Field;

typedef struct wire_list_field {
  struct wire_Field *ptr;
  int32_t len;
} wire_list_field;

typedef struct wire_DataType_Struct {
  struct wire_list_field *field0;
} wire_DataType_Struct;

typedef struct wire_DataType_Unknown {

} wire_DataType_Unknown;

typedef union DataTypeKind {
  struct wire_DataType_Boolean *Boolean;
  struct wire_DataType_UInt8 *UInt8;
  struct wire_DataType_UInt16 *UInt16;
  struct wire_DataType_UInt32 *UInt32;
  struct wire_DataType_UInt64 *UInt64;
  struct wire_DataType_Int8 *Int8;
  struct wire_DataType_Int16 *Int16;
  struct wire_DataType_Int32 *Int32;
  struct wire_DataType_Int64 *Int64;
  struct wire_DataType_Float32 *Float32;
  struct wire_DataType_Float64 *Float64;
  struct wire_DataType_Utf8 *Utf8;
  struct wire_DataType_Binary *Binary;
  struct wire_DataType_Date *Date;
  struct wire_DataType_Datetime *Datetime;
  struct wire_DataType_Duration *Duration;
  struct wire_DataType_Time *Time;
  struct wire_DataType_List *List;
  struct wire_DataType_Struct *Struct;
  struct wire_DataType_Unknown *Unknown;
} DataTypeKind;

typedef struct wire_DataType {
  int32_t tag;
  union DataTypeKind *kind;
} wire_DataType;

typedef struct wire_list_data_type {
  struct wire_DataType *ptr;
  int32_t len;
} wire_list_data_type;

typedef struct wire_StringList {
  struct wire_uint_8_list **ptr;
  int32_t len;
} wire_StringList;

typedef struct wire_RowCount {
  struct wire_uint_8_list *name;
  uint32_t offset;
} wire_RowCount;

typedef struct wire_NullValues_AllColumnsSingle {
  struct wire_uint_8_list *field0;
} wire_NullValues_AllColumnsSingle;

typedef struct wire_NullValues_AllColumns {
  struct wire_StringList *field0;
} wire_NullValues_AllColumns;

typedef union NullValuesKind {
  struct wire_NullValues_AllColumnsSingle *AllColumnsSingle;
  struct wire_NullValues_AllColumns *AllColumns;
} NullValuesKind;

typedef struct wire_NullValues {
  int32_t tag;
  union NullValuesKind *kind;
} wire_NullValues;

typedef struct wire_uint_32_list {
  uint32_t *ptr;
  int32_t len;
} wire_uint_32_list;

typedef struct DartCObject *WireSyncReturn;

typedef struct wire_RwLockPSeries {
  const void *ptr;
} wire_RwLockPSeries;

typedef struct wire_Series {
  struct wire_RwLockPSeries field0;
} wire_Series;

typedef struct wire_list_series {
  struct wire_Series *ptr;
  int32_t len;
} wire_list_series;

typedef struct wire_RwLockPDataFrame {
  const void *ptr;
} wire_RwLockPDataFrame;

typedef struct wire_DataFrame {
  struct wire_RwLockPDataFrame field0;
} wire_DataFrame;

typedef struct wire_float_64_list {
  double *ptr;
  int32_t len;
} wire_float_64_list;

typedef struct wire_RwLockPLazyFrame {
  const void *ptr;
} wire_RwLockPLazyFrame;

typedef struct wire_LazyFrame {
  struct wire_RwLockPLazyFrame field0;
} wire_LazyFrame;

typedef struct wire_Expr_Alias {
  struct wire_Expr *field0;
  struct wire_uint_8_list *field1;
} wire_Expr_Alias;

typedef struct wire_Expr_Column {
  struct wire_uint_8_list *field0;
} wire_Expr_Column;

typedef struct wire_Expr_Columns {
  struct wire_StringList *field0;
} wire_Expr_Columns;

typedef struct wire_Expr_DtypeColumn {
  struct wire_list_data_type *field0;
} wire_Expr_DtypeColumn;

typedef struct wire_LiteralValue_Boolean {
  bool field0;
} wire_LiteralValue_Boolean;

typedef struct wire_LiteralValue_Utf8 {
  struct wire_uint_8_list *field0;
} wire_LiteralValue_Utf8;

typedef struct wire_LiteralValue_Binary {
  struct wire_uint_8_list *field0;
} wire_LiteralValue_Binary;

typedef struct wire_LiteralValue_UInt8 {
  uint8_t field0;
} wire_LiteralValue_UInt8;

typedef struct wire_LiteralValue_UInt16 {
  uint16_t field0;
} wire_LiteralValue_UInt16;

typedef struct wire_LiteralValue_UInt32 {
  uint32_t field0;
} wire_LiteralValue_UInt32;

typedef struct wire_LiteralValue_UInt64 {
  uint64_t field0;
} wire_LiteralValue_UInt64;

typedef struct wire_LiteralValue_Int8 {
  int8_t field0;
} wire_LiteralValue_Int8;

typedef struct wire_LiteralValue_Int16 {
  int16_t field0;
} wire_LiteralValue_Int16;

typedef struct wire_LiteralValue_Int32 {
  int32_t field0;
} wire_LiteralValue_Int32;

typedef struct wire_LiteralValue_Int64 {
  int64_t field0;
} wire_LiteralValue_Int64;

typedef struct wire_LiteralValue_Float32 {
  float field0;
} wire_LiteralValue_Float32;

typedef struct wire_LiteralValue_Float64 {
  double field0;
} wire_LiteralValue_Float64;

typedef struct wire_LiteralValue_Range {
  int64_t low;
  int64_t high;
  struct wire_DataType *data_type;
} wire_LiteralValue_Range;

typedef struct wire_LiteralValue_DateTime {
  int64_t field0;
  int32_t field1;
} wire_LiteralValue_DateTime;

typedef struct wire_LiteralValue_Duration {
  int64_t field0;
  int32_t field1;
} wire_LiteralValue_Duration;

typedef union LiteralValueKind {
  struct wire_LiteralValue_Boolean *Boolean;
  struct wire_LiteralValue_Utf8 *Utf8;
  struct wire_LiteralValue_Binary *Binary;
  struct wire_LiteralValue_UInt8 *UInt8;
  struct wire_LiteralValue_UInt16 *UInt16;
  struct wire_LiteralValue_UInt32 *UInt32;
  struct wire_LiteralValue_UInt64 *UInt64;
  struct wire_LiteralValue_Int8 *Int8;
  struct wire_LiteralValue_Int16 *Int16;
  struct wire_LiteralValue_Int32 *Int32;
  struct wire_LiteralValue_Int64 *Int64;
  struct wire_LiteralValue_Float32 *Float32;
  struct wire_LiteralValue_Float64 *Float64;
  struct wire_LiteralValue_Range *Range;
  struct wire_LiteralValue_DateTime *DateTime;
  struct wire_LiteralValue_Duration *Duration;
} LiteralValueKind;

typedef struct wire_LiteralValue {
  int32_t tag;
  union LiteralValueKind *kind;
} wire_LiteralValue;

typedef struct wire_Expr_Literal {
  struct wire_LiteralValue *field0;
} wire_Expr_Literal;

typedef struct wire_Expr_BinaryExpr {
  struct wire_Expr *left;
  int32_t op;
  struct wire_Expr *right;
} wire_Expr_BinaryExpr;

typedef struct wire_Expr_Cast {
  struct wire_Expr *expr;
  struct wire_DataType *data_type;
  bool strict;
} wire_Expr_Cast;

typedef struct wire_SortOptions {
  bool descending;
  bool nulls_last;
} wire_SortOptions;

typedef struct wire_Expr_Sort {
  struct wire_Expr *expr;
  struct wire_SortOptions *options;
} wire_Expr_Sort;

typedef struct wire_Expr_Take {
  struct wire_Expr *expr;
  struct wire_Expr *idx;
} wire_Expr_Take;

typedef struct wire_AggExpr_Min {
  struct wire_Expr *input;
  bool propagate_nans;
} wire_AggExpr_Min;

typedef struct wire_AggExpr_Max {
  struct wire_Expr *input;
  bool propagate_nans;
} wire_AggExpr_Max;

typedef struct wire_AggExpr_Median {
  struct wire_Expr *field0;
} wire_AggExpr_Median;

typedef struct wire_AggExpr_NUnique {
  struct wire_Expr *field0;
} wire_AggExpr_NUnique;

typedef struct wire_AggExpr_First {
  struct wire_Expr *field0;
} wire_AggExpr_First;

typedef struct wire_AggExpr_Last {
  struct wire_Expr *field0;
} wire_AggExpr_Last;

typedef struct wire_AggExpr_Mean {
  struct wire_Expr *field0;
} wire_AggExpr_Mean;

typedef struct wire_AggExpr_List {
  struct wire_Expr *field0;
} wire_AggExpr_List;

typedef struct wire_AggExpr_Count {
  struct wire_Expr *field0;
} wire_AggExpr_Count;

typedef struct wire_AggExpr_Quantile {
  struct wire_Expr *expr;
  struct wire_Expr *quantile;
  int32_t interpol;
} wire_AggExpr_Quantile;

typedef struct wire_AggExpr_Sum {
  struct wire_Expr *field0;
} wire_AggExpr_Sum;

typedef struct wire_AggExpr_AggGroups {
  struct wire_Expr *field0;
} wire_AggExpr_AggGroups;

typedef struct wire_AggExpr_Std {
  struct wire_Expr *field0;
  uint8_t field1;
} wire_AggExpr_Std;

typedef union AggExprKind {
  struct wire_AggExpr_Min *Min;
  struct wire_AggExpr_Max *Max;
  struct wire_AggExpr_Median *Median;
  struct wire_AggExpr_NUnique *NUnique;
  struct wire_AggExpr_First *First;
  struct wire_AggExpr_Last *Last;
  struct wire_AggExpr_Mean *Mean;
  struct wire_AggExpr_List *List;
  struct wire_AggExpr_Count *Count;
  struct wire_AggExpr_Quantile *Quantile;
  struct wire_AggExpr_Sum *Sum;
  struct wire_AggExpr_AggGroups *AggGroups;
  struct wire_AggExpr_Std *Std;
} AggExprKind;

typedef struct wire_AggExpr {
  int32_t tag;
  union AggExprKind *kind;
} wire_AggExpr;

typedef struct wire_Expr_Agg {
  struct wire_AggExpr *field0;
} wire_Expr_Agg;

typedef struct wire_Expr_Ternary {
  struct wire_Expr *predicate;
  struct wire_Expr *truthy;
  struct wire_Expr *falsy;
} wire_Expr_Ternary;

typedef struct wire_Expr_Explode {
  struct wire_Expr *field0;
} wire_Expr_Explode;

typedef struct wire_Expr_Filter {
  struct wire_Expr *input;
  struct wire_Expr *by;
} wire_Expr_Filter;

typedef struct wire_Expr_Wildcard {

} wire_Expr_Wildcard;

typedef struct wire_Expr_Slice {
  struct wire_Expr *input;
  struct wire_Expr *offset;
  struct wire_Expr *length;
} wire_Expr_Slice;

typedef struct wire_Excluded_Name {
  struct wire_uint_8_list *field0;
} wire_Excluded_Name;

typedef struct wire_Excluded_Dtype {
  struct wire_DataType *field0;
} wire_Excluded_Dtype;

typedef union ExcludedKind {
  struct wire_Excluded_Name *Name;
  struct wire_Excluded_Dtype *Dtype;
} ExcludedKind;

typedef struct wire_Excluded {
  int32_t tag;
  union ExcludedKind *kind;
} wire_Excluded;

typedef struct wire_list_excluded {
  struct wire_Excluded *ptr;
  int32_t len;
} wire_list_excluded;

typedef struct wire_Expr_Exclude {
  struct wire_Expr *field0;
  struct wire_list_excluded *field1;
} wire_Expr_Exclude;

typedef struct wire_Expr_KeepName {
  struct wire_Expr *field0;
} wire_Expr_KeepName;

typedef struct wire_Expr_Count {

} wire_Expr_Count;

typedef struct wire_Expr_Nth {
  int64_t field0;
} wire_Expr_Nth;

typedef union ExprKind {
  struct wire_Expr_Alias *Alias;
  struct wire_Expr_Column *Column;
  struct wire_Expr_Columns *Columns;
  struct wire_Expr_DtypeColumn *DtypeColumn;
  struct wire_Expr_Literal *Literal;
  struct wire_Expr_BinaryExpr *BinaryExpr;
  struct wire_Expr_Cast *Cast;
  struct wire_Expr_Sort *Sort;
  struct wire_Expr_Take *Take;
  struct wire_Expr_Agg *Agg;
  struct wire_Expr_Ternary *Ternary;
  struct wire_Expr_Explode *Explode;
  struct wire_Expr_Filter *Filter;
  struct wire_Expr_Wildcard *Wildcard;
  struct wire_Expr_Slice *Slice;
  struct wire_Expr_Exclude *Exclude;
  struct wire_Expr_KeepName *KeepName;
  struct wire_Expr_Count *Count;
  struct wire_Expr_Nth *Nth;
} ExprKind;

typedef struct wire_Expr {
  int32_t tag;
  union ExprKind *kind;
} wire_Expr;

typedef struct wire_list_expr {
  struct wire_Expr *ptr;
  int32_t len;
} wire_list_expr;

typedef struct wire_list_opt_String {
  struct wire_uint_8_list **ptr;
  int32_t len;
} wire_list_opt_String;

typedef struct wire_list_opt_i32 {
  int32_t **ptr;
  int32_t len;
} wire_list_opt_i32;

typedef struct wire_list_opt_i64 {
  int64_t **ptr;
  int32_t len;
} wire_list_opt_i64;

typedef struct wire_list_opt_Chrono_Duration {
  int64_t **ptr;
  int32_t len;
} wire_list_opt_Chrono_Duration;

typedef struct wire_list_opt_f64 {
  double **ptr;
  int32_t len;
} wire_list_opt_f64;

typedef struct wire_int_64_list {
  int64_t *ptr;
  int32_t len;
} wire_int_64_list;

typedef struct wire_RwLockPLazyGroupBy {
  const void *ptr;
} wire_RwLockPLazyGroupBy;

typedef struct wire_LazyGroupBy {
  struct wire_RwLockPLazyGroupBy field0;
} wire_LazyGroupBy;

void store_dart_post_cobject(DartPostCObjectFnType ptr);

Dart_Handle get_dart_object(uintptr_t ptr);

void drop_dart_object(uintptr_t ptr);

uintptr_t new_dart_opaque(Dart_Handle handle);

intptr_t init_frb_dart_api_dl(void *obj);

void wire_read_csv(int64_t port_,
                   struct wire_uint_8_list *path,
                   struct wire_Schema *dtypes,
                   struct wire_list_data_type *dtypes_slice,
                   bool *has_header,
                   struct wire_StringList *columns,
                   uint32_t *delimiter,
                   uint32_t *comment_char,
                   uint32_t *eol_char,
                   uintptr_t *chunk_size,
                   uintptr_t *sample_size,
                   struct wire_RowCount *row_count,
                   int32_t *encoding,
                   uintptr_t *n_rows,
                   uintptr_t *n_threads,
                   struct wire_NullValues *null_values,
                   struct wire_uint_32_list *projection,
                   uint32_t *quote_char,
                   uintptr_t skip_rows,
                   uintptr_t skip_rows_after_header,
                   bool ignore_parser_errors,
                   bool rechunk,
                   bool parse_dates,
                   bool low_memory);

void wire_scan_csv(int64_t port_,
                   struct wire_uint_8_list *path,
                   struct wire_Schema *dtype_overwrite,
                   bool *has_header,
                   uint32_t *delimiter,
                   uint32_t *comment_char,
                   uint32_t *eol_char,
                   uint32_t *quote_char,
                   uintptr_t skip_rows,
                   uintptr_t skip_rows_after_header,
                   struct wire_RowCount *row_count,
                   int32_t *encoding,
                   uintptr_t *n_rows,
                   struct wire_NullValues *null_values,
                   bool ignore_parser_errors,
                   bool rechunk,
                   bool parse_dates,
                   uintptr_t *infer_schema_length,
                   bool cache);

void wire_read_json(int64_t port_,
                    struct wire_uint_8_list *path,
                    struct wire_Schema *schema,
                    uintptr_t *batch_size,
                    struct wire_StringList *projection);

WireSyncReturn wire_of__static_method__DataFrame(struct wire_list_series *series);

void wire_iter__method__DataFrame(int64_t port_, struct wire_DataFrame that);

WireSyncReturn wire_column__method__DataFrame(struct wire_DataFrame that,
                                              struct wire_uint_8_list *column);

WireSyncReturn wire_columns__method__DataFrame(struct wire_DataFrame that,
                                               struct wire_StringList *columns);

WireSyncReturn wire_column_at__method__DataFrame(struct wire_DataFrame that, uintptr_t index);

void wire_dump__method__DataFrame(int64_t port_, struct wire_DataFrame that);

WireSyncReturn wire_estimated_size__method__DataFrame(struct wire_DataFrame that);

void wire_with_row_count__method__DataFrame(int64_t port_,
                                            struct wire_DataFrame that,
                                            struct wire_uint_8_list *name,
                                            uint32_t *offset);

WireSyncReturn wire_get_column_names__method__DataFrame(struct wire_DataFrame that);

void wire_get_columns__method__DataFrame(int64_t port_, struct wire_DataFrame that);

WireSyncReturn wire_width__method__DataFrame(struct wire_DataFrame that);

WireSyncReturn wire_height__method__DataFrame(struct wire_DataFrame that);

WireSyncReturn wire_is_empty__method__DataFrame(struct wire_DataFrame that);

void wire_sample__method__DataFrame(int64_t port_,
                                    struct wire_DataFrame that,
                                    uintptr_t n,
                                    bool with_replacement,
                                    bool shuffle,
                                    uint64_t *seed);

WireSyncReturn wire_select__method__DataFrame(struct wire_DataFrame that,
                                              struct wire_StringList *columns);

WireSyncReturn wire_head__method__DataFrame(struct wire_DataFrame that, uintptr_t *length);

WireSyncReturn wire_tail__method__DataFrame(struct wire_DataFrame that, uintptr_t *length);

void wire_describe__method__DataFrame(int64_t port_,
                                      struct wire_DataFrame that,
                                      struct wire_float_64_list *percentiles);

WireSyncReturn wire_drop__method__DataFrame(struct wire_DataFrame that,
                                            struct wire_uint_8_list *column);

WireSyncReturn wire_drop_in_place__method__DataFrame(struct wire_DataFrame that,
                                                     struct wire_uint_8_list *column);

WireSyncReturn wire_reverse__method__DataFrame(struct wire_DataFrame that);

WireSyncReturn wire_shape__method__DataFrame(struct wire_DataFrame that);

void wire_max__method__DataFrame(int64_t port_, struct wire_DataFrame that);

void wire_get_row__method__DataFrame(int64_t port_, struct wire_DataFrame that, uintptr_t index);

WireSyncReturn wire_schema__method__DataFrame(struct wire_DataFrame that);

WireSyncReturn wire_dtypes__method__DataFrame(struct wire_DataFrame that);

WireSyncReturn wire_lazy__method__take_self__DataFrame(struct wire_DataFrame that,
                                                       bool allow_copy,
                                                       bool *projection_pushdown,
                                                       bool *predicate_pushdown,
                                                       bool *type_coercion,
                                                       bool *simplify_expressions,
                                                       bool *slice_pushdown,
                                                       bool *streaming);

WireSyncReturn wire_select__method__take_self__LazyFrame(struct wire_LazyFrame that,
                                                         struct wire_list_expr *exprs);

WireSyncReturn wire_filter__method__take_self__LazyFrame(struct wire_LazyFrame that,
                                                         struct wire_Expr pred);

WireSyncReturn wire_groupby__method__take_self__LazyFrame(struct wire_LazyFrame that,
                                                          struct wire_list_expr *exprs,
                                                          bool stable);

WireSyncReturn wire_reverse__method__take_self__LazyFrame(struct wire_LazyFrame that);

WireSyncReturn wire_with_column__method__take_self__LazyFrame(struct wire_LazyFrame that,
                                                              struct wire_Expr expr);

WireSyncReturn wire_with_columns__method__take_self__LazyFrame(struct wire_LazyFrame that,
                                                               struct wire_list_expr *expr);

WireSyncReturn wire_cache__method__take_self__LazyFrame(struct wire_LazyFrame that);

void wire_collect__method__take_self__LazyFrame(int64_t port_, struct wire_LazyFrame that);

WireSyncReturn wire_cross_join__method__take_self__LazyFrame(struct wire_LazyFrame that,
                                                             struct wire_LazyFrame other);

WireSyncReturn wire_left_join__method__take_self__LazyFrame(struct wire_LazyFrame that,
                                                            struct wire_LazyFrame other,
                                                            struct wire_Expr left_on,
                                                            struct wire_Expr right_on);

WireSyncReturn wire_outer_join__method__take_self__LazyFrame(struct wire_LazyFrame that,
                                                             struct wire_LazyFrame other,
                                                             struct wire_Expr left_on,
                                                             struct wire_Expr right_on);

WireSyncReturn wire_inner_join__method__take_self__LazyFrame(struct wire_LazyFrame that,
                                                             struct wire_LazyFrame other,
                                                             struct wire_Expr left_on,
                                                             struct wire_Expr right_on);

WireSyncReturn wire_join__method__take_self__LazyFrame(struct wire_LazyFrame that,
                                                       struct wire_LazyFrame other,
                                                       struct wire_list_expr *on,
                                                       struct wire_list_expr *left_on,
                                                       struct wire_list_expr *right_on,
                                                       struct wire_uint_8_list *suffix,
                                                       int32_t how,
                                                       bool allow_parallel,
                                                       bool force_parallel);

WireSyncReturn wire_max__method__take_self__LazyFrame(struct wire_LazyFrame that);

WireSyncReturn wire_min__method__take_self__LazyFrame(struct wire_LazyFrame that);

WireSyncReturn wire_sum__method__take_self__LazyFrame(struct wire_LazyFrame that);

WireSyncReturn wire_mean__method__take_self__LazyFrame(struct wire_LazyFrame that);

WireSyncReturn wire_median__method__take_self__LazyFrame(struct wire_LazyFrame that);

WireSyncReturn wire_quantile__method__take_self__LazyFrame(struct wire_LazyFrame that,
                                                           struct wire_Expr quantile,
                                                           int32_t interpol);

WireSyncReturn wire_std__method__take_self__LazyFrame(struct wire_LazyFrame that, uint8_t ddof);

WireSyncReturn wire_variance__method__take_self__LazyFrame(struct wire_LazyFrame that,
                                                           uint8_t ddof);

WireSyncReturn wire_explode__method__take_self__LazyFrame(struct wire_LazyFrame that,
                                                          struct wire_list_expr *columns);

WireSyncReturn wire_unique__method__take_self__LazyFrame(struct wire_LazyFrame that,
                                                         struct wire_StringList *subset,
                                                         int32_t keep_strategy);

WireSyncReturn wire_drop_nulls__method__take_self__LazyFrame(struct wire_LazyFrame that,
                                                             struct wire_list_expr *subset);

WireSyncReturn wire_slice__method__take_self__LazyFrame(struct wire_LazyFrame that,
                                                        int64_t offset,
                                                        uint32_t len);

WireSyncReturn wire_first__method__take_self__LazyFrame(struct wire_LazyFrame that);

WireSyncReturn wire_last__method__take_self__LazyFrame(struct wire_LazyFrame that);

WireSyncReturn wire_tail__method__take_self__LazyFrame(struct wire_LazyFrame that, uint32_t n);

WireSyncReturn wire_melt__method__take_self__LazyFrame(struct wire_LazyFrame that,
                                                       struct wire_StringList *id_vars,
                                                       struct wire_StringList *value_vars,
                                                       struct wire_uint_8_list *variable_name,
                                                       struct wire_uint_8_list *value_name);

WireSyncReturn wire_limit__method__take_self__LazyFrame(struct wire_LazyFrame that, uint32_t n);

void wire_fetch__method__take_self__LazyFrame(int64_t port_,
                                              struct wire_LazyFrame that,
                                              uintptr_t n_rows);

WireSyncReturn wire_with_row_count__method__take_self__LazyFrame(struct wire_LazyFrame that,
                                                                 struct wire_uint_8_list *name,
                                                                 uint32_t *offset);

WireSyncReturn wire_of_strings__factory__static_method__Series(struct wire_uint_8_list *name,
                                                               struct wire_list_opt_String *values);

WireSyncReturn wire_of_i32__factory__static_method__Series(struct wire_uint_8_list *name,
                                                           struct wire_list_opt_i32 *values);

WireSyncReturn wire_of_ints__factory__static_method__Series(struct wire_uint_8_list *name,
                                                            struct wire_list_opt_i64 *values);

WireSyncReturn wire_of_durations__factory__static_method__Series(struct wire_uint_8_list *name,
                                                                 struct wire_list_opt_Chrono_Duration *values,
                                                                 int32_t unit);

WireSyncReturn wire_of_doubles__factory__static_method__Series(struct wire_uint_8_list *name,
                                                               struct wire_list_opt_f64 *values);

void wire_append__method__Series(int64_t port_, struct wire_Series that, struct wire_Series other);

void wire_cast__method__Series(int64_t port_,
                               struct wire_Series that,
                               struct wire_DataType dtype,
                               bool strict);

void wire_as_strings__method__Series(int64_t port_, struct wire_Series that);

void wire_as_ints__method__Series(int64_t port_, struct wire_Series that, bool strict);

void wire_as_doubles__method__Series(int64_t port_, struct wire_Series that, bool strict);

void wire_as_durations__method__Series(int64_t port_, struct wire_Series that);

void wire_as_naive_datetime__method__Series(int64_t port_, struct wire_Series that);

void wire_as_utc_datetime__method__Series(int64_t port_, struct wire_Series that);

void wire_as_local_datetime__method__Series(int64_t port_, struct wire_Series that);

void wire_abs__method__Series(int64_t port_, struct wire_Series that);

void wire_sort__method__Series(int64_t port_, struct wire_Series that, bool reverse);

void wire_shuffle__method__Series(int64_t port_, struct wire_Series that, uint64_t *seed);

void wire_sum__method__Series(int64_t port_, struct wire_Series that);

void wire_sum_as_series__method__Series(int64_t port_, struct wire_Series that);

void wire_min__method__Series(int64_t port_, struct wire_Series that);

void wire_max__method__Series(int64_t port_, struct wire_Series that);

void wire_explode__method__Series(int64_t port_, struct wire_Series that);

void wire_explode_by_offsets__method__Series(int64_t port_,
                                             struct wire_Series that,
                                             struct wire_int_64_list *offsets);

void wire_cummax__method__Series(int64_t port_, struct wire_Series that, bool reverse);

void wire_cummin__method__Series(int64_t port_, struct wire_Series that, bool reverse);

void wire_cumprod__method__Series(int64_t port_, struct wire_Series that, bool reverse);

void wire_cumsum__method__Series(int64_t port_, struct wire_Series that, bool reverse);

void wire_product__method__Series(int64_t port_, struct wire_Series that);

WireSyncReturn wire_get_string__method__Series(struct wire_Series that, uintptr_t index);

WireSyncReturn wire_get__method__Series(struct wire_Series that, uintptr_t index);

WireSyncReturn wire_head__method__Series(struct wire_Series that, uintptr_t *length);

WireSyncReturn wire_tail__method__Series(struct wire_Series that, uintptr_t *length);

void wire_mean__method__Series(int64_t port_, struct wire_Series that);

void wire_median__method__Series(int64_t port_, struct wire_Series that);

void wire_mean_as_series__method__Series(int64_t port_, struct wire_Series that);

void wire_median_as_series__method__Series(int64_t port_, struct wire_Series that);

WireSyncReturn wire_estimated_size__method__Series(struct wire_Series that);

WireSyncReturn wire_add_to__method__Series(struct wire_Series that, struct wire_Series other);

WireSyncReturn wire_subtract__method__Series(struct wire_Series that, struct wire_Series other);

WireSyncReturn wire_multiply__method__Series(struct wire_Series that, struct wire_Series other);

WireSyncReturn wire_divide__method__Series(struct wire_Series that, struct wire_Series other);

WireSyncReturn wire_remainder__method__Series(struct wire_Series that, struct wire_Series other);

WireSyncReturn wire_is_bool__method__Series(struct wire_Series that);

WireSyncReturn wire_is_utf8__method__Series(struct wire_Series that);

WireSyncReturn wire_is_numeric__method__Series(struct wire_Series that);

WireSyncReturn wire_is_temporal__method__Series(struct wire_Series that);

void wire_dump__method__Series(int64_t port_, struct wire_Series that);

WireSyncReturn wire_rename__method__Series(struct wire_Series that, struct wire_uint_8_list *name);

void wire_unique__method__Series(int64_t port_, struct wire_Series that, bool stable);

void wire_equal__method__Series(int64_t port_,
                                struct wire_Series that,
                                struct wire_Series other,
                                bool ignore_null);

void wire_apply_scalar__method__Series(int64_t port_,
                                       struct wire_Series that,
                                       int32_t op,
                                       double value);

void wire_reshape__method__Series(int64_t port_,
                                  struct wire_Series that,
                                  struct wire_int_64_list *dims);

void wire_std_as_series__method__Series(int64_t port_, struct wire_Series that, uint8_t ddof);

void wire_var_as_series__method__Series(int64_t port_, struct wire_Series that, uint8_t ddof);

void wire_to_list__method__Series(int64_t port_, struct wire_Series that);

WireSyncReturn wire_into_frame__method__take_self__Series(struct wire_Series that);

void wire_iter__method__Series(int64_t port_, struct wire_Series that);

WireSyncReturn wire_agg__method__take_self__LazyGroupBy(struct wire_LazyGroupBy that,
                                                        struct wire_list_expr *exprs);

WireSyncReturn wire_head__method__take_self__LazyGroupBy(struct wire_LazyGroupBy that,
                                                         uintptr_t *n);

WireSyncReturn wire_tail__method__take_self__LazyGroupBy(struct wire_LazyGroupBy that,
                                                         uintptr_t *n);

WireSyncReturn wire_of__static_method__Schema(struct wire_list_field *fields);

struct wire_RwLockPDataFrame new_RwLockPDataFrame(void);

struct wire_RwLockPLazyFrame new_RwLockPLazyFrame(void);

struct wire_RwLockPLazyGroupBy new_RwLockPLazyGroupBy(void);

struct wire_RwLockPSchema new_RwLockPSchema(void);

struct wire_RwLockPSeries new_RwLockPSeries(void);

struct wire_StringList *new_StringList_0(int32_t len);

struct wire_AggExpr new_agg_expr_0(void);

int64_t *new_box_autoadd_Chrono_Duration_0(int64_t value);

struct wire_AggExpr *new_box_autoadd_agg_expr_0(void);

bool *new_box_autoadd_bool_0(bool value);

uint32_t *new_box_autoadd_char_0(uint32_t value);

int32_t *new_box_autoadd_csv_encoding_0(int32_t value);

struct wire_DataType *new_box_autoadd_data_type_0(void);

double *new_box_autoadd_f64_0(double value);

int32_t *new_box_autoadd_i32_0(int32_t value);

int64_t *new_box_autoadd_i64_0(int64_t value);

struct wire_LiteralValue *new_box_autoadd_literal_value_0(void);

struct wire_NullValues *new_box_autoadd_null_values_0(void);

struct wire_RowCount *new_box_autoadd_row_count_0(void);

struct wire_Schema *new_box_autoadd_schema_0(void);

struct wire_SortOptions *new_box_autoadd_sort_options_0(void);

uint32_t *new_box_autoadd_u32_0(uint32_t value);

uint64_t *new_box_autoadd_u64_0(uint64_t value);

uintptr_t *new_box_autoadd_usize_0(uintptr_t value);

struct wire_DataType *new_box_data_type_0(void);

struct wire_Expr *new_box_expr_0(void);

struct wire_DataFrame new_data_frame_0(void);

struct wire_DataType new_data_type_0(void);

struct wire_Excluded new_excluded_0(void);

struct wire_Expr new_expr_0(void);

struct wire_Field new_field_0(void);

struct wire_float_64_list *new_float_64_list_0(int32_t len);

struct wire_int_64_list *new_int_64_list_0(int32_t len);

struct wire_LazyFrame new_lazy_frame_0(void);

struct wire_LazyGroupBy new_lazy_group_by_0(void);

struct wire_list_data_type *new_list_data_type_0(int32_t len);

struct wire_list_excluded *new_list_excluded_0(int32_t len);

struct wire_list_expr *new_list_expr_0(int32_t len);

struct wire_list_field *new_list_field_0(int32_t len);

struct wire_list_opt_Chrono_Duration *new_list_opt_Chrono_Duration_0(int32_t len);

struct wire_list_opt_String *new_list_opt_String_0(int32_t len);

struct wire_list_opt_f64 *new_list_opt_f64_0(int32_t len);

struct wire_list_opt_i32 *new_list_opt_i32_0(int32_t len);

struct wire_list_opt_i64 *new_list_opt_i64_0(int32_t len);

struct wire_list_series *new_list_series_0(int32_t len);

struct wire_LiteralValue new_literal_value_0(void);

struct wire_NullValues new_null_values_0(void);

struct wire_RowCount new_row_count_0(void);

struct wire_Schema new_schema_0(void);

struct wire_Series new_series_0(void);

struct wire_SortOptions new_sort_options_0(void);

struct wire_uint_32_list *new_uint_32_list_0(int32_t len);

struct wire_uint_8_list *new_uint_8_list_0(int32_t len);

void drop_opaque_RwLockPDataFrame(const void *ptr);

const void *share_opaque_RwLockPDataFrame(const void *ptr);

void drop_opaque_RwLockPLazyFrame(const void *ptr);

const void *share_opaque_RwLockPLazyFrame(const void *ptr);

void drop_opaque_RwLockPLazyGroupBy(const void *ptr);

const void *share_opaque_RwLockPLazyGroupBy(const void *ptr);

void drop_opaque_RwLockPSchema(const void *ptr);

const void *share_opaque_RwLockPSchema(const void *ptr);

void drop_opaque_RwLockPSeries(const void *ptr);

const void *share_opaque_RwLockPSeries(const void *ptr);

union AggExprKind *inflate_AggExpr_Min(void);

union AggExprKind *inflate_AggExpr_Max(void);

union AggExprKind *inflate_AggExpr_Median(void);

union AggExprKind *inflate_AggExpr_NUnique(void);

union AggExprKind *inflate_AggExpr_First(void);

union AggExprKind *inflate_AggExpr_Last(void);

union AggExprKind *inflate_AggExpr_Mean(void);

union AggExprKind *inflate_AggExpr_List(void);

union AggExprKind *inflate_AggExpr_Count(void);

union AggExprKind *inflate_AggExpr_Quantile(void);

union AggExprKind *inflate_AggExpr_Sum(void);

union AggExprKind *inflate_AggExpr_AggGroups(void);

union AggExprKind *inflate_AggExpr_Std(void);

union DataTypeKind *inflate_DataType_Datetime(void);

union DataTypeKind *inflate_DataType_Duration(void);

union DataTypeKind *inflate_DataType_List(void);

union DataTypeKind *inflate_DataType_Struct(void);

union ExcludedKind *inflate_Excluded_Name(void);

union ExcludedKind *inflate_Excluded_Dtype(void);

union ExprKind *inflate_Expr_Alias(void);

union ExprKind *inflate_Expr_Column(void);

union ExprKind *inflate_Expr_Columns(void);

union ExprKind *inflate_Expr_DtypeColumn(void);

union ExprKind *inflate_Expr_Literal(void);

union ExprKind *inflate_Expr_BinaryExpr(void);

union ExprKind *inflate_Expr_Cast(void);

union ExprKind *inflate_Expr_Sort(void);

union ExprKind *inflate_Expr_Take(void);

union ExprKind *inflate_Expr_Agg(void);

union ExprKind *inflate_Expr_Ternary(void);

union ExprKind *inflate_Expr_Explode(void);

union ExprKind *inflate_Expr_Filter(void);

union ExprKind *inflate_Expr_Slice(void);

union ExprKind *inflate_Expr_Exclude(void);

union ExprKind *inflate_Expr_KeepName(void);

union ExprKind *inflate_Expr_Nth(void);

union LiteralValueKind *inflate_LiteralValue_Boolean(void);

union LiteralValueKind *inflate_LiteralValue_Utf8(void);

union LiteralValueKind *inflate_LiteralValue_Binary(void);

union LiteralValueKind *inflate_LiteralValue_UInt8(void);

union LiteralValueKind *inflate_LiteralValue_UInt16(void);

union LiteralValueKind *inflate_LiteralValue_UInt32(void);

union LiteralValueKind *inflate_LiteralValue_UInt64(void);

union LiteralValueKind *inflate_LiteralValue_Int8(void);

union LiteralValueKind *inflate_LiteralValue_Int16(void);

union LiteralValueKind *inflate_LiteralValue_Int32(void);

union LiteralValueKind *inflate_LiteralValue_Int64(void);

union LiteralValueKind *inflate_LiteralValue_Float32(void);

union LiteralValueKind *inflate_LiteralValue_Float64(void);

union LiteralValueKind *inflate_LiteralValue_Range(void);

union LiteralValueKind *inflate_LiteralValue_DateTime(void);

union LiteralValueKind *inflate_LiteralValue_Duration(void);

union NullValuesKind *inflate_NullValues_AllColumnsSingle(void);

union NullValuesKind *inflate_NullValues_AllColumns(void);

void free_WireSyncReturn(WireSyncReturn ptr);

static int64_t dummy_method_to_enforce_bundling(void) {
    int64_t dummy_var = 0;
    dummy_var ^= ((int64_t) (void*) wire_read_csv);
    dummy_var ^= ((int64_t) (void*) wire_scan_csv);
    dummy_var ^= ((int64_t) (void*) wire_read_json);
    dummy_var ^= ((int64_t) (void*) wire_of__static_method__DataFrame);
    dummy_var ^= ((int64_t) (void*) wire_iter__method__DataFrame);
    dummy_var ^= ((int64_t) (void*) wire_column__method__DataFrame);
    dummy_var ^= ((int64_t) (void*) wire_columns__method__DataFrame);
    dummy_var ^= ((int64_t) (void*) wire_column_at__method__DataFrame);
    dummy_var ^= ((int64_t) (void*) wire_dump__method__DataFrame);
    dummy_var ^= ((int64_t) (void*) wire_estimated_size__method__DataFrame);
    dummy_var ^= ((int64_t) (void*) wire_with_row_count__method__DataFrame);
    dummy_var ^= ((int64_t) (void*) wire_get_column_names__method__DataFrame);
    dummy_var ^= ((int64_t) (void*) wire_get_columns__method__DataFrame);
    dummy_var ^= ((int64_t) (void*) wire_width__method__DataFrame);
    dummy_var ^= ((int64_t) (void*) wire_height__method__DataFrame);
    dummy_var ^= ((int64_t) (void*) wire_is_empty__method__DataFrame);
    dummy_var ^= ((int64_t) (void*) wire_sample__method__DataFrame);
    dummy_var ^= ((int64_t) (void*) wire_select__method__DataFrame);
    dummy_var ^= ((int64_t) (void*) wire_head__method__DataFrame);
    dummy_var ^= ((int64_t) (void*) wire_tail__method__DataFrame);
    dummy_var ^= ((int64_t) (void*) wire_describe__method__DataFrame);
    dummy_var ^= ((int64_t) (void*) wire_drop__method__DataFrame);
    dummy_var ^= ((int64_t) (void*) wire_drop_in_place__method__DataFrame);
    dummy_var ^= ((int64_t) (void*) wire_reverse__method__DataFrame);
    dummy_var ^= ((int64_t) (void*) wire_shape__method__DataFrame);
    dummy_var ^= ((int64_t) (void*) wire_max__method__DataFrame);
    dummy_var ^= ((int64_t) (void*) wire_get_row__method__DataFrame);
    dummy_var ^= ((int64_t) (void*) wire_schema__method__DataFrame);
    dummy_var ^= ((int64_t) (void*) wire_dtypes__method__DataFrame);
    dummy_var ^= ((int64_t) (void*) wire_lazy__method__take_self__DataFrame);
    dummy_var ^= ((int64_t) (void*) wire_select__method__take_self__LazyFrame);
    dummy_var ^= ((int64_t) (void*) wire_filter__method__take_self__LazyFrame);
    dummy_var ^= ((int64_t) (void*) wire_groupby__method__take_self__LazyFrame);
    dummy_var ^= ((int64_t) (void*) wire_reverse__method__take_self__LazyFrame);
    dummy_var ^= ((int64_t) (void*) wire_with_column__method__take_self__LazyFrame);
    dummy_var ^= ((int64_t) (void*) wire_with_columns__method__take_self__LazyFrame);
    dummy_var ^= ((int64_t) (void*) wire_cache__method__take_self__LazyFrame);
    dummy_var ^= ((int64_t) (void*) wire_collect__method__take_self__LazyFrame);
    dummy_var ^= ((int64_t) (void*) wire_cross_join__method__take_self__LazyFrame);
    dummy_var ^= ((int64_t) (void*) wire_left_join__method__take_self__LazyFrame);
    dummy_var ^= ((int64_t) (void*) wire_outer_join__method__take_self__LazyFrame);
    dummy_var ^= ((int64_t) (void*) wire_inner_join__method__take_self__LazyFrame);
    dummy_var ^= ((int64_t) (void*) wire_join__method__take_self__LazyFrame);
    dummy_var ^= ((int64_t) (void*) wire_max__method__take_self__LazyFrame);
    dummy_var ^= ((int64_t) (void*) wire_min__method__take_self__LazyFrame);
    dummy_var ^= ((int64_t) (void*) wire_sum__method__take_self__LazyFrame);
    dummy_var ^= ((int64_t) (void*) wire_mean__method__take_self__LazyFrame);
    dummy_var ^= ((int64_t) (void*) wire_median__method__take_self__LazyFrame);
    dummy_var ^= ((int64_t) (void*) wire_quantile__method__take_self__LazyFrame);
    dummy_var ^= ((int64_t) (void*) wire_std__method__take_self__LazyFrame);
    dummy_var ^= ((int64_t) (void*) wire_variance__method__take_self__LazyFrame);
    dummy_var ^= ((int64_t) (void*) wire_explode__method__take_self__LazyFrame);
    dummy_var ^= ((int64_t) (void*) wire_unique__method__take_self__LazyFrame);
    dummy_var ^= ((int64_t) (void*) wire_drop_nulls__method__take_self__LazyFrame);
    dummy_var ^= ((int64_t) (void*) wire_slice__method__take_self__LazyFrame);
    dummy_var ^= ((int64_t) (void*) wire_first__method__take_self__LazyFrame);
    dummy_var ^= ((int64_t) (void*) wire_last__method__take_self__LazyFrame);
    dummy_var ^= ((int64_t) (void*) wire_tail__method__take_self__LazyFrame);
    dummy_var ^= ((int64_t) (void*) wire_melt__method__take_self__LazyFrame);
    dummy_var ^= ((int64_t) (void*) wire_limit__method__take_self__LazyFrame);
    dummy_var ^= ((int64_t) (void*) wire_fetch__method__take_self__LazyFrame);
    dummy_var ^= ((int64_t) (void*) wire_with_row_count__method__take_self__LazyFrame);
    dummy_var ^= ((int64_t) (void*) wire_of_strings__factory__static_method__Series);
    dummy_var ^= ((int64_t) (void*) wire_of_i32__factory__static_method__Series);
    dummy_var ^= ((int64_t) (void*) wire_of_ints__factory__static_method__Series);
    dummy_var ^= ((int64_t) (void*) wire_of_durations__factory__static_method__Series);
    dummy_var ^= ((int64_t) (void*) wire_of_doubles__factory__static_method__Series);
    dummy_var ^= ((int64_t) (void*) wire_append__method__Series);
    dummy_var ^= ((int64_t) (void*) wire_cast__method__Series);
    dummy_var ^= ((int64_t) (void*) wire_as_strings__method__Series);
    dummy_var ^= ((int64_t) (void*) wire_as_ints__method__Series);
    dummy_var ^= ((int64_t) (void*) wire_as_doubles__method__Series);
    dummy_var ^= ((int64_t) (void*) wire_as_durations__method__Series);
    dummy_var ^= ((int64_t) (void*) wire_as_naive_datetime__method__Series);
    dummy_var ^= ((int64_t) (void*) wire_as_utc_datetime__method__Series);
    dummy_var ^= ((int64_t) (void*) wire_as_local_datetime__method__Series);
    dummy_var ^= ((int64_t) (void*) wire_abs__method__Series);
    dummy_var ^= ((int64_t) (void*) wire_sort__method__Series);
    dummy_var ^= ((int64_t) (void*) wire_shuffle__method__Series);
    dummy_var ^= ((int64_t) (void*) wire_sum__method__Series);
    dummy_var ^= ((int64_t) (void*) wire_sum_as_series__method__Series);
    dummy_var ^= ((int64_t) (void*) wire_min__method__Series);
    dummy_var ^= ((int64_t) (void*) wire_max__method__Series);
    dummy_var ^= ((int64_t) (void*) wire_explode__method__Series);
    dummy_var ^= ((int64_t) (void*) wire_explode_by_offsets__method__Series);
    dummy_var ^= ((int64_t) (void*) wire_cummax__method__Series);
    dummy_var ^= ((int64_t) (void*) wire_cummin__method__Series);
    dummy_var ^= ((int64_t) (void*) wire_cumprod__method__Series);
    dummy_var ^= ((int64_t) (void*) wire_cumsum__method__Series);
    dummy_var ^= ((int64_t) (void*) wire_product__method__Series);
    dummy_var ^= ((int64_t) (void*) wire_get_string__method__Series);
    dummy_var ^= ((int64_t) (void*) wire_get__method__Series);
    dummy_var ^= ((int64_t) (void*) wire_head__method__Series);
    dummy_var ^= ((int64_t) (void*) wire_tail__method__Series);
    dummy_var ^= ((int64_t) (void*) wire_mean__method__Series);
    dummy_var ^= ((int64_t) (void*) wire_median__method__Series);
    dummy_var ^= ((int64_t) (void*) wire_mean_as_series__method__Series);
    dummy_var ^= ((int64_t) (void*) wire_median_as_series__method__Series);
    dummy_var ^= ((int64_t) (void*) wire_estimated_size__method__Series);
    dummy_var ^= ((int64_t) (void*) wire_add_to__method__Series);
    dummy_var ^= ((int64_t) (void*) wire_subtract__method__Series);
    dummy_var ^= ((int64_t) (void*) wire_multiply__method__Series);
    dummy_var ^= ((int64_t) (void*) wire_divide__method__Series);
    dummy_var ^= ((int64_t) (void*) wire_remainder__method__Series);
    dummy_var ^= ((int64_t) (void*) wire_is_bool__method__Series);
    dummy_var ^= ((int64_t) (void*) wire_is_utf8__method__Series);
    dummy_var ^= ((int64_t) (void*) wire_is_numeric__method__Series);
    dummy_var ^= ((int64_t) (void*) wire_is_temporal__method__Series);
    dummy_var ^= ((int64_t) (void*) wire_dump__method__Series);
    dummy_var ^= ((int64_t) (void*) wire_rename__method__Series);
    dummy_var ^= ((int64_t) (void*) wire_unique__method__Series);
    dummy_var ^= ((int64_t) (void*) wire_equal__method__Series);
    dummy_var ^= ((int64_t) (void*) wire_apply_scalar__method__Series);
    dummy_var ^= ((int64_t) (void*) wire_reshape__method__Series);
    dummy_var ^= ((int64_t) (void*) wire_std_as_series__method__Series);
    dummy_var ^= ((int64_t) (void*) wire_var_as_series__method__Series);
    dummy_var ^= ((int64_t) (void*) wire_to_list__method__Series);
    dummy_var ^= ((int64_t) (void*) wire_into_frame__method__take_self__Series);
    dummy_var ^= ((int64_t) (void*) wire_iter__method__Series);
    dummy_var ^= ((int64_t) (void*) wire_agg__method__take_self__LazyGroupBy);
    dummy_var ^= ((int64_t) (void*) wire_head__method__take_self__LazyGroupBy);
    dummy_var ^= ((int64_t) (void*) wire_tail__method__take_self__LazyGroupBy);
    dummy_var ^= ((int64_t) (void*) wire_of__static_method__Schema);
    dummy_var ^= ((int64_t) (void*) new_RwLockPDataFrame);
    dummy_var ^= ((int64_t) (void*) new_RwLockPLazyFrame);
    dummy_var ^= ((int64_t) (void*) new_RwLockPLazyGroupBy);
    dummy_var ^= ((int64_t) (void*) new_RwLockPSchema);
    dummy_var ^= ((int64_t) (void*) new_RwLockPSeries);
    dummy_var ^= ((int64_t) (void*) new_StringList_0);
    dummy_var ^= ((int64_t) (void*) new_agg_expr_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_Chrono_Duration_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_agg_expr_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_bool_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_char_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_csv_encoding_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_data_type_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_f64_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_i32_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_i64_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_literal_value_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_null_values_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_row_count_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_schema_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_sort_options_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_u32_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_u64_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_usize_0);
    dummy_var ^= ((int64_t) (void*) new_box_data_type_0);
    dummy_var ^= ((int64_t) (void*) new_box_expr_0);
    dummy_var ^= ((int64_t) (void*) new_data_frame_0);
    dummy_var ^= ((int64_t) (void*) new_data_type_0);
    dummy_var ^= ((int64_t) (void*) new_excluded_0);
    dummy_var ^= ((int64_t) (void*) new_expr_0);
    dummy_var ^= ((int64_t) (void*) new_field_0);
    dummy_var ^= ((int64_t) (void*) new_float_64_list_0);
    dummy_var ^= ((int64_t) (void*) new_int_64_list_0);
    dummy_var ^= ((int64_t) (void*) new_lazy_frame_0);
    dummy_var ^= ((int64_t) (void*) new_lazy_group_by_0);
    dummy_var ^= ((int64_t) (void*) new_list_data_type_0);
    dummy_var ^= ((int64_t) (void*) new_list_excluded_0);
    dummy_var ^= ((int64_t) (void*) new_list_expr_0);
    dummy_var ^= ((int64_t) (void*) new_list_field_0);
    dummy_var ^= ((int64_t) (void*) new_list_opt_Chrono_Duration_0);
    dummy_var ^= ((int64_t) (void*) new_list_opt_String_0);
    dummy_var ^= ((int64_t) (void*) new_list_opt_f64_0);
    dummy_var ^= ((int64_t) (void*) new_list_opt_i32_0);
    dummy_var ^= ((int64_t) (void*) new_list_opt_i64_0);
    dummy_var ^= ((int64_t) (void*) new_list_series_0);
    dummy_var ^= ((int64_t) (void*) new_literal_value_0);
    dummy_var ^= ((int64_t) (void*) new_null_values_0);
    dummy_var ^= ((int64_t) (void*) new_row_count_0);
    dummy_var ^= ((int64_t) (void*) new_schema_0);
    dummy_var ^= ((int64_t) (void*) new_series_0);
    dummy_var ^= ((int64_t) (void*) new_sort_options_0);
    dummy_var ^= ((int64_t) (void*) new_uint_32_list_0);
    dummy_var ^= ((int64_t) (void*) new_uint_8_list_0);
    dummy_var ^= ((int64_t) (void*) drop_opaque_RwLockPDataFrame);
    dummy_var ^= ((int64_t) (void*) share_opaque_RwLockPDataFrame);
    dummy_var ^= ((int64_t) (void*) drop_opaque_RwLockPLazyFrame);
    dummy_var ^= ((int64_t) (void*) share_opaque_RwLockPLazyFrame);
    dummy_var ^= ((int64_t) (void*) drop_opaque_RwLockPLazyGroupBy);
    dummy_var ^= ((int64_t) (void*) share_opaque_RwLockPLazyGroupBy);
    dummy_var ^= ((int64_t) (void*) drop_opaque_RwLockPSchema);
    dummy_var ^= ((int64_t) (void*) share_opaque_RwLockPSchema);
    dummy_var ^= ((int64_t) (void*) drop_opaque_RwLockPSeries);
    dummy_var ^= ((int64_t) (void*) share_opaque_RwLockPSeries);
    dummy_var ^= ((int64_t) (void*) inflate_AggExpr_Min);
    dummy_var ^= ((int64_t) (void*) inflate_AggExpr_Max);
    dummy_var ^= ((int64_t) (void*) inflate_AggExpr_Median);
    dummy_var ^= ((int64_t) (void*) inflate_AggExpr_NUnique);
    dummy_var ^= ((int64_t) (void*) inflate_AggExpr_First);
    dummy_var ^= ((int64_t) (void*) inflate_AggExpr_Last);
    dummy_var ^= ((int64_t) (void*) inflate_AggExpr_Mean);
    dummy_var ^= ((int64_t) (void*) inflate_AggExpr_List);
    dummy_var ^= ((int64_t) (void*) inflate_AggExpr_Count);
    dummy_var ^= ((int64_t) (void*) inflate_AggExpr_Quantile);
    dummy_var ^= ((int64_t) (void*) inflate_AggExpr_Sum);
    dummy_var ^= ((int64_t) (void*) inflate_AggExpr_AggGroups);
    dummy_var ^= ((int64_t) (void*) inflate_AggExpr_Std);
    dummy_var ^= ((int64_t) (void*) inflate_DataType_Datetime);
    dummy_var ^= ((int64_t) (void*) inflate_DataType_Duration);
    dummy_var ^= ((int64_t) (void*) inflate_DataType_List);
    dummy_var ^= ((int64_t) (void*) inflate_DataType_Struct);
    dummy_var ^= ((int64_t) (void*) inflate_Excluded_Name);
    dummy_var ^= ((int64_t) (void*) inflate_Excluded_Dtype);
    dummy_var ^= ((int64_t) (void*) inflate_Expr_Alias);
    dummy_var ^= ((int64_t) (void*) inflate_Expr_Column);
    dummy_var ^= ((int64_t) (void*) inflate_Expr_Columns);
    dummy_var ^= ((int64_t) (void*) inflate_Expr_DtypeColumn);
    dummy_var ^= ((int64_t) (void*) inflate_Expr_Literal);
    dummy_var ^= ((int64_t) (void*) inflate_Expr_BinaryExpr);
    dummy_var ^= ((int64_t) (void*) inflate_Expr_Cast);
    dummy_var ^= ((int64_t) (void*) inflate_Expr_Sort);
    dummy_var ^= ((int64_t) (void*) inflate_Expr_Take);
    dummy_var ^= ((int64_t) (void*) inflate_Expr_Agg);
    dummy_var ^= ((int64_t) (void*) inflate_Expr_Ternary);
    dummy_var ^= ((int64_t) (void*) inflate_Expr_Explode);
    dummy_var ^= ((int64_t) (void*) inflate_Expr_Filter);
    dummy_var ^= ((int64_t) (void*) inflate_Expr_Slice);
    dummy_var ^= ((int64_t) (void*) inflate_Expr_Exclude);
    dummy_var ^= ((int64_t) (void*) inflate_Expr_KeepName);
    dummy_var ^= ((int64_t) (void*) inflate_Expr_Nth);
    dummy_var ^= ((int64_t) (void*) inflate_LiteralValue_Boolean);
    dummy_var ^= ((int64_t) (void*) inflate_LiteralValue_Utf8);
    dummy_var ^= ((int64_t) (void*) inflate_LiteralValue_Binary);
    dummy_var ^= ((int64_t) (void*) inflate_LiteralValue_UInt8);
    dummy_var ^= ((int64_t) (void*) inflate_LiteralValue_UInt16);
    dummy_var ^= ((int64_t) (void*) inflate_LiteralValue_UInt32);
    dummy_var ^= ((int64_t) (void*) inflate_LiteralValue_UInt64);
    dummy_var ^= ((int64_t) (void*) inflate_LiteralValue_Int8);
    dummy_var ^= ((int64_t) (void*) inflate_LiteralValue_Int16);
    dummy_var ^= ((int64_t) (void*) inflate_LiteralValue_Int32);
    dummy_var ^= ((int64_t) (void*) inflate_LiteralValue_Int64);
    dummy_var ^= ((int64_t) (void*) inflate_LiteralValue_Float32);
    dummy_var ^= ((int64_t) (void*) inflate_LiteralValue_Float64);
    dummy_var ^= ((int64_t) (void*) inflate_LiteralValue_Range);
    dummy_var ^= ((int64_t) (void*) inflate_LiteralValue_DateTime);
    dummy_var ^= ((int64_t) (void*) inflate_LiteralValue_Duration);
    dummy_var ^= ((int64_t) (void*) inflate_NullValues_AllColumnsSingle);
    dummy_var ^= ((int64_t) (void*) inflate_NullValues_AllColumns);
    dummy_var ^= ((int64_t) (void*) free_WireSyncReturn);
    dummy_var ^= ((int64_t) (void*) store_dart_post_cobject);
    dummy_var ^= ((int64_t) (void*) get_dart_object);
    dummy_var ^= ((int64_t) (void*) drop_dart_object);
    dummy_var ^= ((int64_t) (void*) new_dart_opaque);
    return dummy_var;
}
