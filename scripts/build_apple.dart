#!/usr/bin/env dart

import 'dart:convert';
import 'dart:io';

import 'package:args/args.dart';

const framework = 'PolarsWrapper.xcframework';
const frameworkZip = '$framework.zip';
const libName = 'libpolars_wrapper.a';
const iosSimLipo = 'ios-sim-lipo/$libName';
const macLipo = 'mac-lipo/$libName';
const headers = '../polars-wrapper/include';
const buildDir = 'platform-build';

Future<void> mainImpl(List<String> args) async {
  final parser = ArgParser()
    ..addFlag('debug', negatable: false)
    ..addFlag('local')
    ..addOption('profile');
  final opts = parser.parse(args);
  final observer = Observer();

  final String profile, profileArg;
  if (opts.wasParsed('profile')) {
    profile = opts['profile'];
    profileArg = '--profile=$profile';
  } else if (opts['debug']) {
    profile = 'debug';
    profileArg = '--profile=dev';
  } else {
    profile = 'release';
    profileArg = '--release';
  }

  print(' Building for profile: $profile');

  final targets = opts['local']
      ? [hostTarget]
      : const [
          'aarch64-apple-ios',
          'x86_64-apple-ios',
          'aarch64-apple-ios-sim',
          'x86_64-apple-darwin',
          'aarch64-apple-darwin'
        ];

  // -- Begin --

  await run('mkdir -p $buildDir');
  Directory.current = buildDir;

  final outputs = targets.map((target) {
    return observer.mark('../target/$target/$profile/$libName');
  }).toList();

  for (final target in targets) {
    print(' Building target $target');
    await run('rustup target add $target');
    await run('cargo build --target=$target $profileArg');
  }

  await run('mkdir -p mac-lipo ios-sim-lipo');
  if (opts['local']) {
    final output = outputs.single;
    final isIos = output.contains('ios');
    final shouldBuildFramework =
        observer.hasChanged(output) || !fileExists(frameworkZip);

    String lipoOut;
    if (shouldBuildFramework) {
      lipoOut = isIos ? iosSimLipo : macLipo;
    } else {
      print('Nothing changed, exiting...');
      return;
    }

    await run('lipo -create -output $lipoOut $output');
    await run('xcodebuild -create-xcframework '
        '-library $lipoOut -headers $headers '
        '-output $framework');
  } else {
    final armIos = '../target/aarch64-apple-ios/$profile/$libName';
    var shouldBuildFramework =
        !fileExists(frameworkZip) || observer.hasChanged(armIos);
    if (!fileExists(iosSimLipo) ||
        outputs
            .where((output) => output.contains('ios'))
            .any(observer.hasChanged)) {
      shouldBuildFramework = true;
      await run('lipo -create -output $iosSimLipo '
          '../target/aarch64-apple-ios-sim/$profile/$libName '
          '../target/x86_64-apple-ios/$profile/$libName ');
    }
    if (!fileExists(macLipo) ||
        outputs
            .where((output) => output.contains('darwin'))
            .any(observer.hasChanged)) {
      shouldBuildFramework = true;
      await run('lipo -create -output $macLipo '
          '../target/aarch64-apple-darwin/$profile/$libName '
          '../target/x86_64-apple-darwin/$profile/$libName');
    }
    if (shouldBuildFramework) {
      await run('xcodebuild -create-xcframework '
          '-library $iosSimLipo -headers $headers '
          '-library $macLipo -headers $headers '
          '-library $armIos -headers $headers '
          '-output $framework');
    }
  }

  print(' Creating $frameworkZip');
  await run('zip -ry $frameworkZip $framework');

  print('✅ Done!');
}

void main(List<String> args) async {
  try {
    await mainImpl(args);
  } finally {
    await run('rm -rf ios-sim-lipo mac-lipo $framework', failFast: false);
  }
}

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
