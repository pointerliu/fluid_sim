mod particle;
mod fluid;

use bevy::prelude::*;
use clap::{arg, Parser};
use crate::particle::{init_particle_grid, init_particles_random};

#[derive(Parser, Resource, Debug)]
struct Args {
    #[arg(default_value_t = 800.0)]
    w_width: f32,
    #[arg(default_value_t = 800.0)]
    w_height: f32,

    #[arg(default_value = "random")]
    particle_gen: String,
    #[arg(default_value_t = 100)]
    particle_num: usize,
    #[arg(default_value_t = 10.0)]
    particle_radius: f32,
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args);

    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(args)
        .add_systems(Startup, greet_msg)
        .add_systems(Startup, (setup, init_particles_random).chain())
        .run();
}

fn greet_msg() {
    info!("hello fluid sim");
}

fn setup(
    mut commands: Commands,
    mut window: Query<&mut Window>,
    args: Res<Args>,
) {
    window.single_mut().resolution.set(args.w_width, args.w_height);
    // Camera
    commands.spawn(Camera2dBundle::default());
}
