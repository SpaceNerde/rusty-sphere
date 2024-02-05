use bevy::prelude::*;
use rand::prelude::*;
use rusty_sphere::GamePlugin;

fn not_to_random_int() -> f32{
    let mut rng = thread_rng();
    let val: i32 = rng.gen_range(-10..10);
    val as f32
}

fn generate_matrix(size: i32) -> Vec<Vec<f32>> {
    let mut matrix = vec![vec![0.; size as usize]; size as usize];

    matrix[0][0] = not_to_random_int();
    matrix[0][(size - 1) as usize] = not_to_random_int();
    matrix[(size - 1) as usize][0] = not_to_random_int();
    matrix[(size - 1) as usize][(size - 1) as usize] = not_to_random_int();

    matrix
}

fn diamond_step(mut matrix: Vec<Vec<f32>>, steps: i32, size: i32) {
    let mut i = 0;
    let mut j = 0;

    while i < size  {
        while j < size {
            println!("({:?})", matrix[(steps / (2 * i)) as usize][(steps / (2 * j)) as usize]);
            j += steps;
        }
        j = 0;



        i += steps;
    }
}

fn main() {
    /*let mut app = App::new();

    app.insert_resource(Msaa::Off)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "World Generator".to_string(),
                canvas: Some("#bevy".to_owned()),
                fit_canvas_to_parent: true,
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(GamePlugin);
    app.run();
     */

    let size = 5;

    let matrix = generate_matrix(size);

    let mut i:usize = 0;

    while i < size as usize {
        println!("{:?}", matrix[i]);
        i += 1;
    }

    diamond_step(matrix, 4, size);
}

