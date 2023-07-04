use nannou::prelude::*;
use fstrings::*;
use rand::Rng;
use rayon::prelude::*;
// use core::time::Duration;
use std::time::{Instant};
use ndarray::arr1;

const NOP: i32 = 5000;

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
        draw.ellipse().color(particle_i.color).x_y(particle_i.position[0],particle_i.position[1]).radius(particle_i.mass);
    }

    draw.to_frame(app, &frame).unwrap();
}

#[allow(bad_style)]
#[derive(Clone)]
struct particle {
    name:           String,
    mass:           f32,
    position:       Vec<f32>,
    velocity:       Vec<f32>,
    acceleration:   Vec<f32>,
    stationary:     bool,
    draw_path:      bool,
    color:          Srgb<u8>
}


fn distance_between_vectors_squared(vector1: &Vec<f32>, vector2: &Vec<f32>) -> f32 {
    let mut sum_of_squares: f32 = 0.0;
    
    for n in 0..vector1.len() {
        sum_of_squares = sum_of_squares + (vector1[n] - vector2[n]).powi(2);
    }

    return sum_of_squares;
}

// fn distance_between_vectors_squared(vector1: &Vec<f32>, vector2: &Vec<f32>) -> f32 {
//     vector1.par_iter().enumerate().map(|(index, component_vector1)| {
//         (component_vector1 - vector2[index]).pow(2)
//     }).collect::<Vec<f32>>().par_iter().sum()
// }


fn add_vectors(vector1: &Vec<f32>, vector2: &Vec<f32>) -> Vec<f32> {
    vector1.par_iter().enumerate().map(|(index, component_vector1)| {
        component_vector1 + vector2[index]
    }).collect()
}


// fn sub_vectors(vector1: &Vec<f32>, vector2: &Vec<f32>) -> Vec<f32> {
//     let mut vector_sum: Vec<f32> = vec![];
    
//         for n in 0..vector1.len() {
//             vector_sum.push(vector1[n] - vector2[n]);
//         }

//     return vector_sum;
// }

// fn iter_to_array(iter: impl ParallelIterator<Item = particle>) -> [particle; NOP] {
//     let mut ret: [particle; NOP];
    
//     iter.enumerate().map(|(i, p)| {
//         ret[i] = p;
//         return ();
//     }).collect::Vec<()>();
    
//     return ret;
// }

fn combination_matrix<'a>(vector1: &'a Vec<particle>, vector2: &'a Vec<particle>) -> Vec<Vec<(&'a particle,&'a particle)>> {
    let comb_matrix:Vec<Vec<(_, _)>> = vector1.par_iter().map(|element_i| {
        vector2.par_iter().map(|element_j| {
            (element_i, element_j)
        }).collect()
    }).collect();
    return comb_matrix;
}

fn initialisation() -> Vec<particle> {
    let mut rng = rand::thread_rng();

    let mut particle_list: Vec<particle> = Vec::new();
    for i in 0..NOP {
        let x = rng.gen_range(-100.0..100.0);
        let y = rng.gen_range(-100.0..100.0);
        let rand_mass = rng.gen_range(0.0..5.0); 
        particle_list.push(
            particle{
                name:           f!("particle{i}"),
                mass:           rand_mass, 
                position:       vec![x, y],
                velocity:       vec![0.0, 0.0],
                // velocity:       vec![-y/100.0, x/100.0], 
                acceleration:   vec![0.0, 0.0],
                stationary:     false,
                draw_path:      true,
                color:          GREEN
            },
        )
    }

    // for i in 0..NOP {
    //     let x = rng.gen_range(-100.0..100.0);
    //     let y = rng.gen_range(-100.0..100.0);
    //     let rand_mass = rng.gen_range(-5.0..0.0); 
    //     particle_list.push(
    //         particle{
    //             name:           f!("particle{i}"),
    //             mass:           rand_mass, 
    //             position:       vec![x, y],
    //             velocity:       vec![0.0, 0.0],
    //             // velocity:       vec![-y/100.0, x/100.0], 
    //             acceleration:   vec![0.0, 0.0],
    //             stationary:     false,
    //             draw_path:      true,
    //             color:          YELLOW
    //         },
    //     )
    // }
    return particle_list;
}

fn update_particles(particle_list: Vec<particle>) -> Vec<particle> {
    let start: Instant = Instant::now();
    
    let grav_constant = 1.0;
    
    // 70ms
    let comb_matrix: Vec<Vec<(&particle, &particle)>> = combination_matrix(&particle_list,&particle_list);
    
    // 80ms
    let grav_factor_array: Vec<Vec<f32>> = comb_matrix.par_iter().map(|inner_vec| {
        return inner_vec.iter().map(|(particle_i, particle_j)| {
            let distance_3_2 = distance_between_vectors_squared(&particle_i.position, &particle_j.position).powf(3.0/2.0);
            if distance_3_2 < 10000.0 {
                return 0.0;
            };
            let grav_factor: f32 = (grav_constant * particle_i.mass * particle_j.mass) / (distance_3_2);
            return grav_factor;
        }).collect();
    }).collect();
    
    // 45ms
    let difference_vector_array0: Vec<Vec<f32>> = comb_matrix.par_iter().map(|inner_vec| {
        return inner_vec.iter().map(|(particle_i, particle_j)| {
            let difference = particle_j.position[0] - particle_i.position[0];
            return difference;
        }).collect();
    }).collect();
    
    // 45ms
    let difference_vector_array1: Vec<Vec<f32>> = comb_matrix.par_iter().map(|inner_vec| {
        return inner_vec.iter().map(|(particle_i, particle_j)| {
            let difference = particle_j.position[1] - particle_i.position[1];
            return difference;
        }).collect();
    }).collect();
    
    // 50ms
    let updated_particle_list = particle_list.into_par_iter().enumerate().map(|(index, mut particle_i)| {
        
        particle_i.velocity[0] = particle_i.velocity[0] + arr1(&grav_factor_array[index]).dot(&arr1(&difference_vector_array0[index]));
        particle_i.velocity[1] = particle_i.velocity[1] + arr1(&grav_factor_array[index]).dot(&arr1(&difference_vector_array1[index]));

        particle_i.position = add_vectors(&particle_i.position, &particle_i.velocity);
        
        return particle_i;
    }).collect();

    println!("Frame time: {:?}", start.elapsed());
    return updated_particle_list;
}

// TODO:
// center of mass point
// moving viewport 
// zooming in out 