import 'wrapper/expr.dart';
import 'expr.dart';

class ListNamespace {
  final Expr expr;

  const ListNamespace(this.expr);

  Expr get len => expr.listLen;
  Expr get max => expr.listMax;
  Expr get min => expr.listMin;
  Expr get sum => expr.listSum;
  Expr get mean => expr.listMean;
  Expr get reverse => expr.listReverse;
  Expr get first => expr.listFirst;
  Expr get last => expr.listLast;
  Expr get argMin => expr.listArgMin;
  Expr get argMax => expr.listArgMax;

  Expr get(Object? index) => expr.listGet(index: index.expr);
  Expr join(Object? separator) => expr.listJoin(separator: separator.expr);
  Expr shift(Object? periods) => expr.listShift(periods: periods.expr);
  Expr slice(Object? offset, Object? length) =>
      expr.listSlice(offset: offset.expr, length: length.expr);
  Expr head([int n = 10]) => expr.listHead(n: n.expr);
  Expr tail([int n = 10]) => expr.listTail(n: n.expr);
  Expr contains(Object? item) => expr.listContains(other: item.expr);
  Expr unique({bool maintainOrder = false}) =>
      expr.listUnique(maintainOrder: maintainOrder);
}
