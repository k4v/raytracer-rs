use super::vec3::Vec3;

pub type Color = Vec3;

pub fn print_color(c: &Color) {
    println!("{} {} {}", (255.999 * c.x()) as usize, (255.999 * c.y()) as usize, (255.999 * c.z()) as usize)
}

pub fn get_color(c: &Color) -> (usize, usize, usize) {
    ((255.999 * c.x()) as usize, (255.999 * c.y()) as usize, (255.999 * c.z()) as usize)
}