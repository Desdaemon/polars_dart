import 'dart:async';
import 'dart:io';

import 'package:flutter_polars/flutter_polars.dart';
import 'package:flutter/material.dart';
import 'package:data_table_2/data_table_2.dart';
import 'package:path_provider/path_provider.dart' as temp;

void main() {
  runApp(const MyApp());
}

extension on String {
  String ellipsis({int length = 30}) {
    if (this.length > length) {
      return '${substring(0, length)}...';
    } else {
      return this;
    }
  }
}

class MyApp extends StatefulWidget {
  const MyApp({super.key});

  @override
  State<MyApp> createState() => _MyAppState();
}

class _MyAppState extends State<MyApp> {
  late Future<List<List>> rows;
  late DataFrame df;
  List<String> columns = const [];
  int? sortColumnIndex;

  @override
  void initState() {
    super.initState();

    rows = readCsv();
  }

  @override
  void dispose() {
    super.dispose();
    df.field0.dispose();
  }

  Future<List<List>> readCsv() async {
    final csv =
        await DefaultAssetBundle.of(context).load('assets/people-100.csv');
    final tempdir = await temp.getApplicationSupportDirectory();
    final csvTemp = tempdir.uri.resolve('people-100.csv').toFilePath();
    final f =
        await File(csvTemp).writeAsBytes(csv.buffer.asUint8List(), flush: true);

    try {
      df = await pl.readCsv(path: csvTemp, hasHeader: true);
      columns = df.getColumnNames();
      return df.iter().asyncMap(parseRow).toList();
    } finally {
      await f.delete();
    }
  }

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: Scaffold(
        appBar: AppBar(
          title: const Text('Native Packages'),
        ),
        body: Padding(
          padding: const EdgeInsets.all(10),
          child: FutureBuilder(
            future: rows,
            initialData: const [],
            builder: (context, snapshot) {
              if (snapshot.hasError) {
                return Text('Error: ${snapshot.error}');
              }

              if (!snapshot.hasData) {
                return const Text('Loading...');
              }

              if (snapshot.requireData.isEmpty) {
                return const Text('No rows.');
              }

              return DataTable2(
                sortColumnIndex: sortColumnIndex,
                columns: [
                  for (final column in columns)
                    DataColumn2(
                      label: Text(column),
                      numeric: column == 'Index',
                      size: const {
                            'Index': ColumnSize.S,
                            'Sex': ColumnSize.S,
                            'Job Title': ColumnSize.L,
                          }[column] ??
                          ColumnSize.M,
                      onSort: (columnIndex, ascending) {
                        final sorted = df.lazy().select(exprs: [
                          nth(columnIndex).sort(descending: !ascending)
                        ]).collect();
                        setState(() {
                          sortColumnIndex = columnIndex;
                          rows = sorted.then((sorted) async {
                            return [
                              for (var i = 0; i < sorted.height(); ++i)
                                await sorted.getRow(index: i).then(parseRow)
                            ];
                          });
                        });
                      },
                    )
                ],
                rows: [
                  for (final row in snapshot.requireData)
                    DataRow(cells: [
                      for (final cell in row)
                        DataCell(
                          Tooltip(
                            message: '$cell',
                            waitDuration: const Duration(seconds: 1),
                            child: Text('$cell'.ellipsis()),
                          ),
                        )
                    ])
                ],
              );
            },
          ),
        ),
      ),
    );
  }
}
