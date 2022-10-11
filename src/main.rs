mod utils;
mod types;

use types::vec3::Vec3;
use types::color;
use utils::config::Config;

use crate::types::color::get_color;
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
    let lower_left_corner = origin
        - horizontal.scaled(0.5)
        - vertical.scaled(0.5)
        - Vec3::new(0.0, 0.0, config.focal_length());

    // Create scene objects
    let mut scene_objects: Vec<Box<dyn Component>> = vec![];
    /*scene_objects.push(Box::new(Sphere::new(&Vec3::new(-1.0, 0.0, -1.0), 0.5)));
    scene_objects.push(Box::new(Sphere::new(&Vec3::new(0.75, 1.0, -1.0), 0.5)));*/

    println!("P3\n{},{}\n255", config.image_width(), config.image_height());

    eprintln!("{:?}", Vec3::new(1.0, 2.0, 3.0).scale(0.5));
    eprintln!("{:?}", Vec3::new(1.0, 2.0, 3.0).scaled(0.5));
    eprintln!("{:?},{:?},{:?}", horizontal, vertical, lower_left_corner);

    for j in (0..config.image_height()).rev() {
        for i in 0..config.image_width() {

            let u = (i as f64) / ((config.image_width()-1) as f64);
            let v = (j as f64) / ((config.image_height()-1) as f64);

            let r = Ray::new(
                &origin,
                &(lower_left_corner + horizontal.scaled(u) + vertical.scaled(v) - origin)
            );

            let pixel_color = &r.ray_color(&scene_objects);
            if pixel_color.x() > 1.0 || pixel_color.y() > 1.0 || pixel_color.z() > 1.0 {
                eprintln!("Bad pixel: i:{}, u:{}, j:{}, v:{}, ray:{:?}, color:{:?}, print_color:{:?}",
                    i, u, j, v, r, pixel_color, get_color(pixel_color));
                    return;
            } else {
                color::print_color(pixel_color);
            }
        }
    }
}
