use bevy::log::info;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::utils::info;
use rand::{Rng, random};
use crate::Args;

#[derive(Component)]
struct Position(Vec2);

#[derive(Component)]
struct Particle;


pub fn init_particles_random(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    args: Res<Args>,
    window: Query<&Window>,
) {
    info!("initial particle generation randomly");
    let win = &window.single().resolution;
    let mut rng = rand::thread_rng();
    for _ in 0..args.particle_num {
        let x = rng.gen_range((-win.width() / 2.0 + args.particle_radius)..(win.width() / 2.0 - args.particle_radius));
        let y = rng.gen_range((-win.height() / 2.0 + args.particle_radius)..(win.height() / 2.0 - args.particle_radius));
        let pos = Vec2::new(x, y);
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(Circle::default()).into(),
                material: materials.add(Color::srgb(0.0, 0.0, 1.0)),
                transform: Transform::from_translation(pos.extend(1.0))
                    .with_scale(Vec2::splat(args.particle_radius).extend(1.)),
                ..default()
            },
            Position(pos),
            Particle
        ));
    }
}

pub fn init_particle_grid() {
    info!("initial particle generation in grid");
    todo!()
}
