mod particle;

use bevy::prelude::*;
use clap::{arg, Parser};

#[derive(Parser, Resource, Debug)]
struct Args {
    #[arg(default_value = "800.0")]
    w_width: Option<f32>,
    #[arg(default_value = "800.0")]
    w_height: Option<f32>,
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args);

    App::new()
        .insert_resource(args)
        .add_plugins(DefaultPlugins)
        // .add_plugins(bevy::log::LogPlugin::default()) // enable logger
        .add_systems(Startup, greet_msg)
        .add_systems(Startup, setup)
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
    window.single_mut().resolution.set(args.w_width.unwrap(), args.w_height.unwrap());
    // Camera
    commands.spawn(Camera2dBundle::default());
}
