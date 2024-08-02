use bevy::log::info;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use rand::Rng;
use crate::Args;
use crate::fluid::calculate_gradient;

const SMOOTH_RADIUS: f32 = 40.;
const SPEED: f32 = 100.;

#[derive(Component, Clone)]
pub struct Position(pub(crate) Vec2);

#[derive(Component)]
pub struct Particle;


pub fn init_particles_random(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    args: Res<Args>,
) {
    info!("initial particle generation randomly");
    let mut rng = rand::thread_rng();
    for _ in 0..args.particle_num {
        let x = rng.gen_range((-args.w_width / 2.0 + args.particle_radius)..(args.w_width / 2.0 - args.particle_radius));
        let y = rng.gen_range((-args.w_height / 2.0 + args.particle_radius)..(args.w_height / 2.0 - args.particle_radius));
        let pos = Vec2::new(x, y);
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(Circle::default()).into(),
                material: materials.add(Color::srgb(0.0, 0.0, 1.0)),
                transform: Transform::from_translation(pos.extend(1.0))
                    .with_scale(Vec2::splat(args.particle_radius * 2.).extend(1.)),
                ..default()
            },
            Position(pos),
            Particle
        ));
    }
}

pub fn init_particle_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    args: Res<Args>,
) {
    info!("initial particle generation in grid");
    let mut n = args.particle_num;
    let w = (n as f32).sqrt() + 1.;
    let bias = Vec2::new(-w / 2. * args.particle_radius * 2., w / 2. * args.particle_radius * 2.);
    for i in 0..w as usize {
        for j in 0..w as usize {
            let pos = args.particle_radius * 2.0 * (i as f32) * Vec2::X - args.particle_radius * 2.0 * (j as f32) * Vec2::Y + bias;
            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: meshes.add(Circle::default()).into(),
                    material: materials.add(Color::srgb(0.0, 0.0, 1.0)),
                    transform: Transform::from_translation(pos.extend(1.0))
                        .with_scale(Vec2::splat(args.particle_radius * 2.).extend(1.)),
                    ..default()
                },
                Position(pos),
                Particle
            ));
            n -= 1;
            if n == 0 { return; }
        }
    }
}

impl Particle {
    pub fn update_position(
        mut particle_query: Query<&mut Position, With<Particle>>,
    ) {
        let v: Vec<Position> = particle_query.into_iter().cloned().collect();
        for mut p1 in &mut particle_query {
            let t = p1.clone().0;
            p1.0 -= calculate_gradient(
                SMOOTH_RADIUS,
                t,
                &v,
            ) * 1. * SPEED;
            // info!("graident = {}", calculate_gradient(
            //     SMOOTH_RADIUS,
            //     t,
            //     &v
            // ));
            // p1.0 += -(Vec2::Y + Vec2::X) * time.delta_seconds() * 500.;
        }
    }
}
