use nannou::prelude::*;
use fstrings::*;
use rand::Rng;
use rayon::prelude::*;
// use core::time::Duration;
use std::{time::{Instant}, usize};
use ndarray::prelude::*;
use itertools::iproduct;
// use itertools::Itertools;
use core::ops::Range;

const NOP: usize = 10;

fn main() { 
    nannou::app(model).update(update).run();
}

struct Model {
    _window:        window::Id,
    particle_list:  Vec<particle>,
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();
    // app.set_loop_mode(LoopMode::Rate { update_interval: (Duration::new(5000,0)) });
    let particle_list = initialisation_particle_list();
    // let comb_matrix: Array2<(usize, usize)> = initialisation_comb_matrix(NOP);

    return Model { _window, particle_list };
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    update_particles(&mut _model.particle_list);
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(WHITE);

    for particle_i in _model.particle_list.clone().iter_mut() {
        draw
            .ellipse()
            .color(particle_i.color)
            .x_y(particle_i.position[0],particle_i.position[1])
            .radius(particle_i.mass);
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


fn initialisation_particle_list() -> Vec<particle> {
    let mut rng = rand::thread_rng();

    let mut particle_list: Vec<particle> = Vec::new();

    // hier ook ff een mooi parallel mapje van maken

    for i in 0..NOP {
        let x = rng.gen_range(-100.0..100.0);
        let y = rng.gen_range(-100.0..100.0);
        let rand_mass = 5.0; 
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
    //     let rand_mass = rng.gen_range(0.0..5.0); 
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
    //             color:          GREEN
    //         },
    //     )
    // }

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


fn get_column_symmetric_square_matrix(matrix: &Vec<f32>, size: usize, column_index: usize, parity: f32) -> Vec<f32> {
    let array_iterator = 0..(NOP);
    // println!("{:?}", &matrix);
    let target_array: Vec<f32> = array_iterator.map(|i| {
        let matrix_coordinate = (i.div_euclid(NOP), i.rem_euclid(NOP));

        if matrix_coordinate.0 < matrix_coordinate.1 {
            return matrix[matrix_coordinate.0 * size + matrix_coordinate.1];
        }
        else {
            return parity * matrix[matrix_coordinate.1 * size + column_index];
        }
    }).collect();
    // println!("{:?}, {:?}", &target_array, &column_index);
    return target_array;
}

fn dot_product(vector1: &Vec<f32>, vector2: &Vec<f32>) -> f32 {
    let inproduct: f32 = (0..vector1.len()).map(|i| {
        return vector1[i] * vector2[i];
    }).collect::<Vec<f32>>().into_iter().sum();

    return inproduct;
}


fn update_particles(particle_list: &mut Vec<particle>) {
    let start: Instant = Instant::now();
    let grav_constant = 1.0;
    let matrix_like_iterator = 0..(NOP).pow(2);
    let matrix_like_iterator2 = 0..(NOP).pow(2);
    let matrix_like_iterator3 = 0..(NOP).pow(2);

    let grav_factor_array: Vec<f32> = matrix_like_iterator.map(|i| {
        
        let matrix_coordinate = (i.div_euclid(NOP), i % NOP);
        let distance_3_2 = distance_between_vectors_squared(&particle_list[matrix_coordinate.0].position, &particle_list[matrix_coordinate.1].position).powf(3.0/2.0);

        if distance_3_2 < 10000.0 {
            return 0.0;
        };

        if matrix_coordinate.0 < matrix_coordinate.1 {
            return (grav_constant * particle_list[matrix_coordinate.0].mass * particle_list[matrix_coordinate.1].mass) / (distance_3_2);
        }
        else {
            return 0.0;
        }

    }).collect();

    let difference_vector_array0: Vec<f32> = matrix_like_iterator2.map(|i| {
        let matrix_coordinate = (i.div_euclid(NOP), i % NOP);

        if matrix_coordinate.0 < matrix_coordinate.1 {
            return particle_list[matrix_coordinate.0].position[0] - particle_list[matrix_coordinate.1].position[0];
        }
        else {
            return 0.0;
        }

    }).collect();
    
    let difference_vector_array1: Vec<f32> = matrix_like_iterator3.map(|i| {
        let matrix_coordinate = (i.div_euclid(NOP), i % NOP);

        if matrix_coordinate.0 < matrix_coordinate.1 {
            return particle_list[matrix_coordinate.0].position[1] - particle_list[matrix_coordinate.1].position[1];
        }
        else {
            return 0.0
        }
        
    }).collect();
    
    particle_list.par_iter_mut().enumerate().for_each(|(i, particle_i)| {
        
        particle_i.velocity[0] = particle_i.velocity[0] + dot_product(&get_column_symmetric_square_matrix(&grav_factor_array, NOP, i, 1.0), &get_column_symmetric_square_matrix(&difference_vector_array0, NOP, i, -1.0));
        particle_i.velocity[1] = particle_i.velocity[1] + dot_product(&get_column_symmetric_square_matrix(&grav_factor_array, NOP, i, 1.0), &get_column_symmetric_square_matrix(&difference_vector_array1, NOP, i, -1.0));

        // particle_i.velocity[0] = particle_i.velocity[0] + &grav_factor_array.index_axis(Axis(0), i).dot(&difference_vector_array0.index_axis(Axis(0), i));
        // particle_i.velocity[1] = particle_i.velocity[1] + &grav_factor_array.index_axis(Axis(0), i).dot(&difference_vector_array1.index_axis(Axis(0), i));

        particle_i.position = add_vectors(&particle_i.position, &particle_i.velocity);
    });

    println!("Frame time: {:?}", start.elapsed());
}




// TODO:
// center of mass point
// moving viewport 
// zooming in out 