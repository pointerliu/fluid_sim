mod particle;
mod fluid;

use bevy::prelude::*;
use clap::{arg, Parser};
use crate::particle::{init_particle_grid, init_particles_random, Particle, Position};

#[derive(Parser, Resource, Debug)]
struct Args {
    #[arg(long, default_value_t = 400.0)]
    w_width: f32,
    #[arg(long, default_value_t = 400.0)]
    w_height: f32,

    #[arg(long, default_value = "random")]
    particle_gen: String,
    #[arg(long, default_value_t = 160)]
    particle_num: usize,
    #[arg(long, default_value_t = 5.0)]
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

fn check_boundary(
    window: Query<&Window>,
    mut ball_query: Query<&mut Position, With<Particle>>,
    cfg: Res<Args>,
) {
    for mut position in &mut ball_query {
        if position.0.x - cfg.particle_radius <= (-window.single().width() / 2.0) {
            position.0.x = cfg.particle_radius + (-window.single().width() / 2.0);
        }
        if position.0.x + cfg.particle_radius >= (window.single().width() / 2.0) {
            position.0.x = -cfg.particle_radius + (window.single().width() / 2.0);
        }
        if position.0.y - cfg.particle_radius <= (-window.single().height() / 2.0) {
            position.0.y = cfg.particle_radius + (-window.single().height() / 2.0);
        }
        if position.0.y + cfg.particle_radius >= (window.single().height() / 2.0) {
            position.0.y = -cfg.particle_radius + (window.single().height() / 2.0);
        }
    }
}

fn draw_particles(
    mut query: Query<(&mut Transform, &Position), With<Particle>>
) {
    for (mut trans, &ref position) in &mut query {
        trans.translation.x = position.0.x;
        trans.translation.y = position.0.y;
    }
}
