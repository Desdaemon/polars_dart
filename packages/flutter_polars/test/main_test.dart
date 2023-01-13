import 'dart:typed_data';

import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_polars/flutter_polars.dart';

void main() {
  testWidgets('polars is correctly loaded', (_) async {
    final series = Series.ofI32(
      bridge: pl,
      name: 'values',
      values: Int32List.fromList([1, 2, 3]),
    );
    expect(series.sum(), completion(6));
  });
}
