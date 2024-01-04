use macroquad::prelude::*;

use crate::{particle::Particle, WINDOW_HEIGHT, WINDOW_WIDTH};

pub fn calculate_interactions(particles: &mut Vec<Particle>, rules: &Rules, dt: f32, m: f32){
    let particle_amount = particles.len();
    for i in 0..particle_amount{
        let mut force_x = 0.0;
        let mut force_y = 0.0;
        for j in 0..particle_amount{
            let dx: f32 = particles[i].pos_x - particles[j].pos_x;
            let dy: f32 = particles[i].pos_y - particles[j].pos_y;
            let distance: f32 = f32::sqrt(dx*dx - dy*dy); // c = sqrt(a^2+b^2)
            if distance > 0. && distance < 80.{
                let rules_factor = rules.get_force(particles[i].color, particles[j].color);
                let force = rules_factor / distance;

                force_x += force * dx * m;
                force_y += force * dy * m;
            }
        }
        particles[i].vel_x = (particles[i].vel_x + force_x) * dt;
        particles[i].vel_y = (particles[i].vel_y + force_y) * dt;

        let next_x: f32 = particles[i].pos_x + particles[i].vel_x;
        let next_y: f32 = particles[i].pos_y + particles[i].vel_y;

        if next_x < 0. || next_x > WINDOW_WIDTH{
            particles[i].vel_x *= -1.;
        }
        if next_y < 0. || next_y > WINDOW_HEIGHT{
            particles[i].vel_y *= -1.;
        }

        particles[i].pos_x += particles[i].vel_x;
        particles[i].pos_y += particles[i].vel_y;

    }
}


pub struct Rules {
    rules: [[f32; 4]; 4],
    //friction: f32,
}

impl Rules {
    pub fn new() -> Self {
        Self {
            rules: [
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
            ],
               
            //friction: 0.0,
        }
    }
    pub fn set_force(&mut self, color_a: Color, color_b: Color, value: f32) {
        
        self.rules[value_of(color_a)][value_of(color_b)] = value;

    }

    pub fn get_force(&self, color_a: Color, color_b: Color) -> f32 {
        self.rules[value_of(color_a)][value_of(color_b)]
    }


}

pub fn value_of(color: Color) -> usize {
    if color.eq(&YELLOW) {
        0
    }
    else if color.eq(&BLUE) {
        1
    }
    else if color.eq(&RED) {
        2
    }
    else if color.eq(&WHITE) {
        3
    }
    else { 0 }

}