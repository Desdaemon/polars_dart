import 'dart:convert';

void main() {}

typedef Identifier = (String name, String? owner);
Identifier ident(String name, [String? owner]) => (name, owner);

class Symbol {
  final String source;

  const Symbol({required this.source});
}

void process(List<Map<String, dynamic>> raw, Map<Identifier, Symbol> symbols) {
  for (final lib in raw) {
    if (lib
        case {
          "declarations": List decls,
          "directive": List directives,
          "source": String name
        }) {
      processLib(decls, directives, name, symbols);
    } else {
      throw new Exception('Unsupported lib ${jsonEncode(raw)}');
    }
  }
}

void processLib(
  List decls,
  List directives,
  String name,
  Map<Identifier, Symbol> symbols,
) {}
