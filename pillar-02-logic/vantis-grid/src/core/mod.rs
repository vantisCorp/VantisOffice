//! Core data structures for Vantis Grid

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Represents a cell value
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CellValue {
    Empty,
    Number(f64),
    Text(String),
    Boolean(bool),
    Error(String),
    Formula(String),
    Date(chrono::NaiveDate),
    DateTime(chrono::NaiveDateTime),
}

impl Default for CellValue {
    fn default() -> Self {
        CellValue::Empty
    }
}

/// Represents a single cell in the grid
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cell {
    pub row: usize,
    pub column: usize,
    pub value: CellValue,
    pub formula: Option<String>,
    pub style: CellStyle,
    pub dependencies: Vec<(usize, usize)>, // Cells this cell depends on
    pub dependents: Vec<(usize, usize)>,   // Cells that depend on this cell
}

impl Cell {
    pub fn new(row: usize, column: usize) -> Self {
        Cell {
            row,
            column,
            value: CellValue::Empty,
            formula: None,
            style: CellStyle::default(),
            dependencies: Vec::new(),
            dependents: Vec::new(),
        }
    }

    pub fn with_value(mut self, value: CellValue) -> Self {
        self.value = value;
        self
    }

    pub fn with_formula(mut self, formula: String) -> Self {
        self.formula = Some(formula);
        self
    }

    pub fn is_empty(&self) -> bool {
        matches!(self.value, CellValue::Empty)
    }

    pub fn reference(&self) -> String {
        format!("{}{}", column_to_letter(self.column), self.row + 1)
    }
}

/// Cell styling
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CellStyle {
    pub font_family: Option<String>,
    pub font_size: Option<u32>,
    pub font_bold: bool,
    pub font_italic: bool,
    pub text_color: Option<String>,
    pub background_color: Option<String>,
    pub border_left: Option<String>,
    pub border_right: Option<String>,
    pub border_top: Option<String>,
    pub border_bottom: Option<String>,
    pub text_alignment: Option<TextAlignment>,
    pub vertical_alignment: Option<VerticalAlignment>,
    pub number_format: Option<String>,
}

impl Default for CellStyle {
    fn default() -> Self {
        CellStyle {
            font_family: None,
            font_size: None,
            font_bold: false,
            font_italic: false,
            text_color: None,
            background_color: None,
            border_left: None,
            border_right: None,
            border_top: None,
            border_bottom: None,
            text_alignment: None,
            vertical_alignment: None,
            number_format: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TextAlignment {
    Left,
    Center,
    Right,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VerticalAlignment {
    Top,
    Middle,
    Bottom,
}

/// Represents a row in the worksheet
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Row {
    pub index: usize,
    pub height: Option<f64>,
    pub hidden: bool,
    pub style: Option<CellStyle>,
}

impl Row {
    pub fn new(index: usize) -> Self {
        Row {
            index,
            height: None,
            hidden: false,
            style: None,
        }
    }
}

/// Represents a column in the worksheet
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Column {
    pub index: usize,
    pub width: Option<f64>,
    pub hidden: bool,
    pub style: Option<CellStyle>,
}

impl Column {
    pub fn new(index: usize) -> Self {
        Column {
            index,
            width: None,
            hidden: false,
            style: None,
        }
    }

    pub fn letter(&self) -> String {
        column_to_letter(self.index)
    }
}

/// Worksheet - a single sheet in the workbook
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Worksheet {
    pub name: String,
    pub cells: HashMap<(usize, usize), Cell>,
    pub rows: Vec<Row>,
    pub columns: Vec<Column>,
    pub active_cell: Option<(usize, usize)>,
    pub frozen_rows: usize,
    pub frozen_columns: usize,
}

impl Worksheet {
    pub fn new(name: String) -> Self {
        Worksheet {
            name,
            cells: HashMap::new(),
            rows: Vec::new(),
            columns: Vec::new(),
            active_cell: None,
            frozen_rows: 0,
            frozen_columns: 0,
        }
    }

    pub fn get_cell(&self, row: usize, column: usize) -> Option<&Cell> {
        self.cells.get(&(row, column))
    }

    pub fn get_cell_mut(&mut self, row: usize, column: usize) -> &mut Cell {
        self.cells
            .entry((row, column))
            .or_insert_with(|| Cell::new(row, column))
    }

    pub fn set_cell_value(&mut self, row: usize, column: usize, value: CellValue) {
        let cell = self.get_cell_mut(row, column);
        cell.value = value;
    }

    pub fn set_cell_formula(&mut self, row: usize, column: usize, formula: String) {
        let cell = self.get_cell_mut(row, column);
        cell.formula = Some(formula);
    }

    pub fn add_row(&mut self) -> &mut Row {
        let index = self.rows.len();
        let row = Row::new(index);
        self.rows.push(row);
        self.rows.last_mut().unwrap()
    }

    pub fn add_column(&mut self) -> &mut Column {
        let index = self.columns.len();
        let column = Column::new(index);
        self.columns.push(column);
        self.columns.last_mut().unwrap()
    }

    pub fn ensure_rows(&mut self, count: usize) {
        while self.rows.len() < count {
            self.add_row();
        }
    }

    pub fn ensure_columns(&mut self, count: usize) {
        while self.columns.len() < count {
            self.add_column();
        }
    }

    pub fn resize(&mut self, rows: usize, columns: usize) {
        self.ensure_rows(rows);
        self.ensure_columns(columns);
    }
}

/// Workbook - contains multiple worksheets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workbook {
    pub name: String,
    pub worksheets: Vec<Worksheet>,
    pub active_worksheet: usize,
    pub metadata: WorkbookMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkbookMetadata {
    pub author: Option<String>,
    pub created: chrono::DateTime<chrono::Utc>,
    pub modified: chrono::DateTime<chrono::Utc>,
    pub description: Option<String>,
    pub tags: Vec<String>,
}

impl Workbook {
    pub fn new(name: String) -> Self {
        let now = chrono::Utc::now();
        Workbook {
            name,
            worksheets: Vec::new(),
            active_worksheet: 0,
            metadata: WorkbookMetadata {
                author: None,
                created: now,
                modified: now,
                description: None,
                tags: Vec::new(),
            },
        }
    }

    pub fn add_worksheet(&mut self, name: String) -> &mut Worksheet {
        let worksheet = Worksheet::new(name);
        self.worksheets.push(worksheet);
        self.worksheets.last_mut().unwrap()
    }

    pub fn get_active_worksheet(&self) -> Option<&Worksheet> {
        self.worksheets.get(self.active_worksheet)
    }

    pub fn get_active_worksheet_mut(&mut self) -> Option<&mut Worksheet> {
        self.worksheets.get_mut(self.active_worksheet)
    }

    pub fn set_active_worksheet(&mut self, index: usize) -> Result<(), String> {
        if index < self.worksheets.len() {
            self.active_worksheet = index;
            Ok(())
        } else {
            Err(format!("Worksheet index {} out of range", index))
        }
    }
}

/// Grid - main interface for spreadsheet operations
#[derive(Debug, Clone)]
pub struct Grid {
    pub workbook: Arc<RwLock<Workbook>>,
    pub calculation_enabled: bool,
    pub auto_save: bool,
}

impl Grid {
    pub fn new(name: String) -> Self {
        let mut workbook = Workbook::new(name);
        workbook.add_worksheet("Sheet1".to_string());

        Grid {
            workbook: Arc::new(RwLock::new(workbook)),
            calculation_enabled: true,
            auto_save: true,
        }
    }

    pub fn with_worksheets(name: String, worksheet_names: Vec<String>) -> Self {
        let mut workbook = Workbook::new(name);
        for ws_name in worksheet_names {
            workbook.add_worksheet(ws_name);
        }

        Grid {
            workbook: Arc::new(RwLock::new(workbook)),
            calculation_enabled: true,
            auto_save: true,
        }
    }

    pub fn get_workbook(&self) -> Arc<RwLock<Workbook>> {
        Arc::clone(&self.workbook)
    }

    pub fn set_cell_value(
        &self,
        row: usize,
        column: usize,
        value: CellValue,
    ) -> Result<(), String> {
        let mut workbook = self
            .workbook
            .write()
            .map_err(|e| format!("Failed to acquire write lock: {}", e))?;

        if let Some(worksheet) = workbook.get_active_worksheet_mut() {
            worksheet.set_cell_value(row, column, value);
            Ok(())
        } else {
            Err("No active worksheet".to_string())
        }
    }

    pub fn get_cell_value(&self, row: usize, column: usize) -> Result<CellValue, String> {
        let workbook = self
            .workbook
            .read()
            .map_err(|e| format!("Failed to acquire read lock: {}", e))?;

        if let Some(worksheet) = workbook.get_active_worksheet() {
            if let Some(cell) = worksheet.get_cell(row, column) {
                Ok(cell.value.clone())
            } else {
                Ok(CellValue::Empty)
            }
        } else {
            Err("No active worksheet".to_string())
        }
    }
}

/// Convert column index to letter (0 -> A, 1 -> B, 26 -> AA, etc.)
pub fn column_to_letter(index: usize) -> String {
    if index < 26 {
        format!("{}", (b'A' + index as u8) as char)
    } else {
        let first = (index - 26) / 26;
        let second = index % 26;
        format!(
            "{}{}",
            (b'A' + first as u8) as char,
            (b'A' + second as u8) as char
        )
    }
}

/// Convert column letter to index (A -> 0, B -> 1, AA -> 26, etc.)
pub fn letter_to_column(letter: &str) -> Result<usize, String> {
    let mut result = 0;
    for (i, c) in letter.chars().rev().enumerate() {
        if !c.is_ascii_uppercase() {
            return Err(format!("Invalid column letter: {}", letter));
        }
        result += (c as usize - b'A' as usize + 1) * 26_usize.pow(i as u32);
    }
    Ok(result - 1)
}

/// Parse cell reference (e.g., "A1" -> (0, 0))
pub fn parse_cell_reference(reference: &str) -> Result<(usize, usize), String> {
    let mut letters = String::new();
    let mut numbers = String::new();

    for c in reference.chars() {
        if c.is_ascii_alphabetic() {
            letters.push(c);
        } else if c.is_ascii_digit() {
            numbers.push(c);
        } else {
            return Err(format!("Invalid cell reference: {}", reference));
        }
    }

    if letters.is_empty() || numbers.is_empty() {
        return Err(format!("Invalid cell reference: {}", reference));
    }

    let column = letter_to_column(&letters)?;
    let row: usize = numbers
        .parse()
        .map_err(|e| format!("Invalid row number: {}", e))?;

    Ok((row - 1, column))
}

/// Initialize core subsystem
pub fn init() -> Result<(), String> {
    // Initialize any required subsystems
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_column_to_letter() {
        assert_eq!(column_to_letter(0), "A");
        assert_eq!(column_to_letter(1), "B");
        assert_eq!(column_to_letter(25), "Z");
        assert_eq!(column_to_letter(26), "AA");
    }

    #[test]
    fn test_letter_to_column() {
        assert_eq!(letter_to_column("A").unwrap(), 0);
        assert_eq!(letter_to_column("B").unwrap(), 1);
        assert_eq!(letter_to_column("Z").unwrap(), 25);
        assert_eq!(letter_to_column("AA").unwrap(), 26);
    }

    #[test]
    fn test_parse_cell_reference() {
        assert_eq!(parse_cell_reference("A1").unwrap(), (0, 0));
        assert_eq!(parse_cell_reference("B2").unwrap(), (1, 1));
        assert_eq!(parse_cell_reference("Z100").unwrap(), (99, 25));
    }

    #[test]
    fn test_grid_creation() {
        let grid = Grid::new("Test".to_string());
        assert_eq!(grid.workbook.read().unwrap().worksheets.len(), 1);
    }
}
