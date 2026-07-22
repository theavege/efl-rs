#![forbid(unsafe_code)]

mod models {
    use std::path::{Path, PathBuf};
    use std::fs;

    #[derive(Default)]
    pub struct Model {
        pub current: Option<PathBuf>,
        pub zoom: f64,
        pub index: usize,
        pub files: Vec<PathBuf>,
    }

    impl Model {
        pub fn open(&mut self, path: String) {
            let p = PathBuf::from(path);
            if p.is_file() {
                self.current = Some(p.clone());
                self.zoom = 1.0;
                self.load_dir(&p);
            }
        }

        fn load_dir(&mut self, current: &Path) {
            if let Some(parent) = current.parent() {
                if let Ok(entries) = fs::read_dir(parent) {
                    let mut imgs: Vec<PathBuf> = entries
                        .filter_map(|e| e.ok().map(|e| e.path()))
                        .filter(|p| p.is_file() && Self::is_image(p))
                        .collect();
                    imgs.sort();
                    self.files = imgs;
                    self.index = self.files.iter().position(|p| p == current).unwrap_or(0);
                }
            }
        }

        fn navigate(&mut self, delta: isize) {
            if self.files.is_empty() { return; }
            let len = self.files.len();
            self.index = ((self.index as isize + delta).rem_euclid(len as isize)) as usize;
            self.current = Some(self.files[self.index].clone());
        }

        fn is_image(p: &Path) -> bool {
            matches!(p.extension().and_then(|s| s.to_str()), Some("png"|"jpg"|"jpeg"|"gif"|"bmp"|"webp"|"tiff"))
        }
    }
}

use efltk::prelude::*;

#[derive(Default)]
pub struct ImageViewer {
    image: Option<efltk::BufferImage>,
    status: Option<efltk::Label>,
}

pub enum Msg {
    Open(String),
    Next,
    Prev,
    Zoom(f64),
    Fit,
    Rotate,
    Quit,
}

impl Component for ImageViewer {
    type Event = Msg;
    type State = models::Model;

    fn handle(msg: Self::Event, model: &mut Self::State, _: Sender<Self::Event>) -> bool {
        match msg {
            Msg::Open(p) => model.open(p),
            Msg::Next => model.navigate(1),
            Msg::Prev => model.navigate(-1),
            Msg::Zoom(d) => model.zoom = (model.zoom + d).clamp(0.2, 5.0),
            Msg::Fit => model.zoom = 1.0,
            Msg::Rotate => {},
            Msg::Quit => { efltk::prelude::exit(); return false; }
        }
        true
    }

    fn update(&self, model: &Self::State) {
        if let Some(img) = &self.image {
            if let Some(p) = &model.current {
                let _ = img.load(&p.to_string_lossy());
            }
        }
        if let Some(lbl) = &self.status {
            let txt = model.current.as_ref()
                .map(|p| p.file_name().unwrap_or_default().to_string_lossy().into_owned())
                .unwrap_or_else(|| "No image".to_string());
            lbl.set_text(&format!("{} | Zoom: {:.1}x", txt, model.zoom));
        }
    }

    fn view(&mut self, prt: &impl ContainerExt, sender: Sender<Self::Event>) {
        efltk::Box::new(prt).with_horizontal(false).inside(|prt| {
            efltk::Box::new(prt).with_horizontal(true).inside(|prt| {
                Button::new(prt).with_text("Open").with_callback({
                    let s = sender.clone();
                    move |_| { s.send(Msg::Open("example.jpg".to_string())).ok(); }
                });
                // Add more buttons similarly
            });

            let frame = efltk::Frame::new(prt).with_defaults();
            self.image = Some(efltk::BufferImage::new(&frame));

            self.status = Some(efltk::Label::new(prt));
        });
    }
}

impl ImageViewer {
    pub fn mount(prt: &impl ContainerExt) {
        <Self as Component>::mount(prt);
    }
}
