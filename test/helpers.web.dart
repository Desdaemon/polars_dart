import 'package:polars_dart/polars_dart.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';

PolarsWrapper initApi({bool? release}) {
  return PolarsWrapperImpl.wasm(WasmModule.initialize(
    kind: const Modules.noModules(root: 'polars-wrapper/pkg'),
  ));
}
