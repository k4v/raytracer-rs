mod utils;
mod types;

use types::vec3::Vec3;
use types::color;
use utils::config::Config;

use crate::types::ray::Ray;

fn main() {

    // Initial configuration object
    let config: Config = Config::from_yaml("settings.yaml");

    eprintln!("Using config: {:?}", &config);

    // World origin
    let origin = config.origin().clone();

    let horizontal = Vec3::from(config.viewport_width() as f64, 0.0, 0.0);
    let vertical = Vec3::from(0.0, config.viewport_height() as f64, 0.0);
    let lower_left_corner = origin
        - horizontal.scaled(0.5)
        - vertical.scaled(0.5)
        - Vec3::from(0.0, 0.0, config.focal_length());

    println!("P3\n{}, {}\n255", config.image_width(), config.image_height());

    for j in (0..config.image_height()).rev() {
        for i in 0..config.image_width() {

            let u = (i as f64) / ((config.image_width()-1) as f64);
            let v = (j as f64) / ((config.image_height()-1) as f64);

            let r = Ray::from(
                &origin,
                &(lower_left_corner + horizontal.scaled(u) + vertical.scaled(v) - origin)
            );

            color::print_color(&r.ray_color());
        }
    }
}
