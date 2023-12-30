// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'expr.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
    'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#custom-getters-and-methods');

/// @nodoc
mixin _$AggExpr {}

/// @nodoc
abstract class $AggExprCopyWith<$Res> {
  factory $AggExprCopyWith(AggExpr value, $Res Function(AggExpr) then) =
      _$AggExprCopyWithImpl<$Res, AggExpr>;
}

/// @nodoc
class _$AggExprCopyWithImpl<$Res, $Val extends AggExpr>
    implements $AggExprCopyWith<$Res> {
  _$AggExprCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;
}

/// @nodoc
abstract class _$$AggExpr_MinImplCopyWith<$Res> {
  factory _$$AggExpr_MinImplCopyWith(
          _$AggExpr_MinImpl value, $Res Function(_$AggExpr_MinImpl) then) =
      __$$AggExpr_MinImplCopyWithImpl<$Res>;
  @useResult
  $Res call({Expr input, bool propagateNans});

  $ExprCopyWith<$Res> get input;
}

/// @nodoc
class __$$AggExpr_MinImplCopyWithImpl<$Res>
    extends _$AggExprCopyWithImpl<$Res, _$AggExpr_MinImpl>
    implements _$$AggExpr_MinImplCopyWith<$Res> {
  __$$AggExpr_MinImplCopyWithImpl(
      _$AggExpr_MinImpl _value, $Res Function(_$AggExpr_MinImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? input = null,
    Object? propagateNans = null,
  }) {
    return _then(_$AggExpr_MinImpl(
      input: null == input
          ? _value.input
          : input // ignore: cast_nullable_to_non_nullable
              as Expr,
      propagateNans: null == propagateNans
          ? _value.propagateNans
          : propagateNans // ignore: cast_nullable_to_non_nullable
              as bool,
    ));
  }

  @override
  @pragma('vm:prefer-inline')
  $ExprCopyWith<$Res> get input {
    return $ExprCopyWith<$Res>(_value.input, (value) {
      return _then(_value.copyWith(input: value));
    });
  }
}

/// @nodoc

class _$AggExpr_MinImpl extends AggExpr_Min {
  const _$AggExpr_MinImpl({required this.input, required this.propagateNans})
      : super._();

  @override
  final Expr input;
  @override
  final bool propagateNans;

  @override
  String toString() {
    return 'AggExpr.min(input: $input, propagateNans: $propagateNans)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$AggExpr_MinImpl &&
            (identical(other.input, input) || other.input == input) &&
            (identical(other.propagateNans, propagateNans) ||
                other.propagateNans == propagateNans));
  }

  @override
  int get hashCode => Object.hash(runtimeType, input, propagateNans);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$AggExpr_MinImplCopyWith<_$AggExpr_MinImpl> get copyWith =>
      __$$AggExpr_MinImplCopyWithImpl<_$AggExpr_MinImpl>(this, _$identity);
}

abstract class AggExpr_Min extends AggExpr {
  const factory AggExpr_Min(
      {required final Expr input,
      required final bool propagateNans}) = _$AggExpr_MinImpl;
  const AggExpr_Min._() : super._();

  Expr get input;
  bool get propagateNans;
  @JsonKey(ignore: true)
  _$$AggExpr_MinImplCopyWith<_$AggExpr_MinImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$AggExpr_MaxImplCopyWith<$Res> {
  factory _$$AggExpr_MaxImplCopyWith(
          _$AggExpr_MaxImpl value, $Res Function(_$AggExpr_MaxImpl) then) =
      __$$AggExpr_MaxImplCopyWithImpl<$Res>;
  @useResult
  $Res call({Expr input, bool propagateNans});

  $ExprCopyWith<$Res> get input;
}

/// @nodoc
class __$$AggExpr_MaxImplCopyWithImpl<$Res>
    extends _$AggExprCopyWithImpl<$Res, _$AggExpr_MaxImpl>
    implements _$$AggExpr_MaxImplCopyWith<$Res> {
  __$$AggExpr_MaxImplCopyWithImpl(
      _$AggExpr_MaxImpl _value, $Res Function(_$AggExpr_MaxImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? input = null,
    Object? propagateNans = null,
  }) {
    return _then(_$AggExpr_MaxImpl(
      input: null == input
          ? _value.input
          : input // ignore: cast_nullable_to_non_nullable
              as Expr,
      propagateNans: null == propagateNans
          ? _value.propagateNans
          : propagateNans // ignore: cast_nullable_to_non_nullable
              as bool,
    ));
  }

  @override
  @pragma('vm:prefer-inline')
  $ExprCopyWith<$Res> get input {
    return $ExprCopyWith<$Res>(_value.input, (value) {
      return _then(_value.copyWith(input: value));
    });
  }
}

/// @nodoc

class _$AggExpr_MaxImpl extends AggExpr_Max {
  const _$AggExpr_MaxImpl({required this.input, required this.propagateNans})
      : super._();

  @override
  final Expr input;
  @override
  final bool propagateNans;

  @override
  String toString() {
    return 'AggExpr.max(input: $input, propagateNans: $propagateNans)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$AggExpr_MaxImpl &&
            (identical(other.input, input) || other.input == input) &&
            (identical(other.propagateNans, propagateNans) ||
                other.propagateNans == propagateNans));
  }

  @override
  int get hashCode => Object.hash(runtimeType, input, propagateNans);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$AggExpr_MaxImplCopyWith<_$AggExpr_MaxImpl> get copyWith =>
      __$$AggExpr_MaxImplCopyWithImpl<_$AggExpr_MaxImpl>(this, _$identity);
}

abstract class AggExpr_Max extends AggExpr {
  const factory AggExpr_Max(
      {required final Expr input,
      required final bool propagateNans}) = _$AggExpr_MaxImpl;
  const AggExpr_Max._() : super._();

  Expr get input;
  bool get propagateNans;
  @JsonKey(ignore: true)
  _$$AggExpr_MaxImplCopyWith<_$AggExpr_MaxImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$AggExpr_MedianImplCopyWith<$Res> {
  factory _$$AggExpr_MedianImplCopyWith(_$AggExpr_MedianImpl value,
          $Res Function(_$AggExpr_MedianImpl) then) =
      __$$AggExpr_MedianImplCopyWithImpl<$Res>;
  @useResult
  $Res call({Expr field0});

  $ExprCopyWith<$Res> get field0;
}

/// @nodoc
class __$$AggExpr_MedianImplCopyWithImpl<$Res>
    extends _$AggExprCopyWithImpl<$Res, _$AggExpr_MedianImpl>
    implements _$$AggExpr_MedianImplCopyWith<$Res> {
  __$$AggExpr_MedianImplCopyWithImpl(
      _$AggExpr_MedianImpl _value, $Res Function(_$AggExpr_MedianImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$AggExpr_MedianImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as Expr,
    ));
  }

  @override
  @pragma('vm:prefer-inline')
  $ExprCopyWith<$Res> get field0 {
    return $ExprCopyWith<$Res>(_value.field0, (value) {
      return _then(_value.copyWith(field0: value));
    });
  }
}

/// @nodoc

class _$AggExpr_MedianImpl extends AggExpr_Median {
  const _$AggExpr_MedianImpl(this.field0) : super._();

  @override
  final Expr field0;

  @override
  String toString() {
    return 'AggExpr.median(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$AggExpr_MedianImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$AggExpr_MedianImplCopyWith<_$AggExpr_MedianImpl> get copyWith =>
      __$$AggExpr_MedianImplCopyWithImpl<_$AggExpr_MedianImpl>(
          this, _$identity);
}

abstract class AggExpr_Median extends AggExpr {
  const factory AggExpr_Median(final Expr field0) = _$AggExpr_MedianImpl;
  const AggExpr_Median._() : super._();

  Expr get field0;
  @JsonKey(ignore: true)
  _$$AggExpr_MedianImplCopyWith<_$AggExpr_MedianImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$AggExpr_NUniqueImplCopyWith<$Res> {
  factory _$$AggExpr_NUniqueImplCopyWith(_$AggExpr_NUniqueImpl value,
          $Res Function(_$AggExpr_NUniqueImpl) then) =
      __$$AggExpr_NUniqueImplCopyWithImpl<$Res>;
  @useResult
  $Res call({Expr field0});

  $ExprCopyWith<$Res> get field0;
}

/// @nodoc
class __$$AggExpr_NUniqueImplCopyWithImpl<$Res>
    extends _$AggExprCopyWithImpl<$Res, _$AggExpr_NUniqueImpl>
    implements _$$AggExpr_NUniqueImplCopyWith<$Res> {
  __$$AggExpr_NUniqueImplCopyWithImpl(
      _$AggExpr_NUniqueImpl _value, $Res Function(_$AggExpr_NUniqueImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$AggExpr_NUniqueImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as Expr,
    ));
  }

  @override
  @pragma('vm:prefer-inline')
  $ExprCopyWith<$Res> get field0 {
    return $ExprCopyWith<$Res>(_value.field0, (value) {
      return _then(_value.copyWith(field0: value));
    });
  }
}

/// @nodoc

class _$AggExpr_NUniqueImpl extends AggExpr_NUnique {
  const _$AggExpr_NUniqueImpl(this.field0) : super._();

  @override
  final Expr field0;

  @override
  String toString() {
    return 'AggExpr.nUnique(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$AggExpr_NUniqueImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$AggExpr_NUniqueImplCopyWith<_$AggExpr_NUniqueImpl> get copyWith =>
      __$$AggExpr_NUniqueImplCopyWithImpl<_$AggExpr_NUniqueImpl>(
          this, _$identity);
}

abstract class AggExpr_NUnique extends AggExpr {
  const factory AggExpr_NUnique(final Expr field0) = _$AggExpr_NUniqueImpl;
  const AggExpr_NUnique._() : super._();

  Expr get field0;
  @JsonKey(ignore: true)
  _$$AggExpr_NUniqueImplCopyWith<_$AggExpr_NUniqueImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$AggExpr_FirstImplCopyWith<$Res> {
  factory _$$AggExpr_FirstImplCopyWith(
          _$AggExpr_FirstImpl value, $Res Function(_$AggExpr_FirstImpl) then) =
      __$$AggExpr_FirstImplCopyWithImpl<$Res>;
  @useResult
  $Res call({Expr field0});

  $ExprCopyWith<$Res> get field0;
}

/// @nodoc
class __$$AggExpr_FirstImplCopyWithImpl<$Res>
    extends _$AggExprCopyWithImpl<$Res, _$AggExpr_FirstImpl>
    implements _$$AggExpr_FirstImplCopyWith<$Res> {
  __$$AggExpr_FirstImplCopyWithImpl(
      _$AggExpr_FirstImpl _value, $Res Function(_$AggExpr_FirstImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$AggExpr_FirstImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as Expr,
    ));
  }

  @override
  @pragma('vm:prefer-inline')
  $ExprCopyWith<$Res> get field0 {
    return $ExprCopyWith<$Res>(_value.field0, (value) {
      return _then(_value.copyWith(field0: value));
    });
  }
}

/// @nodoc

class _$AggExpr_FirstImpl extends AggExpr_First {
  const _$AggExpr_FirstImpl(this.field0) : super._();

  @override
  final Expr field0;

  @override
  String toString() {
    return 'AggExpr.first(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$AggExpr_FirstImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$AggExpr_FirstImplCopyWith<_$AggExpr_FirstImpl> get copyWith =>
      __$$AggExpr_FirstImplCopyWithImpl<_$AggExpr_FirstImpl>(this, _$identity);
}

abstract class AggExpr_First extends AggExpr {
  const factory AggExpr_First(final Expr field0) = _$AggExpr_FirstImpl;
  const AggExpr_First._() : super._();

  Expr get field0;
  @JsonKey(ignore: true)
  _$$AggExpr_FirstImplCopyWith<_$AggExpr_FirstImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$AggExpr_LastImplCopyWith<$Res> {
  factory _$$AggExpr_LastImplCopyWith(
          _$AggExpr_LastImpl value, $Res Function(_$AggExpr_LastImpl) then) =
      __$$AggExpr_LastImplCopyWithImpl<$Res>;
  @useResult
  $Res call({Expr field0});

  $ExprCopyWith<$Res> get field0;
}

/// @nodoc
class __$$AggExpr_LastImplCopyWithImpl<$Res>
    extends _$AggExprCopyWithImpl<$Res, _$AggExpr_LastImpl>
    implements _$$AggExpr_LastImplCopyWith<$Res> {
  __$$AggExpr_LastImplCopyWithImpl(
      _$AggExpr_LastImpl _value, $Res Function(_$AggExpr_LastImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$AggExpr_LastImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as Expr,
    ));
  }

  @override
  @pragma('vm:prefer-inline')
  $ExprCopyWith<$Res> get field0 {
    return $ExprCopyWith<$Res>(_value.field0, (value) {
      return _then(_value.copyWith(field0: value));
    });
  }
}

/// @nodoc

class _$AggExpr_LastImpl extends AggExpr_Last {
  const _$AggExpr_LastImpl(this.field0) : super._();

  @override
  final Expr field0;

  @override
  String toString() {
    return 'AggExpr.last(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$AggExpr_LastImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$AggExpr_LastImplCopyWith<_$AggExpr_LastImpl> get copyWith =>
      __$$AggExpr_LastImplCopyWithImpl<_$AggExpr_LastImpl>(this, _$identity);
}

abstract class AggExpr_Last extends AggExpr {
  const factory AggExpr_Last(final Expr field0) = _$AggExpr_LastImpl;
  const AggExpr_Last._() : super._();

  Expr get field0;
  @JsonKey(ignore: true)
  _$$AggExpr_LastImplCopyWith<_$AggExpr_LastImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$AggExpr_MeanImplCopyWith<$Res> {
  factory _$$AggExpr_MeanImplCopyWith(
          _$AggExpr_MeanImpl value, $Res Function(_$AggExpr_MeanImpl) then) =
      __$$AggExpr_MeanImplCopyWithImpl<$Res>;
  @useResult
  $Res call({Expr field0});

  $ExprCopyWith<$Res> get field0;
}

/// @nodoc
class __$$AggExpr_MeanImplCopyWithImpl<$Res>
    extends _$AggExprCopyWithImpl<$Res, _$AggExpr_MeanImpl>
    implements _$$AggExpr_MeanImplCopyWith<$Res> {
  __$$AggExpr_MeanImplCopyWithImpl(
      _$AggExpr_MeanImpl _value, $Res Function(_$AggExpr_MeanImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$AggExpr_MeanImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as Expr,
    ));
  }

  @override
  @pragma('vm:prefer-inline')
  $ExprCopyWith<$Res> get field0 {
    return $ExprCopyWith<$Res>(_value.field0, (value) {
      return _then(_value.copyWith(field0: value));
    });
  }
}

/// @nodoc

class _$AggExpr_MeanImpl extends AggExpr_Mean {
  const _$AggExpr_MeanImpl(this.field0) : super._();

  @override
  final Expr field0;

  @override
  String toString() {
    return 'AggExpr.mean(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$AggExpr_MeanImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$AggExpr_MeanImplCopyWith<_$AggExpr_MeanImpl> get copyWith =>
      __$$AggExpr_MeanImplCopyWithImpl<_$AggExpr_MeanImpl>(this, _$identity);
}

abstract class AggExpr_Mean extends AggExpr {
  const factory AggExpr_Mean(final Expr field0) = _$AggExpr_MeanImpl;
  const AggExpr_Mean._() : super._();

  Expr get field0;
  @JsonKey(ignore: true)
  _$$AggExpr_MeanImplCopyWith<_$AggExpr_MeanImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$AggExpr_ImplodeImplCopyWith<$Res> {
  factory _$$AggExpr_ImplodeImplCopyWith(_$AggExpr_ImplodeImpl value,
          $Res Function(_$AggExpr_ImplodeImpl) then) =
      __$$AggExpr_ImplodeImplCopyWithImpl<$Res>;
  @useResult
  $Res call({Expr field0});

  $ExprCopyWith<$Res> get field0;
}

/// @nodoc
class __$$AggExpr_ImplodeImplCopyWithImpl<$Res>
    extends _$AggExprCopyWithImpl<$Res, _$AggExpr_ImplodeImpl>
    implements _$$AggExpr_ImplodeImplCopyWith<$Res> {
  __$$AggExpr_ImplodeImplCopyWithImpl(
      _$AggExpr_ImplodeImpl _value, $Res Function(_$AggExpr_ImplodeImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$AggExpr_ImplodeImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as Expr,
    ));
  }

  @override
  @pragma('vm:prefer-inline')
  $ExprCopyWith<$Res> get field0 {
    return $ExprCopyWith<$Res>(_value.field0, (value) {
      return _then(_value.copyWith(field0: value));
    });
  }
}

/// @nodoc

class _$AggExpr_ImplodeImpl extends AggExpr_Implode {
  const _$AggExpr_ImplodeImpl(this.field0) : super._();

  @override
  final Expr field0;

  @override
  String toString() {
    return 'AggExpr.implode(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$AggExpr_ImplodeImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$AggExpr_ImplodeImplCopyWith<_$AggExpr_ImplodeImpl> get copyWith =>
      __$$AggExpr_ImplodeImplCopyWithImpl<_$AggExpr_ImplodeImpl>(
          this, _$identity);
}

abstract class AggExpr_Implode extends AggExpr {
  const factory AggExpr_Implode(final Expr field0) = _$AggExpr_ImplodeImpl;
  const AggExpr_Implode._() : super._();

  Expr get field0;
  @JsonKey(ignore: true)
  _$$AggExpr_ImplodeImplCopyWith<_$AggExpr_ImplodeImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$AggExpr_CountImplCopyWith<$Res> {
  factory _$$AggExpr_CountImplCopyWith(
          _$AggExpr_CountImpl value, $Res Function(_$AggExpr_CountImpl) then) =
      __$$AggExpr_CountImplCopyWithImpl<$Res>;
  @useResult
  $Res call({Expr field0});

  $ExprCopyWith<$Res> get field0;
}

/// @nodoc
class __$$AggExpr_CountImplCopyWithImpl<$Res>
    extends _$AggExprCopyWithImpl<$Res, _$AggExpr_CountImpl>
    implements _$$AggExpr_CountImplCopyWith<$Res> {
  __$$AggExpr_CountImplCopyWithImpl(
      _$AggExpr_CountImpl _value, $Res Function(_$AggExpr_CountImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$AggExpr_CountImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as Expr,
    ));
  }

  @override
  @pragma('vm:prefer-inline')
  $ExprCopyWith<$Res> get field0 {
    return $ExprCopyWith<$Res>(_value.field0, (value) {
      return _then(_value.copyWith(field0: value));
    });
  }
}

/// @nodoc

class _$AggExpr_CountImpl extends AggExpr_Count {
  const _$AggExpr_CountImpl(this.field0) : super._();

  @override
  final Expr field0;

  @override
  String toString() {
    return 'AggExpr.count(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$AggExpr_CountImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$AggExpr_CountImplCopyWith<_$AggExpr_CountImpl> get copyWith =>
      __$$AggExpr_CountImplCopyWithImpl<_$AggExpr_CountImpl>(this, _$identity);
}

abstract class AggExpr_Count extends AggExpr {
  const factory AggExpr_Count(final Expr field0) = _$AggExpr_CountImpl;
  const AggExpr_Count._() : super._();

  Expr get field0;
  @JsonKey(ignore: true)
  _$$AggExpr_CountImplCopyWith<_$AggExpr_CountImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$AggExpr_QuantileImplCopyWith<$Res> {
  factory _$$AggExpr_QuantileImplCopyWith(_$AggExpr_QuantileImpl value,
          $Res Function(_$AggExpr_QuantileImpl) then) =
      __$$AggExpr_QuantileImplCopyWithImpl<$Res>;
  @useResult
  $Res call({Expr expr, Expr quantile, QuantileInterpolOptions interpol});

  $ExprCopyWith<$Res> get expr;
  $ExprCopyWith<$Res> get quantile;
}

/// @nodoc
class __$$AggExpr_QuantileImplCopyWithImpl<$Res>
    extends _$AggExprCopyWithImpl<$Res, _$AggExpr_QuantileImpl>
    implements _$$AggExpr_QuantileImplCopyWith<$Res> {
  __$$AggExpr_QuantileImplCopyWithImpl(_$AggExpr_QuantileImpl _value,
      $Res Function(_$AggExpr_QuantileImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? expr = null,
    Object? quantile = null,
    Object? interpol = null,
  }) {
    return _then(_$AggExpr_QuantileImpl(
      expr: null == expr
          ? _value.expr
          : expr // ignore: cast_nullable_to_non_nullable
              as Expr,
      quantile: null == quantile
          ? _value.quantile
          : quantile // ignore: cast_nullable_to_non_nullable
              as Expr,
      interpol: null == interpol
          ? _value.interpol
          : interpol // ignore: cast_nullable_to_non_nullable
              as QuantileInterpolOptions,
    ));
  }

  @override
  @pragma('vm:prefer-inline')
  $ExprCopyWith<$Res> get expr {
    return $ExprCopyWith<$Res>(_value.expr, (value) {
      return _then(_value.copyWith(expr: value));
    });
  }

  @override
  @pragma('vm:prefer-inline')
  $ExprCopyWith<$Res> get quantile {
    return $ExprCopyWith<$Res>(_value.quantile, (value) {
      return _then(_value.copyWith(quantile: value));
    });
  }
}

/// @nodoc

class _$AggExpr_QuantileImpl extends AggExpr_Quantile {
  const _$AggExpr_QuantileImpl(
      {required this.expr, required this.quantile, required this.interpol})
      : super._();

  @override
  final Expr expr;
  @override
  final Expr quantile;
  @override
  final QuantileInterpolOptions interpol;

  @override
  String toString() {
    return 'AggExpr.quantile(expr: $expr, quantile: $quantile, interpol: $interpol)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$AggExpr_QuantileImpl &&
            (identical(other.expr, expr) || other.expr == expr) &&
            (identical(other.quantile, quantile) ||
                other.quantile == quantile) &&
            (identical(other.interpol, interpol) ||
                other.interpol == interpol));
  }

  @override
  int get hashCode => Object.hash(runtimeType, expr, quantile, interpol);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$AggExpr_QuantileImplCopyWith<_$AggExpr_QuantileImpl> get copyWith =>
      __$$AggExpr_QuantileImplCopyWithImpl<_$AggExpr_QuantileImpl>(
          this, _$identity);
}

abstract class AggExpr_Quantile extends AggExpr {
  const factory AggExpr_Quantile(
          {required final Expr expr,
          required final Expr quantile,
          required final QuantileInterpolOptions interpol}) =
      _$AggExpr_QuantileImpl;
  const AggExpr_Quantile._() : super._();

  Expr get expr;
  Expr get quantile;
  QuantileInterpolOptions get interpol;
  @JsonKey(ignore: true)
  _$$AggExpr_QuantileImplCopyWith<_$AggExpr_QuantileImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$AggExpr_SumImplCopyWith<$Res> {
  factory _$$AggExpr_SumImplCopyWith(
          _$AggExpr_SumImpl value, $Res Function(_$AggExpr_SumImpl) then) =
      __$$AggExpr_SumImplCopyWithImpl<$Res>;
  @useResult
  $Res call({Expr field0});

  $ExprCopyWith<$Res> get field0;
}

/// @nodoc
class __$$AggExpr_SumImplCopyWithImpl<$Res>
    extends _$AggExprCopyWithImpl<$Res, _$AggExpr_SumImpl>
    implements _$$AggExpr_SumImplCopyWith<$Res> {
  __$$AggExpr_SumImplCopyWithImpl(
      _$AggExpr_SumImpl _value, $Res Function(_$AggExpr_SumImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$AggExpr_SumImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as Expr,
    ));
  }

  @override
  @pragma('vm:prefer-inline')
  $ExprCopyWith<$Res> get field0 {
    return $ExprCopyWith<$Res>(_value.field0, (value) {
      return _then(_value.copyWith(field0: value));
    });
  }
}

/// @nodoc

class _$AggExpr_SumImpl extends AggExpr_Sum {
  const _$AggExpr_SumImpl(this.field0) : super._();

  @override
  final Expr field0;

  @override
  String toString() {
    return 'AggExpr.sum(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$AggExpr_SumImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$AggExpr_SumImplCopyWith<_$AggExpr_SumImpl> get copyWith =>
      __$$AggExpr_SumImplCopyWithImpl<_$AggExpr_SumImpl>(this, _$identity);
}

abstract class AggExpr_Sum extends AggExpr {
  const factory AggExpr_Sum(final Expr field0) = _$AggExpr_SumImpl;
  const AggExpr_Sum._() : super._();

  Expr get field0;
  @JsonKey(ignore: true)
  _$$AggExpr_SumImplCopyWith<_$AggExpr_SumImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$AggExpr_AggGroupsImplCopyWith<$Res> {
  factory _$$AggExpr_AggGroupsImplCopyWith(_$AggExpr_AggGroupsImpl value,
          $Res Function(_$AggExpr_AggGroupsImpl) then) =
      __$$AggExpr_AggGroupsImplCopyWithImpl<$Res>;
  @useResult
  $Res call({Expr field0});

  $ExprCopyWith<$Res> get field0;
}

/// @nodoc
class __$$AggExpr_AggGroupsImplCopyWithImpl<$Res>
    extends _$AggExprCopyWithImpl<$Res, _$AggExpr_AggGroupsImpl>
    implements _$$AggExpr_AggGroupsImplCopyWith<$Res> {
  __$$AggExpr_AggGroupsImplCopyWithImpl(_$AggExpr_AggGroupsImpl _value,
      $Res Function(_$AggExpr_AggGroupsImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$AggExpr_AggGroupsImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as Expr,
    ));
  }

  @override
  @pragma('vm:prefer-inline')
  $ExprCopyWith<$Res> get field0 {
    return $ExprCopyWith<$Res>(_value.field0, (value) {
      return _then(_value.copyWith(field0: value));
    });
  }
}

/// @nodoc

class _$AggExpr_AggGroupsImpl extends AggExpr_AggGroups {
  const _$AggExpr_AggGroupsImpl(this.field0) : super._();

  @override
  final Expr field0;

  @override
  String toString() {
    return 'AggExpr.aggGroups(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$AggExpr_AggGroupsImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$AggExpr_AggGroupsImplCopyWith<_$AggExpr_AggGroupsImpl> get copyWith =>
      __$$AggExpr_AggGroupsImplCopyWithImpl<_$AggExpr_AggGroupsImpl>(
          this, _$identity);
}

abstract class AggExpr_AggGroups extends AggExpr {
  const factory AggExpr_AggGroups(final Expr field0) = _$AggExpr_AggGroupsImpl;
  const AggExpr_AggGroups._() : super._();

  Expr get field0;
  @JsonKey(ignore: true)
  _$$AggExpr_AggGroupsImplCopyWith<_$AggExpr_AggGroupsImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$AggExpr_StdImplCopyWith<$Res> {
  factory _$$AggExpr_StdImplCopyWith(
          _$AggExpr_StdImpl value, $Res Function(_$AggExpr_StdImpl) then) =
      __$$AggExpr_StdImplCopyWithImpl<$Res>;
  @useResult
  $Res call({Expr field0, int field1});

  $ExprCopyWith<$Res> get field0;
}

/// @nodoc
class __$$AggExpr_StdImplCopyWithImpl<$Res>
    extends _$AggExprCopyWithImpl<$Res, _$AggExpr_StdImpl>
    implements _$$AggExpr_StdImplCopyWith<$Res> {
  __$$AggExpr_StdImplCopyWithImpl(
      _$AggExpr_StdImpl _value, $Res Function(_$AggExpr_StdImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
    Object? field1 = null,
  }) {
    return _then(_$AggExpr_StdImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as Expr,
      null == field1
          ? _value.field1
          : field1 // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }

  @override
  @pragma('vm:prefer-inline')
  $ExprCopyWith<$Res> get field0 {
    return $ExprCopyWith<$Res>(_value.field0, (value) {
      return _then(_value.copyWith(field0: value));
    });
  }
}

/// @nodoc

class _$AggExpr_StdImpl extends AggExpr_Std {
  const _$AggExpr_StdImpl(this.field0, this.field1) : super._();

  @override
  final Expr field0;
  @override
  final int field1;

  @override
  String toString() {
    return 'AggExpr.std(field0: $field0, field1: $field1)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$AggExpr_StdImpl &&
            (identical(other.field0, field0) || other.field0 == field0) &&
            (identical(other.field1, field1) || other.field1 == field1));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0, field1);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$AggExpr_StdImplCopyWith<_$AggExpr_StdImpl> get copyWith =>
      __$$AggExpr_StdImplCopyWithImpl<_$AggExpr_StdImpl>(this, _$identity);
}

abstract class AggExpr_Std extends AggExpr {
  const factory AggExpr_Std(final Expr field0, final int field1) =
      _$AggExpr_StdImpl;
  const AggExpr_Std._() : super._();

  Expr get field0;
  int get field1;
  @JsonKey(ignore: true)
  _$$AggExpr_StdImplCopyWith<_$AggExpr_StdImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$AggExpr_VarImplCopyWith<$Res> {
  factory _$$AggExpr_VarImplCopyWith(
          _$AggExpr_VarImpl value, $Res Function(_$AggExpr_VarImpl) then) =
      __$$AggExpr_VarImplCopyWithImpl<$Res>;
  @useResult
  $Res call({Expr field0, int field1});

  $ExprCopyWith<$Res> get field0;
}

/// @nodoc
class __$$AggExpr_VarImplCopyWithImpl<$Res>
    extends _$AggExprCopyWithImpl<$Res, _$AggExpr_VarImpl>
    implements _$$AggExpr_VarImplCopyWith<$Res> {
  __$$AggExpr_VarImplCopyWithImpl(
      _$AggExpr_VarImpl _value, $Res Function(_$AggExpr_VarImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
    Object? field1 = null,
  }) {
    return _then(_$AggExpr_VarImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as Expr,
      null == field1
          ? _value.field1
          : field1 // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }

  @override
  @pragma('vm:prefer-inline')
  $ExprCopyWith<$Res> get field0 {
    return $ExprCopyWith<$Res>(_value.field0, (value) {
      return _then(_value.copyWith(field0: value));
    });
  }
}

/// @nodoc

class _$AggExpr_VarImpl extends AggExpr_Var {
  const _$AggExpr_VarImpl(this.field0, this.field1) : super._();

  @override
  final Expr field0;
  @override
  final int field1;

  @override
  String toString() {
    return 'AggExpr.Var(field0: $field0, field1: $field1)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$AggExpr_VarImpl &&
            (identical(other.field0, field0) || other.field0 == field0) &&
            (identical(other.field1, field1) || other.field1 == field1));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0, field1);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$AggExpr_VarImplCopyWith<_$AggExpr_VarImpl> get copyWith =>
      __$$AggExpr_VarImplCopyWithImpl<_$AggExpr_VarImpl>(this, _$identity);
}

abstract class AggExpr_Var extends AggExpr {
  const factory AggExpr_Var(final Expr field0, final int field1) =
      _$AggExpr_VarImpl;
  const AggExpr_Var._() : super._();

  Expr get field0;
  int get field1;
  @JsonKey(ignore: true)
  _$$AggExpr_VarImplCopyWith<_$AggExpr_VarImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$DataType {}

/// @nodoc
abstract class $DataTypeCopyWith<$Res> {
  factory $DataTypeCopyWith(DataType value, $Res Function(DataType) then) =
      _$DataTypeCopyWithImpl<$Res, DataType>;
}

/// @nodoc
class _$DataTypeCopyWithImpl<$Res, $Val extends DataType>
    implements $DataTypeCopyWith<$Res> {
  _$DataTypeCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;
}

/// @nodoc
abstract class _$$DataType_BooleanImplCopyWith<$Res> {
  factory _$$DataType_BooleanImplCopyWith(_$DataType_BooleanImpl value,
          $Res Function(_$DataType_BooleanImpl) then) =
      __$$DataType_BooleanImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$DataType_BooleanImplCopyWithImpl<$Res>
    extends _$DataTypeCopyWithImpl<$Res, _$DataType_BooleanImpl>
    implements _$$DataType_BooleanImplCopyWith<$Res> {
  __$$DataType_BooleanImplCopyWithImpl(_$DataType_BooleanImpl _value,
      $Res Function(_$DataType_BooleanImpl) _then)
      : super(_value, _then);
}

/// @nodoc

class _$DataType_BooleanImpl extends DataType_Boolean {
  const _$DataType_BooleanImpl() : super._();

  @override
  String toString() {
    return 'DataType.boolean()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is _$DataType_BooleanImpl);
  }

  @override
  int get hashCode => runtimeType.hashCode;
}

abstract class DataType_Boolean extends DataType {
  const factory DataType_Boolean() = _$DataType_BooleanImpl;
  const DataType_Boolean._() : super._();
}

/// @nodoc
abstract class _$$DataType_Uint8ImplCopyWith<$Res> {
  factory _$$DataType_Uint8ImplCopyWith(_$DataType_Uint8Impl value,
          $Res Function(_$DataType_Uint8Impl) then) =
      __$$DataType_Uint8ImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$DataType_Uint8ImplCopyWithImpl<$Res>
    extends _$DataTypeCopyWithImpl<$Res, _$DataType_Uint8Impl>
    implements _$$DataType_Uint8ImplCopyWith<$Res> {
  __$$DataType_Uint8ImplCopyWithImpl(
      _$DataType_Uint8Impl _value, $Res Function(_$DataType_Uint8Impl) _then)
      : super(_value, _then);
}

/// @nodoc

class _$DataType_Uint8Impl extends DataType_Uint8 {
  const _$DataType_Uint8Impl() : super._();

  @override
  String toString() {
    return 'DataType.uint8()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is _$DataType_Uint8Impl);
  }

  @override
  int get hashCode => runtimeType.hashCode;
}

abstract class DataType_Uint8 extends DataType {
  const factory DataType_Uint8() = _$DataType_Uint8Impl;
  const DataType_Uint8._() : super._();
}

/// @nodoc
abstract class _$$DataType_Uint16ImplCopyWith<$Res> {
  factory _$$DataType_Uint16ImplCopyWith(_$DataType_Uint16Impl value,
          $Res Function(_$DataType_Uint16Impl) then) =
      __$$DataType_Uint16ImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$DataType_Uint16ImplCopyWithImpl<$Res>
    extends _$DataTypeCopyWithImpl<$Res, _$DataType_Uint16Impl>
    implements _$$DataType_Uint16ImplCopyWith<$Res> {
  __$$DataType_Uint16ImplCopyWithImpl(
      _$DataType_Uint16Impl _value, $Res Function(_$DataType_Uint16Impl) _then)
      : super(_value, _then);
}

/// @nodoc

class _$DataType_Uint16Impl extends DataType_Uint16 {
  const _$DataType_Uint16Impl() : super._();

  @override
  String toString() {
    return 'DataType.uint16()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is _$DataType_Uint16Impl);
  }

  @override
  int get hashCode => runtimeType.hashCode;
}

abstract class DataType_Uint16 extends DataType {
  const factory DataType_Uint16() = _$DataType_Uint16Impl;
  const DataType_Uint16._() : super._();
}

/// @nodoc
abstract class _$$DataType_Uint32ImplCopyWith<$Res> {
  factory _$$DataType_Uint32ImplCopyWith(_$DataType_Uint32Impl value,
          $Res Function(_$DataType_Uint32Impl) then) =
      __$$DataType_Uint32ImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$DataType_Uint32ImplCopyWithImpl<$Res>
    extends _$DataTypeCopyWithImpl<$Res, _$DataType_Uint32Impl>
    implements _$$DataType_Uint32ImplCopyWith<$Res> {
  __$$DataType_Uint32ImplCopyWithImpl(
      _$DataType_Uint32Impl _value, $Res Function(_$DataType_Uint32Impl) _then)
      : super(_value, _then);
}

/// @nodoc

class _$DataType_Uint32Impl extends DataType_Uint32 {
  const _$DataType_Uint32Impl() : super._();

  @override
  String toString() {
    return 'DataType.uint32()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is _$DataType_Uint32Impl);
  }

  @override
  int get hashCode => runtimeType.hashCode;
}

abstract class DataType_Uint32 extends DataType {
  const factory DataType_Uint32() = _$DataType_Uint32Impl;
  const DataType_Uint32._() : super._();
}

/// @nodoc
abstract class _$$DataType_Uint64ImplCopyWith<$Res> {
  factory _$$DataType_Uint64ImplCopyWith(_$DataType_Uint64Impl value,
          $Res Function(_$DataType_Uint64Impl) then) =
      __$$DataType_Uint64ImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$DataType_Uint64ImplCopyWithImpl<$Res>
    extends _$DataTypeCopyWithImpl<$Res, _$DataType_Uint64Impl>
    implements _$$DataType_Uint64ImplCopyWith<$Res> {
  __$$DataType_Uint64ImplCopyWithImpl(
      _$DataType_Uint64Impl _value, $Res Function(_$DataType_Uint64Impl) _then)
      : super(_value, _then);
}

/// @nodoc

class _$DataType_Uint64Impl extends DataType_Uint64 {
  const _$DataType_Uint64Impl() : super._();

  @override
  String toString() {
    return 'DataType.uint64()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is _$DataType_Uint64Impl);
  }

  @override
  int get hashCode => runtimeType.hashCode;
}

abstract class DataType_Uint64 extends DataType {
  const factory DataType_Uint64() = _$DataType_Uint64Impl;
  const DataType_Uint64._() : super._();
}

/// @nodoc
abstract class _$$DataType_Int8ImplCopyWith<$Res> {
  factory _$$DataType_Int8ImplCopyWith(
          _$DataType_Int8Impl value, $Res Function(_$DataType_Int8Impl) then) =
      __$$DataType_Int8ImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$DataType_Int8ImplCopyWithImpl<$Res>
    extends _$DataTypeCopyWithImpl<$Res, _$DataType_Int8Impl>
    implements _$$DataType_Int8ImplCopyWith<$Res> {
  __$$DataType_Int8ImplCopyWithImpl(
      _$DataType_Int8Impl _value, $Res Function(_$DataType_Int8Impl) _then)
      : super(_value, _then);
}

/// @nodoc

class _$DataType_Int8Impl extends DataType_Int8 {
  const _$DataType_Int8Impl() : super._();

  @override
  String toString() {
    return 'DataType.int8()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is _$DataType_Int8Impl);
  }

  @override
  int get hashCode => runtimeType.hashCode;
}

abstract class DataType_Int8 extends DataType {
  const factory DataType_Int8() = _$DataType_Int8Impl;
  const DataType_Int8._() : super._();
}

/// @nodoc
abstract class _$$DataType_Int16ImplCopyWith<$Res> {
  factory _$$DataType_Int16ImplCopyWith(_$DataType_Int16Impl value,
          $Res Function(_$DataType_Int16Impl) then) =
      __$$DataType_Int16ImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$DataType_Int16ImplCopyWithImpl<$Res>
    extends _$DataTypeCopyWithImpl<$Res, _$DataType_Int16Impl>
    implements _$$DataType_Int16ImplCopyWith<$Res> {
  __$$DataType_Int16ImplCopyWithImpl(
      _$DataType_Int16Impl _value, $Res Function(_$DataType_Int16Impl) _then)
      : super(_value, _then);
}

/// @nodoc

class _$DataType_Int16Impl extends DataType_Int16 {
  const _$DataType_Int16Impl() : super._();

  @override
  String toString() {
    return 'DataType.int16()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is _$DataType_Int16Impl);
  }

  @override
  int get hashCode => runtimeType.hashCode;
}

abstract class DataType_Int16 extends DataType {
  const factory DataType_Int16() = _$DataType_Int16Impl;
  const DataType_Int16._() : super._();
}

/// @nodoc
abstract class _$$DataType_Int32ImplCopyWith<$Res> {
  factory _$$DataType_Int32ImplCopyWith(_$DataType_Int32Impl value,
          $Res Function(_$DataType_Int32Impl) then) =
      __$$DataType_Int32ImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$DataType_Int32ImplCopyWithImpl<$Res>
    extends _$DataTypeCopyWithImpl<$Res, _$DataType_Int32Impl>
    implements _$$DataType_Int32ImplCopyWith<$Res> {
  __$$DataType_Int32ImplCopyWithImpl(
      _$DataType_Int32Impl _value, $Res Function(_$DataType_Int32Impl) _then)
      : super(_value, _then);
}

/// @nodoc

class _$DataType_Int32Impl extends DataType_Int32 {
  const _$DataType_Int32Impl() : super._();

  @override
  String toString() {
    return 'DataType.int32()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is _$DataType_Int32Impl);
  }

  @override
  int get hashCode => runtimeType.hashCode;
}

abstract class DataType_Int32 extends DataType {
  const factory DataType_Int32() = _$DataType_Int32Impl;
  const DataType_Int32._() : super._();
}

/// @nodoc
abstract class _$$DataType_Int64ImplCopyWith<$Res> {
  factory _$$DataType_Int64ImplCopyWith(_$DataType_Int64Impl value,
          $Res Function(_$DataType_Int64Impl) then) =
      __$$DataType_Int64ImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$DataType_Int64ImplCopyWithImpl<$Res>
    extends _$DataTypeCopyWithImpl<$Res, _$DataType_Int64Impl>
    implements _$$DataType_Int64ImplCopyWith<$Res> {
  __$$DataType_Int64ImplCopyWithImpl(
      _$DataType_Int64Impl _value, $Res Function(_$DataType_Int64Impl) _then)
      : super(_value, _then);
}

/// @nodoc

class _$DataType_Int64Impl extends DataType_Int64 {
  const _$DataType_Int64Impl() : super._();

  @override
  String toString() {
    return 'DataType.int64()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is _$DataType_Int64Impl);
  }

  @override
  int get hashCode => runtimeType.hashCode;
}

abstract class DataType_Int64 extends DataType {
  const factory DataType_Int64() = _$DataType_Int64Impl;
  const DataType_Int64._() : super._();
}

/// @nodoc
abstract class _$$DataType_Float32ImplCopyWith<$Res> {
  factory _$$DataType_Float32ImplCopyWith(_$DataType_Float32Impl value,
          $Res Function(_$DataType_Float32Impl) then) =
      __$$DataType_Float32ImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$DataType_Float32ImplCopyWithImpl<$Res>
    extends _$DataTypeCopyWithImpl<$Res, _$DataType_Float32Impl>
    implements _$$DataType_Float32ImplCopyWith<$Res> {
  __$$DataType_Float32ImplCopyWithImpl(_$DataType_Float32Impl _value,
      $Res Function(_$DataType_Float32Impl) _then)
      : super(_value, _then);
}

/// @nodoc

class _$DataType_Float32Impl extends DataType_Float32 {
  const _$DataType_Float32Impl() : super._();

  @override
  String toString() {
    return 'DataType.float32()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is _$DataType_Float32Impl);
  }

  @override
  int get hashCode => runtimeType.hashCode;
}

abstract class DataType_Float32 extends DataType {
  const factory DataType_Float32() = _$DataType_Float32Impl;
  const DataType_Float32._() : super._();
}

/// @nodoc
abstract class _$$DataType_Float64ImplCopyWith<$Res> {
  factory _$$DataType_Float64ImplCopyWith(_$DataType_Float64Impl value,
          $Res Function(_$DataType_Float64Impl) then) =
      __$$DataType_Float64ImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$DataType_Float64ImplCopyWithImpl<$Res>
    extends _$DataTypeCopyWithImpl<$Res, _$DataType_Float64Impl>
    implements _$$DataType_Float64ImplCopyWith<$Res> {
  __$$DataType_Float64ImplCopyWithImpl(_$DataType_Float64Impl _value,
      $Res Function(_$DataType_Float64Impl) _then)
      : super(_value, _then);
}

/// @nodoc

class _$DataType_Float64Impl extends DataType_Float64 {
  const _$DataType_Float64Impl() : super._();

  @override
  String toString() {
    return 'DataType.float64()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is _$DataType_Float64Impl);
  }

  @override
  int get hashCode => runtimeType.hashCode;
}

abstract class DataType_Float64 extends DataType {
  const factory DataType_Float64() = _$DataType_Float64Impl;
  const DataType_Float64._() : super._();
}

/// @nodoc
abstract class _$$DataType_Utf8ImplCopyWith<$Res> {
  factory _$$DataType_Utf8ImplCopyWith(
          _$DataType_Utf8Impl value, $Res Function(_$DataType_Utf8Impl) then) =
      __$$DataType_Utf8ImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$DataType_Utf8ImplCopyWithImpl<$Res>
    extends _$DataTypeCopyWithImpl<$Res, _$DataType_Utf8Impl>
    implements _$$DataType_Utf8ImplCopyWith<$Res> {
  __$$DataType_Utf8ImplCopyWithImpl(
      _$DataType_Utf8Impl _value, $Res Function(_$DataType_Utf8Impl) _then)
      : super(_value, _then);
}

/// @nodoc

class _$DataType_Utf8Impl extends DataType_Utf8 {
  const _$DataType_Utf8Impl() : super._();

  @override
  String toString() {
    return 'DataType.utf8()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is _$DataType_Utf8Impl);
  }

  @override
  int get hashCode => runtimeType.hashCode;
}

abstract class DataType_Utf8 extends DataType {
  const factory DataType_Utf8() = _$DataType_Utf8Impl;
  const DataType_Utf8._() : super._();
}

/// @nodoc
abstract class _$$DataType_BinaryImplCopyWith<$Res> {
  factory _$$DataType_BinaryImplCopyWith(_$DataType_BinaryImpl value,
          $Res Function(_$DataType_BinaryImpl) then) =
      __$$DataType_BinaryImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$DataType_BinaryImplCopyWithImpl<$Res>
    extends _$DataTypeCopyWithImpl<$Res, _$DataType_BinaryImpl>
    implements _$$DataType_BinaryImplCopyWith<$Res> {
  __$$DataType_BinaryImplCopyWithImpl(
      _$DataType_BinaryImpl _value, $Res Function(_$DataType_BinaryImpl) _then)
      : super(_value, _then);
}

/// @nodoc

class _$DataType_BinaryImpl extends DataType_Binary {
  const _$DataType_BinaryImpl() : super._();

  @override
  String toString() {
    return 'DataType.binary()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is _$DataType_BinaryImpl);
  }

  @override
  int get hashCode => runtimeType.hashCode;
}

abstract class DataType_Binary extends DataType {
  const factory DataType_Binary() = _$DataType_BinaryImpl;
  const DataType_Binary._() : super._();
}

/// @nodoc
abstract class _$$DataType_DateImplCopyWith<$Res> {
  factory _$$DataType_DateImplCopyWith(
          _$DataType_DateImpl value, $Res Function(_$DataType_DateImpl) then) =
      __$$DataType_DateImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$DataType_DateImplCopyWithImpl<$Res>
    extends _$DataTypeCopyWithImpl<$Res, _$DataType_DateImpl>
    implements _$$DataType_DateImplCopyWith<$Res> {
  __$$DataType_DateImplCopyWithImpl(
      _$DataType_DateImpl _value, $Res Function(_$DataType_DateImpl) _then)
      : super(_value, _then);
}

/// @nodoc

class _$DataType_DateImpl extends DataType_Date {
  const _$DataType_DateImpl() : super._();

  @override
  String toString() {
    return 'DataType.date()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is _$DataType_DateImpl);
  }

  @override
  int get hashCode => runtimeType.hashCode;
}

abstract class DataType_Date extends DataType {
  const factory DataType_Date() = _$DataType_DateImpl;
  const DataType_Date._() : super._();
}

/// @nodoc
abstract class _$$DataType_DatetimeImplCopyWith<$Res> {
  factory _$$DataType_DatetimeImplCopyWith(_$DataType_DatetimeImpl value,
          $Res Function(_$DataType_DatetimeImpl) then) =
      __$$DataType_DatetimeImplCopyWithImpl<$Res>;
  @useResult
  $Res call({TimeUnit field0, String? field1});
}

/// @nodoc
class __$$DataType_DatetimeImplCopyWithImpl<$Res>
    extends _$DataTypeCopyWithImpl<$Res, _$DataType_DatetimeImpl>
    implements _$$DataType_DatetimeImplCopyWith<$Res> {
  __$$DataType_DatetimeImplCopyWithImpl(_$DataType_DatetimeImpl _value,
      $Res Function(_$DataType_DatetimeImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
    Object? field1 = freezed,
  }) {
    return _then(_$DataType_DatetimeImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as TimeUnit,
      freezed == field1
          ? _value.field1
          : field1 // ignore: cast_nullable_to_non_nullable
              as String?,
    ));
  }
}

/// @nodoc

class _$DataType_DatetimeImpl extends DataType_Datetime {
  const _$DataType_DatetimeImpl(this.field0, [this.field1]) : super._();

  @override
  final TimeUnit field0;
  @override
  final String? field1;

  @override
  String toString() {
    return 'DataType.datetime(field0: $field0, field1: $field1)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$DataType_DatetimeImpl &&
            (identical(other.field0, field0) || other.field0 == field0) &&
            (identical(other.field1, field1) || other.field1 == field1));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0, field1);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$DataType_DatetimeImplCopyWith<_$DataType_DatetimeImpl> get copyWith =>
      __$$DataType_DatetimeImplCopyWithImpl<_$DataType_DatetimeImpl>(
          this, _$identity);
}

abstract class DataType_Datetime extends DataType {
  const factory DataType_Datetime(final TimeUnit field0,
      [final String? field1]) = _$DataType_DatetimeImpl;
  const DataType_Datetime._() : super._();

  TimeUnit get field0;
  String? get field1;
  @JsonKey(ignore: true)
  _$$DataType_DatetimeImplCopyWith<_$DataType_DatetimeImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$DataType_DurationImplCopyWith<$Res> {
  factory _$$DataType_DurationImplCopyWith(_$DataType_DurationImpl value,
          $Res Function(_$DataType_DurationImpl) then) =
      __$$DataType_DurationImplCopyWithImpl<$Res>;
  @useResult
  $Res call({TimeUnit field0});
}

/// @nodoc
class __$$DataType_DurationImplCopyWithImpl<$Res>
    extends _$DataTypeCopyWithImpl<$Res, _$DataType_DurationImpl>
    implements _$$DataType_DurationImplCopyWith<$Res> {
  __$$DataType_DurationImplCopyWithImpl(_$DataType_DurationImpl _value,
      $Res Function(_$DataType_DurationImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$DataType_DurationImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as TimeUnit,
    ));
  }
}

/// @nodoc

class _$DataType_DurationImpl extends DataType_Duration {
  const _$DataType_DurationImpl(this.field0) : super._();

  @override
  final TimeUnit field0;

  @override
  String toString() {
    return 'DataType.duration(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$DataType_DurationImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$DataType_DurationImplCopyWith<_$DataType_DurationImpl> get copyWith =>
      __$$DataType_DurationImplCopyWithImpl<_$DataType_DurationImpl>(
          this, _$identity);
}

abstract class DataType_Duration extends DataType {
  const factory DataType_Duration(final TimeUnit field0) =
      _$DataType_DurationImpl;
  const DataType_Duration._() : super._();

  TimeUnit get field0;
  @JsonKey(ignore: true)
  _$$DataType_DurationImplCopyWith<_$DataType_DurationImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$DataType_TimeImplCopyWith<$Res> {
  factory _$$DataType_TimeImplCopyWith(
          _$DataType_TimeImpl value, $Res Function(_$DataType_TimeImpl) then) =
      __$$DataType_TimeImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$DataType_TimeImplCopyWithImpl<$Res>
    extends _$DataTypeCopyWithImpl<$Res, _$DataType_TimeImpl>
    implements _$$DataType_TimeImplCopyWith<$Res> {
  __$$DataType_TimeImplCopyWithImpl(
      _$DataType_TimeImpl _value, $Res Function(_$DataType_TimeImpl) _then)
      : super(_value, _then);
}

/// @nodoc

class _$DataType_TimeImpl extends DataType_Time {
  const _$DataType_TimeImpl() : super._();

  @override
  String toString() {
    return 'DataType.time()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is _$DataType_TimeImpl);
  }

  @override
  int get hashCode => runtimeType.hashCode;
}

abstract class DataType_Time extends DataType {
  const factory DataType_Time() = _$DataType_TimeImpl;
  const DataType_Time._() : super._();
}

/// @nodoc
abstract class _$$DataType_ListImplCopyWith<$Res> {
  factory _$$DataType_ListImplCopyWith(
          _$DataType_ListImpl value, $Res Function(_$DataType_ListImpl) then) =
      __$$DataType_ListImplCopyWithImpl<$Res>;
  @useResult
  $Res call({DataType field0});

  $DataTypeCopyWith<$Res> get field0;
}

/// @nodoc
class __$$DataType_ListImplCopyWithImpl<$Res>
    extends _$DataTypeCopyWithImpl<$Res, _$DataType_ListImpl>
    implements _$$DataType_ListImplCopyWith<$Res> {
  __$$DataType_ListImplCopyWithImpl(
      _$DataType_ListImpl _value, $Res Function(_$DataType_ListImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$DataType_ListImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as DataType,
    ));
  }

  @override
  @pragma('vm:prefer-inline')
  $DataTypeCopyWith<$Res> get field0 {
    return $DataTypeCopyWith<$Res>(_value.field0, (value) {
      return _then(_value.copyWith(field0: value));
    });
  }
}

/// @nodoc

class _$DataType_ListImpl extends DataType_List {
  const _$DataType_ListImpl(this.field0) : super._();

  @override
  final DataType field0;

  @override
  String toString() {
    return 'DataType.list(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$DataType_ListImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$DataType_ListImplCopyWith<_$DataType_ListImpl> get copyWith =>
      __$$DataType_ListImplCopyWithImpl<_$DataType_ListImpl>(this, _$identity);
}

abstract class DataType_List extends DataType {
  const factory DataType_List(final DataType field0) = _$DataType_ListImpl;
  const DataType_List._() : super._();

  DataType get field0;
  @JsonKey(ignore: true)
  _$$DataType_ListImplCopyWith<_$DataType_ListImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$DataType_StructImplCopyWith<$Res> {
  factory _$$DataType_StructImplCopyWith(_$DataType_StructImpl value,
          $Res Function(_$DataType_StructImpl) then) =
      __$$DataType_StructImplCopyWithImpl<$Res>;
  @useResult
  $Res call({List<Field> field0});
}

/// @nodoc
class __$$DataType_StructImplCopyWithImpl<$Res>
    extends _$DataTypeCopyWithImpl<$Res, _$DataType_StructImpl>
    implements _$$DataType_StructImplCopyWith<$Res> {
  __$$DataType_StructImplCopyWithImpl(
      _$DataType_StructImpl _value, $Res Function(_$DataType_StructImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$DataType_StructImpl(
      null == field0
          ? _value._field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as List<Field>,
    ));
  }
}

/// @nodoc

class _$DataType_StructImpl extends DataType_Struct {
  const _$DataType_StructImpl(final List<Field> field0)
      : _field0 = field0,
        super._();

  final List<Field> _field0;
  @override
  List<Field> get field0 {
    if (_field0 is EqualUnmodifiableListView) return _field0;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(_field0);
  }

  @override
  String toString() {
    return 'DataType.struct(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$DataType_StructImpl &&
            const DeepCollectionEquality().equals(other._field0, _field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(_field0));

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$DataType_StructImplCopyWith<_$DataType_StructImpl> get copyWith =>
      __$$DataType_StructImplCopyWithImpl<_$DataType_StructImpl>(
          this, _$identity);
}

abstract class DataType_Struct extends DataType {
  const factory DataType_Struct(final List<Field> field0) =
      _$DataType_StructImpl;
  const DataType_Struct._() : super._();

  List<Field> get field0;
  @JsonKey(ignore: true)
  _$$DataType_StructImplCopyWith<_$DataType_StructImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$DataType_NullImplCopyWith<$Res> {
  factory _$$DataType_NullImplCopyWith(
          _$DataType_NullImpl value, $Res Function(_$DataType_NullImpl) then) =
      __$$DataType_NullImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$DataType_NullImplCopyWithImpl<$Res>
    extends _$DataTypeCopyWithImpl<$Res, _$DataType_NullImpl>
    implements _$$DataType_NullImplCopyWith<$Res> {
  __$$DataType_NullImplCopyWithImpl(
      _$DataType_NullImpl _value, $Res Function(_$DataType_NullImpl) _then)
      : super(_value, _then);
}

/// @nodoc

class _$DataType_NullImpl extends DataType_Null {
  const _$DataType_NullImpl() : super._();

  @override
  String toString() {
    return 'DataType.Null()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is _$DataType_NullImpl);
  }

  @override
  int get hashCode => runtimeType.hashCode;
}

abstract class DataType_Null extends DataType {
  const factory DataType_Null() = _$DataType_NullImpl;
  const DataType_Null._() : super._();
}

/// @nodoc
abstract class _$$DataType_UnknownImplCopyWith<$Res> {
  factory _$$DataType_UnknownImplCopyWith(_$DataType_UnknownImpl value,
          $Res Function(_$DataType_UnknownImpl) then) =
      __$$DataType_UnknownImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$DataType_UnknownImplCopyWithImpl<$Res>
    extends _$DataTypeCopyWithImpl<$Res, _$DataType_UnknownImpl>
    implements _$$DataType_UnknownImplCopyWith<$Res> {
  __$$DataType_UnknownImplCopyWithImpl(_$DataType_UnknownImpl _value,
      $Res Function(_$DataType_UnknownImpl) _then)
      : super(_value, _then);
}

/// @nodoc

class _$DataType_UnknownImpl extends DataType_Unknown {
  const _$DataType_UnknownImpl() : super._();

  @override
  String toString() {
    return 'DataType.unknown()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is _$DataType_UnknownImpl);
  }

  @override
  int get hashCode => runtimeType.hashCode;
}

abstract class DataType_Unknown extends DataType {
  const factory DataType_Unknown() = _$DataType_UnknownImpl;
  const DataType_Unknown._() : super._();
}

/// @nodoc
mixin _$Excluded {
  Object get field0 => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $ExcludedCopyWith<$Res> {
  factory $ExcludedCopyWith(Excluded value, $Res Function(Excluded) then) =
      _$ExcludedCopyWithImpl<$Res, Excluded>;
}

/// @nodoc
class _$ExcludedCopyWithImpl<$Res, $Val extends Excluded>
    implements $ExcludedCopyWith<$Res> {
  _$ExcludedCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;
}

/// @nodoc
abstract class _$$Excluded_NameImplCopyWith<$Res> {
  factory _$$Excluded_NameImplCopyWith(
          _$Excluded_NameImpl value, $Res Function(_$Excluded_NameImpl) then) =
      __$$Excluded_NameImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String field0});
}

/// @nodoc
class __$$Excluded_NameImplCopyWithImpl<$Res>
    extends _$ExcludedCopyWithImpl<$Res, _$Excluded_NameImpl>
    implements _$$Excluded_NameImplCopyWith<$Res> {
  __$$Excluded_NameImplCopyWithImpl(
      _$Excluded_NameImpl _value, $Res Function(_$Excluded_NameImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$Excluded_NameImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$Excluded_NameImpl extends Excluded_Name {
  const _$Excluded_NameImpl(this.field0) : super._();

  @override
  final String field0;

  @override
  String toString() {
    return 'Excluded.name(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Excluded_NameImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$Excluded_NameImplCopyWith<_$Excluded_NameImpl> get copyWith =>
      __$$Excluded_NameImplCopyWithImpl<_$Excluded_NameImpl>(this, _$identity);
}

abstract class Excluded_Name extends Excluded {
  const factory Excluded_Name(final String field0) = _$Excluded_NameImpl;
  const Excluded_Name._() : super._();

  @override
  String get field0;
  @JsonKey(ignore: true)
  _$$Excluded_NameImplCopyWith<_$Excluded_NameImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$Excluded_DtypeImplCopyWith<$Res> {
  factory _$$Excluded_DtypeImplCopyWith(_$Excluded_DtypeImpl value,
          $Res Function(_$Excluded_DtypeImpl) then) =
      __$$Excluded_DtypeImplCopyWithImpl<$Res>;
  @useResult
  $Res call({DataType field0});

  $DataTypeCopyWith<$Res> get field0;
}

/// @nodoc
class __$$Excluded_DtypeImplCopyWithImpl<$Res>
    extends _$ExcludedCopyWithImpl<$Res, _$Excluded_DtypeImpl>
    implements _$$Excluded_DtypeImplCopyWith<$Res> {
  __$$Excluded_DtypeImplCopyWithImpl(
      _$Excluded_DtypeImpl _value, $Res Function(_$Excluded_DtypeImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$Excluded_DtypeImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as DataType,
    ));
  }

  @override
  @pragma('vm:prefer-inline')
  $DataTypeCopyWith<$Res> get field0 {
    return $DataTypeCopyWith<$Res>(_value.field0, (value) {
      return _then(_value.copyWith(field0: value));
    });
  }
}

/// @nodoc

class _$Excluded_DtypeImpl extends Excluded_Dtype {
  const _$Excluded_DtypeImpl(this.field0) : super._();

  @override
  final DataType field0;

  @override
  String toString() {
    return 'Excluded.dtype(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Excluded_DtypeImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$Excluded_DtypeImplCopyWith<_$Excluded_DtypeImpl> get copyWith =>
      __$$Excluded_DtypeImplCopyWithImpl<_$Excluded_DtypeImpl>(
          this, _$identity);
}

abstract class Excluded_Dtype extends Excluded {
  const factory Excluded_Dtype(final DataType field0) = _$Excluded_DtypeImpl;
  const Excluded_Dtype._() : super._();

  @override
  DataType get field0;
  @JsonKey(ignore: true)
  _$$Excluded_DtypeImplCopyWith<_$Excluded_DtypeImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$Expr {}

/// @nodoc
abstract class $ExprCopyWith<$Res> {
  factory $ExprCopyWith(Expr value, $Res Function(Expr) then) =
      _$ExprCopyWithImpl<$Res, Expr>;
}

/// @nodoc
class _$ExprCopyWithImpl<$Res, $Val extends Expr>
    implements $ExprCopyWith<$Res> {
  _$ExprCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;
}

/// @nodoc
abstract class _$$Expr_AliasImplCopyWith<$Res> {
  factory _$$Expr_AliasImplCopyWith(
          _$Expr_AliasImpl value, $Res Function(_$Expr_AliasImpl) then) =
      __$$Expr_AliasImplCopyWithImpl<$Res>;
  @useResult
  $Res call({Expr field0, String field1});

  $ExprCopyWith<$Res> get field0;
}

/// @nodoc
class __$$Expr_AliasImplCopyWithImpl<$Res>
    extends _$ExprCopyWithImpl<$Res, _$Expr_AliasImpl>
    implements _$$Expr_AliasImplCopyWith<$Res> {
  __$$Expr_AliasImplCopyWithImpl(
      _$Expr_AliasImpl _value, $Res Function(_$Expr_AliasImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
    Object? field1 = null,
  }) {
    return _then(_$Expr_AliasImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as Expr,
      null == field1
          ? _value.field1
          : field1 // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }

  @override
  @pragma('vm:prefer-inline')
  $ExprCopyWith<$Res> get field0 {
    return $ExprCopyWith<$Res>(_value.field0, (value) {
      return _then(_value.copyWith(field0: value));
    });
  }
}

/// @nodoc

class _$Expr_AliasImpl extends Expr_Alias {
  const _$Expr_AliasImpl(this.field0, this.field1) : super._();

  @override
  final Expr field0;
  @override
  final String field1;

  @override
  String toString() {
    return 'Expr.alias(field0: $field0, field1: $field1)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Expr_AliasImpl &&
            (identical(other.field0, field0) || other.field0 == field0) &&
            (identical(other.field1, field1) || other.field1 == field1));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0, field1);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$Expr_AliasImplCopyWith<_$Expr_AliasImpl> get copyWith =>
      __$$Expr_AliasImplCopyWithImpl<_$Expr_AliasImpl>(this, _$identity);
}

abstract class Expr_Alias extends Expr {
  const factory Expr_Alias(final Expr field0, final String field1) =
      _$Expr_AliasImpl;
  const Expr_Alias._() : super._();

  Expr get field0;
  String get field1;
  @JsonKey(ignore: true)
  _$$Expr_AliasImplCopyWith<_$Expr_AliasImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$Expr_ColumnImplCopyWith<$Res> {
  factory _$$Expr_ColumnImplCopyWith(
          _$Expr_ColumnImpl value, $Res Function(_$Expr_ColumnImpl) then) =
      __$$Expr_ColumnImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String field0});
}

/// @nodoc
class __$$Expr_ColumnImplCopyWithImpl<$Res>
    extends _$ExprCopyWithImpl<$Res, _$Expr_ColumnImpl>
    implements _$$Expr_ColumnImplCopyWith<$Res> {
  __$$Expr_ColumnImplCopyWithImpl(
      _$Expr_ColumnImpl _value, $Res Function(_$Expr_ColumnImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$Expr_ColumnImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$Expr_ColumnImpl extends Expr_Column {
  const _$Expr_ColumnImpl(this.field0) : super._();

  @override
  final String field0;

  @override
  String toString() {
    return 'Expr.column(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Expr_ColumnImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$Expr_ColumnImplCopyWith<_$Expr_ColumnImpl> get copyWith =>
      __$$Expr_ColumnImplCopyWithImpl<_$Expr_ColumnImpl>(this, _$identity);
}

abstract class Expr_Column extends Expr {
  const factory Expr_Column(final String field0) = _$Expr_ColumnImpl;
  const Expr_Column._() : super._();

  String get field0;
  @JsonKey(ignore: true)
  _$$Expr_ColumnImplCopyWith<_$Expr_ColumnImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$Expr_ColumnsImplCopyWith<$Res> {
  factory _$$Expr_ColumnsImplCopyWith(
          _$Expr_ColumnsImpl value, $Res Function(_$Expr_ColumnsImpl) then) =
      __$$Expr_ColumnsImplCopyWithImpl<$Res>;
  @useResult
  $Res call({List<String> field0});
}

/// @nodoc
class __$$Expr_ColumnsImplCopyWithImpl<$Res>
    extends _$ExprCopyWithImpl<$Res, _$Expr_ColumnsImpl>
    implements _$$Expr_ColumnsImplCopyWith<$Res> {
  __$$Expr_ColumnsImplCopyWithImpl(
      _$Expr_ColumnsImpl _value, $Res Function(_$Expr_ColumnsImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$Expr_ColumnsImpl(
      null == field0
          ? _value._field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as List<String>,
    ));
  }
}

/// @nodoc

class _$Expr_ColumnsImpl extends Expr_Columns {
  const _$Expr_ColumnsImpl(final List<String> field0)
      : _field0 = field0,
        super._();

  final List<String> _field0;
  @override
  List<String> get field0 {
    if (_field0 is EqualUnmodifiableListView) return _field0;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(_field0);
  }

  @override
  String toString() {
    return 'Expr.columns(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Expr_ColumnsImpl &&
            const DeepCollectionEquality().equals(other._field0, _field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(_field0));

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$Expr_ColumnsImplCopyWith<_$Expr_ColumnsImpl> get copyWith =>
      __$$Expr_ColumnsImplCopyWithImpl<_$Expr_ColumnsImpl>(this, _$identity);
}

abstract class Expr_Columns extends Expr {
  const factory Expr_Columns(final List<String> field0) = _$Expr_ColumnsImpl;
  const Expr_Columns._() : super._();

  List<String> get field0;
  @JsonKey(ignore: true)
  _$$Expr_ColumnsImplCopyWith<_$Expr_ColumnsImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$Expr_DtypeColumnImplCopyWith<$Res> {
  factory _$$Expr_DtypeColumnImplCopyWith(_$Expr_DtypeColumnImpl value,
          $Res Function(_$Expr_DtypeColumnImpl) then) =
      __$$Expr_DtypeColumnImplCopyWithImpl<$Res>;
  @useResult
  $Res call({List<DataType> field0});
}

/// @nodoc
class __$$Expr_DtypeColumnImplCopyWithImpl<$Res>
    extends _$ExprCopyWithImpl<$Res, _$Expr_DtypeColumnImpl>
    implements _$$Expr_DtypeColumnImplCopyWith<$Res> {
  __$$Expr_DtypeColumnImplCopyWithImpl(_$Expr_DtypeColumnImpl _value,
      $Res Function(_$Expr_DtypeColumnImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$Expr_DtypeColumnImpl(
      null == field0
          ? _value._field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as List<DataType>,
    ));
  }
}

/// @nodoc

class _$Expr_DtypeColumnImpl extends Expr_DtypeColumn {
  const _$Expr_DtypeColumnImpl(final List<DataType> field0)
      : _field0 = field0,
        super._();

  final List<DataType> _field0;
  @override
  List<DataType> get field0 {
    if (_field0 is EqualUnmodifiableListView) return _field0;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(_field0);
  }

  @override
  String toString() {
    return 'Expr.dtypeColumn(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Expr_DtypeColumnImpl &&
            const DeepCollectionEquality().equals(other._field0, _field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(_field0));

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$Expr_DtypeColumnImplCopyWith<_$Expr_DtypeColumnImpl> get copyWith =>
      __$$Expr_DtypeColumnImplCopyWithImpl<_$Expr_DtypeColumnImpl>(
          this, _$identity);
}

abstract class Expr_DtypeColumn extends Expr {
  const factory Expr_DtypeColumn(final List<DataType> field0) =
      _$Expr_DtypeColumnImpl;
  const Expr_DtypeColumn._() : super._();

  List<DataType> get field0;
  @JsonKey(ignore: true)
  _$$Expr_DtypeColumnImplCopyWith<_$Expr_DtypeColumnImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$Expr_LiteralImplCopyWith<$Res> {
  factory _$$Expr_LiteralImplCopyWith(
          _$Expr_LiteralImpl value, $Res Function(_$Expr_LiteralImpl) then) =
      __$$Expr_LiteralImplCopyWithImpl<$Res>;
  @useResult
  $Res call({LiteralValue field0});

  $LiteralValueCopyWith<$Res> get field0;
}

/// @nodoc
class __$$Expr_LiteralImplCopyWithImpl<$Res>
    extends _$ExprCopyWithImpl<$Res, _$Expr_LiteralImpl>
    implements _$$Expr_LiteralImplCopyWith<$Res> {
  __$$Expr_LiteralImplCopyWithImpl(
      _$Expr_LiteralImpl _value, $Res Function(_$Expr_LiteralImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$Expr_LiteralImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as LiteralValue,
    ));
  }

  @override
  @pragma('vm:prefer-inline')
  $LiteralValueCopyWith<$Res> get field0 {
    return $LiteralValueCopyWith<$Res>(_value.field0, (value) {
      return _then(_value.copyWith(field0: value));
    });
  }
}

/// @nodoc

class _$Expr_LiteralImpl extends Expr_Literal {
  const _$Expr_LiteralImpl(this.field0) : super._();

  @override
  final LiteralValue field0;

  @override
  String toString() {
    return 'Expr.literal(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Expr_LiteralImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$Expr_LiteralImplCopyWith<_$Expr_LiteralImpl> get copyWith =>
      __$$Expr_LiteralImplCopyWithImpl<_$Expr_LiteralImpl>(this, _$identity);
}

abstract class Expr_Literal extends Expr {
  const factory Expr_Literal(final LiteralValue field0) = _$Expr_LiteralImpl;
  const Expr_Literal._() : super._();

  LiteralValue get field0;
  @JsonKey(ignore: true)
  _$$Expr_LiteralImplCopyWith<_$Expr_LiteralImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$Expr_BinaryExprImplCopyWith<$Res> {
  factory _$$Expr_BinaryExprImplCopyWith(_$Expr_BinaryExprImpl value,
          $Res Function(_$Expr_BinaryExprImpl) then) =
      __$$Expr_BinaryExprImplCopyWithImpl<$Res>;
  @useResult
  $Res call({Expr left, Operator op, Expr right});

  $ExprCopyWith<$Res> get left;
  $ExprCopyWith<$Res> get right;
}

/// @nodoc
class __$$Expr_BinaryExprImplCopyWithImpl<$Res>
    extends _$ExprCopyWithImpl<$Res, _$Expr_BinaryExprImpl>
    implements _$$Expr_BinaryExprImplCopyWith<$Res> {
  __$$Expr_BinaryExprImplCopyWithImpl(
      _$Expr_BinaryExprImpl _value, $Res Function(_$Expr_BinaryExprImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? left = null,
    Object? op = null,
    Object? right = null,
  }) {
    return _then(_$Expr_BinaryExprImpl(
      left: null == left
          ? _value.left
          : left // ignore: cast_nullable_to_non_nullable
              as Expr,
      op: null == op
          ? _value.op
          : op // ignore: cast_nullable_to_non_nullable
              as Operator,
      right: null == right
          ? _value.right
          : right // ignore: cast_nullable_to_non_nullable
              as Expr,
    ));
  }

  @override
  @pragma('vm:prefer-inline')
  $ExprCopyWith<$Res> get left {
    return $ExprCopyWith<$Res>(_value.left, (value) {
      return _then(_value.copyWith(left: value));
    });
  }

  @override
  @pragma('vm:prefer-inline')
  $ExprCopyWith<$Res> get right {
    return $ExprCopyWith<$Res>(_value.right, (value) {
      return _then(_value.copyWith(right: value));
    });
  }
}

/// @nodoc

class _$Expr_BinaryExprImpl extends Expr_BinaryExpr {
  const _$Expr_BinaryExprImpl(
      {required this.left, required this.op, required this.right})
      : super._();

  @override
  final Expr left;
  @override
  final Operator op;
  @override
  final Expr right;

  @override
  String toString() {
    return 'Expr.binaryExpr(left: $left, op: $op, right: $right)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Expr_BinaryExprImpl &&
            (identical(other.left, left) || other.left == left) &&
            (identical(other.op, op) || other.op == op) &&
            (identical(other.right, right) || other.right == right));
  }

  @override
  int get hashCode => Object.hash(runtimeType, left, op, right);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$Expr_BinaryExprImplCopyWith<_$Expr_BinaryExprImpl> get copyWith =>
      __$$Expr_BinaryExprImplCopyWithImpl<_$Expr_BinaryExprImpl>(
          this, _$identity);
}

abstract class Expr_BinaryExpr extends Expr {
  const factory Expr_BinaryExpr(
      {required final Expr left,
      required final Operator op,
      required final Expr right}) = _$Expr_BinaryExprImpl;
  const Expr_BinaryExpr._() : super._();

  Expr get left;
  Operator get op;
  Expr get right;
  @JsonKey(ignore: true)
  _$$Expr_BinaryExprImplCopyWith<_$Expr_BinaryExprImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$Expr_CastImplCopyWith<$Res> {
  factory _$$Expr_CastImplCopyWith(
          _$Expr_CastImpl value, $Res Function(_$Expr_CastImpl) then) =
      __$$Expr_CastImplCopyWithImpl<$Res>;
  @useResult
  $Res call({Expr expr, DataType dataType, bool strict});

  $ExprCopyWith<$Res> get expr;
  $DataTypeCopyWith<$Res> get dataType;
}

/// @nodoc
class __$$Expr_CastImplCopyWithImpl<$Res>
    extends _$ExprCopyWithImpl<$Res, _$Expr_CastImpl>
    implements _$$Expr_CastImplCopyWith<$Res> {
  __$$Expr_CastImplCopyWithImpl(
      _$Expr_CastImpl _value, $Res Function(_$Expr_CastImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? expr = null,
    Object? dataType = null,
    Object? strict = null,
  }) {
    return _then(_$Expr_CastImpl(
      expr: null == expr
          ? _value.expr
          : expr // ignore: cast_nullable_to_non_nullable
              as Expr,
      dataType: null == dataType
          ? _value.dataType
          : dataType // ignore: cast_nullable_to_non_nullable
              as DataType,
      strict: null == strict
          ? _value.strict
          : strict // ignore: cast_nullable_to_non_nullable
              as bool,
    ));
  }

  @override
  @pragma('vm:prefer-inline')
  $ExprCopyWith<$Res> get expr {
    return $ExprCopyWith<$Res>(_value.expr, (value) {
      return _then(_value.copyWith(expr: value));
    });
  }

  @override
  @pragma('vm:prefer-inline')
  $DataTypeCopyWith<$Res> get dataType {
    return $DataTypeCopyWith<$Res>(_value.dataType, (value) {
      return _then(_value.copyWith(dataType: value));
    });
  }
}

/// @nodoc

class _$Expr_CastImpl extends Expr_Cast {
  const _$Expr_CastImpl(
      {required this.expr, required this.dataType, required this.strict})
      : super._();

  @override
  final Expr expr;
  @override
  final DataType dataType;
  @override
  final bool strict;

  @override
  String toString() {
    return 'Expr.cast(expr: $expr, dataType: $dataType, strict: $strict)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Expr_CastImpl &&
            (identical(other.expr, expr) || other.expr == expr) &&
            (identical(other.dataType, dataType) ||
                other.dataType == dataType) &&
            (identical(other.strict, strict) || other.strict == strict));
  }

  @override
  int get hashCode => Object.hash(runtimeType, expr, dataType, strict);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$Expr_CastImplCopyWith<_$Expr_CastImpl> get copyWith =>
      __$$Expr_CastImplCopyWithImpl<_$Expr_CastImpl>(this, _$identity);
}

abstract class Expr_Cast extends Expr {
  const factory Expr_Cast(
      {required final Expr expr,
      required final DataType dataType,
      required final bool strict}) = _$Expr_CastImpl;
  const Expr_Cast._() : super._();

  Expr get expr;
  DataType get dataType;
  bool get strict;
  @JsonKey(ignore: true)
  _$$Expr_CastImplCopyWith<_$Expr_CastImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$Expr_SortImplCopyWith<$Res> {
  factory _$$Expr_SortImplCopyWith(
          _$Expr_SortImpl value, $Res Function(_$Expr_SortImpl) then) =
      __$$Expr_SortImplCopyWithImpl<$Res>;
  @useResult
  $Res call({Expr expr, SortOptions options});

  $ExprCopyWith<$Res> get expr;
}

/// @nodoc
class __$$Expr_SortImplCopyWithImpl<$Res>
    extends _$ExprCopyWithImpl<$Res, _$Expr_SortImpl>
    implements _$$Expr_SortImplCopyWith<$Res> {
  __$$Expr_SortImplCopyWithImpl(
      _$Expr_SortImpl _value, $Res Function(_$Expr_SortImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? expr = null,
    Object? options = null,
  }) {
    return _then(_$Expr_SortImpl(
      expr: null == expr
          ? _value.expr
          : expr // ignore: cast_nullable_to_non_nullable
              as Expr,
      options: null == options
          ? _value.options
          : options // ignore: cast_nullable_to_non_nullable
              as SortOptions,
    ));
  }

  @override
  @pragma('vm:prefer-inline')
  $ExprCopyWith<$Res> get expr {
    return $ExprCopyWith<$Res>(_value.expr, (value) {
      return _then(_value.copyWith(expr: value));
    });
  }
}

/// @nodoc

class _$Expr_SortImpl extends Expr_Sort {
  const _$Expr_SortImpl(
      {required this.expr, this.options = const SortOptions()})
      : super._();

  @override
  final Expr expr;
  @override
  @JsonKey()
  final SortOptions options;

  @override
  String toString() {
    return 'Expr.sort(expr: $expr, options: $options)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Expr_SortImpl &&
            (identical(other.expr, expr) || other.expr == expr) &&
            (identical(other.options, options) || other.options == options));
  }

  @override
  int get hashCode => Object.hash(runtimeType, expr, options);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$Expr_SortImplCopyWith<_$Expr_SortImpl> get copyWith =>
      __$$Expr_SortImplCopyWithImpl<_$Expr_SortImpl>(this, _$identity);
}

abstract class Expr_Sort extends Expr {
  const factory Expr_Sort(
      {required final Expr expr, final SortOptions options}) = _$Expr_SortImpl;
  const Expr_Sort._() : super._();

  Expr get expr;
  SortOptions get options;
  @JsonKey(ignore: true)
  _$$Expr_SortImplCopyWith<_$Expr_SortImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$Expr_GatherImplCopyWith<$Res> {
  factory _$$Expr_GatherImplCopyWith(
          _$Expr_GatherImpl value, $Res Function(_$Expr_GatherImpl) then) =
      __$$Expr_GatherImplCopyWithImpl<$Res>;
  @useResult
  $Res call({Expr expr, Expr idx, bool returnsScalar});

  $ExprCopyWith<$Res> get expr;
  $ExprCopyWith<$Res> get idx;
}

/// @nodoc
class __$$Expr_GatherImplCopyWithImpl<$Res>
    extends _$ExprCopyWithImpl<$Res, _$Expr_GatherImpl>
    implements _$$Expr_GatherImplCopyWith<$Res> {
  __$$Expr_GatherImplCopyWithImpl(
      _$Expr_GatherImpl _value, $Res Function(_$Expr_GatherImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? expr = null,
    Object? idx = null,
    Object? returnsScalar = null,
  }) {
    return _then(_$Expr_GatherImpl(
      expr: null == expr
          ? _value.expr
          : expr // ignore: cast_nullable_to_non_nullable
              as Expr,
      idx: null == idx
          ? _value.idx
          : idx // ignore: cast_nullable_to_non_nullable
              as Expr,
      returnsScalar: null == returnsScalar
          ? _value.returnsScalar
          : returnsScalar // ignore: cast_nullable_to_non_nullable
              as bool,
    ));
  }

  @override
  @pragma('vm:prefer-inline')
  $ExprCopyWith<$Res> get expr {
    return $ExprCopyWith<$Res>(_value.expr, (value) {
      return _then(_value.copyWith(expr: value));
    });
  }

  @override
  @pragma('vm:prefer-inline')
  $ExprCopyWith<$Res> get idx {
    return $ExprCopyWith<$Res>(_value.idx, (value) {
      return _then(_value.copyWith(idx: value));
    });
  }
}

/// @nodoc

class _$Expr_GatherImpl extends Expr_Gather {
  const _$Expr_GatherImpl(
      {required this.expr, required this.idx, required this.returnsScalar})
      : super._();

  @override
  final Expr expr;
  @override
  final Expr idx;
  @override
  final bool returnsScalar;

  @override
  String toString() {
    return 'Expr.gather(expr: $expr, idx: $idx, returnsScalar: $returnsScalar)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Expr_GatherImpl &&
            (identical(other.expr, expr) || other.expr == expr) &&
            (identical(other.idx, idx) || other.idx == idx) &&
            (identical(other.returnsScalar, returnsScalar) ||
                other.returnsScalar == returnsScalar));
  }

  @override
  int get hashCode => Object.hash(runtimeType, expr, idx, returnsScalar);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$Expr_GatherImplCopyWith<_$Expr_GatherImpl> get copyWith =>
      __$$Expr_GatherImplCopyWithImpl<_$Expr_GatherImpl>(this, _$identity);
}

abstract class Expr_Gather extends Expr {
  const factory Expr_Gather(
      {required final Expr expr,
      required final Expr idx,
      required final bool returnsScalar}) = _$Expr_GatherImpl;
  const Expr_Gather._() : super._();

  Expr get expr;
  Expr get idx;
  bool get returnsScalar;
  @JsonKey(ignore: true)
  _$$Expr_GatherImplCopyWith<_$Expr_GatherImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$Expr_SortByImplCopyWith<$Res> {
  factory _$$Expr_SortByImplCopyWith(
          _$Expr_SortByImpl value, $Res Function(_$Expr_SortByImpl) then) =
      __$$Expr_SortByImplCopyWithImpl<$Res>;
  @useResult
  $Res call({Expr expr, List<Expr> by, List<bool> descending});

  $ExprCopyWith<$Res> get expr;
}

/// @nodoc
class __$$Expr_SortByImplCopyWithImpl<$Res>
    extends _$ExprCopyWithImpl<$Res, _$Expr_SortByImpl>
    implements _$$Expr_SortByImplCopyWith<$Res> {
  __$$Expr_SortByImplCopyWithImpl(
      _$Expr_SortByImpl _value, $Res Function(_$Expr_SortByImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? expr = null,
    Object? by = null,
    Object? descending = null,
  }) {
    return _then(_$Expr_SortByImpl(
      expr: null == expr
          ? _value.expr
          : expr // ignore: cast_nullable_to_non_nullable
              as Expr,
      by: null == by
          ? _value._by
          : by // ignore: cast_nullable_to_non_nullable
              as List<Expr>,
      descending: null == descending
          ? _value._descending
          : descending // ignore: cast_nullable_to_non_nullable
              as List<bool>,
    ));
  }

  @override
  @pragma('vm:prefer-inline')
  $ExprCopyWith<$Res> get expr {
    return $ExprCopyWith<$Res>(_value.expr, (value) {
      return _then(_value.copyWith(expr: value));
    });
  }
}

/// @nodoc

class _$Expr_SortByImpl extends Expr_SortBy {
  const _$Expr_SortByImpl(
      {required this.expr,
      final List<Expr> by = const [],
      final List<bool> descending = const []})
      : _by = by,
        _descending = descending,
        super._();

  @override
  final Expr expr;
  final List<Expr> _by;
  @override
  @JsonKey()
  List<Expr> get by {
    if (_by is EqualUnmodifiableListView) return _by;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(_by);
  }

  final List<bool> _descending;
  @override
  @JsonKey()
  List<bool> get descending {
    if (_descending is EqualUnmodifiableListView) return _descending;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(_descending);
  }

  @override
  String toString() {
    return 'Expr.sortBy(expr: $expr, by: $by, descending: $descending)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Expr_SortByImpl &&
            (identical(other.expr, expr) || other.expr == expr) &&
            const DeepCollectionEquality().equals(other._by, _by) &&
            const DeepCollectionEquality()
                .equals(other._descending, _descending));
  }

  @override
  int get hashCode => Object.hash(
      runtimeType,
      expr,
      const DeepCollectionEquality().hash(_by),
      const DeepCollectionEquality().hash(_descending));

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$Expr_SortByImplCopyWith<_$Expr_SortByImpl> get copyWith =>
      __$$Expr_SortByImplCopyWithImpl<_$Expr_SortByImpl>(this, _$identity);
}

abstract class Expr_SortBy extends Expr {
  const factory Expr_SortBy(
      {required final Expr expr,
      final List<Expr> by,
      final List<bool> descending}) = _$Expr_SortByImpl;
  const Expr_SortBy._() : super._();

  Expr get expr;
  List<Expr> get by;
  List<bool> get descending;
  @JsonKey(ignore: true)
  _$$Expr_SortByImplCopyWith<_$Expr_SortByImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$Expr_AggImplCopyWith<$Res> {
  factory _$$Expr_AggImplCopyWith(
          _$Expr_AggImpl value, $Res Function(_$Expr_AggImpl) then) =
      __$$Expr_AggImplCopyWithImpl<$Res>;
  @useResult
  $Res call({AggExpr field0});

  $AggExprCopyWith<$Res> get field0;
}

/// @nodoc
class __$$Expr_AggImplCopyWithImpl<$Res>
    extends _$ExprCopyWithImpl<$Res, _$Expr_AggImpl>
    implements _$$Expr_AggImplCopyWith<$Res> {
  __$$Expr_AggImplCopyWithImpl(
      _$Expr_AggImpl _value, $Res Function(_$Expr_AggImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$Expr_AggImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as AggExpr,
    ));
  }

  @override
  @pragma('vm:prefer-inline')
  $AggExprCopyWith<$Res> get field0 {
    return $AggExprCopyWith<$Res>(_value.field0, (value) {
      return _then(_value.copyWith(field0: value));
    });
  }
}

/// @nodoc

class _$Expr_AggImpl extends Expr_Agg {
  const _$Expr_AggImpl(this.field0) : super._();

  @override
  final AggExpr field0;

  @override
  String toString() {
    return 'Expr.agg(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Expr_AggImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$Expr_AggImplCopyWith<_$Expr_AggImpl> get copyWith =>
      __$$Expr_AggImplCopyWithImpl<_$Expr_AggImpl>(this, _$identity);
}

abstract class Expr_Agg extends Expr {
  const factory Expr_Agg(final AggExpr field0) = _$Expr_AggImpl;
  const Expr_Agg._() : super._();

  AggExpr get field0;
  @JsonKey(ignore: true)
  _$$Expr_AggImplCopyWith<_$Expr_AggImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$Expr_TernaryImplCopyWith<$Res> {
  factory _$$Expr_TernaryImplCopyWith(
          _$Expr_TernaryImpl value, $Res Function(_$Expr_TernaryImpl) then) =
      __$$Expr_TernaryImplCopyWithImpl<$Res>;
  @useResult
  $Res call({Expr predicate, Expr truthy, Expr falsy});

  $ExprCopyWith<$Res> get predicate;
  $ExprCopyWith<$Res> get truthy;
  $ExprCopyWith<$Res> get falsy;
}

/// @nodoc
class __$$Expr_TernaryImplCopyWithImpl<$Res>
    extends _$ExprCopyWithImpl<$Res, _$Expr_TernaryImpl>
    implements _$$Expr_TernaryImplCopyWith<$Res> {
  __$$Expr_TernaryImplCopyWithImpl(
      _$Expr_TernaryImpl _value, $Res Function(_$Expr_TernaryImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? predicate = null,
    Object? truthy = null,
    Object? falsy = null,
  }) {
    return _then(_$Expr_TernaryImpl(
      predicate: null == predicate
          ? _value.predicate
          : predicate // ignore: cast_nullable_to_non_nullable
              as Expr,
      truthy: null == truthy
          ? _value.truthy
          : truthy // ignore: cast_nullable_to_non_nullable
              as Expr,
      falsy: null == falsy
          ? _value.falsy
          : falsy // ignore: cast_nullable_to_non_nullable
              as Expr,
    ));
  }

  @override
  @pragma('vm:prefer-inline')
  $ExprCopyWith<$Res> get predicate {
    return $ExprCopyWith<$Res>(_value.predicate, (value) {
      return _then(_value.copyWith(predicate: value));
    });
  }

  @override
  @pragma('vm:prefer-inline')
  $ExprCopyWith<$Res> get truthy {
    return $ExprCopyWith<$Res>(_value.truthy, (value) {
      return _then(_value.copyWith(truthy: value));
    });
  }

  @override
  @pragma('vm:prefer-inline')
  $ExprCopyWith<$Res> get falsy {
    return $ExprCopyWith<$Res>(_value.falsy, (value) {
      return _then(_value.copyWith(falsy: value));
    });
  }
}

/// @nodoc

class _$Expr_TernaryImpl extends Expr_Ternary {
  const _$Expr_TernaryImpl(
      {required this.predicate, required this.truthy, required this.falsy})
      : super._();

  @override
  final Expr predicate;
  @override
  final Expr truthy;
  @override
  final Expr falsy;

  @override
  String toString() {
    return 'Expr.ternary(predicate: $predicate, truthy: $truthy, falsy: $falsy)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Expr_TernaryImpl &&
            (identical(other.predicate, predicate) ||
                other.predicate == predicate) &&
            (identical(other.truthy, truthy) || other.truthy == truthy) &&
            (identical(other.falsy, falsy) || other.falsy == falsy));
  }

  @override
  int get hashCode => Object.hash(runtimeType, predicate, truthy, falsy);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$Expr_TernaryImplCopyWith<_$Expr_TernaryImpl> get copyWith =>
      __$$Expr_TernaryImplCopyWithImpl<_$Expr_TernaryImpl>(this, _$identity);
}

abstract class Expr_Ternary extends Expr {
  const factory Expr_Ternary(
      {required final Expr predicate,
      required final Expr truthy,
      required final Expr falsy}) = _$Expr_TernaryImpl;
  const Expr_Ternary._() : super._();

  Expr get predicate;
  Expr get truthy;
  Expr get falsy;
  @JsonKey(ignore: true)
  _$$Expr_TernaryImplCopyWith<_$Expr_TernaryImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$Expr_ExplodeImplCopyWith<$Res> {
  factory _$$Expr_ExplodeImplCopyWith(
          _$Expr_ExplodeImpl value, $Res Function(_$Expr_ExplodeImpl) then) =
      __$$Expr_ExplodeImplCopyWithImpl<$Res>;
  @useResult
  $Res call({Expr field0});

  $ExprCopyWith<$Res> get field0;
}

/// @nodoc
class __$$Expr_ExplodeImplCopyWithImpl<$Res>
    extends _$ExprCopyWithImpl<$Res, _$Expr_ExplodeImpl>
    implements _$$Expr_ExplodeImplCopyWith<$Res> {
  __$$Expr_ExplodeImplCopyWithImpl(
      _$Expr_ExplodeImpl _value, $Res Function(_$Expr_ExplodeImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$Expr_ExplodeImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as Expr,
    ));
  }

  @override
  @pragma('vm:prefer-inline')
  $ExprCopyWith<$Res> get field0 {
    return $ExprCopyWith<$Res>(_value.field0, (value) {
      return _then(_value.copyWith(field0: value));
    });
  }
}

/// @nodoc

class _$Expr_ExplodeImpl extends Expr_Explode {
  const _$Expr_ExplodeImpl(this.field0) : super._();

  @override
  final Expr field0;

  @override
  String toString() {
    return 'Expr.explode(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Expr_ExplodeImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$Expr_ExplodeImplCopyWith<_$Expr_ExplodeImpl> get copyWith =>
      __$$Expr_ExplodeImplCopyWithImpl<_$Expr_ExplodeImpl>(this, _$identity);
}

abstract class Expr_Explode extends Expr {
  const factory Expr_Explode(final Expr field0) = _$Expr_ExplodeImpl;
  const Expr_Explode._() : super._();

  Expr get field0;
  @JsonKey(ignore: true)
  _$$Expr_ExplodeImplCopyWith<_$Expr_ExplodeImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$Expr_FilterImplCopyWith<$Res> {
  factory _$$Expr_FilterImplCopyWith(
          _$Expr_FilterImpl value, $Res Function(_$Expr_FilterImpl) then) =
      __$$Expr_FilterImplCopyWithImpl<$Res>;
  @useResult
  $Res call({Expr input, Expr by});

  $ExprCopyWith<$Res> get input;
  $ExprCopyWith<$Res> get by;
}

/// @nodoc
class __$$Expr_FilterImplCopyWithImpl<$Res>
    extends _$ExprCopyWithImpl<$Res, _$Expr_FilterImpl>
    implements _$$Expr_FilterImplCopyWith<$Res> {
  __$$Expr_FilterImplCopyWithImpl(
      _$Expr_FilterImpl _value, $Res Function(_$Expr_FilterImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? input = null,
    Object? by = null,
  }) {
    return _then(_$Expr_FilterImpl(
      input: null == input
          ? _value.input
          : input // ignore: cast_nullable_to_non_nullable
              as Expr,
      by: null == by
          ? _value.by
          : by // ignore: cast_nullable_to_non_nullable
              as Expr,
    ));
  }

  @override
  @pragma('vm:prefer-inline')
  $ExprCopyWith<$Res> get input {
    return $ExprCopyWith<$Res>(_value.input, (value) {
      return _then(_value.copyWith(input: value));
    });
  }

  @override
  @pragma('vm:prefer-inline')
  $ExprCopyWith<$Res> get by {
    return $ExprCopyWith<$Res>(_value.by, (value) {
      return _then(_value.copyWith(by: value));
    });
  }
}

/// @nodoc

class _$Expr_FilterImpl extends Expr_Filter {
  const _$Expr_FilterImpl({required this.input, required this.by}) : super._();

  @override
  final Expr input;
  @override
  final Expr by;

  @override
  String toString() {
    return 'Expr.filter(input: $input, by: $by)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Expr_FilterImpl &&
            (identical(other.input, input) || other.input == input) &&
            (identical(other.by, by) || other.by == by));
  }

  @override
  int get hashCode => Object.hash(runtimeType, input, by);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$Expr_FilterImplCopyWith<_$Expr_FilterImpl> get copyWith =>
      __$$Expr_FilterImplCopyWithImpl<_$Expr_FilterImpl>(this, _$identity);
}

abstract class Expr_Filter extends Expr {
  const factory Expr_Filter(
      {required final Expr input, required final Expr by}) = _$Expr_FilterImpl;
  const Expr_Filter._() : super._();

  Expr get input;
  Expr get by;
  @JsonKey(ignore: true)
  _$$Expr_FilterImplCopyWith<_$Expr_FilterImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$Expr_WildcardImplCopyWith<$Res> {
  factory _$$Expr_WildcardImplCopyWith(
          _$Expr_WildcardImpl value, $Res Function(_$Expr_WildcardImpl) then) =
      __$$Expr_WildcardImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$Expr_WildcardImplCopyWithImpl<$Res>
    extends _$ExprCopyWithImpl<$Res, _$Expr_WildcardImpl>
    implements _$$Expr_WildcardImplCopyWith<$Res> {
  __$$Expr_WildcardImplCopyWithImpl(
      _$Expr_WildcardImpl _value, $Res Function(_$Expr_WildcardImpl) _then)
      : super(_value, _then);
}

/// @nodoc

class _$Expr_WildcardImpl extends Expr_Wildcard {
  const _$Expr_WildcardImpl() : super._();

  @override
  String toString() {
    return 'Expr.wildcard()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is _$Expr_WildcardImpl);
  }

  @override
  int get hashCode => runtimeType.hashCode;
}

abstract class Expr_Wildcard extends Expr {
  const factory Expr_Wildcard() = _$Expr_WildcardImpl;
  const Expr_Wildcard._() : super._();
}

/// @nodoc
abstract class _$$Expr_WindowImplCopyWith<$Res> {
  factory _$$Expr_WindowImplCopyWith(
          _$Expr_WindowImpl value, $Res Function(_$Expr_WindowImpl) then) =
      __$$Expr_WindowImplCopyWithImpl<$Res>;
  @useResult
  $Res call({Expr function, List<Expr> partitionBy, WindowType options});

  $ExprCopyWith<$Res> get function;
  $WindowTypeCopyWith<$Res> get options;
}

/// @nodoc
class __$$Expr_WindowImplCopyWithImpl<$Res>
    extends _$ExprCopyWithImpl<$Res, _$Expr_WindowImpl>
    implements _$$Expr_WindowImplCopyWith<$Res> {
  __$$Expr_WindowImplCopyWithImpl(
      _$Expr_WindowImpl _value, $Res Function(_$Expr_WindowImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? function = null,
    Object? partitionBy = null,
    Object? options = null,
  }) {
    return _then(_$Expr_WindowImpl(
      function: null == function
          ? _value.function
          : function // ignore: cast_nullable_to_non_nullable
              as Expr,
      partitionBy: null == partitionBy
          ? _value._partitionBy
          : partitionBy // ignore: cast_nullable_to_non_nullable
              as List<Expr>,
      options: null == options
          ? _value.options
          : options // ignore: cast_nullable_to_non_nullable
              as WindowType,
    ));
  }

  @override
  @pragma('vm:prefer-inline')
  $ExprCopyWith<$Res> get function {
    return $ExprCopyWith<$Res>(_value.function, (value) {
      return _then(_value.copyWith(function: value));
    });
  }

  @override
  @pragma('vm:prefer-inline')
  $WindowTypeCopyWith<$Res> get options {
    return $WindowTypeCopyWith<$Res>(_value.options, (value) {
      return _then(_value.copyWith(options: value));
    });
  }
}

/// @nodoc

class _$Expr_WindowImpl extends Expr_Window {
  const _$Expr_WindowImpl(
      {required this.function,
      required final List<Expr> partitionBy,
      required this.options})
      : _partitionBy = partitionBy,
        super._();

  @override
  final Expr function;
  final List<Expr> _partitionBy;
  @override
  List<Expr> get partitionBy {
    if (_partitionBy is EqualUnmodifiableListView) return _partitionBy;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(_partitionBy);
  }

  @override
  final WindowType options;

  @override
  String toString() {
    return 'Expr.window(function: $function, partitionBy: $partitionBy, options: $options)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Expr_WindowImpl &&
            (identical(other.function, function) ||
                other.function == function) &&
            const DeepCollectionEquality()
                .equals(other._partitionBy, _partitionBy) &&
            (identical(other.options, options) || other.options == options));
  }

  @override
  int get hashCode => Object.hash(runtimeType, function,
      const DeepCollectionEquality().hash(_partitionBy), options);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$Expr_WindowImplCopyWith<_$Expr_WindowImpl> get copyWith =>
      __$$Expr_WindowImplCopyWithImpl<_$Expr_WindowImpl>(this, _$identity);
}

abstract class Expr_Window extends Expr {
  const factory Expr_Window(
      {required final Expr function,
      required final List<Expr> partitionBy,
      required final WindowType options}) = _$Expr_WindowImpl;
  const Expr_Window._() : super._();

  Expr get function;
  List<Expr> get partitionBy;
  WindowType get options;
  @JsonKey(ignore: true)
  _$$Expr_WindowImplCopyWith<_$Expr_WindowImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$Expr_SliceImplCopyWith<$Res> {
  factory _$$Expr_SliceImplCopyWith(
          _$Expr_SliceImpl value, $Res Function(_$Expr_SliceImpl) then) =
      __$$Expr_SliceImplCopyWithImpl<$Res>;
  @useResult
  $Res call({Expr input, Expr offset, Expr length});

  $ExprCopyWith<$Res> get input;
  $ExprCopyWith<$Res> get offset;
  $ExprCopyWith<$Res> get length;
}

/// @nodoc
class __$$Expr_SliceImplCopyWithImpl<$Res>
    extends _$ExprCopyWithImpl<$Res, _$Expr_SliceImpl>
    implements _$$Expr_SliceImplCopyWith<$Res> {
  __$$Expr_SliceImplCopyWithImpl(
      _$Expr_SliceImpl _value, $Res Function(_$Expr_SliceImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? input = null,
    Object? offset = null,
    Object? length = null,
  }) {
    return _then(_$Expr_SliceImpl(
      input: null == input
          ? _value.input
          : input // ignore: cast_nullable_to_non_nullable
              as Expr,
      offset: null == offset
          ? _value.offset
          : offset // ignore: cast_nullable_to_non_nullable
              as Expr,
      length: null == length
          ? _value.length
          : length // ignore: cast_nullable_to_non_nullable
              as Expr,
    ));
  }

  @override
  @pragma('vm:prefer-inline')
  $ExprCopyWith<$Res> get input {
    return $ExprCopyWith<$Res>(_value.input, (value) {
      return _then(_value.copyWith(input: value));
    });
  }

  @override
  @pragma('vm:prefer-inline')
  $ExprCopyWith<$Res> get offset {
    return $ExprCopyWith<$Res>(_value.offset, (value) {
      return _then(_value.copyWith(offset: value));
    });
  }

  @override
  @pragma('vm:prefer-inline')
  $ExprCopyWith<$Res> get length {
    return $ExprCopyWith<$Res>(_value.length, (value) {
      return _then(_value.copyWith(length: value));
    });
  }
}

/// @nodoc

class _$Expr_SliceImpl extends Expr_Slice {
  const _$Expr_SliceImpl(
      {required this.input, required this.offset, required this.length})
      : super._();

  @override
  final Expr input;
  @override
  final Expr offset;
  @override
  final Expr length;

  @override
  String toString() {
    return 'Expr.slice(input: $input, offset: $offset, length: $length)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Expr_SliceImpl &&
            (identical(other.input, input) || other.input == input) &&
            (identical(other.offset, offset) || other.offset == offset) &&
            (identical(other.length, length) || other.length == length));
  }

  @override
  int get hashCode => Object.hash(runtimeType, input, offset, length);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$Expr_SliceImplCopyWith<_$Expr_SliceImpl> get copyWith =>
      __$$Expr_SliceImplCopyWithImpl<_$Expr_SliceImpl>(this, _$identity);
}

abstract class Expr_Slice extends Expr {
  const factory Expr_Slice(
      {required final Expr input,
      required final Expr offset,
      required final Expr length}) = _$Expr_SliceImpl;
  const Expr_Slice._() : super._();

  Expr get input;
  Expr get offset;
  Expr get length;
  @JsonKey(ignore: true)
  _$$Expr_SliceImplCopyWith<_$Expr_SliceImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$Expr_ExcludeImplCopyWith<$Res> {
  factory _$$Expr_ExcludeImplCopyWith(
          _$Expr_ExcludeImpl value, $Res Function(_$Expr_ExcludeImpl) then) =
      __$$Expr_ExcludeImplCopyWithImpl<$Res>;
  @useResult
  $Res call({Expr field0, List<Excluded> field1});

  $ExprCopyWith<$Res> get field0;
}

/// @nodoc
class __$$Expr_ExcludeImplCopyWithImpl<$Res>
    extends _$ExprCopyWithImpl<$Res, _$Expr_ExcludeImpl>
    implements _$$Expr_ExcludeImplCopyWith<$Res> {
  __$$Expr_ExcludeImplCopyWithImpl(
      _$Expr_ExcludeImpl _value, $Res Function(_$Expr_ExcludeImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
    Object? field1 = null,
  }) {
    return _then(_$Expr_ExcludeImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as Expr,
      null == field1
          ? _value._field1
          : field1 // ignore: cast_nullable_to_non_nullable
              as List<Excluded>,
    ));
  }

  @override
  @pragma('vm:prefer-inline')
  $ExprCopyWith<$Res> get field0 {
    return $ExprCopyWith<$Res>(_value.field0, (value) {
      return _then(_value.copyWith(field0: value));
    });
  }
}

/// @nodoc

class _$Expr_ExcludeImpl extends Expr_Exclude {
  const _$Expr_ExcludeImpl(this.field0, final List<Excluded> field1)
      : _field1 = field1,
        super._();

  @override
  final Expr field0;
  final List<Excluded> _field1;
  @override
  List<Excluded> get field1 {
    if (_field1 is EqualUnmodifiableListView) return _field1;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(_field1);
  }

  @override
  String toString() {
    return 'Expr.exclude(field0: $field0, field1: $field1)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Expr_ExcludeImpl &&
            (identical(other.field0, field0) || other.field0 == field0) &&
            const DeepCollectionEquality().equals(other._field1, _field1));
  }

  @override
  int get hashCode => Object.hash(
      runtimeType, field0, const DeepCollectionEquality().hash(_field1));

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$Expr_ExcludeImplCopyWith<_$Expr_ExcludeImpl> get copyWith =>
      __$$Expr_ExcludeImplCopyWithImpl<_$Expr_ExcludeImpl>(this, _$identity);
}

abstract class Expr_Exclude extends Expr {
  const factory Expr_Exclude(final Expr field0, final List<Excluded> field1) =
      _$Expr_ExcludeImpl;
  const Expr_Exclude._() : super._();

  Expr get field0;
  List<Excluded> get field1;
  @JsonKey(ignore: true)
  _$$Expr_ExcludeImplCopyWith<_$Expr_ExcludeImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$Expr_KeepNameImplCopyWith<$Res> {
  factory _$$Expr_KeepNameImplCopyWith(
          _$Expr_KeepNameImpl value, $Res Function(_$Expr_KeepNameImpl) then) =
      __$$Expr_KeepNameImplCopyWithImpl<$Res>;
  @useResult
  $Res call({Expr field0});

  $ExprCopyWith<$Res> get field0;
}

/// @nodoc
class __$$Expr_KeepNameImplCopyWithImpl<$Res>
    extends _$ExprCopyWithImpl<$Res, _$Expr_KeepNameImpl>
    implements _$$Expr_KeepNameImplCopyWith<$Res> {
  __$$Expr_KeepNameImplCopyWithImpl(
      _$Expr_KeepNameImpl _value, $Res Function(_$Expr_KeepNameImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$Expr_KeepNameImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as Expr,
    ));
  }

  @override
  @pragma('vm:prefer-inline')
  $ExprCopyWith<$Res> get field0 {
    return $ExprCopyWith<$Res>(_value.field0, (value) {
      return _then(_value.copyWith(field0: value));
    });
  }
}

/// @nodoc

class _$Expr_KeepNameImpl extends Expr_KeepName {
  const _$Expr_KeepNameImpl(this.field0) : super._();

  @override
  final Expr field0;

  @override
  String toString() {
    return 'Expr.keepName(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Expr_KeepNameImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$Expr_KeepNameImplCopyWith<_$Expr_KeepNameImpl> get copyWith =>
      __$$Expr_KeepNameImplCopyWithImpl<_$Expr_KeepNameImpl>(this, _$identity);
}

abstract class Expr_KeepName extends Expr {
  const factory Expr_KeepName(final Expr field0) = _$Expr_KeepNameImpl;
  const Expr_KeepName._() : super._();

  Expr get field0;
  @JsonKey(ignore: true)
  _$$Expr_KeepNameImplCopyWith<_$Expr_KeepNameImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$Expr_CountImplCopyWith<$Res> {
  factory _$$Expr_CountImplCopyWith(
          _$Expr_CountImpl value, $Res Function(_$Expr_CountImpl) then) =
      __$$Expr_CountImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$Expr_CountImplCopyWithImpl<$Res>
    extends _$ExprCopyWithImpl<$Res, _$Expr_CountImpl>
    implements _$$Expr_CountImplCopyWith<$Res> {
  __$$Expr_CountImplCopyWithImpl(
      _$Expr_CountImpl _value, $Res Function(_$Expr_CountImpl) _then)
      : super(_value, _then);
}

/// @nodoc

class _$Expr_CountImpl extends Expr_Count {
  const _$Expr_CountImpl() : super._();

  @override
  String toString() {
    return 'Expr.count()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is _$Expr_CountImpl);
  }

  @override
  int get hashCode => runtimeType.hashCode;
}

abstract class Expr_Count extends Expr {
  const factory Expr_Count() = _$Expr_CountImpl;
  const Expr_Count._() : super._();
}

/// @nodoc
abstract class _$$Expr_NthImplCopyWith<$Res> {
  factory _$$Expr_NthImplCopyWith(
          _$Expr_NthImpl value, $Res Function(_$Expr_NthImpl) then) =
      __$$Expr_NthImplCopyWithImpl<$Res>;
  @useResult
  $Res call({int field0});
}

/// @nodoc
class __$$Expr_NthImplCopyWithImpl<$Res>
    extends _$ExprCopyWithImpl<$Res, _$Expr_NthImpl>
    implements _$$Expr_NthImplCopyWith<$Res> {
  __$$Expr_NthImplCopyWithImpl(
      _$Expr_NthImpl _value, $Res Function(_$Expr_NthImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$Expr_NthImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// @nodoc

class _$Expr_NthImpl extends Expr_Nth {
  const _$Expr_NthImpl(this.field0) : super._();

  @override
  final int field0;

  @override
  String toString() {
    return 'Expr.nth(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Expr_NthImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$Expr_NthImplCopyWith<_$Expr_NthImpl> get copyWith =>
      __$$Expr_NthImplCopyWithImpl<_$Expr_NthImpl>(this, _$identity);
}

abstract class Expr_Nth extends Expr {
  const factory Expr_Nth(final int field0) = _$Expr_NthImpl;
  const Expr_Nth._() : super._();

  int get field0;
  @JsonKey(ignore: true)
  _$$Expr_NthImplCopyWith<_$Expr_NthImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$Expr_InternalImplCopyWith<$Res> {
  factory _$$Expr_InternalImplCopyWith(
          _$Expr_InternalImpl value, $Res Function(_$Expr_InternalImpl) then) =
      __$$Expr_InternalImplCopyWithImpl<$Res>;
  @useResult
  $Res call({PExpr field0});
}

/// @nodoc
class __$$Expr_InternalImplCopyWithImpl<$Res>
    extends _$ExprCopyWithImpl<$Res, _$Expr_InternalImpl>
    implements _$$Expr_InternalImplCopyWith<$Res> {
  __$$Expr_InternalImplCopyWithImpl(
      _$Expr_InternalImpl _value, $Res Function(_$Expr_InternalImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$Expr_InternalImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as PExpr,
    ));
  }
}

/// @nodoc

class _$Expr_InternalImpl extends Expr_Internal {
  const _$Expr_InternalImpl(this.field0) : super._();

  @override
  final PExpr field0;

  @override
  String toString() {
    return 'Expr.internal(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Expr_InternalImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$Expr_InternalImplCopyWith<_$Expr_InternalImpl> get copyWith =>
      __$$Expr_InternalImplCopyWithImpl<_$Expr_InternalImpl>(this, _$identity);
}

abstract class Expr_Internal extends Expr {
  const factory Expr_Internal(final PExpr field0) = _$Expr_InternalImpl;
  const Expr_Internal._() : super._();

  PExpr get field0;
  @JsonKey(ignore: true)
  _$$Expr_InternalImplCopyWith<_$Expr_InternalImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$LiteralValue {}

/// @nodoc
abstract class $LiteralValueCopyWith<$Res> {
  factory $LiteralValueCopyWith(
          LiteralValue value, $Res Function(LiteralValue) then) =
      _$LiteralValueCopyWithImpl<$Res, LiteralValue>;
}

/// @nodoc
class _$LiteralValueCopyWithImpl<$Res, $Val extends LiteralValue>
    implements $LiteralValueCopyWith<$Res> {
  _$LiteralValueCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;
}

/// @nodoc
abstract class _$$LiteralValue_NullImplCopyWith<$Res> {
  factory _$$LiteralValue_NullImplCopyWith(_$LiteralValue_NullImpl value,
          $Res Function(_$LiteralValue_NullImpl) then) =
      __$$LiteralValue_NullImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$LiteralValue_NullImplCopyWithImpl<$Res>
    extends _$LiteralValueCopyWithImpl<$Res, _$LiteralValue_NullImpl>
    implements _$$LiteralValue_NullImplCopyWith<$Res> {
  __$$LiteralValue_NullImplCopyWithImpl(_$LiteralValue_NullImpl _value,
      $Res Function(_$LiteralValue_NullImpl) _then)
      : super(_value, _then);
}

/// @nodoc

class _$LiteralValue_NullImpl extends LiteralValue_Null {
  const _$LiteralValue_NullImpl() : super._();

  @override
  String toString() {
    return 'LiteralValue.Null()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is _$LiteralValue_NullImpl);
  }

  @override
  int get hashCode => runtimeType.hashCode;
}

abstract class LiteralValue_Null extends LiteralValue {
  const factory LiteralValue_Null() = _$LiteralValue_NullImpl;
  const LiteralValue_Null._() : super._();
}

/// @nodoc
abstract class _$$LiteralValue_BooleanImplCopyWith<$Res> {
  factory _$$LiteralValue_BooleanImplCopyWith(_$LiteralValue_BooleanImpl value,
          $Res Function(_$LiteralValue_BooleanImpl) then) =
      __$$LiteralValue_BooleanImplCopyWithImpl<$Res>;
  @useResult
  $Res call({bool field0});
}

/// @nodoc
class __$$LiteralValue_BooleanImplCopyWithImpl<$Res>
    extends _$LiteralValueCopyWithImpl<$Res, _$LiteralValue_BooleanImpl>
    implements _$$LiteralValue_BooleanImplCopyWith<$Res> {
  __$$LiteralValue_BooleanImplCopyWithImpl(_$LiteralValue_BooleanImpl _value,
      $Res Function(_$LiteralValue_BooleanImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$LiteralValue_BooleanImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as bool,
    ));
  }
}

/// @nodoc

class _$LiteralValue_BooleanImpl extends LiteralValue_Boolean {
  const _$LiteralValue_BooleanImpl(this.field0) : super._();

  @override
  final bool field0;

  @override
  String toString() {
    return 'LiteralValue.boolean(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$LiteralValue_BooleanImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$LiteralValue_BooleanImplCopyWith<_$LiteralValue_BooleanImpl>
      get copyWith =>
          __$$LiteralValue_BooleanImplCopyWithImpl<_$LiteralValue_BooleanImpl>(
              this, _$identity);
}

abstract class LiteralValue_Boolean extends LiteralValue {
  const factory LiteralValue_Boolean(final bool field0) =
      _$LiteralValue_BooleanImpl;
  const LiteralValue_Boolean._() : super._();

  bool get field0;
  @JsonKey(ignore: true)
  _$$LiteralValue_BooleanImplCopyWith<_$LiteralValue_BooleanImpl>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$LiteralValue_Utf8ImplCopyWith<$Res> {
  factory _$$LiteralValue_Utf8ImplCopyWith(_$LiteralValue_Utf8Impl value,
          $Res Function(_$LiteralValue_Utf8Impl) then) =
      __$$LiteralValue_Utf8ImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String field0});
}

/// @nodoc
class __$$LiteralValue_Utf8ImplCopyWithImpl<$Res>
    extends _$LiteralValueCopyWithImpl<$Res, _$LiteralValue_Utf8Impl>
    implements _$$LiteralValue_Utf8ImplCopyWith<$Res> {
  __$$LiteralValue_Utf8ImplCopyWithImpl(_$LiteralValue_Utf8Impl _value,
      $Res Function(_$LiteralValue_Utf8Impl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$LiteralValue_Utf8Impl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$LiteralValue_Utf8Impl extends LiteralValue_Utf8 {
  const _$LiteralValue_Utf8Impl(this.field0) : super._();

  @override
  final String field0;

  @override
  String toString() {
    return 'LiteralValue.utf8(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$LiteralValue_Utf8Impl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$LiteralValue_Utf8ImplCopyWith<_$LiteralValue_Utf8Impl> get copyWith =>
      __$$LiteralValue_Utf8ImplCopyWithImpl<_$LiteralValue_Utf8Impl>(
          this, _$identity);
}

abstract class LiteralValue_Utf8 extends LiteralValue {
  const factory LiteralValue_Utf8(final String field0) =
      _$LiteralValue_Utf8Impl;
  const LiteralValue_Utf8._() : super._();

  String get field0;
  @JsonKey(ignore: true)
  _$$LiteralValue_Utf8ImplCopyWith<_$LiteralValue_Utf8Impl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$LiteralValue_BinaryImplCopyWith<$Res> {
  factory _$$LiteralValue_BinaryImplCopyWith(_$LiteralValue_BinaryImpl value,
          $Res Function(_$LiteralValue_BinaryImpl) then) =
      __$$LiteralValue_BinaryImplCopyWithImpl<$Res>;
  @useResult
  $Res call({Uint8List field0});
}

/// @nodoc
class __$$LiteralValue_BinaryImplCopyWithImpl<$Res>
    extends _$LiteralValueCopyWithImpl<$Res, _$LiteralValue_BinaryImpl>
    implements _$$LiteralValue_BinaryImplCopyWith<$Res> {
  __$$LiteralValue_BinaryImplCopyWithImpl(_$LiteralValue_BinaryImpl _value,
      $Res Function(_$LiteralValue_BinaryImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$LiteralValue_BinaryImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as Uint8List,
    ));
  }
}

/// @nodoc

class _$LiteralValue_BinaryImpl extends LiteralValue_Binary {
  const _$LiteralValue_BinaryImpl(this.field0) : super._();

  @override
  final Uint8List field0;

  @override
  String toString() {
    return 'LiteralValue.binary(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$LiteralValue_BinaryImpl &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$LiteralValue_BinaryImplCopyWith<_$LiteralValue_BinaryImpl> get copyWith =>
      __$$LiteralValue_BinaryImplCopyWithImpl<_$LiteralValue_BinaryImpl>(
          this, _$identity);
}

abstract class LiteralValue_Binary extends LiteralValue {
  const factory LiteralValue_Binary(final Uint8List field0) =
      _$LiteralValue_BinaryImpl;
  const LiteralValue_Binary._() : super._();

  Uint8List get field0;
  @JsonKey(ignore: true)
  _$$LiteralValue_BinaryImplCopyWith<_$LiteralValue_BinaryImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$LiteralValue_Uint32ImplCopyWith<$Res> {
  factory _$$LiteralValue_Uint32ImplCopyWith(_$LiteralValue_Uint32Impl value,
          $Res Function(_$LiteralValue_Uint32Impl) then) =
      __$$LiteralValue_Uint32ImplCopyWithImpl<$Res>;
  @useResult
  $Res call({int field0});
}

/// @nodoc
class __$$LiteralValue_Uint32ImplCopyWithImpl<$Res>
    extends _$LiteralValueCopyWithImpl<$Res, _$LiteralValue_Uint32Impl>
    implements _$$LiteralValue_Uint32ImplCopyWith<$Res> {
  __$$LiteralValue_Uint32ImplCopyWithImpl(_$LiteralValue_Uint32Impl _value,
      $Res Function(_$LiteralValue_Uint32Impl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$LiteralValue_Uint32Impl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// @nodoc

class _$LiteralValue_Uint32Impl extends LiteralValue_Uint32 {
  const _$LiteralValue_Uint32Impl(this.field0) : super._();

  @override
  final int field0;

  @override
  String toString() {
    return 'LiteralValue.uint32(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$LiteralValue_Uint32Impl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$LiteralValue_Uint32ImplCopyWith<_$LiteralValue_Uint32Impl> get copyWith =>
      __$$LiteralValue_Uint32ImplCopyWithImpl<_$LiteralValue_Uint32Impl>(
          this, _$identity);
}

abstract class LiteralValue_Uint32 extends LiteralValue {
  const factory LiteralValue_Uint32(final int field0) =
      _$LiteralValue_Uint32Impl;
  const LiteralValue_Uint32._() : super._();

  int get field0;
  @JsonKey(ignore: true)
  _$$LiteralValue_Uint32ImplCopyWith<_$LiteralValue_Uint32Impl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$LiteralValue_Uint64ImplCopyWith<$Res> {
  factory _$$LiteralValue_Uint64ImplCopyWith(_$LiteralValue_Uint64Impl value,
          $Res Function(_$LiteralValue_Uint64Impl) then) =
      __$$LiteralValue_Uint64ImplCopyWithImpl<$Res>;
  @useResult
  $Res call({int field0});
}

/// @nodoc
class __$$LiteralValue_Uint64ImplCopyWithImpl<$Res>
    extends _$LiteralValueCopyWithImpl<$Res, _$LiteralValue_Uint64Impl>
    implements _$$LiteralValue_Uint64ImplCopyWith<$Res> {
  __$$LiteralValue_Uint64ImplCopyWithImpl(_$LiteralValue_Uint64Impl _value,
      $Res Function(_$LiteralValue_Uint64Impl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$LiteralValue_Uint64Impl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// @nodoc

class _$LiteralValue_Uint64Impl extends LiteralValue_Uint64 {
  const _$LiteralValue_Uint64Impl(this.field0) : super._();

  @override
  final int field0;

  @override
  String toString() {
    return 'LiteralValue.uint64(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$LiteralValue_Uint64Impl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$LiteralValue_Uint64ImplCopyWith<_$LiteralValue_Uint64Impl> get copyWith =>
      __$$LiteralValue_Uint64ImplCopyWithImpl<_$LiteralValue_Uint64Impl>(
          this, _$identity);
}

abstract class LiteralValue_Uint64 extends LiteralValue {
  const factory LiteralValue_Uint64(final int field0) =
      _$LiteralValue_Uint64Impl;
  const LiteralValue_Uint64._() : super._();

  int get field0;
  @JsonKey(ignore: true)
  _$$LiteralValue_Uint64ImplCopyWith<_$LiteralValue_Uint64Impl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$LiteralValue_Int32ImplCopyWith<$Res> {
  factory _$$LiteralValue_Int32ImplCopyWith(_$LiteralValue_Int32Impl value,
          $Res Function(_$LiteralValue_Int32Impl) then) =
      __$$LiteralValue_Int32ImplCopyWithImpl<$Res>;
  @useResult
  $Res call({int field0});
}

/// @nodoc
class __$$LiteralValue_Int32ImplCopyWithImpl<$Res>
    extends _$LiteralValueCopyWithImpl<$Res, _$LiteralValue_Int32Impl>
    implements _$$LiteralValue_Int32ImplCopyWith<$Res> {
  __$$LiteralValue_Int32ImplCopyWithImpl(_$LiteralValue_Int32Impl _value,
      $Res Function(_$LiteralValue_Int32Impl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$LiteralValue_Int32Impl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// @nodoc

class _$LiteralValue_Int32Impl extends LiteralValue_Int32 {
  const _$LiteralValue_Int32Impl(this.field0) : super._();

  @override
  final int field0;

  @override
  String toString() {
    return 'LiteralValue.int32(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$LiteralValue_Int32Impl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$LiteralValue_Int32ImplCopyWith<_$LiteralValue_Int32Impl> get copyWith =>
      __$$LiteralValue_Int32ImplCopyWithImpl<_$LiteralValue_Int32Impl>(
          this, _$identity);
}

abstract class LiteralValue_Int32 extends LiteralValue {
  const factory LiteralValue_Int32(final int field0) = _$LiteralValue_Int32Impl;
  const LiteralValue_Int32._() : super._();

  int get field0;
  @JsonKey(ignore: true)
  _$$LiteralValue_Int32ImplCopyWith<_$LiteralValue_Int32Impl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$LiteralValue_Int64ImplCopyWith<$Res> {
  factory _$$LiteralValue_Int64ImplCopyWith(_$LiteralValue_Int64Impl value,
          $Res Function(_$LiteralValue_Int64Impl) then) =
      __$$LiteralValue_Int64ImplCopyWithImpl<$Res>;
  @useResult
  $Res call({int field0});
}

/// @nodoc
class __$$LiteralValue_Int64ImplCopyWithImpl<$Res>
    extends _$LiteralValueCopyWithImpl<$Res, _$LiteralValue_Int64Impl>
    implements _$$LiteralValue_Int64ImplCopyWith<$Res> {
  __$$LiteralValue_Int64ImplCopyWithImpl(_$LiteralValue_Int64Impl _value,
      $Res Function(_$LiteralValue_Int64Impl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$LiteralValue_Int64Impl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// @nodoc

class _$LiteralValue_Int64Impl extends LiteralValue_Int64 {
  const _$LiteralValue_Int64Impl(this.field0) : super._();

  @override
  final int field0;

  @override
  String toString() {
    return 'LiteralValue.int64(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$LiteralValue_Int64Impl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$LiteralValue_Int64ImplCopyWith<_$LiteralValue_Int64Impl> get copyWith =>
      __$$LiteralValue_Int64ImplCopyWithImpl<_$LiteralValue_Int64Impl>(
          this, _$identity);
}

abstract class LiteralValue_Int64 extends LiteralValue {
  const factory LiteralValue_Int64(final int field0) = _$LiteralValue_Int64Impl;
  const LiteralValue_Int64._() : super._();

  int get field0;
  @JsonKey(ignore: true)
  _$$LiteralValue_Int64ImplCopyWith<_$LiteralValue_Int64Impl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$LiteralValue_Float32ImplCopyWith<$Res> {
  factory _$$LiteralValue_Float32ImplCopyWith(_$LiteralValue_Float32Impl value,
          $Res Function(_$LiteralValue_Float32Impl) then) =
      __$$LiteralValue_Float32ImplCopyWithImpl<$Res>;
  @useResult
  $Res call({double field0});
}

/// @nodoc
class __$$LiteralValue_Float32ImplCopyWithImpl<$Res>
    extends _$LiteralValueCopyWithImpl<$Res, _$LiteralValue_Float32Impl>
    implements _$$LiteralValue_Float32ImplCopyWith<$Res> {
  __$$LiteralValue_Float32ImplCopyWithImpl(_$LiteralValue_Float32Impl _value,
      $Res Function(_$LiteralValue_Float32Impl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$LiteralValue_Float32Impl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as double,
    ));
  }
}

/// @nodoc

class _$LiteralValue_Float32Impl extends LiteralValue_Float32 {
  const _$LiteralValue_Float32Impl(this.field0) : super._();

  @override
  final double field0;

  @override
  String toString() {
    return 'LiteralValue.float32(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$LiteralValue_Float32Impl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$LiteralValue_Float32ImplCopyWith<_$LiteralValue_Float32Impl>
      get copyWith =>
          __$$LiteralValue_Float32ImplCopyWithImpl<_$LiteralValue_Float32Impl>(
              this, _$identity);
}

abstract class LiteralValue_Float32 extends LiteralValue {
  const factory LiteralValue_Float32(final double field0) =
      _$LiteralValue_Float32Impl;
  const LiteralValue_Float32._() : super._();

  double get field0;
  @JsonKey(ignore: true)
  _$$LiteralValue_Float32ImplCopyWith<_$LiteralValue_Float32Impl>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$LiteralValue_Float64ImplCopyWith<$Res> {
  factory _$$LiteralValue_Float64ImplCopyWith(_$LiteralValue_Float64Impl value,
          $Res Function(_$LiteralValue_Float64Impl) then) =
      __$$LiteralValue_Float64ImplCopyWithImpl<$Res>;
  @useResult
  $Res call({double field0});
}

/// @nodoc
class __$$LiteralValue_Float64ImplCopyWithImpl<$Res>
    extends _$LiteralValueCopyWithImpl<$Res, _$LiteralValue_Float64Impl>
    implements _$$LiteralValue_Float64ImplCopyWith<$Res> {
  __$$LiteralValue_Float64ImplCopyWithImpl(_$LiteralValue_Float64Impl _value,
      $Res Function(_$LiteralValue_Float64Impl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$LiteralValue_Float64Impl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as double,
    ));
  }
}

/// @nodoc

class _$LiteralValue_Float64Impl extends LiteralValue_Float64 {
  const _$LiteralValue_Float64Impl(this.field0) : super._();

  @override
  final double field0;

  @override
  String toString() {
    return 'LiteralValue.float64(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$LiteralValue_Float64Impl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$LiteralValue_Float64ImplCopyWith<_$LiteralValue_Float64Impl>
      get copyWith =>
          __$$LiteralValue_Float64ImplCopyWithImpl<_$LiteralValue_Float64Impl>(
              this, _$identity);
}

abstract class LiteralValue_Float64 extends LiteralValue {
  const factory LiteralValue_Float64(final double field0) =
      _$LiteralValue_Float64Impl;
  const LiteralValue_Float64._() : super._();

  double get field0;
  @JsonKey(ignore: true)
  _$$LiteralValue_Float64ImplCopyWith<_$LiteralValue_Float64Impl>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$LiteralValue_RangeImplCopyWith<$Res> {
  factory _$$LiteralValue_RangeImplCopyWith(_$LiteralValue_RangeImpl value,
          $Res Function(_$LiteralValue_RangeImpl) then) =
      __$$LiteralValue_RangeImplCopyWithImpl<$Res>;
  @useResult
  $Res call({int low, int high, DataType dataType});

  $DataTypeCopyWith<$Res> get dataType;
}

/// @nodoc
class __$$LiteralValue_RangeImplCopyWithImpl<$Res>
    extends _$LiteralValueCopyWithImpl<$Res, _$LiteralValue_RangeImpl>
    implements _$$LiteralValue_RangeImplCopyWith<$Res> {
  __$$LiteralValue_RangeImplCopyWithImpl(_$LiteralValue_RangeImpl _value,
      $Res Function(_$LiteralValue_RangeImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? low = null,
    Object? high = null,
    Object? dataType = null,
  }) {
    return _then(_$LiteralValue_RangeImpl(
      low: null == low
          ? _value.low
          : low // ignore: cast_nullable_to_non_nullable
              as int,
      high: null == high
          ? _value.high
          : high // ignore: cast_nullable_to_non_nullable
              as int,
      dataType: null == dataType
          ? _value.dataType
          : dataType // ignore: cast_nullable_to_non_nullable
              as DataType,
    ));
  }

  @override
  @pragma('vm:prefer-inline')
  $DataTypeCopyWith<$Res> get dataType {
    return $DataTypeCopyWith<$Res>(_value.dataType, (value) {
      return _then(_value.copyWith(dataType: value));
    });
  }
}

/// @nodoc

class _$LiteralValue_RangeImpl extends LiteralValue_Range {
  const _$LiteralValue_RangeImpl(
      {required this.low, required this.high, required this.dataType})
      : super._();

  /// The starting value of the range.
  @override
  final int low;

  /// The ending value of the range.
  @override
  final int high;

  /// The datatype of this range's ends.
  @override
  final DataType dataType;

  @override
  String toString() {
    return 'LiteralValue.range(low: $low, high: $high, dataType: $dataType)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$LiteralValue_RangeImpl &&
            (identical(other.low, low) || other.low == low) &&
            (identical(other.high, high) || other.high == high) &&
            (identical(other.dataType, dataType) ||
                other.dataType == dataType));
  }

  @override
  int get hashCode => Object.hash(runtimeType, low, high, dataType);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$LiteralValue_RangeImplCopyWith<_$LiteralValue_RangeImpl> get copyWith =>
      __$$LiteralValue_RangeImplCopyWithImpl<_$LiteralValue_RangeImpl>(
          this, _$identity);
}

abstract class LiteralValue_Range extends LiteralValue {
  const factory LiteralValue_Range(
      {required final int low,
      required final int high,
      required final DataType dataType}) = _$LiteralValue_RangeImpl;
  const LiteralValue_Range._() : super._();

  /// The starting value of the range.
  int get low;

  /// The ending value of the range.
  int get high;

  /// The datatype of this range's ends.
  DataType get dataType;
  @JsonKey(ignore: true)
  _$$LiteralValue_RangeImplCopyWith<_$LiteralValue_RangeImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$LiteralValue_DateTimeImplCopyWith<$Res> {
  factory _$$LiteralValue_DateTimeImplCopyWith(
          _$LiteralValue_DateTimeImpl value,
          $Res Function(_$LiteralValue_DateTimeImpl) then) =
      __$$LiteralValue_DateTimeImplCopyWithImpl<$Res>;
  @useResult
  $Res call({int field0, TimeUnit field1, String? field2});
}

/// @nodoc
class __$$LiteralValue_DateTimeImplCopyWithImpl<$Res>
    extends _$LiteralValueCopyWithImpl<$Res, _$LiteralValue_DateTimeImpl>
    implements _$$LiteralValue_DateTimeImplCopyWith<$Res> {
  __$$LiteralValue_DateTimeImplCopyWithImpl(_$LiteralValue_DateTimeImpl _value,
      $Res Function(_$LiteralValue_DateTimeImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
    Object? field1 = null,
    Object? field2 = freezed,
  }) {
    return _then(_$LiteralValue_DateTimeImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as int,
      null == field1
          ? _value.field1
          : field1 // ignore: cast_nullable_to_non_nullable
              as TimeUnit,
      freezed == field2
          ? _value.field2
          : field2 // ignore: cast_nullable_to_non_nullable
              as String?,
    ));
  }
}

/// @nodoc

class _$LiteralValue_DateTimeImpl extends LiteralValue_DateTime {
  const _$LiteralValue_DateTimeImpl(this.field0, this.field1, [this.field2])
      : super._();

  @override
  final int field0;
  @override
  final TimeUnit field1;
  @override
  final String? field2;

  @override
  String toString() {
    return 'LiteralValue.dateTime(field0: $field0, field1: $field1, field2: $field2)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$LiteralValue_DateTimeImpl &&
            (identical(other.field0, field0) || other.field0 == field0) &&
            (identical(other.field1, field1) || other.field1 == field1) &&
            (identical(other.field2, field2) || other.field2 == field2));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0, field1, field2);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$LiteralValue_DateTimeImplCopyWith<_$LiteralValue_DateTimeImpl>
      get copyWith => __$$LiteralValue_DateTimeImplCopyWithImpl<
          _$LiteralValue_DateTimeImpl>(this, _$identity);
}

abstract class LiteralValue_DateTime extends LiteralValue {
  const factory LiteralValue_DateTime(final int field0, final TimeUnit field1,
      [final String? field2]) = _$LiteralValue_DateTimeImpl;
  const LiteralValue_DateTime._() : super._();

  int get field0;
  TimeUnit get field1;
  String? get field2;
  @JsonKey(ignore: true)
  _$$LiteralValue_DateTimeImplCopyWith<_$LiteralValue_DateTimeImpl>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$LiteralValue_DurationImplCopyWith<$Res> {
  factory _$$LiteralValue_DurationImplCopyWith(
          _$LiteralValue_DurationImpl value,
          $Res Function(_$LiteralValue_DurationImpl) then) =
      __$$LiteralValue_DurationImplCopyWithImpl<$Res>;
  @useResult
  $Res call({int field0, TimeUnit field1});
}

/// @nodoc
class __$$LiteralValue_DurationImplCopyWithImpl<$Res>
    extends _$LiteralValueCopyWithImpl<$Res, _$LiteralValue_DurationImpl>
    implements _$$LiteralValue_DurationImplCopyWith<$Res> {
  __$$LiteralValue_DurationImplCopyWithImpl(_$LiteralValue_DurationImpl _value,
      $Res Function(_$LiteralValue_DurationImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
    Object? field1 = null,
  }) {
    return _then(_$LiteralValue_DurationImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as int,
      null == field1
          ? _value.field1
          : field1 // ignore: cast_nullable_to_non_nullable
              as TimeUnit,
    ));
  }
}

/// @nodoc

class _$LiteralValue_DurationImpl extends LiteralValue_Duration {
  const _$LiteralValue_DurationImpl(this.field0, this.field1) : super._();

  @override
  final int field0;
  @override
  final TimeUnit field1;

  @override
  String toString() {
    return 'LiteralValue.duration(field0: $field0, field1: $field1)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$LiteralValue_DurationImpl &&
            (identical(other.field0, field0) || other.field0 == field0) &&
            (identical(other.field1, field1) || other.field1 == field1));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0, field1);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$LiteralValue_DurationImplCopyWith<_$LiteralValue_DurationImpl>
      get copyWith => __$$LiteralValue_DurationImplCopyWithImpl<
          _$LiteralValue_DurationImpl>(this, _$identity);
}

abstract class LiteralValue_Duration extends LiteralValue {
  const factory LiteralValue_Duration(final int field0, final TimeUnit field1) =
      _$LiteralValue_DurationImpl;
  const LiteralValue_Duration._() : super._();

  int get field0;
  TimeUnit get field1;
  @JsonKey(ignore: true)
  _$$LiteralValue_DurationImplCopyWith<_$LiteralValue_DurationImpl>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$LiteralValue_SeriesImplCopyWith<$Res> {
  factory _$$LiteralValue_SeriesImplCopyWith(_$LiteralValue_SeriesImpl value,
          $Res Function(_$LiteralValue_SeriesImpl) then) =
      __$$LiteralValue_SeriesImplCopyWithImpl<$Res>;
  @useResult
  $Res call({SpecialEqPSeries field0});
}

/// @nodoc
class __$$LiteralValue_SeriesImplCopyWithImpl<$Res>
    extends _$LiteralValueCopyWithImpl<$Res, _$LiteralValue_SeriesImpl>
    implements _$$LiteralValue_SeriesImplCopyWith<$Res> {
  __$$LiteralValue_SeriesImplCopyWithImpl(_$LiteralValue_SeriesImpl _value,
      $Res Function(_$LiteralValue_SeriesImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$LiteralValue_SeriesImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as SpecialEqPSeries,
    ));
  }
}

/// @nodoc

class _$LiteralValue_SeriesImpl extends LiteralValue_Series {
  const _$LiteralValue_SeriesImpl(this.field0) : super._();

  @override
  final SpecialEqPSeries field0;

  @override
  String toString() {
    return 'LiteralValue.series(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$LiteralValue_SeriesImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$LiteralValue_SeriesImplCopyWith<_$LiteralValue_SeriesImpl> get copyWith =>
      __$$LiteralValue_SeriesImplCopyWithImpl<_$LiteralValue_SeriesImpl>(
          this, _$identity);
}

abstract class LiteralValue_Series extends LiteralValue {
  const factory LiteralValue_Series(final SpecialEqPSeries field0) =
      _$LiteralValue_SeriesImpl;
  const LiteralValue_Series._() : super._();

  SpecialEqPSeries get field0;
  @JsonKey(ignore: true)
  _$$LiteralValue_SeriesImplCopyWith<_$LiteralValue_SeriesImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$LiteralValue_DateImplCopyWith<$Res> {
  factory _$$LiteralValue_DateImplCopyWith(_$LiteralValue_DateImpl value,
          $Res Function(_$LiteralValue_DateImpl) then) =
      __$$LiteralValue_DateImplCopyWithImpl<$Res>;
  @useResult
  $Res call({int field0});
}

/// @nodoc
class __$$LiteralValue_DateImplCopyWithImpl<$Res>
    extends _$LiteralValueCopyWithImpl<$Res, _$LiteralValue_DateImpl>
    implements _$$LiteralValue_DateImplCopyWith<$Res> {
  __$$LiteralValue_DateImplCopyWithImpl(_$LiteralValue_DateImpl _value,
      $Res Function(_$LiteralValue_DateImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$LiteralValue_DateImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// @nodoc

class _$LiteralValue_DateImpl extends LiteralValue_Date {
  const _$LiteralValue_DateImpl(this.field0) : super._();

  @override
  final int field0;

  @override
  String toString() {
    return 'LiteralValue.date(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$LiteralValue_DateImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$LiteralValue_DateImplCopyWith<_$LiteralValue_DateImpl> get copyWith =>
      __$$LiteralValue_DateImplCopyWithImpl<_$LiteralValue_DateImpl>(
          this, _$identity);
}

abstract class LiteralValue_Date extends LiteralValue {
  const factory LiteralValue_Date(final int field0) = _$LiteralValue_DateImpl;
  const LiteralValue_Date._() : super._();

  int get field0;
  @JsonKey(ignore: true)
  _$$LiteralValue_DateImplCopyWith<_$LiteralValue_DateImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$LiteralValue_TimeImplCopyWith<$Res> {
  factory _$$LiteralValue_TimeImplCopyWith(_$LiteralValue_TimeImpl value,
          $Res Function(_$LiteralValue_TimeImpl) then) =
      __$$LiteralValue_TimeImplCopyWithImpl<$Res>;
  @useResult
  $Res call({int field0});
}

/// @nodoc
class __$$LiteralValue_TimeImplCopyWithImpl<$Res>
    extends _$LiteralValueCopyWithImpl<$Res, _$LiteralValue_TimeImpl>
    implements _$$LiteralValue_TimeImplCopyWith<$Res> {
  __$$LiteralValue_TimeImplCopyWithImpl(_$LiteralValue_TimeImpl _value,
      $Res Function(_$LiteralValue_TimeImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$LiteralValue_TimeImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// @nodoc

class _$LiteralValue_TimeImpl extends LiteralValue_Time {
  const _$LiteralValue_TimeImpl(this.field0) : super._();

  @override
  final int field0;

  @override
  String toString() {
    return 'LiteralValue.time(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$LiteralValue_TimeImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$LiteralValue_TimeImplCopyWith<_$LiteralValue_TimeImpl> get copyWith =>
      __$$LiteralValue_TimeImplCopyWithImpl<_$LiteralValue_TimeImpl>(
          this, _$identity);
}

abstract class LiteralValue_Time extends LiteralValue {
  const factory LiteralValue_Time(final int field0) = _$LiteralValue_TimeImpl;
  const LiteralValue_Time._() : super._();

  int get field0;
  @JsonKey(ignore: true)
  _$$LiteralValue_TimeImplCopyWith<_$LiteralValue_TimeImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$WindowType {
  WindowMapping get field0 => throw _privateConstructorUsedError;

  @JsonKey(ignore: true)
  $WindowTypeCopyWith<WindowType> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $WindowTypeCopyWith<$Res> {
  factory $WindowTypeCopyWith(
          WindowType value, $Res Function(WindowType) then) =
      _$WindowTypeCopyWithImpl<$Res, WindowType>;
  @useResult
  $Res call({WindowMapping field0});
}

/// @nodoc
class _$WindowTypeCopyWithImpl<$Res, $Val extends WindowType>
    implements $WindowTypeCopyWith<$Res> {
  _$WindowTypeCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_value.copyWith(
      field0: null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as WindowMapping,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$WindowType_OverImplCopyWith<$Res>
    implements $WindowTypeCopyWith<$Res> {
  factory _$$WindowType_OverImplCopyWith(_$WindowType_OverImpl value,
          $Res Function(_$WindowType_OverImpl) then) =
      __$$WindowType_OverImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({WindowMapping field0});
}

/// @nodoc
class __$$WindowType_OverImplCopyWithImpl<$Res>
    extends _$WindowTypeCopyWithImpl<$Res, _$WindowType_OverImpl>
    implements _$$WindowType_OverImplCopyWith<$Res> {
  __$$WindowType_OverImplCopyWithImpl(
      _$WindowType_OverImpl _value, $Res Function(_$WindowType_OverImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$WindowType_OverImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as WindowMapping,
    ));
  }
}

/// @nodoc

class _$WindowType_OverImpl extends WindowType_Over {
  const _$WindowType_OverImpl(this.field0) : super._();

  @override
  final WindowMapping field0;

  @override
  String toString() {
    return 'WindowType.over(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$WindowType_OverImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$WindowType_OverImplCopyWith<_$WindowType_OverImpl> get copyWith =>
      __$$WindowType_OverImplCopyWithImpl<_$WindowType_OverImpl>(
          this, _$identity);
}

abstract class WindowType_Over extends WindowType {
  const factory WindowType_Over(final WindowMapping field0) =
      _$WindowType_OverImpl;
  const WindowType_Over._() : super._();

  @override
  WindowMapping get field0;
  @override
  @JsonKey(ignore: true)
  _$$WindowType_OverImplCopyWith<_$WindowType_OverImpl> get copyWith =>
      throw _privateConstructorUsedError;
}
