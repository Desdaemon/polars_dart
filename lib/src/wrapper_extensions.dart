import 'package:polars_dart/polars_dart.dart';

extension SeriesExt on Series {
  double? operator [](int index) => get(index: index);
  Series operator +(Series other) => addTo(other: other);
  Series operator -(Series other) => subtract(other: other);
  Series operator *(Series other) => multiply(other: other);
  Series operator /(Series other) => divide(other: other);
  Series operator %(Series other) => remainder(other: other);
}

extension ExprExt on Expr {
  Expr equals(Expr other) =>
      Expr.binaryExpr(left: this, right: other, op: Operator.Eq);
  Expr notEquals(Expr other) =>
      Expr.binaryExpr(left: this, right: other, op: Operator.NotEq);
  Expr operator <(Expr other) =>
      Expr.binaryExpr(left: this, right: other, op: Operator.Lt);
  Expr operator <=(Expr other) =>
      Expr.binaryExpr(left: this, right: other, op: Operator.LtEq);
  Expr operator >(Expr other) =>
      Expr.binaryExpr(left: this, right: other, op: Operator.Gt);
  Expr operator >=(Expr other) =>
      Expr.binaryExpr(left: this, right: other, op: Operator.GtEq);
  Expr operator +(Expr other) =>
      Expr.binaryExpr(left: this, right: other, op: Operator.Plus);
  Expr operator -(Expr other) =>
      Expr.binaryExpr(left: this, right: other, op: Operator.Minus);
  Expr operator *(Expr other) =>
      Expr.binaryExpr(left: this, right: other, op: Operator.Multiply);
  Expr operator /(Expr other) =>
      Expr.binaryExpr(left: this, right: other, op: Operator.Divide);
  Expr operator ~/(Expr other) =>
      Expr.binaryExpr(left: this, right: other, op: Operator.TrueDivide);
  Expr operator %(Expr other) =>
      Expr.binaryExpr(left: this, right: other, op: Operator.Modulus);
  Expr operator &(Expr other) =>
      Expr.binaryExpr(left: this, right: other, op: Operator.And);
  Expr operator |(Expr other) =>
      Expr.binaryExpr(left: this, right: other, op: Operator.And);
  Expr operator ^(Expr other) =>
      Expr.binaryExpr(left: this, right: other, op: Operator.Xor);
  Expr floorDivide(Expr other) =>
      Expr.binaryExpr(left: this, right: other, op: Operator.FloorDivide);
}
