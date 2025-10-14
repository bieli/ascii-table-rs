use unicode_width::UnicodeWidthStr;
use std::fmt;

const TOP_LEFT: char = '╭';
const TOP_RIGHT: char = '╮';
const BOTTOM_LEFT: char = '╰';
const BOTTOM_RIGHT: char = '╯';
const HORIZONTAL: char = '─';
const VERTICAL: char = '│';

const HEADER_LEFT: char = '├';
const HEADER_RIGHT: char = '┤';
const HEADER_MID: char = '┬';

const ROW_LEFT: char = '├';
const ROW_RIGHT: char = '┤';
const ROW_MID: char = '┼';

const FOOT_MID: char = '┴';


#[derive(Debug, Clone)]
pub enum CellValue {
    Str(String),
    Int(i64),
    Float(f64),
}

impl CellValue {
    pub fn to_string_with_precision(&self, decimal_places: usize) -> String {
        match self {
            CellValue::Str(s) => s.clone(),
            CellValue::Int(i) => i.to_string(),
            CellValue::Float(f) => {
                let factor = 10f64.powi(decimal_places as i32);
                let truncated = (f * factor).trunc() / factor;
                format!("{:.1$}", truncated, decimal_places)
            }
        }
    }
}

impl fmt::Display for CellValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CellValue::Str(s) => write!(f, "{}", s),
            CellValue::Int(i) => write!(f, "{}", i),
            CellValue::Float(fl) => {
                let default_precision = 2;
                let factor = 10f64.powi(default_precision as i32);
                let truncated = (fl * factor).trunc() / factor;
                write!(f, "{:.1$}", truncated, default_precision)
            }
        }
    }
}

pub struct AsciiTable {
    title: String,
    headers: Vec<String>,
    rows: Vec<Vec<CellValue>>,
    summary: Option<Vec<CellValue>>,
    decimal_places: usize,
}

impl AsciiTable {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            headers: vec![],
            rows: vec![],
            summary: None,
            decimal_places: 2,
        }
    }

    pub fn headers(&self) -> &[String] {
        &self.headers
    }

    pub fn rows(&self) -> &[Vec<CellValue>] {
        &self.rows
    }

    pub fn summary(&self) -> Option<&[CellValue]> {
        self.summary.as_deref()
    }

    pub fn set_headers(&mut self, headers: Vec<impl Into<String>>) {
        self.headers = headers.into_iter().map(|h| h.into()).collect();
    }

    pub fn add_row(&mut self, row: Vec<CellValue>) {
        self.rows.push(row);
    }

    pub fn set_summary(&mut self, summary: Vec<CellValue>) {
        self.summary = Some(summary);
    }

    pub fn set_decimal_places(&mut self, places: usize) {
        self.decimal_places = places;
    }

    pub fn render_to_string(&self) -> String {
        let mut output = String::new();
        let mut col_widths = vec![0; self.headers.len()];

        for (i, header) in self.headers.iter().enumerate() {
            col_widths[i] = UnicodeWidthStr::width(header.as_str());
        }

        for row in &self.rows {
            for (i, cell) in row.iter().enumerate() {
                let cell_str = cell.to_string_with_precision(self.decimal_places);
                col_widths[i] = col_widths[i].max(UnicodeWidthStr::width(cell_str.as_str()));
            }
        }

        if let Some(summary) = &self.summary {
            for (i, cell) in summary.iter().enumerate() {
                let cell_str = cell.to_string_with_precision(self.decimal_places);
                col_widths[i] = col_widths[i].max(UnicodeWidthStr::width(cell_str.as_str()));
            }
        }

        let total_width: usize = col_widths.iter().map(|w| w + 2).sum::<usize>() + (col_widths.len() - 1);

        output += &format!("{}{}{}\n", TOP_LEFT, HORIZONTAL.to_string().repeat(total_width), TOP_RIGHT);
        output += &format!("{}{:^width$}{}\n", VERTICAL, self.title, VERTICAL, width = total_width);
        output += &format_separator(&col_widths, HEADER_LEFT, HEADER_MID, HEADER_RIGHT);
        output += &format_row(&self.headers, &col_widths);
        output += &format_separator(&col_widths, ROW_LEFT, ROW_MID, ROW_RIGHT);

        for row in &self.rows {
            let cells: Vec<String> = row.iter()
                .map(|c| c.to_string_with_precision(self.decimal_places))
                .collect();
            output += &format_row(&cells, &col_widths);
        }

        if let Some(summary) = &self.summary {
            output += &format_separator(&col_widths, ROW_LEFT, ROW_MID, ROW_RIGHT);
            let cells: Vec<String> = summary.iter()
                .map(|c| c.to_string_with_precision(self.decimal_places))
                .collect();
            output += &format_row(&cells, &col_widths);
        }

        output += &format_separator(&col_widths, BOTTOM_LEFT, FOOT_MID, BOTTOM_RIGHT);
        output
    }

    pub fn render(&self) {
        print!("{}", self.render_to_string());
    }
}

fn format_separator(widths: &[usize], left: char, mid: char, right: char) -> String {
    let mut line = String::new();
    line += &left.to_string();
    for (i, w) in widths.iter().enumerate() {
        line += &HORIZONTAL.to_string().repeat(*w + 2);
        if i < widths.len() - 1 {
            line += &mid.to_string();
        }
    }
    line += &right.to_string();
    line += "\n";
    line
}

fn format_row(values: &[String], widths: &[usize]) -> String {
    let mut line = String::new();
    line += &VERTICAL.to_string();
    for (value, width) in values.iter().zip(widths.iter()) {
        let content_width = UnicodeWidthStr::width(value.as_str());
        let padding = width.saturating_sub(content_width);
        line += &format!(
            " {:left$}{:right$} {}",
            value,
            "",
            VERTICAL,
            left = content_width,
            right = padding
        );
    }
    line += "\n";
    line
}

