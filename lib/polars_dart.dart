/// Dart bindings for the polars library.
library polars_dart;

// export 'src/wrapper.io.dart';
export 'src/wrapper.dart';
export 'src/wrapper.io.dart' if (dart.library.html) 'src/wrapper.web.dart'
    show PolarsWrapperImpl;
export 'src/wrapper_extensions.dart';
