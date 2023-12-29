import 'package:test/test.dart';

import 'helpers.dart';

void main() {
  group('main', () {
    setUpAll(() async {
      await initApi();
    });

    test('readCsv', () async {
      final data = await readCsv(path: 'test/foo.csv');
      final firstNames = data.column(column: 'first');
      expect(firstNames.asStrings(), ['John', 'Bob']);
    });
  });
}
