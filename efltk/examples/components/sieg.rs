mod models {
    use std::{collections::HashMap, thread};

    #[derive(Default, Clone, serde::Deserialize)]
    pub struct Model {
        pub proto: String,
        pub result: String,
        pub port: u16,
        pub concurrent: u8,
        pub targets: Vec<String>,
        pub endpoints: Vec<String>,
    }

    impl Model {
        pub fn run(&self) -> HashMap<String, Vec<Vec<String>>> {
            let mut children: HashMap<String, thread::JoinHandle<Vec<Vec<String>>>> =
                HashMap::new();
            for node in self.targets.clone() {
                let endpoints: Vec<String> = self.endpoints.clone();
                let proto: String = self.proto.clone();
                let port: u16 = self.port;
                let concurrent: u8 = self.concurrent;
                children.insert(
                    node.clone(),
                    thread::spawn(move || -> Vec<Vec<String>> {
                        let mut report: Vec<Vec<String>> = Vec::new();
                        report.push(Vec::from([
                            "endpoint".to_string(),
                            "transactions".to_string(),
                            "availability".to_string(),
                            "elapsed_time".to_string(),
                            "data_transferred".to_string(),
                            "response_time".to_string(),
                            "transaction_rate".to_string(),
                            "throughput".to_string(),
                            "concurrency".to_string(),
                            "successful_transactions".to_string(),
                            "failed_transactions".to_string(),
                            "longest_transaction".to_string(),
                            "shortest_transaction".to_string(),
                        ]));
                        for endpoint in endpoints {
                            report.push(get_raw(
                                format!("{proto}://{node}:{port}{endpoint}"),
                                concurrent,
                            ));
                        }
                        report
                    }),
                );
            }
            let mut data: HashMap<String, Vec<Vec<String>>> = HashMap::new();
            for (node, thread) in children {
                data.insert(node, thread.join().unwrap());
            }
            data
        }
    }
    fn get_raw(url: String, concurrent: u8) -> Vec<String> {
        let run = std::process::Command::new("siege")
            .args([
                "--json-output",
                "--concurrent",
                &concurrent.to_string(),
                &url,
            ])
            .output()
            .expect("\x1b[31mFailed to execute siege!\x1b[0m");
        let result: HashMap<String, f32> = if run.status.success() {
            eprintln!("\x1b[32m{}\x1b[0m", String::from_utf8_lossy(&run.stderr));
            serde_json::from_str(&String::from_utf8_lossy(&run.stdout)).unwrap()
        } else {
            panic!("\x1b[31m{}\x1b[0m", String::from_utf8_lossy(&run.stderr))
        };
        vec![
            url,
            result["transactions"].to_string(),
            result["availability"].to_string(),
            result["elapsed_time"].to_string(),
            result["data_transferred"].to_string(),
            result["response_time"].to_string(),
            result["transaction_rate"].to_string(),
            result["throughput"].to_string(),
            result["concurrency"].to_string(),
            result["successful_transactions"].to_string(),
            result["failed_transactions"].to_string(),
            result["longest_transaction"].to_string(),
            result["shortest_transaction"].to_string(),
        ]
    }
}

use efltk::prelude::*;
use std::collections::HashMap;

#[derive(Default)]
pub struct Siege {}

pub enum Msg {
    Proto(String),
    Target(String),
    Endpoint(String),
    Concurrent(u8),
    Port(u16),
    Result(String),
    Push,
}

impl Component for Siege {
    type Event = Msg;
    type State = models::Model;
    fn handle(msg: Self::Event, model: &mut Self::State, sender: Sender<Self::Event>) -> bool {
        match msg {
            Msg::Push => {
                std::thread::spawn({
                    let model = model.clone();
                    let sender = sender.clone();
                    move || {
                        let file = std::env::var("HOME").unwrap() + "/report.xlsx";
                        to_xlsx(&model.run(), &file);
                        sender.send(Msg::Result(file)).unwrap();
                    }
                });
            }
            Msg::Result(value) => model.result = value,
            Msg::Proto(value) => model.proto = value,
            Msg::Port(value) => model.port = value,
            Msg::Concurrent(value) => model.concurrent = value,
            Msg::Target(value) => {
                model.targets = value
                    .split_whitespace()
                    .map(str::to_string)
                    .collect::<Vec<String>>()
            }
            Msg::Endpoint(value) => {
                model.endpoints = value
                    .split_whitespace()
                    .map(str::to_string)
                    .collect::<Vec<String>>()
            }
        };
        false
    }
    fn update(&self, _model: &Self::State) {}
    fn view(&mut self, prt: &impl ContainerExt, sender: Sender<Self::Event>) {
        const WIDTH: i32 = 150;
        efltk::Box::new(prt).inside(|prt| {
            efltk::Box::new(prt).with_horizontal(true).inside(|prt| {
                efltk::Button::new(prt)
                    .with_size(WIDTH, -1)
                    .set_text("PROTO: ");
                efltk::Entry::with_menu(
                    prt,
                    efltk::Menu::popup(prt)
                        .with_items(&["https", "http"])
                        .with_value(0)
                        .with_callback({
                            let sender = sender.clone();
                            move |wgt| {
                                sender.send(Msg::Proto(wgt.selected().text())).unwrap();
                            }
                        }),
                );
            });
            efltk::Box::new(prt).with_horizontal(true).inside(|prt| {
                efltk::Button::new(prt)
                    .with_size(WIDTH, -1)
                    .set_text("CONCURRENT: ");
                efltk::Spinner::new(prt)
                    .with_range(1_f64, 8f64)
                    .with_step(1_f64)
                    .with_value(8f64)
                    .with_callback({
                        let sender = sender.clone();
                        move |wgt| {
                            sender.send(Msg::Concurrent(wgt.value() as u8)).unwrap();
                        }
                    })
                    .do_callback();
            });
            efltk::Box::new(prt).with_horizontal(true).inside(|prt| {
                efltk::Button::new(prt)
                    .with_size(WIDTH, -1)
                    .set_text("PORT: ");
                efltk::Entry::new(prt)
                    .with_value("443")
                    .with_callback({
                        let sender = sender.clone();
                        move |wgt| {
                            let value = wgt.value().parse::<u16>().unwrap_or_default();
                            sender.send(Msg::Port(value)).unwrap();
                        }
                    })
                    .do_callback();
            });
            efltk::Box::new(prt).with_horizontal(true).inside(|prt| {
                efltk::Button::new(prt)
                    .with_size(WIDTH, -1)
                    .set_text("TARGET: ");
                efltk::Entry::new(prt)
                    .with_single_line(false)
                    .with_value("127.0.0.1")
                    .with_callback({
                        let sender = sender.clone();
                        move |wgt| sender.send(Msg::Target(wgt.value())).unwrap()
                    })
                    .do_callback();
            });
            efltk::Box::new(prt).with_horizontal(true).inside(|prt| {
                efltk::Button::new(prt)
                    .with_size(WIDTH, -1)
                    .set_text("ENDPOINTS: ");
                efltk::Entry::new(prt)
                    .with_single_line(false)
                    .with_value("/")
                    .with_callback({
                        let sender = sender.clone();
                        move |wgt| sender.send(Msg::Endpoint(wgt.value())).unwrap()
                    })
                    .do_callback();
            });
            efltk::Box::new(prt).with_horizontal(true).inside(|prt| {
                efltk::Button::new(prt)
                    .with_size(WIDTH, -1)
                    .set_text("RESULT: ");
                efltk::Entry::new(prt)
                    .with_single_line(false)
                    .with_editable(false)
                    .with_signal(EntrySignal::Clicked, {
                        let sender = sender.clone();
                        move |_| sender.send(Msg::Push).unwrap()
                    });
            });
            efltk::Label::new(prt);
        });
    }
}

fn to_xlsx(date: &HashMap<String, Vec<Vec<String>>>, output: &str) {
    let workbook = xlsxwriter::Workbook::new(output).unwrap();
    for list in date.keys() {
        let mut sheet = workbook.add_worksheet(Some(list)).unwrap();
        for (row_ord, row) in date[list].iter().enumerate() {
            for (col_ord, cell) in row.iter().enumerate() {
                sheet
                    .write_string(row_ord as u32, col_ord as u16, cell, None)
                    .unwrap();
            }
        }
    }
    workbook.close().unwrap();
}
