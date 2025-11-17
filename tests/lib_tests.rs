use ascii_table_rs::{AsciiTable, CellValue};
use unicode_width::UnicodeWidthStr;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cellvalue_to_string() {
        assert_eq!(CellValue::Str("hello".into()).to_string(), "hello");
        assert_eq!(CellValue::Int(42).to_string(), "42");
        assert_eq!(CellValue::Float(3.14159).to_string(), "3.14");
    }

    #[test]
    fn test_float_precision_truncation() {
        let f1 = CellValue::Float(3.14159);
        let f2 = CellValue::Float(2.999);
        let f3 = CellValue::Float(1.0);

        assert_eq!(f1.to_string_with_precision(2), "3.14");
        assert_eq!(f2.to_string_with_precision(2), "2.99");
        assert_eq!(f3.to_string_with_precision(3), "1.000");
        assert_eq!(f1.to_string_with_precision(0), "3");
    }

    #[test]
    fn test_table_with_headers_only() {
        let mut table = AsciiTable::new("Test Table");
        table.set_headers(vec!["Col1", "Col2", "Col3"]);
        assert_eq!(table.headers().len(), 3);
        assert_eq!(table.rows().len(), 0);
    }

    #[test]
    fn test_add_row_and_summary() {
        let mut table = AsciiTable::new("Test Table");
        table.set_headers(vec!["A", "B"]);
        table.add_row(vec![CellValue::Int(1), CellValue::Float(2.5)]);
        table.set_summary(vec![CellValue::Str("Total".into()), CellValue::Float(2.5)]);

        assert_eq!(table.rows().len(), 1);
        assert!(table.summary().is_some());
    }

    #[test]
    fn test_column_width_calculation() {
        let mut table = AsciiTable::new("Widths");
        table.set_headers(vec!["Short", "LongerHeader"]);
        table.add_row(vec![
            CellValue::Str("tiny".into()),
            CellValue::Str("this is a longer cell".into()),
        ]);

        let headers: &[String] = table.headers();
        let rows: &[Vec<CellValue>] = table.rows();

        let mut col_widths: Vec<usize> = vec![0; headers.len()];
        for (i, header) in headers.iter().enumerate() {
            col_widths[i] = UnicodeWidthStr::width(header.as_str());
        }

        for row in rows {
            for (i, cell) in row.iter().enumerate() {
                let cell_str: String = cell.to_string_with_precision(2);
                col_widths[i] = col_widths[i].max(UnicodeWidthStr::width(cell_str.as_str()));
            }
        }

        assert!(col_widths[1] > col_widths[0]);
    }

    #[test]
    fn test_render_to_string_output_exact() {
        let mut table = AsciiTable::new("Test Table");
        table.set_headers(vec!["Name", "Score"]);
        table.add_row(vec![
            CellValue::Str("Alice".into()),
            CellValue::Float(95.6789),
        ]);
        table.add_row(vec![
            CellValue::Str("Bob".into()),
            CellValue::Float(88.1234),
        ]);
        table.set_summary(vec![
            CellValue::Str("Total".into()),
            CellValue::Float(183.8023),
        ]);
        table.set_decimal_places(2);

        let output = table.render_to_string();

        let expected = "\
╭────────────────╮
│   Test Table   │
├───────┬────────┤
│ Name  │ Score  │
├───────┼────────┤
│ Alice │ 95.67  │
│ Bob   │ 88.12  │
├───────┼────────┤
│ Total │ 183.80 │
╰───────┴────────╯
";

        assert_eq!(output, expected);
    }

    #[test]
    fn test_render_to_string_with_colors_output_exact() {
        let mut table = AsciiTable::new("Test Table");
        table.set_headers(vec!["Name", "Score"]);
        table.add_row(vec![
            CellValue::Str("\x1b[93mAlice\x1b[0m".into()),
            CellValue::Float(95.6789),
        ]);
        table.add_row(vec![
            CellValue::Str("Bob".into()),
            CellValue::Float(88.1234),
        ]);
        table.set_summary(vec![
            CellValue::Str("Total".into()),
            CellValue::Float(183.8023),
        ]);
        table.set_decimal_places(2);

        let output = table.render_to_string();

        let expected = "\
╭────────────────╮
│   Test Table   │
├───────┬────────┤
│ Name  │ Score  │
├───────┼────────┤
│ \x1b[93mAlice\x1b[0m │ 95.67  │
│ Bob   │ 88.12  │
├───────┼────────┤
│ Total │ 183.80 │
╰───────┴────────╯
";

        assert_eq!(output, expected);
    }
}
