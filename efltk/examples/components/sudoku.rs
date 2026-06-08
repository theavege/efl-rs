mod models {
    #[derive(Default)]
    pub struct Model(pub [[u32; 9]; 9]);

    impl Model {
        pub fn clear(&mut self) {
            self.0 = [[0; 9]; 9];
        }
        fn check_solvable(&mut self) -> bool {
            let mut items: [u32; 9];
            for row in self.0 {
                items = [0; 9];
                for value in row {
                    if value > 0 && value < 10 {
                        items[(value - 1) as usize] += 1;
                    }
                }
                if items.iter().any(|&n| n > 1) {
                    return false;
                }
            }
            for i in 0..9 {
                items = [0; 9];
                for row in self.0 {
                    if row[i] > 0 && row[i] < 10 {
                        items[(row[i] - 1) as usize] += 1;
                    }
                }
                if items.iter().any(|&n| n > 1) {
                    return false;
                }
            }
            for &x in [0, 3, 6].iter() {
                for &y in [0, 3, 6].iter() {
                    items = [0; 9];
                    for i in 0..3 {
                        for j in 0..3 {
                            if self.0[y + i][x + j] > 0 && self.0[y + i][x + j] < 10 {
                                items[(self.0[y + i][x + j] - 1) as usize] += 1;
                            }
                        }
                    }
                    if items.iter().any(|&n| n > 1) {
                        return false;
                    }
                }
            }
            true
        }
        fn check_possible(&self, y: usize, x: usize, number: u32) -> bool {
            if self.0[y].contains(&number) {
                return false;
            }
            if self.0.iter().any(|n| n[x] == number) {
                return false;
            }
            let x0: usize = (x / 3) * 3;
            let y0: usize = (y / 3) * 3;
            for i in 0..3 {
                for j in 0..3 {
                    if self.0[y0 + i][x0 + j] == number {
                        return false;
                    }
                }
            }
            true
        }
        fn find_next_cell2fill(&self) -> (usize, usize) {
            for (x, row) in self.0.iter().enumerate() {
                for (y, &val) in row.iter().enumerate() {
                    if val == 0 {
                        return (x, y);
                    }
                }
            }
            (99, 99)
        }
        fn solve(&mut self) -> bool {
            let (i, j) = self.find_next_cell2fill();
            if i == 99 {
                return true;
            }
            for e in 1..10 {
                if self.check_possible(i, j, e) {
                    self.0[i][j] = e;
                    if self.solve() {
                        return true;
                    }
                    self.0[i][j] = 0;
                }
            }
            false
        }
        pub fn answer(&mut self) {
            if self.check_solvable() {
                self.solve();
            } else {
                self.clear();
            }
        }
    }
}

use efltk::prelude::*;

#[derive(Default)]
pub struct Sudoku([[efltk::HoverSel; 9]; 9]);

pub enum Msg {
    Push(usize, usize, u32),
    Solve,
    Clear,
}

impl Component for Sudoku {
    type Event = Msg;
    type State = models::Model;
    fn handle(msg: Self::Event, model: &mut Self::State, _: Sender<Self::Event>) -> bool {
        match msg {
            Msg::Push(row, col, value) => model.0[row][col] = value,
            Msg::Clear => model.clear(),
            Msg::Solve => model.answer(),
        };
        true
    }
    fn update(&self, model: &Self::State) {
        for row in 0..self.0.len() {
            for col in 0..self.0[row].len() {
                if model.0[row][col] > 0 {
                    self.0[row][col].set_text(&model.0[row][col].to_string());
                } else {
                    self.0[row][col].set_text("");
                }
            }
        }
    }
    fn view(&mut self, prt: &impl ContainerExt, sender: Sender<Self::Event>) {
        efltk::Box::new(prt).with_homogeneous(true).inside(|prt| {
            efltk::Label::new(prt);
            for row in 0..9 {
                efltk::Box::new(prt)
                    .with_horizontal(true)
                    .with_homogeneous(true)
                    .inside(|prt| {
                        for col in 0..9 {
                            self.0[row][col] = efltk::HoverSel::new(prt);
                            for item in 0..=9 {
                                self.0[row][col].add_item(&item.to_string(), &item.to_string(), {
                                    let sender = sender.clone();
                                    move |_| {
                                        sender.send(Msg::Push(row, col, item)).unwrap();
                                    }
                                });
                            }
                        }
                    });
            }
            efltk::Box::new(prt)
                .with_horizontal(true)
                .with_homogeneous(true)
                .inside(|prt| {
                    efltk::Button::new(prt).with_text("Answer").with_callback({
                        let sender = sender.clone();
                        move |_wgt| {
                            sender.send(Msg::Solve).unwrap();
                        }
                    });
                    efltk::Button::new(prt).with_text("Clear").with_callback({
                        let sender = sender.clone();
                        move |_wgt| {
                            sender.send(Msg::Clear).unwrap();
                        }
                    });
                });
            efltk::Label::new(prt);
        });
    }
}
