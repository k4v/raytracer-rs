pub mod tests;
mod types;
mod utils;

use types::color;
use types::vec3::Vec3;
use utils::config::Config;

use crate::types::component::{Component, Sphere};
use crate::types::ray::Ray;

fn main() {
    // Initial configuration object
    let config: Config = Config::from_yaml("settings.yaml");

    eprintln!("Using config: {:?}", &config);

    // World origin
    let origin = config.origin().clone();

    let horizontal = Vec3::new(config.viewport_width() as f64, 0.0, 0.0);
    let vertical = Vec3::new(0.0, config.viewport_height() as f64, 0.0);
    let lower_left_corner =
        origin - Vec3::new(horizontal.x() / 2.0, vertical.y() / 2.0, config.focal_length());

    // Create scene objects
    let mut scene_objects: Vec<Box<dyn Component>> = vec![];
    scene_objects.push(Box::new(Sphere::new(&Vec3::new(0.0, 0.0, -1.5), 0.5)));
    scene_objects.push(Box::new(Sphere::new(&Vec3::new(0.75, 1.0, -2.0), 0.5)));

    println!("P3\n{},{}\n255", config.image_width(), config.image_height());

    eprintln!("{:?},{:?},{:?}", horizontal, vertical, lower_left_corner);

    for j in (0..config.image_height()).rev() {
        for i in 0..config.image_width() {
            let u = (i as f64) / (config.image_width() as f64);
            let v = (j as f64) / (config.image_height() as f64);

            let uv = lower_left_corner + horizontal.scaled(u) + vertical.scaled(v);
            let r = Ray::new(&origin, &(uv - origin));

            color::print_color(&r.ray_color(&scene_objects));
        }
        println!();
    }
}
