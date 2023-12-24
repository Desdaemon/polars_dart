// import 'dart:ffi';
// import 'dart:io';
// import 'wrapper/wrapper.dart';

// PolarsWrapper? _wrapper;

// Never bail(String mes) {
//   throw Exception(mes);
// }

// PolarsWrapper get wrapper {
//   return _wrapper ?? bail('Polars wrapper not initialized');
// }

// void initialize({
//   String? path,
//   bool preferStatic = false,
//   DynamicLibrary? dylib,
// }) {
//   if (Platform.isIOS || (preferStatic && Platform.isMacOS)) {
//     dylib ??= DynamicLibrary.executable();
//   } else {
//     dylib ??= DynamicLibrary.open(path ??
//         bail(
//           'Dylib path is required to initialize on ${Platform.operatingSystem}',
//         ));
//   }
//   _wrapper = PolarsWrapperImpl(dylib);
// }
