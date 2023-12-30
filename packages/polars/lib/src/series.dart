import 'df.dart';
import 'wrapper/series.dart';
import 'wrapper/expr.dart';

Series series<T>(
  List<T> data, {
  String name = '',
  DataType dtype = const DataType.utf8(),
}) =>
    Series.ofLits(values: LiteralsExt.from(data, dtype), name: name);
