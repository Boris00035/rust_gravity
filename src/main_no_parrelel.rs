use nannou::prelude::*;
use fstrings::*;
use rand::Rng;
use rayon::prelude::*;
// use core::time::Duration;

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


// chatgpt code
// impl IntoParallelIterator for Vec<particle> {
//     type Item = particle;
//     type Iter = rayon::slice::IterMut<'static, particle>;

//     fn into_par_iter(self) -> Self::Iter {
//         // Convert the Vec<particle> to a mutable slice
//         let slice = self.into_boxed_slice();
//         let slice_ref = unsafe { &mut *Box::into_raw(slice) };

//         // Create a mutable parallel iterator over the slice
//         slice_ref.par_iter_mut()
//     }
// }


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
    // let particle_list = 
    // vec![
    //     particle{
    //         name:       "test",
    //         mass:       20.0, 
    //         velocity:   vec![0.0, 0.3], 
    //         position:   vec![0.0, 100.0],
    //         stationary: false,
    //         draw_path:  true,
    //     },

    //     particle{
    //         name:       "test2",
    //         mass:       20.0, 
    //         velocity:   vec![0.0, -0.3], 
    //         position:   vec![50.0, 20.0],
    //         stationary: false,
    //         draw_path:  true,
    //     },

    //     particle{
    //         name:       "test3",
    //         mass:       5.0, 
    //         velocity:   vec![0.0, 0.6], 
    //         position:   vec![0.0, 50.0],
    //         stationary: false,
    //         draw_path:  true
    //     },

    //     // particle{
    //     //     name:       "test4",
    //     //     mass:       1.5, 
    //     //     velocity:   vec![0.0, 0.0], 
    //     //     position:   vec![100.3, 300.3],
    //     //     stationary: false,
    //     //     draw_path:  true
    //     // },

    // ];

    let mut particle_list: Vec<particle> = Vec::new();
    const NOP: i32 = 400;
    for i in 0..NOP {
        let x = rng.gen_range(-200.0..200.0);
        let y = rng.gen_range(-200.0..200.0);
        let rand_mass = rng.gen_range(1.0..5.0); 
        particle_list.push(
            particle{
                name:       f!("particle{i}"),
                mass:       rand_mass, 
                velocity:   vec![-y/90.0, x/90.0], 
                // velocity:   vec![0.0,0.0], 
                position:   vec![x, y],
                stationary: false,
                draw_path:  true
            },
        )
    }

    
    return particle_list;
}

fn update_particles(mut particle_list: Vec<particle>) -> Vec<particle> {
    let grav_constant = 1.0;
    let particle_list_copy = particle_list.clone(); 

    for particle_i in particle_list.iter_mut() {
        if particle_i.stationary == true {continue};
        
        let mut velocity_change_tot: Vec<f32> = vec![0.0 , 0.0];
        for particle_j in &particle_list_copy { 
            let distance_squaredij = distance_between_vectors_squared(particle_i.position.clone(), particle_j.position.clone());
            let distanceij = distance_squaredij.sqrt();
            
            if particle_i.name == particle_j.name {continue};
            if distanceij < 10.0 {continue};
            
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
        particle_i.velocity = add_vectors(particle_i.velocity.clone(), velocity_change_tot.clone());
        particle_i.position = add_vectors(particle_i.position.clone(), particle_i.velocity.clone());

    }

    return particle_list;
}


