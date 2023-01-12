import 'dart:io';
import 'dart:convert';

Future<void> run(String script, {bool failFast = true}) async {
  print(' $script');
  final proc = await Process.start('bash', ['-c', script],
      mode: ProcessStartMode.inheritStdio);
  final exit = await proc.exitCode;
  if (exit != 0 && failFast) {
    final stderr = await proc.stderr.transform(const Utf8Decoder()).join('\n');
    throw Exception(
        "'${script.split(' ').first}' failed with code $exit:\n$stderr");
  }
}

String get hostTarget {
  final res = Process.runSync('rustc', const ['-vV']);
  return (res.stdout as String)
      .split('\n')
      .firstWhere((line) => line.startsWith('host:'))
      .split(':')
      .last
      .trim();
}

bool fileExists(String path) => File(path).existsSync();

class Observer {
  var fileMap = <String, DateTime?>{};

  String mark(String file) {
    final path = Uri.base.resolve(file).toFilePath();
    final f = File(path);
    fileMap[path] = f.existsSync() ? f.lastModifiedSync() : null;
    return path;
  }

  bool hasChanged(String file) {
    final path = Uri.base.resolve(file).toFilePath();

    if (!fileMap.containsKey(path)) {
      print('❌ Path not marked yet: $path');
      return true;
    }

    final lastModified = fileMap[path];
    if (lastModified == null) {
      print(' Path nonexistent: $path');
      return true;
    }

    return File(path).lastModifiedSync().isAfter(lastModified);
  }
}
