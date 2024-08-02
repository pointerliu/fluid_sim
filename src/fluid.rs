use std::f32::consts::PI;
use bevy::math::Vec2;
use crate::particle::Position;

const MASS: f32 = 1000.;

fn smooth_kernel(radius: f32, dst: f32) -> f32 {
    let volume = PI * f32::powi(radius, 8) / 4.;
    let value = f32::max(0.0, radius * radius - dst * dst);
    return value * value * value / volume;
}

fn calculate_density<T>(smooth_radius: f32, sample_pos: Vec2, positions: &T) -> f32
    where T: IntoIterator<Item=Position> + Clone
{
    let mut density = 0.;
    for position in positions.clone() {
        let dst = (position.0 - sample_pos).length();
        let influence = smooth_kernel(smooth_radius, dst);
        density += influence * MASS;
    }
    return density;
}

pub fn calculate_gradient<T>(
    smooth_radius: f32,
    sample_pos: Vec2,
    positions: &T
) -> Vec2
    where T: IntoIterator<Item=Position> + Clone
{
    let step_size = 10.;
    let dx = calculate_density(smooth_radius, sample_pos + Vec2::X * step_size, positions) - calculate_density(smooth_radius, sample_pos, positions);
    let dy = calculate_density(smooth_radius, sample_pos + Vec2::Y * step_size, positions) - calculate_density(smooth_radius, sample_pos, positions);
    let gradient = Vec2::new(dx, dy) / step_size;
    return gradient;
}
