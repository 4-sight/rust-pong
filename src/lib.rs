use graphics::*;
use opengl_graphics::GlGraphics;
use piston::input::{Button, Key, RenderArgs, UpdateArgs};
use std::collections::HashSet;
use std::process;

const BACKGROUND: [f32; 4] = [0.0, 0.5, 0.5, 1.0];
const FOREGROUND: [f32; 4] = [0.0, 0.0, 1.0, 1.0];

pub struct Size {
    pub width: u32,
    pub height: u32,
}

#[derive(Clone)]
pub struct Paddle {
    color: [f32; 4],
    vel: f32,
    pos_x: i32,
    pos_y: i32,
    score: i32,
    sprite: [f64; 4],
}

impl Paddle {
    fn new(color: [f32; 4], pos_x: i32) -> Paddle {
        Paddle {
            color,
            vel: 0.0,
            pos_x,
            pos_y: 1,
            score: 0,
            sprite: rectangle::square(0.0, 0.0, 50.0),
        }
    }

    fn inc_score(&mut self) {
        self.score += 1;
    }

    fn move_paddle(&mut self, vh: u32) {
        if self.vel > 0.0 {
            let base = vh as i32 - 50;
            self.pos_y += self.vel as i32;
            self.pos_y = if self.pos_y >= base { base } else { self.pos_y };
        } else {
            self.pos_y += self.vel as i32;
            self.pos_y = if self.pos_y <= 0 { 0 } else { self.pos_y };
        }
    }
}

pub struct App {
    gl: GlGraphics,
    keys: HashSet<Key>,
    height: u32,
    width: u32,
    scale: f64,
    player_1: Paddle,
    player_2: Paddle,
    ball_x: i32,
    ball_y: i32,
    vel_x: i32,
    vel_y: i32,
}

impl App {
    pub fn new(gl: GlGraphics, size: Size) -> App {
        App {
            gl,
            keys: HashSet::new(),
            width: size.width,
            height: size.height,
            scale: 1.0,
            player_1: Paddle::new(FOREGROUND, -40),
            player_2: Paddle::new(FOREGROUND, size.width as i32 - 10),
            ball_x: 0,
            ball_y: 0,
            vel_x: 1,
            vel_y: 1,
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        self.set_scale(args.height, args.width);

        let ball = rectangle::square(0.0, 0.0, 10.0);

        let ball_x = self.ball_x as f64;
        let ball_y = self.ball_y as f64;
        let App {
            player_1, player_2, ..
        } = self;

        let p1 = player_1.clone();
        let p2 = player_2.clone();

        self.gl.draw(args.viewport(), |c, gl| {
            clear(BACKGROUND, gl);
            rectangle(
                p1.color,
                p1.sprite,
                c.transform.trans(p1.pos_x as f64, p1.pos_y as f64),
                gl,
            );
            rectangle(
                p2.color,
                p2.sprite,
                c.transform.trans(p2.pos_x as f64, p2.pos_y as f64),
                gl,
            );
            rectangle(FOREGROUND, ball, c.transform.trans(ball_x, ball_y), gl)
        })
    }

    pub fn update(&mut self, _args: &UpdateArgs) {
        self.get_player_input();
        // Move paddles
        self.player_1.move_paddle(self.height);
        self.player_2.move_paddle(self.height);

        //Move ball
        self.ball_x += self.vel_x;
        if self.ball_x > 502 {
            self.vel_x = -self.vel_x;
            if self.ball_y < self.player_2.pos_y || self.ball_y > self.player_2.pos_y + 50 {
                self.player_1.inc_score();
                if self.player_1.score >= 5 {
                    println!("Left wins!");
                    process::exit(0);
                }
                self.ball_x = 256;
                self.ball_y = 171;
            }
        }
        if self.ball_x < 1 {
            self.vel_x = -self.vel_x;
            if self.ball_y < self.player_1.pos_y || self.ball_y > self.player_1.pos_y + 50 {
                self.player_2.inc_score();
                if self.player_2.score >= 5 {
                    println!("Right wins!");
                    process::exit(0);
                }
                self.ball_x = 256;
                self.ball_y = 171;
            }
        }

        self.ball_y += self.vel_y;
        if self.ball_y > 332 || self.ball_y < 1 {
            self.vel_y = -self.vel_y;
        }
    }

    pub fn get_player_input(&mut self) {
        if self.keys.contains(&Key::Up) {
            self.player_2.vel = -1.0;
        } else if self.keys.contains(&Key::Down) {
            self.player_2.vel = 1.0;
        } else {
            self.player_2.vel = 0.0;
        }

        if self.keys.contains(&Key::W) {
            self.player_1.vel = -1.0;
        } else if self.keys.contains(&Key::S) {
            self.player_1.vel = 1.0;
        } else {
            self.player_1.vel = 0.0;
        }
    }

    pub fn press(&mut self, args: &Button) {
        if let &Button::Keyboard(key) = args {
            self.keys.insert(key);
        }
    }

    pub fn release(&mut self, args: &Button) {
        if let &Button::Keyboard(key) = args {
            self.keys.remove(&key);
        }
    }

    fn set_scale(&mut self, height: u32, width: u32) {
        let scale_x = width as f64 / self.width as f64;
        let scale_y = height as f64 / self.height as f64;

        if scale_x <= scale_y {
            self.scale = scale_x
        } else {
            self.scale = scale_y
        }
    }
}
