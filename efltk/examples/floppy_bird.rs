//! Floppy Bird — a [LÖVE](https://love2d.org/wiki/love)-style loop on Elementary `GLView`.
//!
//! | Callback          | LÖVE     | here                 |
//! |-------------------|----------|----------------------|
//! | `love.load`       | setup    | `Love::load`         |
//! | `love.update`     | `dt`     | `Love::update`       |
//! | `love.draw`       | frame    | `Love::draw` via GLES|
//! | `love.keypressed` | keyboard | `Love::keypressed`   |
//! | `love.mousepressed` | click  | `with_mouse_down`    |
//!
//! Keys: Space / Up / W or click to flap, Space/click to restart after a crash, Esc to quit.
//!
//! ```sh
//! cargo run -p efltk --example floppy_bird
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

const WORLD_W: f32 = 400.0;
const WORLD_H: f32 = 480.0;
const GROUND: f32 = 48.0;
const BIRD_X: f32 = 92.0;
const BIRD_W: f32 = 28.0;
const BIRD_H: f32 = 22.0;
const PIPE_W: f32 = 54.0;
const GAP: f32 = 132.0;
const GRAVITY: f32 = 1500.0;
const FLAP: f32 = -420.0;
const SPEED: f32 = 150.0;
const SPAWN: f64 = 1.45;

/// LÖVE-shaped game callbacks. The EFL host below is `love.run`.
trait Love {
    fn load() -> Self;
    fn update(&mut self, dt: f64);
    fn draw(&self, gl: &GlApi, w: i32, h: i32);
    fn keypressed(&mut self, key: &str);
}

struct Pipe {
    x: f32,
    gap_y: f32,
    scored: bool,
}

struct Bird {
    y: f32,
    vy: f32,
    pipes: Vec<Pipe>,
    spawn: f64,
    alive: bool,
    score: u32,
    best: u32,
    seed: u32,
}

impl Bird {
    fn rnd_gap(&mut self) -> f32 {
        self.seed = self.seed.wrapping_mul(1664525).wrapping_add(1013904223);
        let loft = 72.0;
        let span = (WORLD_H - GROUND - GAP - loft * 2.0).max(1.0);
        loft + ((self.seed >> 16) as f32 % span)
    }

    fn flap(&mut self) {
        if self.alive {
            self.vy = FLAP;
        } else {
            let best = self.best;
            *self = Self::load();
            self.best = best;
        }
    }

    fn hits_pipe(&self, pipe: &Pipe) -> bool {
        let bx0 = BIRD_X;
        let bx1 = BIRD_X + BIRD_W;
        let by0 = self.y;
        let by1 = self.y + BIRD_H;
        let px0 = pipe.x;
        let px1 = pipe.x + PIPE_W;
        if bx1 < px0 || bx0 > px1 {
            return false;
        }
        by0 < pipe.gap_y || by1 > pipe.gap_y + GAP
    }

    fn status(&self) -> String {
        if self.alive {
            format!(
                "Score: {}  Best: {}   click/Space flap  Esc quit",
                self.score, self.best
            )
        } else {
            format!(
                "Crashed — {} (best {})  click/Space restart  Esc quit",
                self.score, self.best
            )
        }
    }
}

impl Love for Bird {
    fn load() -> Self {
        Self {
            y: WORLD_H * 0.4,
            vy: 0.0,
            pipes: Vec::new(),
            spawn: 0.6,
            alive: true,
            score: 0,
            best: 0,
            seed: 0xB1BD,
        }
    }

    fn update(&mut self, dt: f64) {
        if !self.alive {
            return;
        }
        let dt32 = dt as f32;
        self.vy += GRAVITY * dt32;
        self.y += self.vy * dt32;
        let floor = WORLD_H - GROUND - BIRD_H;
        if self.y < 0.0 || self.y > floor {
            self.y = self.y.clamp(0.0, floor);
            self.alive = false;
            self.best = self.best.max(self.score);
            return;
        }
        self.spawn -= dt;
        if self.spawn <= 0.0 {
            self.spawn += SPAWN;
            let gap_y = self.rnd_gap();
            self.pipes.push(Pipe {
                x: WORLD_W,
                gap_y,
                scored: false,
            });
        }
        for pipe in &mut self.pipes {
            pipe.x -= SPEED * dt32;
            if !pipe.scored && pipe.x + PIPE_W < BIRD_X {
                pipe.scored = true;
                self.score += 1;
            }
        }
        self.pipes.retain(|pipe| pipe.x + PIPE_W > -8.0);
        if self.pipes.iter().any(|pipe| self.hits_pipe(pipe)) {
            self.alive = false;
            self.best = self.best.max(self.score);
        }
    }

    fn draw(&self, gl: &GlApi, w: i32, h: i32) {
        gl.viewport(0, 0, w, h);
        gl.clear_color(0.40, 0.74, 0.86, 1.0);
        gl.clear();
        let sx = w as f32 / WORLD_W;
        let sy = h as f32 / WORLD_H;
        let rect = |x: f32, y: f32, rw: f32, rh: f32, rgb: [f32; 3]| {
            gl.fill_rect(
                (x * sx).round() as i32,
                (y * sy).round() as i32,
                (rw * sx).round().max(1.0) as i32,
                (rh * sy).round().max(1.0) as i32,
                h,
                rgb,
            );
        };
        for pipe in &self.pipes {
            rect(pipe.x, 0.0, PIPE_W, pipe.gap_y, [0.22, 0.62, 0.28]);
            let bottom = pipe.gap_y + GAP;
            rect(
                pipe.x,
                bottom,
                PIPE_W,
                WORLD_H - GROUND - bottom,
                [0.22, 0.62, 0.28],
            );
            rect(
                pipe.x - 3.0,
                pipe.gap_y - 14.0,
                PIPE_W + 6.0,
                14.0,
                [0.18, 0.52, 0.24],
            );
            rect(pipe.x - 3.0, bottom, PIPE_W + 6.0, 14.0, [0.18, 0.52, 0.24]);
        }
        rect(0.0, WORLD_H - GROUND, WORLD_W, GROUND, [0.82, 0.70, 0.32]);
        rect(0.0, WORLD_H - GROUND, WORLD_W, 6.0, [0.34, 0.72, 0.32]);
        let bird = if self.alive {
            [0.98, 0.84, 0.22]
        } else {
            [0.72, 0.42, 0.28]
        };
        rect(BIRD_X, self.y, BIRD_W, BIRD_H, bird);
        rect(
            BIRD_X + BIRD_W - 8.0,
            self.y + 6.0,
            6.0,
            6.0,
            [0.12, 0.12, 0.14],
        );
        rect(BIRD_X + BIRD_W, self.y + 10.0, 8.0, 6.0, [0.92, 0.42, 0.22]);
    }

    fn keypressed(&mut self, key: &str) {
        match key {
            "space" | "spacebar" | "Up" | "w" | "W" => self.flap(),
            "Escape" | "q" | "Q" => exit(),
            _ => {}
        }
    }
}

fn main() {
    run(|| {
        let win = Window::new("floppy-bird", "Floppy Bird")
            .with_min_size(400, 520)
            .with_center(true);
        efltk::Box::new(&win).inside(|prt| {
            let game = Rc::new(RefCell::new(Bird::load()));
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
                .with_mouse_down({
                    let game = game.clone();
                    move |_x, _y, button| {
                        if button == 1 {
                            game.borrow_mut().flap();
                        }
                    }
                })
                .with_tick(1.0 / 60.0, {
                    let game = game.clone();
                    let status = status.clone();
                    move |dt| {
                        let mut bird = game.borrow_mut();
                        bird.update(dt);
                        status.set_text(&bird.status());
                    }
                });
        });
        win
    });
}
