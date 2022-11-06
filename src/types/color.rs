use crate::utils::utilities::clamp;

use super::vec3::Vec3;

pub type Color = Vec3;

pub fn print_color(color: &Color, samples_per_pixel: u64) {
    let c = color.scaled(1.0 / (samples_per_pixel as f64));

    print!(
        "{} {} {}\t",
        (256.0 * clamp(c.x(), 0.0, 0.999)) as usize,
        (256.0 * clamp(c.y(), 0.0, 0.999)) as usize,
        (256.0 * clamp(c.z(), 0.0, 0.999)) as usize
    )
}
