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
        final $series = series(flavors);
        expect($series.asStrings(), flavors);
        expect(() => $series.asInts(), throwsFrbException);
        expect(() => $series.asDoubles(), throwsFrbException);
      });

      test('ints', () {
        final numbers = [4, 42, 69, null];
        final $series = series(numbers);
        expect($series.asInts(), numbers);
      });

      test('doubles', () {
        final numbers = [math.pi, math.e, math.log10e];
        final $series = series(numbers);
        expect($series.asDoubles(), numbers);
      });

      test('durations', () async {
        const durations = [
          Duration(milliseconds: 10),
          Duration(microseconds: 10),
          Duration(seconds: 10),
        ];
        final $series = series(durations);
        expect($series.asDurations(), durations);
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
          expect(
              () => firstNames.append(other: firstNames), throwsFrbException);
        },
      );
    });

    test('sort', () async {
      final $series = series([42, 2, 12, 84]);
      final sorted = $series.sort();
      expect(sorted.asInts(), [2, 12, 42, 84]);
      final sortedReverse = $series.sort(reverse: true);
      expect(sortedReverse.asInts(), [84, 42, 12, 2]);
    });

    test('sum', () async {
      final $series = series([1, 2, 3, 4]);
      expect($series.sum(), 10);
    });

    test('min', () async {
      final $series = series([-1, -10, 23]);
      expect($series.min(), -10);
    });

    test('max', () async {
      final $series = series([10, 100, 1000]);
      expect($series.max(), 1000);
    });

    test('explode', () async {
      final $series = series(['Johnson', 'Louisoix']);
      final exploded = $series.explode();
      expect(exploded.asStrings(), 'JohnsonLouisoix'.split(''));
      // TODO(Desdaemon): Test exploding lists
    });

    group('cumulative', () {
      test('max', () async {
        final $series = series([10, 1, 23, 5, 26]);
        final cummax = $series.rollingMax();
        expect(cummax.asDoubles(), completion([10, 10, 23, 23, 26]));
      });

      // test('product', () async {
      //   final series = Series.ofDoubles(values: [2, -1, 6, 10]);
      //   final cumprod = await series.cumprod();
      //   expect(cumprod.asDoubles(), completion([2, -2, -12, -120]));
      // });
    });

    test('product', () async {
      final $series = series([12, 2, -1]);
      final prod = $series.product();
      expect(prod.asDoubles(), completion([-24]));
    });

    test('get', () {
      final $series = series([
        123,
        double.nan,
        double.infinity,
        double.negativeInfinity,
      ]);
      expect($series[0], 123);
      expect($series[1], isNaN);
      expect($series[2], double.infinity);
      expect($series[3], double.negativeInfinity);
      expect($series[-1], null);
    });

    test('getString', () {
      final $series = series([-1.1]);
      expect($series.getString(index: 0), '-1.1');
      expect($series.getString(index: -1), null);
    });

    test('mean', () {
      final $series = series([1, 5, 2, 10]);
      expect($series.mean(), 4.5);
    });

    test('meanAsSeries', () async {
      final $series = series([1, 5, 2, 10]);
      final mean = $series.meanAsSeries();
      expect(mean.asDoubles(), completion([4.5]));
    });

    test('median', () {
      final $series = series([1, 5, 2, 10]);
      expect($series.median(), 3.5);
    });

    test('medianAsSeries', () async {
      final $series = series([1, 5, 2, 10]);
      final mean = $series.medianAsSeries();
      expect(mean.asDoubles(), completion([3.5]));
    });

    test('estimatedSize', () {
      final $series = series([0, 0, 0, 1, 2]);
      expect($series.estimatedSize(), 5 * 8);
    });
  });
}

Expr get person => col('first_name') + ' ' + col('last_name');

Future<void> what() async {
  final _df = df({
    'foo': [1, 2, 3],
  });
  final data = await _df
      .clone()
      .lazy()
      .sort(byColumn: 'birthday', descending: true, nullsLast: true)
      .groupBy(exprs: ['state'.expr])
      .agg(exprs: [
        person.first.alias('youngest'),
        person.last.alias('oldest'),
      ])
      .limit(n: 5)
      .collect();
}
