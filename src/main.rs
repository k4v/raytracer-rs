mod components;
pub mod tests;
mod types;
mod utils;

use types::color;
use types::vec3::Vec3;
use utils::config::Config;

use crate::components::{camera::Camera, sphere::Sphere, traceable::TraceableGroup};
use crate::types::color::Color;
use crate::utils::utilities::random_f64;

fn main() {
    // Initial configuration object
    let config: Config = Config::from_yaml("settings.yaml");

    eprintln!("Using config: {:?}", &config);

    let camera = Camera::configure(&config);

    // Create scene objects
    let scene_objects = TraceableGroup {
        objects: vec![
            Box::new(Sphere::new(&Vec3::new(0.0, 0.0, -1.0), 0.5).unwrap()),
            Box::new(Sphere::new(&Vec3::new(0.0, -100.5, -1.0), 100.0).unwrap()),
        ],
    };

    println!("P3\n{},{}\n255", config.image_width(), config.image_height());

    for j in (0..config.image_height()).rev() {
        for i in 0..config.image_width() {
            let mut pixel_color = Color::zero_vec();
            for _ in 0..config.samples_per_pixel() {
                let u = (i as f64 + random_f64()) / (config.image_width() as f64 - 1.0);
                let v = (j as f64 + random_f64()) / (config.image_height() as f64 - 1.0);

                let r = camera.get_ray(u, v);
                pixel_color += r.ray_color(&scene_objects);
            }

            color::print_color(&pixel_color, config.samples_per_pixel());
        }
        println!();
    }
}
