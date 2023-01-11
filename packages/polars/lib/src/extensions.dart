import 'dart:async';
import 'wrapper.dart';

extension SeriesExt on Series {
  double? operator [](int index) => get(index: index);
  Series operator +(Series other) => addTo(other: other);
  Series operator -(Series other) => subtract(other: other);
  Series operator *(Series other) => multiply(other: other);
  Series operator /(Series other) => divide(other: other);
  Series operator %(Series other) => remainder(other: other);
}

/// Use this function to parse the results of `DataFrame.iter` and similar methods.
Future<List> parseRow(FutureOr<List> row) async {
  final row_ = await Future<List>.value(row);
  return row_.map(parseCell).toList();
}

// List<dynamic> parseRow(List<dynamic> raw)
final _epoch = DateTime.fromMillisecondsSinceEpoch(0);
dynamic parseCell(dynamic raw) {
  if (raw is List && raw.length == 2) {
    final int data = raw.last;
    switch (raw.first) {
      case 'date':
        // data == days since epoch
        return _epoch.add(Duration(days: data)).toUtc();
      case 'time':
        // data == nanoseconds since midnight
        final seconds = data ~/ 1000000000;
        final microseconds = data % 1000000;
        final now = DateTime.now().toUtc();
        return DateTime.utc(now.year, now.month, now.day)
            .add(Duration(seconds: seconds, microseconds: microseconds));
      case 'duration':
        // data == microseconds
        return Duration(microseconds: data);
      case 'datetime':
        // data == microseconds since epoch
        return _epoch.add(Duration(microseconds: data));
    }
  }

  return raw;
}
