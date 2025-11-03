# ascii-table-rs

![CI status](https://github.com/bieli/ascii-table-rs/actions/workflows/test.yaml/badge.svg)
![github_tag](https://img.shields.io/github/v/tag/bieli/ascii-table-rs)
[![Crates.io](https://img.shields.io/crates/v/ascii-table-rs.svg)](https://crates.io/crates/ascii-table-rs)


**ascii-table-rs** is a lightweight, flexible Rust library for rendering beautiful ASCII tables in the terminal. 

It supports automatic column sizing, customizable float precision, summary rows, and Unicode box-drawing characters for a polished CLI/terminals experience.

## Features

- Easy-to-use API for defining headers, rows, and summaries
- Auto-resizing columns based on content width
- Supports `String`, `i64`, and `f64` cell types
- Configurable decimal precision for floats (with truncation, not rounding)
- Unicode box-drawing borders for clean layout
- Unit-tested and ready for production use

## Demo example

Run demo example program with:

```bash
$ cargo run --example demo
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/examples/demo`
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
```

## Quick Start

### Add to your `Cargo.toml`

```toml
[dependencies]
ascii-table-rs = "1.0.0"
```

### Create and render a table

```rust
use ascii_table_rs::{AsciiTable, CellValue};

fn main() {
    let mut table = AsciiTable::new("Cluster Overview");
    table.set_headers(vec!["Cluster", "Nodes", "Gateways", "Load"]);
    table.set_decimal_places(3); // Optional: set float precision

    table.add_row(vec![
        CellValue::Str("west".into()),
        CellValue::Int(3),
        CellValue::Int(5),
        CellValue::Float(0.8234),
    ]);

    table.add_row(vec![
        CellValue::Str("east".into()),
        CellValue::Int(2),
        CellValue::Int(4),
        CellValue::Float(0.9127),
    ]);

    table.set_summary(vec![
        CellValue::Str("Total".into()),
        CellValue::Int(5),
        CellValue::Int(9),
        CellValue::Float(1.7361),
    ]);

    table.render();
}
```

After running below program, you can see ASCII table like this:

```bash
╭────────────────────────────────────╮
│          Cluster Overview          │
├─────────┬───────┬──────────┬───────┤
│ Cluster │ Nodes │ Gateways │ Load  │
├─────────┼───────┼──────────┼───────┤
│ west    │ 3     │ 5        │ 0.823 │
│ east    │ 2     │ 4        │ 0.912 │
├─────────┼───────┼──────────┼───────┤
│ Total   │ 5     │ 9        │ 1.736 │
╰─────────┴───────┴──────────┴───────╯
```

## Public API overview for this library

- `AsciiTable::new(title: &str)` - Creates a new table with a title.
- `set_headers(Vec<&str>)` - Defines column headers.
- `add_row(Vec<CellValue>)` - Adds a row of data.
- `set_summary(Vec<CellValue>)` -Adds a summary row at the bottom.
- `set_decimal_places(usize)` - Sets float precision (default is 2 digits, truncates not rounds).
- `render_to_string(&self) -> String`  - Renders the table to String (without printing to STDOUT).
- `render()` - Prints the table to STDOUT.


## Testing

Run unit tests with:

```bash
$ cargo test
```

## TODO list
- [X] create tag and release for first version
- [X] add CI for Rust in GIthub Actions
- [X] update unit tests
- [X] publish library to cargo repository
- [ ] add summary auto aggregates (sum, mul, avg, mean, std_dev, percentile_99)
- [ ] add CSV file as input to table to visualize CSV cells
- [ ] connect previous features to auto aggregates and CSV view, to calculate aggregates from CSV files
- [ ] create bin program for exucure below features in terminal pipeline like `cat salaries.csv | csv2tab --summary sum`
- [ ] add feature to auto refresh table on terminal with defined seconds
- [ ] add code test coverage metrics


## Contributing
Pull requests, issues, and feedback are welcome! 
Let’s make ASCII tables beautiful together.

