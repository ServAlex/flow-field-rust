use std::{
    array::{self, from_fn},
    f64,
};

use raylib::prelude::*;

struct RealVector {
    x: f64,
    y: f64,
}

impl RealVector {
    fn add(&self, second: &RealVector) -> RealVector {
        RealVector {
            x: self.x + second.x,
            y: self.y + second.y,
        }
    }

    fn subtract(&self, second: &RealVector) -> RealVector {
        RealVector {
            x: self.x - second.x,
            y: self.y - second.y,
        }
    }

    fn multiply(&self, factor: f64) -> RealVector {
        RealVector {
            x: self.x * factor,
            y: self.y * factor,
        }
    }

    fn limit(&self, min: f64, max: f64) -> RealVector {
        let magnitude = self.get_magnitude();

        if magnitude < 0.0001 {
            return RealVector {
                x: self.x,
                y: self.y,
            };
        }

        if magnitude > max {
            self.multiply(max / magnitude)
        } else if magnitude < min {
            self.multiply(min / magnitude)
        } else {
            RealVector {
                x: self.x,
                y: self.y,
            }
        }
    }

    fn bring_to_limits_looping(value: f64, min: f64, max: f64) -> f64 {
        let length = max - min;
        let loops = ((value - min) / length).floor();
        value - loops * length
    }

    fn bring_to_box(&mut self, width: i32, height: i32) -> RealVector {
        RealVector {
            x: RealVector::bring_to_limits_looping(self.x, 0.0, width as f64),
            y: RealVector::bring_to_limits_looping(self.y, 0.0, height as f64),
        }
    }

    fn get_magnitude(&self) -> f64 {
        f64::sqrt(self.x.powi(2) + self.y.powi(2))
    }

    fn get_angle(&self) -> f64 {
        f64::atan2(self.y, self.x)
    }
}

struct Slot {
    vector: RealVector,
    start_point: RealVector,
}

fn map_value(min: f64, max: f64, new_min: f64, new_max: f64, value: f64) -> f64 {
    if value < min {
        return new_min;
    }
    if value > max {
        return new_max;
    }

    let p = (value - min) / (max - min);
    (new_max - new_min) * p + new_min
}

fn field_function(x: f64, y: f64) -> f64 {
    RealVector {
        x: y.cos(),
        y: x.sin(),
    }
        .get_angle()
}

const MULTIPLIER: f64 = 0.6;
const WIDTH: i32 = 1280;
const HEIGHT: i32 = 720;
const SCALE: i32 = 20;
const LENGTH: f64 = SCALE as f64 * MULTIPLIER;
const ROWS: usize = (HEIGHT / SCALE) as usize;
const COLUMNS: usize = (WIDTH / SCALE) as usize;
const PARTICLE_COUNT: usize = 5000;
const PARTICLE_MIN_SPEED: f64 = 0.3;
//const PARTICLE_MAX_SPEED: f64 = 1.0;
const PARTICLE_MAX_SPEED: f64 = 2.0;

struct Particle {
    position: RealVector,
    velocity: RealVector,
    min_speed: f64,
    max_speed: f64,
}

impl Particle {
    fn new(position: RealVector, velocity: RealVector, min_speed: f64, max_speed: f64) -> Particle {
        Particle {
            position,
            velocity,
            min_speed,
            max_speed,
        }
    }

    fn apply_force(&mut self, force: &RealVector) {
        self.velocity = self
            .velocity
            .add(force)
            .limit(self.min_speed, self.max_speed);
    }

    fn update(&mut self) {
        self.position = self.position.add(&self.velocity);
        self.position = self.position.bring_to_box(WIDTH, HEIGHT);
    }
}

fn get_column(i: usize) -> [Slot; ROWS] {
    array::from_fn(|j| Slot {
        vector: RealVector { x: LENGTH, y: 0.0 },
        start_point: RealVector {
            x: (i as i32 * SCALE) as f64 + SCALE as f64 * 0.5,
            y: (j as i32 * SCALE) as f64 + SCALE as f64 * 0.5,
        },
    })
}

fn main() {
    let mut board: [[Slot; ROWS]; COLUMNS] = array::from_fn(get_column);
    let mut particles: [Particle; PARTICLE_COUNT] = array::from_fn(|i| {
        Particle::new(
            RealVector {
                x: i as f64 * 10.0,
                y: i as f64 * 10.0,
            }
                .bring_to_box(WIDTH, HEIGHT),
            RealVector {
                x: 0.0,
                y: PARTICLE_MIN_SPEED,
            },
            PARTICLE_MIN_SPEED,
            PARTICLE_MAX_SPEED,
        )
    });

    let (mut rl, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("flow field rust")
        .build();

    rl.set_target_fps(60);

    let modification_speed = 0.02;
    let mut modifier = 0.0;
    const ANGLE_SCALE: f64 = 0.2;
    const DRAW_VECTORS: bool = false;
    const DRAW_DEBUG: bool = false;
    //const FORCE_LIMIT_MIN: f64 = 0.001;
    //const FORCE_LIMIT_MAX: f64 = 0.02;
    const FORCE_LIMIT_MIN: f64 = 0.01;
    const FORCE_LIMIT_MAX: f64 = 0.10;

    while !rl.window_should_close() {
        let mut drawing_context = rl.begin_drawing(&thread);

        //drawing_context.clear_background(Color::BLACK);
        drawing_context.draw_rectangle(0, 0, WIDTH, HEIGHT, Color::new(0, 0, 0, 20));

        modifier += modification_speed;

        for x in 0..COLUMNS {
            for y in 0..ROWS {
                let angle = field_function(
                    x as f64 * ANGLE_SCALE + modifier,
                    y as f64 * ANGLE_SCALE + modifier,
                );
                board[x][y].vector = RealVector {
                    x: angle.cos() * LENGTH,
                    y: angle.sin() * LENGTH,
                };

                if DRAW_VECTORS {
                    let slot = &board[x][y];

                    let end = slot.start_point.add(&slot.vector);
                    drawing_context.draw_line(
                        slot.start_point.x as i32,
                        slot.start_point.y as i32,
                        end.x as i32,
                        end.y as i32,
                        Color::GREEN,
                    );
                }
            }
        }

        for i in 0..PARTICLE_COUNT {
            let particle = &mut particles[i];

            let x_index = (particle.position.x.floor() / (SCALE as f64)) as usize;
            let y_index = (particle.position.y.floor() / (SCALE as f64)) as usize;

            particle.apply_force(
                &board[x_index][y_index]
                    .vector
                    .limit(FORCE_LIMIT_MIN, FORCE_LIMIT_MAX),
            );
            particle.update();

            drawing_context.draw_circle(
                particle.position.x as i32,
                particle.position.y as i32,
                2.0,
                Color::RED,
            );
        }

        if DRAW_DEBUG {
            let p = &particles[0];

            let first_particle_position_info = format!("x: {} y: {}", p.position.x, p.position.y);
            let first_particle_velocity_info =
                format!("x: {:.4} y: {:.4}", p.velocity.x, p.velocity.y);
            drawing_context.draw_text(&first_particle_position_info, 10, 10, 20, Color::WHITE);
            drawing_context.draw_text(&first_particle_velocity_info, 200, 10, 20, Color::WHITE);
        }
    }
}