import 'dart:async';
import 'wrapper/series.dart';
import 'wrapper/df.dart';

extension SeriesExt on Series {
  double? operator [](int index) => get(index: index);
  Series operator +(Series other) => addTo(other: other);
  Series operator -(Series other) => subtract(other: other);
  Series operator *(Series other) => multiply(other: other);
  Series operator /(Series other) => divide(other: other);
  Series operator %(Series other) => remainder(other: other);
}

extension DataFrameExt on DataFrame {
  /// Retrieves the columns either by name or index.
  Series operator [](Object key) {
    if (key is String) {
      return column(column: key);
    }
    if (key is num) {
      return columnAt(index: key.toInt());
    }

    throw ArgumentError.value(key, 'key', 'must be a String or an integer');
  }
}

/// Use this function to parse the results of [DataFrame.iter] and similar methods.
Future<List<dynamic>> parseRow(FutureOr<List<dynamic>> row,
    {bool growable = true}) async {
  final row_ = await Future.value(row);
  return row_.map(parseCell).toList(growable: growable);
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
