import 'package:test/test.dart';

import 'helpers.dart';

void main() {
  group('dataframe', () {
    setUpAll(() async {
      await initApi();
    });

    late DataFrame foo;
    setUp(() async {
      foo = await readCsv(path: 'test/foo.csv');
    });

    test('getRow', () async {
      final row = foo.getRow(index: 0);
      expect(row, [
        'Stevenson',
        'John',
        ['date', 15320],
        12000
      ]);
      expect(row.map(parseCell).toList(), [
        'Stevenson',
        'John',
        DateTime.utc(2011, 12, 12),
        12000,
      ]);
    });

    test('iter', () async {
      expect(
        foo.head(length: 2).iter(),
        emitsInAnyOrder([
          ['Stevenson', 'John', '2011/12/12', 12000],
          ['Power', 'Bob', '1998/12/12', 13000],
        ]),
      );
    });
  });
}
