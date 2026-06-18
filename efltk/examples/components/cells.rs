mod models {
    use std::collections::HashMap;

    #[derive(Default, Clone)]
    pub struct Cell {
        pub formula: String,
        pub value: String,
    }

    #[derive(Default)]
    pub struct Model {
        pub cells: HashMap<String, Cell>,
        pub editing: Option<(i32, i32)>, // (row, col)
        pub edit_text: String,
    }

    impl Model {
        pub fn new() -> Self {
            let mut model = Self::default();
            // Initialize some cells with formulas
            model.cells.insert("A0".to_string(), Cell {
                formula: "10".to_string(),
                value: "10".to_string(),
            });
            model.cells.insert("A1".to_string(), Cell {
                formula: "20".to_string(),
                value: "20".to_string(),
            });
            model.cells.insert("B0".to_string(), Cell {
                formula: "A0 + A1".to_string(),
                value: "30".to_string(),
            });
            model
        }

        pub fn get_cell(&self, row: i32, col: i32) -> Option<&Cell> {
            let col_name = (b'A' as i32 + col) as u8 as char;
            let key = format!("{}{}", col_name, row);
            self.cells.get(&key)
        }

        pub fn get_cell_mut(&mut self, row: i32, col: i32) -> Option<&mut Cell> {
            let col_name = (b'A' as i32 + col) as u8 as char;
            let key = format!("{}{}", col_name, row);
            self.cells.get_mut(&key)
        }

        pub fn set_cell_formula(&mut self, row: i32, col: i32, formula: String) {
            let col_name = (b'A' as i32 + col) as u8 as char;
            let key = format!("{}{}", col_name, row);
            let cell = self.cells.entry(key).or_insert_with(|| Cell {
                formula: formula.clone(),
                value: String::new(),
            });
            cell.formula = formula;
            // Recalculate value and propagate changes
            self.recalculate();
        }

        pub fn recalculate(&mut self) {
            // Simple recalculation - in a real implementation, this would track dependencies
            for (key, cell) in &mut self.cells {
                // For now, just evaluate simple numeric formulas
                if let Ok(num) = cell.formula.parse::<f64>() {
                    cell.value = num.to_string();
                } else if cell.formula.contains('+') {
                    // Simple addition: "A0 + A1"
                    let parts: Vec<&str> = cell.formula.split('+').map(|s| s.trim()).collect();
                    let mut sum = 0.0;
                    let mut all_valid = true;
                    for part in parts {
                        if let Some(referenced_cell) = self.cells.get(part) {
                            if let Ok(num) = referenced_cell.value.parse::<f64>() {
                                sum += num;
                            } else {
                                all_valid = false;
                                break;
                            }
                        } else {
                            all_valid = false;
                            break;
                        }
                    }
                    if all_valid {
                        cell.value = sum.to_string();
                    }
                }
            }
        }
    }
}

use efltk::prelude::*;

pub enum Msg {
    EditCell(i32, i32),
    UpdateFormula(String),
    FinishEdit,
    CellClicked(i32, i32),
}

#[derive(Default)]
pub struct Cells {
    grid: efltk::Box,
    cells: Vec<Vec<efltk::Entry>>,
    edit_entry: efltk::Entry,
}

impl Component for Cells {
    type Event = Msg;
    type State = models::Model;

    fn handle(msg: Self::Event, model: &mut Self::State, sender: Sender<Self::Event>) -> bool {
        match msg {
            Msg::EditCell(row, col) => {
                if let Some(cell) = model.get_cell(row, col) {
                    model.editing = Some((row, col));
                    model.edit_text = cell.formula.clone();
                }
            }
            Msg::UpdateFormula(formula) => {
                model.edit_text = formula;
            }
            Msg::FinishEdit => {
                if let Some((row, col)) = model.editing {
                    model.set_cell_formula(row, col, model.edit_text.clone());
                    model.editing = None;
                    model.edit_text = String::new();
                }
            }
            Msg::CellClicked(row, col) => {
                // Double-click to edit
                // For simplicity, we'll just edit on single click
                sender.send(Msg::EditCell(row, col)).unwrap();
            }
        }
        true
    }

    fn update(&self, model: &Self::State) {
        // Update all cell displays
        for (row_idx, row) in self.cells.iter().enumerate() {
            for (col_idx, cell_entry) in row.iter().enumerate() {
                if let Some(cell) = model.get_cell(row_idx as i32, col_idx as i32) {
                    cell_entry.set_value(&cell.value);
                } else {
                    cell_entry.set_value("");
                }
            }
        }

        // Show/hide edit entry
        if let Some((row, col)) = model.editing {
            self.edit_entry.set_value(&model.edit_text);
            self.edit_entry.show();
        } else {
            self.edit_entry.set_value("");
            self.edit_entry.del();
        }
    }

    fn view(&mut self, prt: &impl ContainerExt, sender: Sender<Self::Event>) {
        // Create scrollable container
        self.grid = efltk::Box::new(prt).with_vertical(true);

        // Create grid of cells (5x5 for demo)
        const ROWS: usize = 5;
        const COLS: usize = 5;

        self.cells = vec![vec![efltk::Entry::new(&self.grid); COLS]; ROWS];

        for (row_idx, row) in self.cells.iter_mut().enumerate() {
            let row_box = efltk::Box::new(&self.grid).with_horizontal(true);
            for (col_idx, cell_entry) in row.iter_mut().enumerate() {
                let col_name = (b'A' as usize + col_idx) as u8 as char;
                let cell = cell_entry
                    .with_size(80, 30)
                    .with_callback({
                        let sender = sender.clone();
                        let row = row_idx as i32;
                        let col = col_idx as i32;
                        move |_| sender.send(Msg::CellClicked(row, col)).unwrap()
                    });
                row_box.add(&cell);
            }
        }

        // Edit entry (shown when editing a cell)
        self.edit_entry = efltk::Entry::new(prt)
            .with_size(200, 30)
            .with_callback({
                let sender = sender.clone();
                move |wgt| sender.send(Msg::UpdateFormula(wgt.value())).unwrap()
            })
            .with_signal(InputSignal::Unfocused, {
                let sender = sender.clone();
                move |_| sender.send(Msg::FinishEdit).unwrap()
            });
    }
}
