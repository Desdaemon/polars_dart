import 'dart:math' as math;

import 'package:test/test.dart';

import 'helpers.dart';

void main() {
  setUpAll(() {
    initApi();
  });
  group('Series', () {
    group('constructors', () {
      test('strings', () {
        const flavors = ['ice cream', 'chocolate', 'mint'];
        final series = Series.ofStrings(name: 'flavors', values: flavors);
        expect(series.asStrings(), completion(flavors));
        expect(series.asInts(), throwsFfiException);
        expect(series.asDoubles(), throwsFfiException);
      });

      test('ints', () {
        final numbers = [4, 42, 69, null];
        final series = Series.ofI32(name: 'numbers', values: numbers);
        expect(series.asInts(), completion(numbers));
      });

      test('doubles', () {
        final numbers = [math.pi, math.e, math.log10e];
        final series = Series.ofDoubles(name: 'numbers', values: numbers);
        expect(series.asDoubles(), completion(numbers));
      });

      test('durations', () async {
        const durations = [
          Duration(milliseconds: 10),
          Duration(microseconds: 10),
          Duration(seconds: 10),
        ];
        final series = Series.ofDurations(name: 'durations', values: durations);
        expect(series.asDurations(), completion(durations));
      });
    });

    group('append', () {
      test('works', () async {
        final data = await wrapper.readCsv(path: 'test/foo.csv');
        final firstNames = data.column(column: 'first');
        final lastNames = data.column(column: 'last');
        await firstNames.append(other: lastNames);
        expect(
          firstNames.asStrings(),
          completion(['John', 'Bob', 'Stevenson', 'Power']),
        );
      });

      test(
        'fails when appending to self',
        () async {
          final data = await wrapper.readCsv(path: 'test/foo.csv');
          final firstNames = data.column(column: 'first');
          expect(firstNames.append(other: firstNames), throwsFfiException);
        },
      );
    });

    test('abs', () async {
      final series =
          Series.ofDoubles(name: 'floats', values: [-1, -2, -4, -8, 3]);
      final abs = await series.abs();
      expect(abs.asDoubles(), completion([1, 2, 4, 8, 3]));
    });

    test('sort', () async {
      final series = Series.ofI32(name: 'numbers', values: [42, 2, 12, 84]);
      final sorted = await series.sort();
      expect(sorted.asInts(), completion([2, 12, 42, 84]));
      final sortedReverse = await series.sort(reverse: true);
      expect(sortedReverse.asInts(), completion([84, 42, 12, 2]));
    });

    test('sum', () async {
      final series = Series.ofI32(name: 'numbers', values: [1, 2, 3, 4]);
      expect(series.sum(), completion(10));
    });

    test('min', () async {
      final series = Series.ofDoubles(name: 'floats', values: [-1, -10, 23]);
      expect(series.min(), completion(-10));
    });

    test('max', () async {
      final series = Series.ofDoubles(name: 'floats', values: [10, 100, 1000]);
      expect(series.max(), completion(1000));
    });

    test('explode', () async {
      final series =
          Series.ofStrings(name: 'names', values: ['Johnson', 'Louisoix']);
      final exploded = await series.explode();
      expect(exploded.asStrings(), completion('JohnsonLouisoix'.split('')));
      // TODO(Desdaemon): Test exploding lists
    });

    group('cumulative', () {
      test('max', () async {
        final series =
            Series.ofDoubles(name: 'floats', values: [10, 1, 23, 5, 26]);
        final cummax = await series.cummax();
        expect(cummax.asDoubles(), completion([10, 10, 23, 23, 26]));
        final reversed = await series.cummax(reverse: true);
        expect(reversed.asDoubles(), completion([26, 26, 26, 26, 26]));
      });

      test('product', () async {
        final series = Series.ofDoubles(name: 'floats', values: [2, -1, 6, 10]);
        final cumprod = await series.cumprod();
        expect(cumprod.asDoubles(), completion([2, -2, -12, -120]));
        final reversed = await series.cumprod(reverse: true);
        expect(reversed.asDoubles(), completion([-120, -60, 60, 10]));
      });
    });

    test('product', () async {
      final series = Series.ofDoubles(name: 'floats', values: [12, 2, -1]);
      final prod = await series.product();
      expect(prod.asDoubles(), completion([-24]));
    });

    test('get', () {
      final series = Series.ofDoubles(name: 'floats', values: [
        123,
        double.nan,
        double.infinity,
        double.negativeInfinity,
      ]);
      expect(series[0], 123);
      expect(series[1], isNaN);
      expect(series[2], double.infinity);
      expect(series[3], double.negativeInfinity);
      expect(series[-1], null);
    });

    test('getString', () {
      final series = Series.ofDoubles(name: 'floats', values: [-1.1]);
      expect(series.getString(index: 0), '-1.1');
      expect(series.getString(index: -1), null);
    });

    test('mean', () {
      final series = Series.ofDoubles(name: 'floats', values: [1, 5, 2, 10]);
      expect(series.mean(), completion(4.5));
    });

    test('meanAsSeries', () async {
      final series = Series.ofDoubles(name: 'floats', values: [1, 5, 2, 10]);
      final mean = await series.meanAsSeries();
      expect(mean.asDoubles(), completion([4.5]));
    });

    test('median', () {
      final series = Series.ofDoubles(name: 'floats', values: [1, 5, 2, 10]);
      expect(series.median(), completion(3.5));
    });

    test('medianAsSeries', () async {
      final series = Series.ofDoubles(name: 'floats', values: [1, 5, 2, 10]);
      final mean = await series.medianAsSeries();
      expect(mean.asDoubles(), completion([3.5]));
    });

    test('estimatedSize', () {
      final series = Series.ofDoubles(
        name: 'floats',
        values: [0, 0, 0, 1, 2],
      );
      expect(series.estimatedSize(), 5 * 8);
    });
  });
}
