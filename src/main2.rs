use nannou::prelude::*;
use fstrings::*;
use rand::Rng;
use rayon::prelude::*;
// use core::time::Duration;
use std::time::{Instant};
use itertools::iproduct;

fn main() { 
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
    objects: Vec<particle>
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();
    // app.set_loop_mode(LoopMode::Rate { update_interval: (Duration::new(5000,0)) });
    let objects = initialisation();
    return Model { _window, objects };
    
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    _model.objects = update_particles(_model.objects.clone());
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    
    draw.background().color(WHITE);

    for particle_i in _model.objects.clone().iter_mut() {
        draw.ellipse().color(GREEN).x_y(particle_i.position[0],particle_i.position[1]).radius(particle_i.mass);
    }

    draw.to_frame(app, &frame).unwrap();
}

#[allow(bad_style)]
#[derive(Clone)]
struct particle {
    name:           String,
    mass:           f32,
    velocity:       Vec<f32>,
    position:       Vec<f32>,
    stationary:     bool,
    draw_path:      bool
}


fn distance_between_vectors_squared(vector1: Vec<f32>, vector2: Vec<f32>) -> f32 {
    let mut sum_of_squares: f32 = 0.0;
    
    for n in 0..=vector1.len() - 1 {
        sum_of_squares = sum_of_squares + (vector1[n] - vector2[n]).powi(2);
    }

    return sum_of_squares;
}

fn add_vectors(vector1: Vec<f32>, vector2: Vec<f32>) -> Vec<f32> {
    let mut vector_sum: Vec<f32> = vec![];
    
        for n in 0..=vector1.len() - 1 {
            vector_sum.push(vector1[n] + vector2[n]);
        }

    return vector_sum;
}

fn sub_vectors(vector1: Vec<f32>, vector2: Vec<f32>) -> Vec<f32> {
    let mut vector_sum: Vec<f32> = vec![];
    
        for n in 0..=vector1.len() - 1 {
            vector_sum.push(vector1[n] - vector2[n]);
        }

    return vector_sum;
}

fn initialisation() -> Vec<particle> {
    let mut rng = rand::thread_rng();

    let mut particle_list: Vec<particle> = Vec::new();
    const NOP: i32 = 5000;
    for i in 0..NOP {
        let x = rng.gen_range(-200.0..200.0);
        let y = rng.gen_range(-200.0..200.0);
        let rand_mass = rng.gen_range(1.0..5.0); 
        particle_list.push(
            particle{
                name:       f!("particle{i}"),
                mass:       rand_mass, 
                velocity:   vec![-y/50.0, x/50.0], 
                // velocity:   vec![0.0,0.0], 
                position:   vec![x, y],
                stationary: false,
                draw_path:  true
            },
        )
    }

    
    return particle_list;
}

fn update_particles(particle_list: Vec<particle>) -> Vec<particle> {
    let start: Instant = Instant::now();
    
    let grav_constant = 1.0;
    let particle_list_copy = particle_list.clone();
    
    let new_particle_list = particle_list.par_iter().map(|original_particle: &particle| {

        let mut particle_i: particle = original_particle.clone();

        if particle_i.stationary == true { return particle_i };
        
        let mut velocity_change_tot: Vec<f32> = vec![0.0 , 0.0];
        for particle_j in &particle_list_copy {
            let distance_squaredij = distance_between_vectors_squared(particle_i.position.clone(), particle_j.position.clone());
            let distanceij = distance_squaredij.sqrt();
            
            if particle_i.name == particle_j.name {break;};

            if distanceij < 10.0 {break;};
            
            let normal_vectorij = vec![
                sub_vectors(particle_j.position.clone(), particle_i.position.clone())[0] / distanceij,
                sub_vectors(particle_j.position.clone(), particle_i.position.clone())[1] / distanceij
            ];
            
            velocity_change_tot = vec![velocity_change_tot[0] + 
                normal_vectorij[0] * (grav_constant * particle_j.mass / distance_squaredij), 
                velocity_change_tot[1] + 
                normal_vectorij[1] * (grav_constant * particle_j.mass / distance_squaredij)];

        }

        // println!("{:?}", velocity_change_tot);
        particle_i.velocity = add_vectors(particle_i.velocity, velocity_change_tot);
        particle_i.position = add_vectors(particle_i.position, particle_i.velocity.clone());

        // particle_i.position[0] = 0.0;
        return particle_i;
    }).collect();
    println!("Time elapsed: {:?}", start.elapsed());


    return new_particle_list;
}

// TODO:
// center of mass point
// moving viewport 
// zooming in out 