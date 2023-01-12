#!/usr/bin/env dart

import 'dart:io';

import 'package:args/args.dart';

import 'utils.dart';

const libName = 'polars_wrapper';
const linuxLibName = 'lib$libName.so';
const windowsLibName = '$libName.dll';
const buildDir = 'platform-build';

Future<void> mainImpl(List<String> args) async {
  final parser = ArgParser()
    ..addFlag('debug')
    ..addFlag('local')
    ..addOption('profile');
  final opts = parser.parse(args);

  String profile, profileArg;
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

  // -- Begin --
  await run('mkdir -p $buildDir');
  Directory.current = buildDir;

  await run('cargo install cargo-zigbuild cargo-xwin');

  final targets = opts['local'] ? [Targets.host] : Targets.values;
  final compilerOpts = opts.rest;
  for (final target in targets) {
    final triple = target.triple;
    final flutterIdentifier = target.flutterIdentifier;
    await run('rustup target add $triple');
    await run(
        '${target.compiler} --target $triple $profileArg ${compilerOpts.join(' ')}');
    await run('mkdir -p $flutterIdentifier');
    await run('cp ../target/$triple/$profile/${target.libName} '
        '$flutterIdentifier/');
  }

  final hasLinux = targets.any((target) => !target.isWindows);
  final hasWindows = targets.any((target) => target.isWindows);
  await run('tar -czvf other.tar.gz '
      '${hasLinux ? 'linux-*' : ''} '
      '${hasWindows ? 'windows-*' : ''}');
}

void main(List<String> args) async {
  try {
    await mainImpl(args);
  } finally {
    await run('rm -rf linux-* windows-*', failFast: false);
  }
}

enum Targets {
  linuxArm64('aarch64-unknown-linux-gnu', 'linux-arm64'),
  linuxX64('x86_64-unknown-linux-gnu', 'linux-x64'),
  windowsArm64('aarch64-pc-windows-msvc', 'windows-arm64', isWindows: true),
  windowsX64('x86_64-pc-windows-msvc', 'windows-x64', isWindows: true);

  final String triple;
  final String flutterIdentifier;
  final bool isWindows;
  const Targets(this.triple, this.flutterIdentifier, {this.isWindows = false});

  static Targets get host {
    final host = hostTarget;
    return values.firstWhere((target) => target.triple == host);
  }

  String get compiler => isWindows ? 'cargo xwin build' : 'cargo zigbuild';
  String get libName => isWindows ? windowsLibName : linuxLibName;
}
