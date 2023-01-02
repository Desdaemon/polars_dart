import 'dart:ffi';
import 'dart:math' as math;

import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:polars_dart/polars_dart.dart';
import 'package:test/test.dart';

import 'helpers.dart';

final dylibPath = Uri.base
    .resolve('polars-wrapper/target/$triple/debug'
        '/${formatDylib('polars_wrapper')}')
    .toFilePath();

final throwsFfiException = throwsA(isA<FfiException>());

void main() {
  final dylib = DynamicLibrary.open(dylibPath);
  final api = PolarsWrapperImpl(dylib);
  test('readCsv', () async {
    final data = await api.readCsv(path: 'test/foo.csv');
    final firstNames = data.column(column: 'first');
    expect(firstNames.asStrings(), completion(['John', 'Bob']));
  });

  group('Series.of', () {
    test('strings', () async {
      const flavors = ['ice cream', 'chocolate', 'mint'];
      final series = Series.ofStrings(
        bridge: api,
        name: 'flavors',
        values: flavors,
      );
      expect(series.asStrings(), completion(flavors));
      expect(series.asI32(), throwsFfiException);
      expect(series.asF64(), throwsFfiException);
    });

    test('ints', () async {
      final numbers = Int32List.fromList([42, 110, 696]);
      final series = Series.ofI32(
        bridge: api,
        name: 'numbers',
        values: numbers,
      );
      expect(series.asI32(), completion(numbers));
    });

    test('doubles', () async {
      final numbers = Float64List.fromList([math.pi, math.e, math.log10e]);
      final series = Series.ofF64(
        bridge: api,
        name: 'numbers',
        values: numbers,
      );
      expect(series.asF64(), completion(numbers));
    });
  });

  group('Series.append', () {
    test('works', () async {
      final data = await api.readCsv(path: 'test/foo.csv');
      final firstNames = data.column(column: 'first');
      final lastNames = data.column(column: 'last');
      await firstNames.append(other: lastNames);
      expect(
        firstNames.asStrings(),
        completion(['John', 'Bob', 'Stevenson', 'Power']),
      );
    });

    test('fails when appending to self', () async {
      final data = await api.readCsv(path: 'test/foo.csv');
      final firstNames = data.column(column: 'first');
      expect(firstNames.append(other: firstNames), throwsFfiException);
    });
  });
}
