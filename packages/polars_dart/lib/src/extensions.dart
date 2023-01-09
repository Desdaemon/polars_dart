import 'wrapper.dart';

extension SeriesExt on Series {
  double? operator [](int index) => get(index: index);
  Series operator +(Series other) => addTo(other: other);
  Series operator -(Series other) => subtract(other: other);
  Series operator *(Series other) => multiply(other: other);
  Series operator /(Series other) => divide(other: other);
  Series operator %(Series other) => remainder(other: other);
}
