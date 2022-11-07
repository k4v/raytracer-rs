#![allow(dead_code)]

use serde::Deserialize;

use crate::types::vec3::Vec3;

#[derive(Debug, Deserialize)]
pub struct Image {
    image_width: u64,
    image_height: u64,
    samples_per_pixel: u64,
}

#[derive(Debug, Deserialize)]
pub struct Camera {
    viewport_height: u64,
    origin: Vec3,
    focal_length: f64,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    image: Image,
    camera: Camera,
}

impl Config {
    /********* Initializers *********/

    pub fn default() -> Self {
        DEFAULT_CONFIG_OBJECT
    }

    pub fn from_yaml(config_yaml_file: &str) -> Self {
        if let Ok(toml_content) = std::fs::read_to_string(config_yaml_file) {
            return toml::from_str(toml_content.as_str()).unwrap_or_else(|toml_error| {
                eprintln!("Unable to load config file: {}", toml_error.to_string());
                return DEFAULT_CONFIG_OBJECT;
            });
        }

        DEFAULT_CONFIG_OBJECT
    }

    /************ Getters ************/

    pub fn image_width(&self) -> u64 {
        self.image.image_width
    }

    pub fn image_height(&self) -> u64 {
        self.image.image_height
    }

    pub fn aspect_ratio(&self) -> f64 {
        (self.image.image_width as f64) / (self.image.image_height as f64)
    }

    pub fn samples_per_pixel(&self) -> u64 {
        self.image.samples_per_pixel
    }

    pub fn viewport_height(&self) -> u64 {
        self.camera.viewport_height
    }

    pub fn viewport_width(&self) -> u64 {
        (self.camera.viewport_height as f64 * self.aspect_ratio()) as u64
    }

    pub fn focal_length(&self) -> f64 {
        self.camera.focal_length
    }

    pub fn origin(&self) -> &Vec3 {
        &self.camera.origin
    }
}

const DEFAULT_CONFIG_OBJECT: Config = Config {
    image: Image {
        image_width: 256,
        image_height: 256,
        samples_per_pixel: 50,
    },
    camera: Camera {
        viewport_height: 2,
        origin: Vec3::new(0.0, 0.0, 0.0),
        focal_length: 1.0,
    },
};
