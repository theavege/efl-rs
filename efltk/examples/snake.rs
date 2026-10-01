//! Snake — a [LÖVE](https://love2d.org/wiki/love)-style loop on Elementary `GLView`.
//!
//! | Callback        | LÖVE              | here                    |
//! |-----------------|-------------------|-------------------------|
//! | `love.load`     | setup             | `Love::load`            |
//! | `love.update`   | `dt` seconds      | `Love::update`          |
//! | `love.draw`     | frame             | `Love::draw` via GLES   |
//! | `love.keypressed` | keyboard        | `Love::keypressed`      |
//!
//! Keys: arrows / WASD to steer, Space to restart, Esc to quit.
//!
//! ```sh
//! cargo run -p efltk --example snake
//! ```

#![forbid(unsafe_code)]

use {
    efltk::{
        Glview, Label, Window,
        prelude::{
            BoxExt, ContainerExt, GlApi, GlviewExt, LabelExt, TextExt, WidgetExt, WindowExt, exit,
            run,
        },
    },
    std::{cell::RefCell, rc::Rc},
};

const GRID: i32 = 20;
const STEP: f64 = 0.12;

/// LÖVE-shaped game callbacks. The EFL host below is `love.run`.
trait Love {
    fn load() -> Self;
    fn update(&mut self, dt: f64);
    fn draw(&self, gl: &GlApi, w: i32, h: i32);
    fn keypressed(&mut self, key: &str);
}

struct Snake {
    body: Vec<(i32, i32)>,
    dir: (i32, i32),
    queued: (i32, i32),
    food: (i32, i32),
    accum: f64,
    alive: bool,
    score: u32,
    seed: u32,
}

impl Snake {
    fn spawn_food(&mut self) {
        for _ in 0..GRID * GRID {
            self.seed = self.seed.wrapping_mul(1664525).wrapping_add(1013904223);
            let x = (self.seed >> 16) as i32 % GRID;
            self.seed = self.seed.wrapping_mul(1664525).wrapping_add(1013904223);
            let y = (self.seed >> 16) as i32 % GRID;
            if !self.body.contains(&(x, y)) {
                self.food = (x, y);
                return;
            }
        }
        self.food = (0, 0);
    }

    fn step(&mut self) {
        if !self.alive {
            return;
        }
        if self.queued != (-self.dir.0, -self.dir.1) || self.body.len() == 1 {
            self.dir = self.queued;
        }
        let head = self.body[0];
        let next = (head.0 + self.dir.0, head.1 + self.dir.1);
        if !(0..GRID).contains(&next.0) || !(0..GRID).contains(&next.1) || self.body.contains(&next)
        {
            self.alive = false;
            return;
        }
        self.body.insert(0, next);
        if next == self.food {
            self.score += 1;
            self.spawn_food();
        } else {
            self.body.pop();
        }
    }

    fn status(&self) -> String {
        if self.alive {
            format!(
                "Score: {}   arrows/WASD  Space restart  Esc quit",
                self.score
            )
        } else {
            format!("Game over — {}  Space restart  Esc quit", self.score)
        }
    }
}

impl Love for Snake {
    fn load() -> Self {
        let mut snake = Self {
            body: vec![(6, 10), (5, 10), (4, 10)],
            dir: (1, 0),
            queued: (1, 0),
            food: (0, 0),
            accum: 0.0,
            alive: true,
            score: 0,
            seed: 0xC0FFEE,
        };
        snake.spawn_food();
        snake
    }

    fn update(&mut self, dt: f64) {
        self.accum += dt;
        while self.accum >= STEP {
            self.accum -= STEP;
            self.step();
        }
    }

    fn draw(&self, gl: &GlApi, w: i32, h: i32) {
        gl.viewport(0, 0, w, h);
        gl.clear_color(0.07, 0.08, 0.10, 1.0);
        gl.clear();
        let cell = (w.min(h) / GRID).max(1);
        let ox = (w - cell * GRID) / 2;
        let oy = (h - cell * GRID) / 2;
        let gap = i32::from(cell > 2);
        let paint = |x: i32, y: i32, rgb: [f32; 3]| {
            gl.fill_rect(ox + x * cell, oy + y * cell, cell - gap, cell - gap, h, rgb);
        };
        paint(self.food.0, self.food.1, [0.85, 0.28, 0.32]);
        for (i, &(x, y)) in self.body.iter().enumerate() {
            if i == 0 {
                paint(x, y, [0.45, 0.85, 0.40]);
            } else {
                paint(x, y, [0.25, 0.62, 0.32]);
            }
        }
    }

    fn keypressed(&mut self, key: &str) {
        let turn = match key {
            "Up" | "w" | "W" => (0, -1),
            "Down" | "s" | "S" => (0, 1),
            "Left" | "a" | "A" => (-1, 0),
            "Right" | "d" | "D" => (1, 0),
            "space" | "spacebar" => {
                *self = Self::load();
                return;
            }
            "Escape" | "q" | "Q" => {
                exit();
                return;
            }
            _ => return,
        };
        if turn != (-self.dir.0, -self.dir.1) || self.body.len() == 1 {
            self.queued = turn;
        }
    }
}

fn main() {
    run(|| {
        let win = Window::new("snake", "Snake")
            .with_min_size(480, 520)
            .with_center(true);
        efltk::Box::new(&win).inside(|prt| {
            let game = Rc::new(RefCell::new(Snake::load()));
            let status = Rc::new(Label::new(prt).with_text(&game.borrow().status()));
            Glview::new(prt)
                .with_weight(true, true)
                .with_render({
                    let game = game.clone();
                    move |gl, w, h| game.borrow().draw(gl, w, h)
                })
                .with_key_down({
                    let game = game.clone();
                    move |key| game.borrow_mut().keypressed(key)
                })
                .with_tick(1.0 / 30.0, {
                    let game = game.clone();
                    let status = status.clone();
                    move |dt| {
                        let mut snake = game.borrow_mut();
                        snake.update(dt);
                        status.set_text(&snake.status());
                    }
                });
        });
        win
    });
}
