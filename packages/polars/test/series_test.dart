import 'dart:math' as math;

import 'package:test/test.dart';

import 'helpers.dart';

void main() {
  setUpAll(() async {
    await initApi();
  });
  group('Series', () {
    group('constructors', () {
      test('strings', () {
        const flavors = ['ice cream', 'chocolate', 'mint'];
        final series = Series.ofStrings(values: flavors);
        expect(series.asStrings(), flavors);
        expect(() => series.asInts(), throwsFrbException);
        expect(series.asDoubles(), throwsFrbException);
      });

      test('ints', () {
        final numbers = [4, 42, 69, null];
        final series = Series.ofI32(values: numbers);
        expect(series.asInts(), numbers);
      });

      test('doubles', () {
        final numbers = [math.pi, math.e, math.log10e];
        final series = Series.ofDoubles(values: numbers);
        expect(series.asDoubles(), completion(numbers));
      });

      test('durations', () async {
        const durations = [
          Duration(milliseconds: 10),
          Duration(microseconds: 10),
          Duration(seconds: 10),
        ];
        final series = Series.ofDurations(values: durations);
        expect(series.asDurations(), durations);
      });
    });

    group('append', () {
      test('works', () async {
        final data = await readCsv(path: 'test/foo.csv');
        final firstNames = data['first'];
        final lastNames = data['last'];
        firstNames.append(other: lastNames);
        expect(firstNames.asStrings(), ['John', 'Bob', 'Stevenson', 'Power']);
      });

      test(
        'fails when appending to self',
        () async {
          final data = await readCsv(path: 'test/foo.csv');
          final firstNames = data['first'];
          // FIXME: Without ..move, the thread deadlocks.
          expect(() => firstNames.append(other: firstNames..move = false),
              throwsFrbException);
        },
      );
    });

    test('sort', () async {
      final series = Series.ofI32(values: [42, 2, 12, 84]);
      final sorted = series.sort();
      expect(sorted.asInts(), [2, 12, 42, 84]);
      final sortedReverse = series.sort(reverse: true);
      expect(sortedReverse.asInts(), [84, 42, 12, 2]);
    });

    test('sum', () async {
      final series = Series.ofI32(values: [1, 2, 3, 4]);
      expect(series.sum(), 10);
    });

    test('min', () async {
      final series = Series.ofDoubles(values: [-1, -10, 23]);
      expect(series.min(), -10);
    });

    test('max', () async {
      final series = Series.ofDoubles(values: [10, 100, 1000]);
      expect(series.max(), 1000);
    });

    test('explode', () async {
      final series = Series.ofStrings(values: ['Johnson', 'Louisoix']);
      final exploded = series.explode();
      expect(exploded.asStrings(), 'JohnsonLouisoix'.split(''));
      // TODO(Desdaemon): Test exploding lists
    });

    group('cumulative', () {
      test('max', () async {
        final series = Series.ofDoubles(values: [10, 1, 23, 5, 26]);
        final cummax = series.rollingMax();
        expect(cummax.asDoubles(), completion([10, 10, 23, 23, 26]));
      });

      // test('product', () async {
      //   final series = Series.ofDoubles(values: [2, -1, 6, 10]);
      //   final cumprod = await series.cumprod();
      //   expect(cumprod.asDoubles(), completion([2, -2, -12, -120]));
      // });
    });

    test('product', () async {
      final series = Series.ofDoubles(values: [12, 2, -1]);
      final prod = series.product();
      expect(prod.asDoubles(), completion([-24]));
    });

    test('get', () {
      final series = Series.ofDoubles(values: [
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
      final series = Series.ofDoubles(values: [-1.1]);
      expect(series.getString(index: 0), '-1.1');
      expect(series.getString(index: -1), null);
    });

    test('mean', () {
      final series = Series.ofDoubles(values: [1, 5, 2, 10]);
      expect(series.mean(), 4.5);
    });

    test('meanAsSeries', () async {
      final series = Series.ofDoubles(values: [1, 5, 2, 10]);
      final mean = series.meanAsSeries();
      expect(mean.asDoubles(), completion([4.5]));
    });

    test('median', () {
      final series = Series.ofDoubles(values: [1, 5, 2, 10]);
      expect(series.median(), 3.5);
    });

    test('medianAsSeries', () async {
      final series = Series.ofDoubles(values: [1, 5, 2, 10]);
      final mean = series.medianAsSeries();
      expect(mean.asDoubles(), completion([3.5]));
    });

    test('estimatedSize', () {
      final series = Series.ofDoubles(values: [0, 0, 0, 1, 2]);
      expect(series.estimatedSize(), 5 * 8);
    });
  });
}
