mod models {
    #[derive(Default)]
    pub struct Calc {
        pub prev: f64,
        pub operation: String,
        pub current: String,
        pub output: String,
    }

    impl Calc {
        pub fn click(&mut self, value: &str) {
            match value {
                "/" | "x" | "+" | "-" | "%" => {
                    if self.current != "0" {
                        if self.operation.is_empty() {
                            self.prev = self.current.parse().unwrap();
                        } else {
                            self.equil();
                        }
                        self.output.push_str(&format!("{} {}", self.prev, value));
                        self.operation = value.to_string();
                        self.current = String::from("0");
                    }
                }
                "=" => {
                    if !self.operation.is_empty() {
                        self.equil();
                        self.operation.clear();
                    }
                }
                "CE" => {
                    self.output.clear();
                    self.operation.clear();
                    self.current = String::from("0");
                    self.prev = 0f64;
                }
                "@<-" => {
                    let label = self.current.clone();
                    self.current = if label.len() > 1 {
                        String::from(&label[..label.len() - 1])
                    } else {
                        String::from("0")
                    };
                }
                "C" => self.current = String::from("0"),
                "." => {
                    if !self.current.contains('.') {
                        self.current.push('.');
                    }
                }
                _ => {
                    if self.current == "0" {
                        self.current.clear();
                    }
                    self.current.push_str(value);
                }
            };
        }
        fn equil(&mut self) {
            self.output.push_str(&format!(" {}<br>", self.current));
            let current: f64 = self.current.parse().unwrap();
            self.prev = match self.operation.as_str() {
                "/" => self.prev / current,
                "x" => self.prev * current,
                "+" => self.prev + current,
                "-" => self.prev - current,
                _ => self.prev / 100.0 * current,
            };
            self.output.push_str(&format!("    = {}<br>", self.prev));
            self.current = String::from("0");
        }
    }
}

use efltk::prelude::*;

pub enum Msg {
    Push(String),
}

#[derive(Default)]
pub struct Calc {
    outp: efltk::Entry,
    prev: efltk::Entry,
    oper: efltk::Label,
    curr: efltk::Entry,
}

impl Component for Calc {
    type Event = Msg;
    type State = models::Calc;
    fn handle(msg: Self::Event, model: &mut Self::State, _: Sender<Self::Event>) -> bool {
        match msg {
            Msg::Push(value) => model.click(&value),
        };
        true
    }
    fn update(&self, model: &Self::State) {
        self.outp
            .update(&format!("<bigger><code>{}</bigger></code>", model.output));
        self.prev
            .update(&format!("<bigger><code>{}</bigger></code>", model.prev));
        self.oper.set_text(&format!(
            "<bigger><code>{}</bigger></code>",
            model.operation
        ));
        self.curr
            .update(&format!("<bigger><code>{}</bigger></code>", model.current));
    }
    fn view(&mut self, prt: &impl ContainerExt, sender: Sender<Self::Event>) {
        efltk::Box::new(prt).inside(|prt| {
            self.outp = efltk::Entry::new(prt)
                .with_editable(false)
                .with_single_line(false)
                .with_tooltip("Output");
            efltk::Separator::new(prt);
            efltk::Box::new(prt)
                .with_homogeneous(true)
                .with_horizontal(true)
                .inside(|prt| {
                    self.oper = efltk::Label::new(prt).with_tooltip("Operation");
                    efltk::Box::new(prt).with_homogeneous(true).inside(|prt| {
                        self.prev = efltk::Entry::new(prt)
                            .with_editable(false)
                            .with_tooltip("Previos");
                        self.curr = efltk::Entry::new(prt)
                            .with_editable(false)
                            .with_tooltip("Current");
                    });
                });
            efltk::Separator::new(prt);
            for row in [
                ["CE", "C", "%", "/"],
                ["7", "8", "9", "x"],
                ["4", "5", "6", "-"],
                ["1", "2", "3", "+"],
                ["0", ".", "<", "="],
            ] {
                efltk::Box::new(prt)
                    .with_homogeneous(true)
                    .with_horizontal(true)
                    .inside(|prt| {
                        for cell in row {
                            efltk::Button::new(prt)
                                .with_size(-1, 45)
                                .with_text(cell)
                                .with_tooltip(cell)
                                .with_cursor(Cursor::Hand1)
                                .with_callback({
                                    let sender = sender.clone();
                                    move |wgt| {
                                        sender.send(Msg::Push(wgt.text())).unwrap();
                                    }
                                });
                        }
                    });
            }
        });
    }
}
