import '../polars.dart';

/// Sugar for `Expr.str*` methods.
///
/// See [Strings](https://docs.pola.rs/user-guide/expressions/strings) for more details.
///
/// Many functions in this namespace accept a `pat` parameter, which is parsed
/// as a regular expression except when the parameter `literal` is set to `true`.
class StrNamespace {
  final Expr expr;
  const StrNamespace(this.expr);

  Expr containsLiteral(Object? pat) => expr.strContainsLiteral(pat: pat.expr);
  Expr contains(Object? pat, {bool strict = true}) =>
      expr.strContains(pat: pat.expr, strict: strict);
  Expr endsWith(Object? pat) => expr.strEndsWith(pat: pat.expr);
  Expr extract(String pat, {required int groupIndex}) =>
      expr.strExtract(pat: pat, groupIndex: groupIndex);
  Expr extractAll(Object? pat) => expr.strExtractAll(pat: pat.expr);
  Expr countMatches(Object? pat, {bool literal = false}) =>
      expr.strCountMatches(pat: pat.expr, literal: literal);
  Expr concat(String delimiter, {bool ignoreNulls = true}) =>
      expr.strConcat(delimiter: delimiter, ignoreNulls: ignoreNulls);
  Expr splitN({required Expr by, required int n}) =>
      expr.strSplitn(by: by, n: n);
  Expr replace(Object? pat, Object? val, {bool literal = false}) =>
      expr.strReplace(pat: pat.expr, val: val.expr, literal: literal);
  Expr replaceN(
    Object? pat,
    Object? val, {
    required int n,
    bool literal = false,
  }) =>
      expr.strReplaceN(pat: pat.expr, val: val.expr, n: n, literal: literal);
  Expr replaceAll(Object? pat, Object? val, {bool literal = false}) =>
      expr.strReplaceAll(pat: pat.expr, val: val.expr, literal: literal);
  Expr stripChars(Object? matches) => expr.strStripChars(matches: matches.expr);
  Expr stripCharsStart(Object? matches) =>
      expr.strStripCharsStart(matches: matches.expr);
  Expr stripCharsEnd(Object? matches) =>
      expr.strStripCharsEnd(matches: matches.expr);
  Expr stripPrefix(Object? prefix) => expr.strStripPrefix(prefix: prefix.expr);
  Expr stripSuffix(Object? suffix) => expr.strStripSuffix(suffix: suffix.expr);
  Expr get toLowercase => expr.strToLowercase();
  Expr get toUppercase => expr.strToUppercase();
  Expr toInteger({required int base, bool strict = true}) =>
      expr.strToInteger(base: base, strict: strict);
  Expr get lenBytes => expr.strLenBytes();
  Expr get lenChars => expr.strLenChars();
  Expr slice(int start, int? length) =>
      expr.strSlice(start: start, length: length);
  Expr get explode => expr.strExplode();

  Expr strptime(
    DataType dtype, {
    String? format,
    bool strict = true,
    bool exact = true,
    bool cache = true,
    Ambiguous ambiguous = Ambiguous.raise,
  }) =>
      expr.strptime(
        dtype: dtype,
        format: format,
        strict: strict,
        exact: exact,
        cache: cache,
        ambiguous: ambiguous,
      );

  Expr toDate({
    String? format,
    bool strict = true,
    bool exact = true,
    bool cache = true,
  }) =>
      expr.strToDate(
        format: format,
        strict: strict,
        exact: exact,
        cache: cache,
      );

  Expr toDatetime({
    String? format,
    TimeUnit? timeUnit,
    String? timeZone,
  }) =>
      expr.strToDatetime(
        format: format,
        timeUnit: timeUnit,
        timeZone: timeZone,
      );

  Expr toTime({
    String? format,
    bool strict = true,
    bool exact = true,
    bool cache = true,
  }) =>
      expr.strToTime(
        format: format,
        strict: strict,
        exact: exact,
        cache: cache,
      );

  Expr split({
    required Object? by,
    bool inclusive = false,
  }) =>
      expr.strSplit(by: by.expr, inclusive: inclusive);

  Expr splitExact({
    required Object? by,
    required int n,
    bool inclusive = false,
  }) =>
      expr.strSplitExact(by: by.expr, n: n, inclusive: inclusive);
}
