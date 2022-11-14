mod components;
pub mod tests;
mod types;
mod utils;

use types::color;
use types::vec3::Vec3;
use utils::config::Config;

use crate::components::{camera::Camera, sphere::Sphere, traceable::TraceableGroup};
use crate::types::color::Color;
use crate::types::diffuse_mat::DiffuseMaterial;
use crate::types::metal_mat::MetalMaterial;
use crate::utils::config::AntialiasingMode;
use crate::utils::utilities::random_f64;

fn main() {
    // Initial configuration object
    let config: Config = Config::from_toml("settings.toml");

    eprintln!("Using config: {:?}", &config);

    let camera = Camera::configure(&config);

    // Create scene objects
    let ground_material = Box::new(DiffuseMaterial::new(&Color::new(0.8, 0.8, 0.0)));
    let center_material = Box::new(DiffuseMaterial::new(&Color::new(0.7, 0.3, 0.3)));
    let left_material = Box::new(MetalMaterial::new(&Color::new(0.8, 0.8, 0.8), 0.5));
    let right_material = Box::new(MetalMaterial::new(&Color::new(0.8, 0.6, 0.2), 1.0));

    let scene_objects = TraceableGroup {
        objects: vec![
            Box::new(Sphere::new(&Vec3::new(0.0, -100.5, -1.0), 100.0, ground_material).unwrap()),
            Box::new(Sphere::new(&Vec3::new(0.0, 0.0, -1.0), 0.5, center_material).unwrap()),
            Box::new(Sphere::new(&Vec3::new(-1.5, 0.0, -1.0), 0.5, left_material).unwrap()),
            Box::new(Sphere::new(&Vec3::new(1.5, 0.0, -1.0), 0.5, right_material).unwrap()),
        ],
    };

    println!(
        "P3\n{},{}\n255",
        config.image_config().image_width(),
        config.image_config().image_height()
    );

    for j in (0..config.image_config().image_height()).rev() {
        for i in 0..config.image_config().image_width() {
            let mut pixel_color = Color::zero_vec();
            let aa_multiplier = if *config.image_config().aa_mode() == AntialiasingMode::MSAA {
                1.0
            } else {
                0.0
            };
            for _ in 0..config.image_config().samples_per_pixel() {
                let u = (i as f64 + (aa_multiplier * random_f64()))
                    / (config.image_config().image_width() as f64 - 1.0);
                let v = (j as f64 + (aa_multiplier * random_f64()))
                    / (config.image_config().image_height() as f64 - 1.0);

                let r = camera.get_ray(u, v);
                pixel_color += r.ray_color(&scene_objects, &config);
            }

            color::print_color(
                &pixel_color,
                config.image_config().samples_per_pixel(),
                config.image_config().gamma(),
            );
        }
        println!();
    }
}
