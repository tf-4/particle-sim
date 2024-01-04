use macroquad::prelude::*;
// use std::fmt;
use ::rand::Rng;

use super::{WINDOW_HEIGHT, WINDOW_WIDTH};

// #[derive(Debug)]
pub struct Particle {
    pub pos_x: f32,
    pub pos_y: f32,
    pub vel_x: f32,
    pub vel_y: f32,
    pub color: Color,
}

// impl fmt::Debug for Particle {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         f.debug_struct("Particle")
//         .field("pos_x", &self.pos_x)
//         .field("pos_y", &self.pos_y)
//         .field("vel_x", &self.vel_x)
//         .field("vel_y", &self.vel_y)
//         .field("color", &self.color)
//     }
// }

impl Particle {
    pub fn new(pos_x: f32, pos_y: f32, color: Color) -> Self {
        Self {
            pos_x,
            pos_y,
            vel_x: 0.,
            vel_y: 0.,
            color,
        }
    }
}

pub fn particle_spawner(yellow: i32, blue: i32, red: i32, white: i32) -> Vec<Particle>{
    let mut particles: Vec<Particle> = Vec::new();
    let mut rng = ::rand::thread_rng();
    for _i in 0..yellow{
        let new_x: u32 = rng.gen_range(0..(WINDOW_WIDTH as u32));
        let new_y: u32 = rng.gen_range(0..(WINDOW_HEIGHT as u32));
        let new_particle = Particle::new(new_x as f32, new_y as f32, YELLOW);
        particles.push(new_particle);
    }
    for _i in 0..blue{
        let new_x: u32 = rng.gen_range(0..(WINDOW_WIDTH as u32));
        let new_y: u32 = rng.gen_range(0..(WINDOW_HEIGHT as u32));
        particles.push(Particle::new(new_x as f32, new_y as f32, BLUE));
    }
    for _i in 0..red{
        let new_x: u32 = rng.gen_range(0..(WINDOW_WIDTH as u32));
        let new_y: u32 = rng.gen_range(0..(WINDOW_HEIGHT as u32));
        particles.push(Particle::new(new_x as f32, new_y as f32, RED));
    }
    for _i in 0..white{
        let new_x: u32 = rng.gen_range(0..(WINDOW_WIDTH as u32));
        let new_y: u32 = rng.gen_range(0..(WINDOW_HEIGHT as u32));
        particles.push(Particle::new(new_x as f32, new_y as f32, WHITE));
    }

    particles
}