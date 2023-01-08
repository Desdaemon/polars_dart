import 'package:test/test.dart';

import 'helpers.dart';

void main() {
  group('dataframe', () {
    late PolarsWrapper api;
    setUpAll(() {
      api = initApi();
    });

    late DataFrame foo;
    setUp(() async {
      foo = await api.readCsv(path: 'test/foo.csv');
    });

    test('getRow', () async {
      expect(
        foo.getRow(index: 0),
        completion(['Stevenson', 'John', '2011-12-12', 12000]),
      );
    });

    test('iter', () async {
      expect(
        foo.head(length: 2).iter(),
        emitsInAnyOrder([
          ['Stevenson', 'John', '2011-12-12', 12000],
          ['Power', 'Bob', '1998-12-12', 13000],
        ]),
      );
    });
  });
}
