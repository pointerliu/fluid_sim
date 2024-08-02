mod particle;
mod fluid;

use bevy::prelude::*;
use clap::{arg, Parser};
use crate::particle::{init_particle_grid, init_particles_random, Particle, Position};

#[derive(Parser, Resource, Debug)]
struct Args {
    #[arg(default_value_t = 400.0)]
    w_width: f32,
    #[arg(default_value_t = 400.0)]
    w_height: f32,

    #[arg(default_value = "random")]
    particle_gen: String,
    #[arg(default_value_t = 160)]
    particle_num: usize,
    #[arg(default_value_t = 5.0)]
    particle_radius: f32,
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args);

    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, greet_msg)
        .add_systems(
            Startup,
            (
                setup,
                if args.particle_gen == "random" { init_particles_random } else { init_particle_grid }
            ).chain(),
        )
        .add_systems(FixedUpdate, (Particle::update_position, check_boundary, draw_particles).chain())
        .insert_resource(args)
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
