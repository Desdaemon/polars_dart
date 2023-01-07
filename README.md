# polars_dart

All the greatness of [polars](https://www.pola.rs), now in Dart.

```dart
import 'package:polars_dart/polars_dart.dart';

final pl = PolarsWrapperImpl(dylib);
final iris = await pl.readCsv(path: 'iris.csv');
final df = await iris
  .lazy()
  .filter(pred: col('sepal_length') > 5)
  .groupBy(exprs: ['species'.expr])
  .collect();
```

## Contributing

Closing issues in [TODO.md](TODO.md) will help make this project grow!

## License

Dual-licensed under Apache 2.0 and MIT.