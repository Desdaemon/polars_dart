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

typedef struct wire_cst_DataType_Datetime {
  int32_t field0;
  struct wire_cst_list_prim_u_8 *field1;
} wire_cst_DataType_Datetime;

typedef struct wire_cst_DataType_Duration {
  int32_t field0;
} wire_cst_DataType_Duration;

typedef struct wire_cst_DataType_List {
  struct wire_cst_data_type *field0;
} wire_cst_DataType_List;

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

typedef struct wire_cst_list_data_type {
  struct wire_cst_data_type *ptr;
  int32_t len;
} wire_cst_list_data_type;

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
  struct wire_cst_NullValues_AllColumnsSingle AllColumnsSingle;
  struct wire_cst_NullValues_AllColumns AllColumns;
  struct wire_cst_NullValues_Named Named;
} NullValuesKind;

typedef struct wire_cst_null_values {
  int32_t tag;
  union NullValuesKind kind;
} wire_cst_null_values;

typedef struct wire_cst_list_prim_u_32 {
  uint32_t *ptr;
  int32_t len;
} wire_cst_list_prim_u_32;

typedef struct wire_cst_LiteralValue_Boolean {
  bool field0;
} wire_cst_LiteralValue_Boolean;

typedef struct wire_cst_LiteralValue_Utf8 {
  struct wire_cst_list_prim_u_8 *field0;
} wire_cst_LiteralValue_Utf8;

typedef struct wire_cst_LiteralValue_Binary {
  struct wire_cst_list_prim_u_8 *field0;
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
  struct wire_cst_list_prim_u_8 *field2;
} wire_cst_LiteralValue_DateTime;

typedef struct wire_cst_LiteralValue_Duration {
  int64_t field0;
  int32_t field1;
} wire_cst_LiteralValue_Duration;

typedef struct wire_cst_LiteralValue_Series {
  const void *field0;
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

typedef struct wire_cst_list_prim_i_64 {
  int64_t *ptr;
  int32_t len;
} wire_cst_list_prim_i_64;

typedef struct wire_cst_list_opt_box_autoadd_bool {
  bool **ptr;
  int32_t len;
} wire_cst_list_opt_box_autoadd_bool;

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

typedef struct wire_cst_record_usize_usize {
  uintptr_t field0;
  uintptr_t field1;
} wire_cst_record_usize_usize;

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

WireSyncRust2DartDco wire_DataFrame_get_row(const void *that, uintptr_t index);

WireSyncRust2DartDco wire_DataFrame_head(const void *that, uintptr_t *length);

WireSyncRust2DartDco wire_DataFrame_height(const void *that);

WireSyncRust2DartDco wire_DataFrame_is_empty(const void *that);

void wire_DataFrame_iter(int64_t port_, const void *that);

WireSyncRust2DartDco wire_DataFrame_lazy(const void *that,
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

WireSyncRust2DartDco wire_DataFrame_sort_in_place(const void *that,
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

WireSyncRust2DartDco wire_LazyFrame_drop_nulls(const void *that, const void *subset);

WireSyncRust2DartDco wire_LazyFrame_explode(const void *that, const void *columns);

void wire_LazyFrame_fetch(int64_t port_, const void *that, uintptr_t n_rows);

WireSyncRust2DartDco wire_LazyFrame_filter(const void *that, const void *pred);

WireSyncRust2DartDco wire_LazyFrame_first(const void *that);

WireSyncRust2DartDco wire_LazyFrame_group_by(const void *that,
                                             const void *exprs,
                                             bool maintain_order);

WireSyncRust2DartDco wire_LazyFrame_inner_join(const void *that,
                                               const void *other,
                                               const void *left_on,
                                               const void *right_on);

WireSyncRust2DartDco wire_LazyFrame_join(const void *that,
                                         const void *other,
                                         const void *on,
                                         const void *left_on,
                                         const void *right_on,
                                         struct wire_cst_list_prim_u_8 *suffix,
                                         int32_t how,
                                         bool allow_parallel,
                                         bool force_parallel);

WireSyncRust2DartDco wire_LazyFrame_last(const void *that);

WireSyncRust2DartDco wire_LazyFrame_left_join(const void *that,
                                              const void *other,
                                              const void *left_on,
                                              const void *right_on);

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

WireSyncRust2DartDco wire_LazyFrame_min(const void *that);

WireSyncRust2DartDco wire_LazyFrame_outer_join(const void *that,
                                               const void *other,
                                               const void *left_on,
                                               const void *right_on);

WireSyncRust2DartDco wire_LazyFrame_quantile(const void *that,
                                             const void *quantile,
                                             int32_t interpol);

WireSyncRust2DartDco wire_LazyFrame_reverse(const void *that);

WireSyncRust2DartDco wire_LazyFrame_select(const void *that, const void *exprs);

WireSyncRust2DartDco wire_LazyFrame_slice(const void *that, int64_t offset, uint32_t len);

WireSyncRust2DartDco wire_LazyFrame_std(const void *that, uint8_t ddof);

WireSyncRust2DartDco wire_LazyFrame_sum(const void *that);

WireSyncRust2DartDco wire_LazyFrame_tail(const void *that, uint32_t n);

WireSyncRust2DartDco wire_LazyFrame_unique(const void *that,
                                           struct wire_cst_list_String *subset,
                                           int32_t keep_strategy);

WireSyncRust2DartDco wire_LazyFrame_variance(const void *that, uint8_t ddof);

WireSyncRust2DartDco wire_LazyFrame_with_column(const void *that, const void *expr);

WireSyncRust2DartDco wire_LazyFrame_with_columns(const void *that, const void *exprs);

WireSyncRust2DartDco wire_LazyFrame_with_row_count(const void *that,
                                                   struct wire_cst_list_prim_u_8 *name,
                                                   uint32_t *offset);

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

WireSyncRust2DartDco wire_Expr_abs(const void *that);

WireSyncRust2DartDco wire_Expr_add(const void *that, const void *other);

WireSyncRust2DartDco wire_Expr_agg_groups(const void *that);

WireSyncRust2DartDco wire_Expr_alias(const void *that, struct wire_cst_list_prim_u_8 *name);

WireSyncRust2DartDco wire_Expr_all(const void *that, bool ignore_nulls);

WireSyncRust2DartDco wire_Expr_and(const void *that, const void *expr);

WireSyncRust2DartDco wire_Expr_any(const void *that, bool ignore_nulls);

WireSyncRust2DartDco wire_Expr_append(const void *that, const void *other, bool upcast);

WireSyncRust2DartDco wire_Expr_arccos(const void *that);

WireSyncRust2DartDco wire_Expr_arccosh(const void *that);

WireSyncRust2DartDco wire_Expr_arcsin(const void *that);

WireSyncRust2DartDco wire_Expr_arcsinh(const void *that);

WireSyncRust2DartDco wire_Expr_arctan(const void *that);

WireSyncRust2DartDco wire_Expr_arctan2(const void *that, const void *x);

WireSyncRust2DartDco wire_Expr_arctanh(const void *that);

WireSyncRust2DartDco wire_Expr_arg_max(const void *that);

WireSyncRust2DartDco wire_Expr_arg_min(const void *that);

WireSyncRust2DartDco wire_Expr_arg_sort(const void *that,
                                        bool descending,
                                        bool nulls_last,
                                        bool multithreaded,
                                        bool maintain_order);

WireSyncRust2DartDco wire_Expr_arg_unique(const void *that);

WireSyncRust2DartDco wire_Expr_backward_fill(const void *that, uint32_t *limit);

WireSyncRust2DartDco wire_Expr_cast(const void *that, struct wire_cst_data_type *data_type);

WireSyncRust2DartDco wire_Expr_cbrt(const void *that);

WireSyncRust2DartDco wire_Expr_ceil(const void *that);

WireSyncRust2DartDco wire_Expr_clip(const void *that, const void *min, const void *max);

WireSyncRust2DartDco wire_Expr_clip_max(const void *that, const void *max);

WireSyncRust2DartDco wire_Expr_clip_min(const void *that, const void *min);

WireSyncRust2DartDco wire_Expr_cos(const void *that);

WireSyncRust2DartDco wire_Expr_cosh(const void *that);

WireSyncRust2DartDco wire_Expr_cot(const void *that);

WireSyncRust2DartDco wire_Expr_count(const void *that);

WireSyncRust2DartDco wire_Expr_cum_count(const void *that, bool reverse);

WireSyncRust2DartDco wire_Expr_cum_max(const void *that, bool reverse);

WireSyncRust2DartDco wire_Expr_cum_min(const void *that, bool reverse);

WireSyncRust2DartDco wire_Expr_cum_prod(const void *that, bool reverse);

WireSyncRust2DartDco wire_Expr_cum_sum(const void *that, bool reverse);

WireSyncRust2DartDco wire_Expr_degrees(const void *that);

WireSyncRust2DartDco wire_Expr_div(const void *that, const void *other);

WireSyncRust2DartDco wire_Expr_dot(const void *that, const void *other);

WireSyncRust2DartDco wire_Expr_drop_nans(const void *that);

WireSyncRust2DartDco wire_Expr_drop_nulls(const void *that);

WireSyncRust2DartDco wire_Expr_entropy(const void *that, double base, bool normalize);

WireSyncRust2DartDco wire_Expr_eq(const void *that, const void *other);

WireSyncRust2DartDco wire_Expr_eq_missing(const void *that, const void *other);

WireSyncRust2DartDco wire_Expr_exclude(const void *that, struct wire_cst_list_String *columns);

WireSyncRust2DartDco wire_Expr_exp(const void *that);

WireSyncRust2DartDco wire_Expr_explode(const void *that);

WireSyncRust2DartDco wire_Expr_fill_nan(const void *that, const void *value);

WireSyncRust2DartDco wire_Expr_fill_null(const void *that, const void *value);

WireSyncRust2DartDco wire_Expr_filter(const void *that, const void *cond);

WireSyncRust2DartDco wire_Expr_first(const void *that);

WireSyncRust2DartDco wire_Expr_flatten(const void *that);

WireSyncRust2DartDco wire_Expr_floor(const void *that);

WireSyncRust2DartDco wire_Expr_floor_div(const void *that, const void *rhs);

WireSyncRust2DartDco wire_Expr_forward_fill(const void *that, uint32_t *limit);

WireSyncRust2DartDco wire_Expr_gather(const void *that, const void *idx);

WireSyncRust2DartDco wire_Expr_get(const void *that, const void *idx);

WireSyncRust2DartDco wire_Expr_gt(const void *that, const void *other);

WireSyncRust2DartDco wire_Expr_gt_eq(const void *that, const void *other);

WireSyncRust2DartDco wire_Expr_head(const void *that, uintptr_t *length);

WireSyncRust2DartDco wire_Expr_implode(const void *that);

WireSyncRust2DartDco wire_Expr_is_finite(const void *that);

WireSyncRust2DartDco wire_Expr_is_in(const void *that, const void *other);

WireSyncRust2DartDco wire_Expr_is_nan(const void *that);

WireSyncRust2DartDco wire_Expr_is_not_nan(const void *that);

WireSyncRust2DartDco wire_Expr_is_not_null(const void *that);

WireSyncRust2DartDco wire_Expr_is_null(const void *that);

WireSyncRust2DartDco wire_Expr_last(const void *that);

WireSyncRust2DartDco wire_Expr_literal(struct wire_cst_literal_value *value);

WireSyncRust2DartDco wire_Expr_log(const void *that, double base);

WireSyncRust2DartDco wire_Expr_log1p(const void *that);

WireSyncRust2DartDco wire_Expr_lower_bound(const void *that);

WireSyncRust2DartDco wire_Expr_lt(const void *that, const void *other);

WireSyncRust2DartDco wire_Expr_lt_eq(const void *that, const void *other);

WireSyncRust2DartDco wire_Expr_mul(const void *that, const void *other);

WireSyncRust2DartDco wire_Expr_n_unique(const void *that);

WireSyncRust2DartDco wire_Expr_nan_max(const void *that);

WireSyncRust2DartDco wire_Expr_nan_min(const void *that);

WireSyncRust2DartDco wire_Expr_neq(const void *that, const void *other);

WireSyncRust2DartDco wire_Expr_neq_missing(const void *that, const void *other);

WireSyncRust2DartDco wire_Expr_not(const void *that);

WireSyncRust2DartDco wire_Expr_null_count(const void *that);

WireSyncRust2DartDco wire_Expr_or(const void *that, const void *expr);

WireSyncRust2DartDco wire_Expr_over(const void *that, const void *partiion_by, int32_t *kind);

WireSyncRust2DartDco wire_Expr_pow(const void *that, double exponent);

WireSyncRust2DartDco wire_Expr_product(const void *that);

WireSyncRust2DartDco wire_Expr_quantile(const void *that, const void *quantile, int32_t *interpol);

WireSyncRust2DartDco wire_Expr_radians(const void *that);

WireSyncRust2DartDco wire_Expr_rem(const void *that, const void *other);

WireSyncRust2DartDco wire_Expr_reshape(const void *that, struct wire_cst_list_prim_i_64 *dims);

WireSyncRust2DartDco wire_Expr_reverse(const void *that);

WireSyncRust2DartDco wire_Expr_rolling_max(const void *that,
                                           int64_t *window_size,
                                           uintptr_t *min_periods,
                                           struct wire_cst_list_prim_f_64 *weights,
                                           bool center,
                                           struct wire_cst_list_prim_u_8 *by,
                                           int32_t *closed_window);

WireSyncRust2DartDco wire_Expr_rolling_mean(const void *that,
                                            int64_t *window_size,
                                            uintptr_t *min_periods,
                                            struct wire_cst_list_prim_f_64 *weights,
                                            bool center,
                                            struct wire_cst_list_prim_u_8 *by,
                                            int32_t *closed_window);

WireSyncRust2DartDco wire_Expr_rolling_median(const void *that,
                                              int64_t *window_size,
                                              uintptr_t *min_periods,
                                              struct wire_cst_list_prim_f_64 *weights,
                                              bool center,
                                              struct wire_cst_list_prim_u_8 *by,
                                              int32_t *closed_window);

WireSyncRust2DartDco wire_Expr_rolling_min(const void *that,
                                           int64_t *window_size,
                                           uintptr_t *min_periods,
                                           struct wire_cst_list_prim_f_64 *weights,
                                           bool center,
                                           struct wire_cst_list_prim_u_8 *by,
                                           int32_t *closed_window);

WireSyncRust2DartDco wire_Expr_rolling_quantile(const void *that,
                                                int64_t *window_size,
                                                uintptr_t *min_periods,
                                                struct wire_cst_list_prim_f_64 *weights,
                                                bool center,
                                                struct wire_cst_list_prim_u_8 *by,
                                                int32_t *closed_window);

WireSyncRust2DartDco wire_Expr_rolling_std(const void *that,
                                           int64_t *window_size,
                                           uintptr_t *min_periods,
                                           struct wire_cst_list_prim_f_64 *weights,
                                           bool center,
                                           struct wire_cst_list_prim_u_8 *by,
                                           int32_t *closed_window);

WireSyncRust2DartDco wire_Expr_rolling_sum(const void *that,
                                           int64_t *window_size,
                                           uintptr_t *min_periods,
                                           struct wire_cst_list_prim_f_64 *weights,
                                           bool center,
                                           struct wire_cst_list_prim_u_8 *by,
                                           int32_t *closed_window);

WireSyncRust2DartDco wire_Expr_rolling_var(const void *that,
                                           int64_t *window_size,
                                           uintptr_t *min_periods,
                                           struct wire_cst_list_prim_f_64 *weights,
                                           bool center,
                                           struct wire_cst_list_prim_u_8 *by,
                                           int32_t *closed_window);

WireSyncRust2DartDco wire_Expr_round(const void *that, uint32_t decimals);

WireSyncRust2DartDco wire_Expr_round_sig_figs(const void *that, int32_t digits);

WireSyncRust2DartDco wire_Expr_set_sorted_flag(const void *that, int32_t sorted);

WireSyncRust2DartDco wire_Expr_shift(const void *that, const void *n);

WireSyncRust2DartDco wire_Expr_shift_and_fill(const void *that,
                                              const void *n,
                                              const void *fill_value);

WireSyncRust2DartDco wire_Expr_shrink_dtype(const void *that);

WireSyncRust2DartDco wire_Expr_sin(const void *that);

WireSyncRust2DartDco wire_Expr_sinh(const void *that);

WireSyncRust2DartDco wire_Expr_slice(const void *that, const void *offset, const void *length);

WireSyncRust2DartDco wire_Expr_sort(const void *that,
                                    bool descending,
                                    bool nulls_last,
                                    bool multithreaded,
                                    bool maintain_order);

WireSyncRust2DartDco wire_Expr_sqrt(const void *that);

WireSyncRust2DartDco wire_Expr_std(const void *that, uint8_t ddof);

WireSyncRust2DartDco wire_Expr_strict_cast(const void *that, struct wire_cst_data_type *data_type);

WireSyncRust2DartDco wire_Expr_sub(const void *that, const void *other);

WireSyncRust2DartDco wire_Expr_sum(const void *that);

WireSyncRust2DartDco wire_Expr_tail(const void *that, uintptr_t *length);

WireSyncRust2DartDco wire_Expr_tan(const void *that);

WireSyncRust2DartDco wire_Expr_tanh(const void *that);

WireSyncRust2DartDco wire_Expr_then(const void *that, const void *value, const void *otherwise);

WireSyncRust2DartDco wire_Expr_to_dot(const void *that);

WireSyncRust2DartDco wire_Expr_to_physical(const void *that);

WireSyncRust2DartDco wire_Expr_unique(const void *that);

WireSyncRust2DartDco wire_Expr_unique_stable(const void *that);

WireSyncRust2DartDco wire_Expr_upper_bound(const void *that);

WireSyncRust2DartDco wire_Expr_value_counts(const void *that, bool sort, bool parallel);

WireSyncRust2DartDco wire_Expr_variance(const void *that, uint8_t ddof);

WireSyncRust2DartDco wire_Expr_xor(const void *that, const void *expr);

WireSyncRust2DartDco wire_col(struct wire_cst_list_prim_u_8 *name);

WireSyncRust2DartDco wire_cols(struct wire_cst_list_String *names);

WireSyncRust2DartDco wire_count(void);

WireSyncRust2DartDco wire_dtypes(struct wire_cst_list_data_type *types);

WireSyncRust2DartDco wire_nth(int64_t idx);

WireSyncRust2DartDco wire_LazyGroupBy_agg(const void *that, const void *exprs);

WireSyncRust2DartDco wire_LazyGroupBy_head(const void *that, uintptr_t *n);

WireSyncRust2DartDco wire_LazyGroupBy_tail(const void *that, uintptr_t *n);

WireSyncRust2DartDco wire_Schema_of(struct wire_cst_list_field *fields);

WireSyncRust2DartDco wire_Series_add_to(const void *that, const void *other);

WireSyncRust2DartDco wire_Series_append(const void *that, const void *other);

WireSyncRust2DartDco wire_Series_apply_scalar(const void *that, int32_t op, double value);

void wire_Series_as_doubles(int64_t port_, const void *that, bool strict);

WireSyncRust2DartDco wire_Series_as_durations(const void *that);

WireSyncRust2DartDco wire_Series_as_ints(const void *that, bool strict);

WireSyncRust2DartDco wire_Series_as_local_datetime(const void *that);

WireSyncRust2DartDco wire_Series_as_naive_datetime(const void *that);

WireSyncRust2DartDco wire_Series_as_strings(const void *that);

WireSyncRust2DartDco wire_Series_as_utc_datetime(const void *that);

WireSyncRust2DartDco wire_Series_cast(const void *that,
                                      struct wire_cst_data_type *dtype,
                                      bool strict);

WireSyncRust2DartDco wire_Series_divide(const void *that, const void *other);

WireSyncRust2DartDco wire_Series_dump(const void *that);

WireSyncRust2DartDco wire_Series_equal(const void *that, const void *other, bool ignore_null);

WireSyncRust2DartDco wire_Series_estimated_size(const void *that);

WireSyncRust2DartDco wire_Series_explode(const void *that);

WireSyncRust2DartDco wire_Series_explode_by_offsets(const void *that,
                                                    struct wire_cst_list_prim_i_64 *offsets);

WireSyncRust2DartDco wire_Series_get(const void *that, uintptr_t index);

WireSyncRust2DartDco wire_Series_get_string(const void *that, uintptr_t index);

WireSyncRust2DartDco wire_Series_head(const void *that, uintptr_t *length);

WireSyncRust2DartDco wire_Series_into_frame(const void *that);

WireSyncRust2DartDco wire_Series_into_literal(const void *that);

WireSyncRust2DartDco wire_Series_is_bool(const void *that);

WireSyncRust2DartDco wire_Series_is_numeric(const void *that);

WireSyncRust2DartDco wire_Series_is_temporal(const void *that);

WireSyncRust2DartDco wire_Series_is_utf8(const void *that);

void wire_Series_iter(int64_t port_, const void *that);

WireSyncRust2DartDco wire_Series_max(const void *that);

WireSyncRust2DartDco wire_Series_mean(const void *that);

WireSyncRust2DartDco wire_Series_mean_as_series(const void *that);

WireSyncRust2DartDco wire_Series_median(const void *that);

WireSyncRust2DartDco wire_Series_median_as_series(const void *that);

WireSyncRust2DartDco wire_Series_min(const void *that);

WireSyncRust2DartDco wire_Series_multiply(const void *that, const void *other);

WireSyncRust2DartDco wire_Series_of_bools(struct wire_cst_list_prim_u_8 *name,
                                          struct wire_cst_list_opt_box_autoadd_bool *values);

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

WireSyncRust2DartDco wire_Series_product(const void *that);

WireSyncRust2DartDco wire_Series_remainder(const void *that, const void *other);

WireSyncRust2DartDco wire_Series_rename(const void *that, struct wire_cst_list_prim_u_8 *name);

WireSyncRust2DartDco wire_Series_reshape(const void *that, struct wire_cst_list_prim_i_64 *dims);

WireSyncRust2DartDco wire_Series_shuffle(const void *that, uint64_t *seed);

WireSyncRust2DartDco wire_Series_sort(const void *that, bool reverse);

WireSyncRust2DartDco wire_Series_std_as_series(const void *that, uint8_t ddof);

WireSyncRust2DartDco wire_Series_subtract(const void *that, const void *other);

WireSyncRust2DartDco wire_Series_sum(const void *that);

WireSyncRust2DartDco wire_Series_sum_as_series(const void *that);

WireSyncRust2DartDco wire_Series_tail(const void *that, uintptr_t *length);

WireSyncRust2DartDco wire_Series_to_list(const void *that);

void wire_Series_unique(int64_t port_, const void *that, bool maintain_order);

WireSyncRust2DartDco wire_Series_var_as_series(const void *that, uint8_t ddof);

void rust_arc_increment_strong_count_RustOpaque_AssertUnwindSafeSpecialEqPSeries(const void *ptr);

void rust_arc_decrement_strong_count_RustOpaque_AssertUnwindSafeSpecialEqPSeries(const void *ptr);

void rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockDataFrame(const void *ptr);

void rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockDataFrame(const void *ptr);

void rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockExpr(const void *ptr);

void rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockExpr(const void *ptr);

void rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockLazyFrame(const void *ptr);

void rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockLazyFrame(const void *ptr);

void rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockLazyGroupBy(const void *ptr);

void rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockLazyGroupBy(const void *ptr);

void rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockOptionSchema(const void *ptr);

void rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockOptionSchema(const void *ptr);

void rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockOptionVecExpr(const void *ptr);

void rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockOptionVecExpr(const void *ptr);

void rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockOptionVecSeries(const void *ptr);

void rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockOptionVecSeries(const void *ptr);

void rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockSchema(const void *ptr);

void rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockSchema(const void *ptr);

void rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockSeries(const void *ptr);

void rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockSeries(const void *ptr);

void rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockVecExpr(const void *ptr);

void rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockVecExpr(const void *ptr);

void rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockVecSeries(const void *ptr);

void rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockVecSeries(const void *ptr);

int64_t *cst_new_box_autoadd_Chrono_Duration(int64_t value);

int64_t *cst_new_box_autoadd_Chrono_Local(int64_t value);

int64_t *cst_new_box_autoadd_Chrono_Naive(int64_t value);

int64_t *cst_new_box_autoadd_Chrono_Utc(int64_t value);

bool *cst_new_box_autoadd_bool(bool value);

int32_t *cst_new_box_autoadd_closed_window(int32_t value);

int32_t *cst_new_box_autoadd_csv_encoding(int32_t value);

struct wire_cst_data_type *cst_new_box_autoadd_data_type(void);

double *cst_new_box_autoadd_f_64(double value);

int32_t *cst_new_box_autoadd_i_32(int32_t value);

int64_t *cst_new_box_autoadd_i_64(int64_t value);

struct wire_cst_literal_value *cst_new_box_autoadd_literal_value(void);

struct wire_cst_null_values *cst_new_box_autoadd_null_values(void);

int32_t *cst_new_box_autoadd_quantile_interpol_options(int32_t value);

struct wire_cst_row_count *cst_new_box_autoadd_row_count(void);

uint32_t *cst_new_box_autoadd_u_32(uint32_t value);

uint64_t *cst_new_box_autoadd_u_64(uint64_t value);

uintptr_t *cst_new_box_autoadd_usize(uintptr_t value);

int32_t *cst_new_box_autoadd_window_mapping(int32_t value);

struct wire_cst_data_type *cst_new_box_data_type(void);

struct wire_cst_list_String *cst_new_list_String(int32_t len);

struct wire_cst_list_bool *cst_new_list_bool(int32_t len);

struct wire_cst_list_dartabi *cst_new_list_dartabi(int32_t len);

struct wire_cst_list_data_type *cst_new_list_data_type(int32_t len);

struct wire_cst_list_field *cst_new_list_field(int32_t len);

struct wire_cst_list_opt_String *cst_new_list_opt_String(int32_t len);

struct wire_cst_list_opt_box_autoadd_Chrono_Duration *cst_new_list_opt_box_autoadd_Chrono_Duration(int32_t len);

struct wire_cst_list_opt_box_autoadd_Chrono_Local *cst_new_list_opt_box_autoadd_Chrono_Local(int32_t len);

struct wire_cst_list_opt_box_autoadd_Chrono_Naive *cst_new_list_opt_box_autoadd_Chrono_Naive(int32_t len);

struct wire_cst_list_opt_box_autoadd_Chrono_Utc *cst_new_list_opt_box_autoadd_Chrono_Utc(int32_t len);

struct wire_cst_list_opt_box_autoadd_bool *cst_new_list_opt_box_autoadd_bool(int32_t len);

struct wire_cst_list_opt_box_autoadd_f_64 *cst_new_list_opt_box_autoadd_f_64(int32_t len);

struct wire_cst_list_opt_box_autoadd_i_32 *cst_new_list_opt_box_autoadd_i_32(int32_t len);

struct wire_cst_list_opt_box_autoadd_i_64 *cst_new_list_opt_box_autoadd_i_64(int32_t len);

struct wire_cst_list_prim_f_64 *cst_new_list_prim_f_64(int32_t len);

struct wire_cst_list_prim_i_64 *cst_new_list_prim_i_64(int32_t len);

struct wire_cst_list_prim_u_32 *cst_new_list_prim_u_32(int32_t len);

struct wire_cst_list_prim_u_8 *cst_new_list_prim_u_8(int32_t len);

struct wire_cst_list_record_string_string *cst_new_list_record_string_string(int32_t len);
static int64_t dummy_method_to_enforce_bundling(void) {
    int64_t dummy_var = 0;
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_Chrono_Duration);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_Chrono_Local);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_Chrono_Naive);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_Chrono_Utc);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_bool);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_closed_window);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_csv_encoding);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_data_type);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_f_64);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_i_32);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_i_64);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_literal_value);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_null_values);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_quantile_interpol_options);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_row_count);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_u_32);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_u_64);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_usize);
    dummy_var ^= ((int64_t) (void*) cst_new_box_autoadd_window_mapping);
    dummy_var ^= ((int64_t) (void*) cst_new_box_data_type);
    dummy_var ^= ((int64_t) (void*) cst_new_list_String);
    dummy_var ^= ((int64_t) (void*) cst_new_list_bool);
    dummy_var ^= ((int64_t) (void*) cst_new_list_dartabi);
    dummy_var ^= ((int64_t) (void*) cst_new_list_data_type);
    dummy_var ^= ((int64_t) (void*) cst_new_list_field);
    dummy_var ^= ((int64_t) (void*) cst_new_list_opt_String);
    dummy_var ^= ((int64_t) (void*) cst_new_list_opt_box_autoadd_Chrono_Duration);
    dummy_var ^= ((int64_t) (void*) cst_new_list_opt_box_autoadd_Chrono_Local);
    dummy_var ^= ((int64_t) (void*) cst_new_list_opt_box_autoadd_Chrono_Naive);
    dummy_var ^= ((int64_t) (void*) cst_new_list_opt_box_autoadd_Chrono_Utc);
    dummy_var ^= ((int64_t) (void*) cst_new_list_opt_box_autoadd_bool);
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
    dummy_var ^= ((int64_t) (void*) rust_arc_decrement_strong_count_RustOpaque_AssertUnwindSafeSpecialEqPSeries);
    dummy_var ^= ((int64_t) (void*) rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockDataFrame);
    dummy_var ^= ((int64_t) (void*) rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockExpr);
    dummy_var ^= ((int64_t) (void*) rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockLazyFrame);
    dummy_var ^= ((int64_t) (void*) rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockLazyGroupBy);
    dummy_var ^= ((int64_t) (void*) rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockOptionSchema);
    dummy_var ^= ((int64_t) (void*) rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockOptionVecExpr);
    dummy_var ^= ((int64_t) (void*) rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockOptionVecSeries);
    dummy_var ^= ((int64_t) (void*) rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockSchema);
    dummy_var ^= ((int64_t) (void*) rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockSeries);
    dummy_var ^= ((int64_t) (void*) rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockVecExpr);
    dummy_var ^= ((int64_t) (void*) rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLockVecSeries);
    dummy_var ^= ((int64_t) (void*) rust_arc_increment_strong_count_RustOpaque_AssertUnwindSafeSpecialEqPSeries);
    dummy_var ^= ((int64_t) (void*) rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockDataFrame);
    dummy_var ^= ((int64_t) (void*) rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockExpr);
    dummy_var ^= ((int64_t) (void*) rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockLazyFrame);
    dummy_var ^= ((int64_t) (void*) rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockLazyGroupBy);
    dummy_var ^= ((int64_t) (void*) rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockOptionSchema);
    dummy_var ^= ((int64_t) (void*) rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockOptionVecExpr);
    dummy_var ^= ((int64_t) (void*) rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockOptionVecSeries);
    dummy_var ^= ((int64_t) (void*) rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockSchema);
    dummy_var ^= ((int64_t) (void*) rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockSeries);
    dummy_var ^= ((int64_t) (void*) rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockVecExpr);
    dummy_var ^= ((int64_t) (void*) rust_arc_increment_strong_count_RustOpaque_stdsyncRwLockVecSeries);
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
    dummy_var ^= ((int64_t) (void*) wire_DataFrame_iter);
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
    dummy_var ^= ((int64_t) (void*) wire_Expr_abs);
    dummy_var ^= ((int64_t) (void*) wire_Expr_add);
    dummy_var ^= ((int64_t) (void*) wire_Expr_agg_groups);
    dummy_var ^= ((int64_t) (void*) wire_Expr_alias);
    dummy_var ^= ((int64_t) (void*) wire_Expr_all);
    dummy_var ^= ((int64_t) (void*) wire_Expr_and);
    dummy_var ^= ((int64_t) (void*) wire_Expr_any);
    dummy_var ^= ((int64_t) (void*) wire_Expr_append);
    dummy_var ^= ((int64_t) (void*) wire_Expr_arccos);
    dummy_var ^= ((int64_t) (void*) wire_Expr_arccosh);
    dummy_var ^= ((int64_t) (void*) wire_Expr_arcsin);
    dummy_var ^= ((int64_t) (void*) wire_Expr_arcsinh);
    dummy_var ^= ((int64_t) (void*) wire_Expr_arctan);
    dummy_var ^= ((int64_t) (void*) wire_Expr_arctan2);
    dummy_var ^= ((int64_t) (void*) wire_Expr_arctanh);
    dummy_var ^= ((int64_t) (void*) wire_Expr_arg_max);
    dummy_var ^= ((int64_t) (void*) wire_Expr_arg_min);
    dummy_var ^= ((int64_t) (void*) wire_Expr_arg_sort);
    dummy_var ^= ((int64_t) (void*) wire_Expr_arg_unique);
    dummy_var ^= ((int64_t) (void*) wire_Expr_backward_fill);
    dummy_var ^= ((int64_t) (void*) wire_Expr_cast);
    dummy_var ^= ((int64_t) (void*) wire_Expr_cbrt);
    dummy_var ^= ((int64_t) (void*) wire_Expr_ceil);
    dummy_var ^= ((int64_t) (void*) wire_Expr_clip);
    dummy_var ^= ((int64_t) (void*) wire_Expr_clip_max);
    dummy_var ^= ((int64_t) (void*) wire_Expr_clip_min);
    dummy_var ^= ((int64_t) (void*) wire_Expr_cos);
    dummy_var ^= ((int64_t) (void*) wire_Expr_cosh);
    dummy_var ^= ((int64_t) (void*) wire_Expr_cot);
    dummy_var ^= ((int64_t) (void*) wire_Expr_count);
    dummy_var ^= ((int64_t) (void*) wire_Expr_cum_count);
    dummy_var ^= ((int64_t) (void*) wire_Expr_cum_max);
    dummy_var ^= ((int64_t) (void*) wire_Expr_cum_min);
    dummy_var ^= ((int64_t) (void*) wire_Expr_cum_prod);
    dummy_var ^= ((int64_t) (void*) wire_Expr_cum_sum);
    dummy_var ^= ((int64_t) (void*) wire_Expr_degrees);
    dummy_var ^= ((int64_t) (void*) wire_Expr_div);
    dummy_var ^= ((int64_t) (void*) wire_Expr_dot);
    dummy_var ^= ((int64_t) (void*) wire_Expr_drop_nans);
    dummy_var ^= ((int64_t) (void*) wire_Expr_drop_nulls);
    dummy_var ^= ((int64_t) (void*) wire_Expr_entropy);
    dummy_var ^= ((int64_t) (void*) wire_Expr_eq);
    dummy_var ^= ((int64_t) (void*) wire_Expr_eq_missing);
    dummy_var ^= ((int64_t) (void*) wire_Expr_exclude);
    dummy_var ^= ((int64_t) (void*) wire_Expr_exp);
    dummy_var ^= ((int64_t) (void*) wire_Expr_explode);
    dummy_var ^= ((int64_t) (void*) wire_Expr_fill_nan);
    dummy_var ^= ((int64_t) (void*) wire_Expr_fill_null);
    dummy_var ^= ((int64_t) (void*) wire_Expr_filter);
    dummy_var ^= ((int64_t) (void*) wire_Expr_first);
    dummy_var ^= ((int64_t) (void*) wire_Expr_flatten);
    dummy_var ^= ((int64_t) (void*) wire_Expr_floor);
    dummy_var ^= ((int64_t) (void*) wire_Expr_floor_div);
    dummy_var ^= ((int64_t) (void*) wire_Expr_forward_fill);
    dummy_var ^= ((int64_t) (void*) wire_Expr_gather);
    dummy_var ^= ((int64_t) (void*) wire_Expr_get);
    dummy_var ^= ((int64_t) (void*) wire_Expr_gt);
    dummy_var ^= ((int64_t) (void*) wire_Expr_gt_eq);
    dummy_var ^= ((int64_t) (void*) wire_Expr_head);
    dummy_var ^= ((int64_t) (void*) wire_Expr_implode);
    dummy_var ^= ((int64_t) (void*) wire_Expr_is_finite);
    dummy_var ^= ((int64_t) (void*) wire_Expr_is_in);
    dummy_var ^= ((int64_t) (void*) wire_Expr_is_nan);
    dummy_var ^= ((int64_t) (void*) wire_Expr_is_not_nan);
    dummy_var ^= ((int64_t) (void*) wire_Expr_is_not_null);
    dummy_var ^= ((int64_t) (void*) wire_Expr_is_null);
    dummy_var ^= ((int64_t) (void*) wire_Expr_last);
    dummy_var ^= ((int64_t) (void*) wire_Expr_literal);
    dummy_var ^= ((int64_t) (void*) wire_Expr_log);
    dummy_var ^= ((int64_t) (void*) wire_Expr_log1p);
    dummy_var ^= ((int64_t) (void*) wire_Expr_lower_bound);
    dummy_var ^= ((int64_t) (void*) wire_Expr_lt);
    dummy_var ^= ((int64_t) (void*) wire_Expr_lt_eq);
    dummy_var ^= ((int64_t) (void*) wire_Expr_mul);
    dummy_var ^= ((int64_t) (void*) wire_Expr_n_unique);
    dummy_var ^= ((int64_t) (void*) wire_Expr_nan_max);
    dummy_var ^= ((int64_t) (void*) wire_Expr_nan_min);
    dummy_var ^= ((int64_t) (void*) wire_Expr_neq);
    dummy_var ^= ((int64_t) (void*) wire_Expr_neq_missing);
    dummy_var ^= ((int64_t) (void*) wire_Expr_not);
    dummy_var ^= ((int64_t) (void*) wire_Expr_null_count);
    dummy_var ^= ((int64_t) (void*) wire_Expr_or);
    dummy_var ^= ((int64_t) (void*) wire_Expr_over);
    dummy_var ^= ((int64_t) (void*) wire_Expr_pow);
    dummy_var ^= ((int64_t) (void*) wire_Expr_product);
    dummy_var ^= ((int64_t) (void*) wire_Expr_quantile);
    dummy_var ^= ((int64_t) (void*) wire_Expr_radians);
    dummy_var ^= ((int64_t) (void*) wire_Expr_rem);
    dummy_var ^= ((int64_t) (void*) wire_Expr_reshape);
    dummy_var ^= ((int64_t) (void*) wire_Expr_reverse);
    dummy_var ^= ((int64_t) (void*) wire_Expr_rolling_max);
    dummy_var ^= ((int64_t) (void*) wire_Expr_rolling_mean);
    dummy_var ^= ((int64_t) (void*) wire_Expr_rolling_median);
    dummy_var ^= ((int64_t) (void*) wire_Expr_rolling_min);
    dummy_var ^= ((int64_t) (void*) wire_Expr_rolling_quantile);
    dummy_var ^= ((int64_t) (void*) wire_Expr_rolling_std);
    dummy_var ^= ((int64_t) (void*) wire_Expr_rolling_sum);
    dummy_var ^= ((int64_t) (void*) wire_Expr_rolling_var);
    dummy_var ^= ((int64_t) (void*) wire_Expr_round);
    dummy_var ^= ((int64_t) (void*) wire_Expr_round_sig_figs);
    dummy_var ^= ((int64_t) (void*) wire_Expr_set_sorted_flag);
    dummy_var ^= ((int64_t) (void*) wire_Expr_shift);
    dummy_var ^= ((int64_t) (void*) wire_Expr_shift_and_fill);
    dummy_var ^= ((int64_t) (void*) wire_Expr_shrink_dtype);
    dummy_var ^= ((int64_t) (void*) wire_Expr_sin);
    dummy_var ^= ((int64_t) (void*) wire_Expr_sinh);
    dummy_var ^= ((int64_t) (void*) wire_Expr_slice);
    dummy_var ^= ((int64_t) (void*) wire_Expr_sort);
    dummy_var ^= ((int64_t) (void*) wire_Expr_sqrt);
    dummy_var ^= ((int64_t) (void*) wire_Expr_std);
    dummy_var ^= ((int64_t) (void*) wire_Expr_strict_cast);
    dummy_var ^= ((int64_t) (void*) wire_Expr_sub);
    dummy_var ^= ((int64_t) (void*) wire_Expr_sum);
    dummy_var ^= ((int64_t) (void*) wire_Expr_tail);
    dummy_var ^= ((int64_t) (void*) wire_Expr_tan);
    dummy_var ^= ((int64_t) (void*) wire_Expr_tanh);
    dummy_var ^= ((int64_t) (void*) wire_Expr_then);
    dummy_var ^= ((int64_t) (void*) wire_Expr_to_dot);
    dummy_var ^= ((int64_t) (void*) wire_Expr_to_physical);
    dummy_var ^= ((int64_t) (void*) wire_Expr_unique);
    dummy_var ^= ((int64_t) (void*) wire_Expr_unique_stable);
    dummy_var ^= ((int64_t) (void*) wire_Expr_upper_bound);
    dummy_var ^= ((int64_t) (void*) wire_Expr_value_counts);
    dummy_var ^= ((int64_t) (void*) wire_Expr_variance);
    dummy_var ^= ((int64_t) (void*) wire_Expr_xor);
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
    dummy_var ^= ((int64_t) (void*) wire_LazyFrame_select);
    dummy_var ^= ((int64_t) (void*) wire_LazyFrame_slice);
    dummy_var ^= ((int64_t) (void*) wire_LazyFrame_std);
    dummy_var ^= ((int64_t) (void*) wire_LazyFrame_sum);
    dummy_var ^= ((int64_t) (void*) wire_LazyFrame_tail);
    dummy_var ^= ((int64_t) (void*) wire_LazyFrame_unique);
    dummy_var ^= ((int64_t) (void*) wire_LazyFrame_variance);
    dummy_var ^= ((int64_t) (void*) wire_LazyFrame_with_column);
    dummy_var ^= ((int64_t) (void*) wire_LazyFrame_with_columns);
    dummy_var ^= ((int64_t) (void*) wire_LazyFrame_with_row_count);
    dummy_var ^= ((int64_t) (void*) wire_LazyGroupBy_agg);
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
    dummy_var ^= ((int64_t) (void*) wire_Series_into_literal);
    dummy_var ^= ((int64_t) (void*) wire_Series_is_bool);
    dummy_var ^= ((int64_t) (void*) wire_Series_is_numeric);
    dummy_var ^= ((int64_t) (void*) wire_Series_is_temporal);
    dummy_var ^= ((int64_t) (void*) wire_Series_is_utf8);
    dummy_var ^= ((int64_t) (void*) wire_Series_iter);
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
    dummy_var ^= ((int64_t) (void*) wire_col);
    dummy_var ^= ((int64_t) (void*) wire_cols);
    dummy_var ^= ((int64_t) (void*) wire_count);
    dummy_var ^= ((int64_t) (void*) wire_dtypes);
    dummy_var ^= ((int64_t) (void*) wire_nth);
    dummy_var ^= ((int64_t) (void*) wire_read_csv);
    dummy_var ^= ((int64_t) (void*) wire_read_json);
    dummy_var ^= ((int64_t) (void*) wire_scan_csv);
    return dummy_var;
}
