use macroquad::prelude::*;
// use std::time::{SystemTime, UNIX_EPOCH};

mod particle;
use particle::*;

mod interaction;
use interaction::*;

const WINDOW_WIDTH: f32 = 1000.;
const WINDOW_HEIGHT: f32 = 500.;
// const UI_WIDTH: f32 = 300.;


#[macroquad::main(window_conf)]
async fn main(){
    let dt: f32 = 0.25;
    let m: f32 = 1.0;
    let mut rules = Rules::new();
    set_rules(&mut rules);
    let mut particles: Vec<Particle> = particle_spawner(200, 200, 200, 200);
    clear_background(BLACK);
    loop{
        calculate_interactions(&mut particles, &rules, dt, m);
        for particle in particles.iter(){
            draw_circle(particle.pos_x, particle.pos_y, 3., particle.color);
        }
        next_frame().await
    }
}

fn set_rules(rules: &mut Rules){
    /*
     * Positive Value = Repel
     * Negative Value = Attract
     */
    // Interesting rulesets
    /* Weird bird thing
    rules.set_force(RED, RED, -0.32);
    rules.set_force(RED, BLUE, -0.17);
    rules.set_force(RED, YELLOW, 0.34);
    rules.set_force(BLUE, BLUE, -0.10);
    rules.set_force(BLUE, RED, -0.34);
    rules.set_force(YELLOW, YELLOW, 0.15);
    rules.set_force(YELLOW, RED, -0.20);
    */
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Particles".to_owned(),
        window_width: WINDOW_WIDTH as i32,
        window_height: WINDOW_HEIGHT as i32,
        fullscreen: false,
        high_dpi: true,
        ..Default::default()
    }
}