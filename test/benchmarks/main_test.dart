import 'dart:math';

import 'package:benchmark_harness/benchmark_harness.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:test/scaffolding.dart';

import '../helpers.dart';

final api = initApi(release: true);

void main() {
  group('benchmarks', benchmarks, tags: 'bench');
}

int get timeBasedSeed => DateTime.now().millisecondsSinceEpoch;

void benchmarks() {
  group('sum', () {
    final seed = timeBasedSeed;
    test('Int64List', Int64ListSum(size: 100, seed: seed).report);
    test('Int64List', Int64ListSum(size: 10000, seed: seed).report);
    test('Int64List', Int64ListSum(size: 1000000, seed: seed).report);
    test('Series<i64>', Int64SeriesSum(size: 100, seed: seed).report);
    test('Series<i64>', Int64SeriesSum(size: 10000, seed: seed).report);
    test('Series<i64>', Int64SeriesSum(size: 1000000, seed: seed).report);
  });

  group('max', () {
    final seed = timeBasedSeed;
    test('Int64List', Int64ListMax(size: 100, seed: seed).report);
    test('Int64List', Int64ListMax(size: 10000, seed: seed).report);
    test('Int64List', Int64ListMax(size: 1000000, seed: seed).report);
    test('Series<i64>', Int64SeriesMax(size: 100, seed: seed).report);
    test('Series<i64>', Int64SeriesMax(size: 10000, seed: seed).report);
    test('Series<i64>', Int64SeriesMax(size: 1000000, seed: seed).report);
  });
}

class Int64ListSum extends TypedListBase<Int64List> {
  Int64ListSum({required super.size, super.seed}) : super(message: 'sum');
  @override
  Int64List makeBuffer(List<int> raw) => Int64List.fromList(raw);
  @override
  void run() {
    // ignore: unused_local_variable
    var sum = BigInt.zero;
    for (final i in buf) {
      sum += i;
    }
  }
}

class Int64ListMax extends TypedListBase<Int64List> {
  Int64ListMax({required super.size, super.seed}) : super(message: 'max');
  @override
  Int64List makeBuffer(List<int> raw) => Int64List.fromList(raw);
  @override
  void run() {
    // ignore: unused_local_variable
    var max = BigInt.from(-1);
    for (final i in buf) {
      if (i > max) max = i;
    }
  }
}

class Int64SeriesSum extends Int64SeriesBase {
  Int64SeriesSum({required super.size, super.seed}) : super(message: 'sum');
  @override
  Future<void> run() => series.sum();
}

class Int64SeriesMax extends Int64SeriesBase {
  Int64SeriesMax({required super.size, super.seed}) : super(message: 'max');
  @override
  Future<void> run() => series.max();
}

abstract class TypedListBase<T> extends BenchmarkBase {
  final int size;
  final int? seed;
  TypedListBase({required this.size, this.seed, String message = 'benchmark'})
      : super('$message: $T(size: $size)');
  late T buf;
  T makeBuffer(List<int> raw);
  @override
  void setup() {
    buf = makeBuffer(randomInts(seed: seed).take(size).toList(growable: false));
  }
}

abstract class Int64SeriesBase extends AsyncBenchmarkBase {
  final int size;
  final int? seed;
  Int64SeriesBase({required this.size, this.seed, String message = 'benchmark'})
      : super('$message: Series<i64>(size: $size)');
  late Series series;
  @override
  Future<void> setup() async {
    series = Series.ofI64(
      bridge: api,
      name: 'numbers',
      values: Int64List.fromList(
        randomInts(seed: seed).take(size).toList(growable: false),
      ),
    );
  }

  @override
  Future<void> teardown() async {
    series.field0.dispose();
  }
}

Iterable<int> randomInts({int max = 1000, int? seed}) sync* {
  final rng = Random(seed);
  while (true) {
    yield rng.nextInt(max);
  }
}
