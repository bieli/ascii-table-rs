use ascii_table_rs::{AsciiTable, CellValue};

/*
Output table view from this demo.rs program:

╭───────────────────────────────────────────────────────────────────────────────────────╮
│                                   Cluster Overview                                    │
├─────────┬────────────┬───────────────────┬───────────────────┬─────────────┬──────────┤
│ Cluster │ Node Count │ Outgoing Gateways │ Incoming Gateways │ Connections │ RTT [ms] │
├─────────┼────────────┼───────────────────┼───────────────────┼─────────────┼──────────┤
│ west    │ 1          │ 2                 │ 2                 │ 0           │ 1.234    │
│ east    │ 1          │ 2                 │ 2                 │ 0           │ 4.321    │
│ central │ 1          │ 2                 │ 2                 │ 1           │ 3.345    │
├─────────┼────────────┼───────────────────┼───────────────────┼─────────────┼──────────┤
│         │ 3          │ 6                 │ 6                 │ 1           │ 8.900    │
╰─────────┴────────────┴───────────────────┴───────────────────┴─────────────┴──────────╯

*/

fn main() {
    let mut table = AsciiTable::new("Cluster Overview");
    table.set_decimal_places(3);

    table.set_headers(vec![
        "Cluster",
        "Node Count",
        "Outgoing Gateways",
        "Incoming Gateways",
        "Connections",
        "RTT [ms]",
    ]);

    table.add_row(vec![
        CellValue::Str("west".into()),
        CellValue::Int(1),
        CellValue::Int(2),
        CellValue::Int(2),
        CellValue::Int(0),
        CellValue::Float(1.23456),
    ]);

    table.add_row(vec![
        CellValue::Str("east".into()),
        CellValue::Int(1),
        CellValue::Int(2),
        CellValue::Int(2),
        CellValue::Int(0),
        CellValue::Float(4.3210),
    ]);

    table.add_row(vec![
        CellValue::Str("central".into()),
        CellValue::Int(1),
        CellValue::Int(2),
        CellValue::Int(2),
        CellValue::Int(1),
        CellValue::Float(3.345678),
    ]);

    table.set_summary(vec![
        CellValue::Str("".into()),
        CellValue::Int(3),
        CellValue::Int(6),
        CellValue::Int(6),
        CellValue::Int(1),
        CellValue::Float(8.90000000),
    ]);

    table.render();
}
