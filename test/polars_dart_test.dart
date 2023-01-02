import 'package:test/test.dart';

import 'helpers.dart';

void main() {
  final api = initApi();

  test('readCsv', () async {
    final data = await api.readCsv(path: 'test/foo.csv');
    final firstNames = data.column(column: 'first');
    expect(firstNames.asStrings(), completion(['John', 'Bob']));
  });
}
