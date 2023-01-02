import 'dart:io';

final triple = () {
  final result = Process.runSync('rustc', const ['-vV']);
  final stdout = result.stdout;
  if (stdout is! String) throw Exception('invalid stdout');

  return stdout
      .split('\n')
      .firstWhere((line) => line.startsWith('host:'))
      .substring(6);
}();

String formatDylib(String name) {
  if (Platform.isWindows) {
    return '$name.dll';
  }
  if (Platform.isMacOS) {
    return 'lib$name.dylib';
  }
  return 'lib$name.so';
}
