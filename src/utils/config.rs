#![allow(dead_code)]

extern crate serde_yaml;

use std::fs::File;
use serde::{Deserialize, Serialize};
use serde_yaml::from_reader;

use crate::types::vec3::Vec3;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    image_width: u64,
    image_height: u64,
    viewport_height: u64,
    focal_length: f64,
    origin: Vec3
}

impl Config {
    /********* Initializers *********/

    pub fn default() -> Self {
        DEFAULT_CONFIG_OBJECT
    }

    pub fn from_yaml(config_yaml: &str) -> Self {

        match File::open(config_yaml) {
            Ok(config_file) => {
                match from_reader::<std::fs::File, serde_yaml::Value>(config_file) {
                    Ok(config_object) => {

                        // If config yaml available and valid, build Config object from that file and return
                        Config {
                            image_width: config_object["image"]["image_width"].as_u64().unwrap_or(DEFAULT_CONFIG_OBJECT.image_width),
                            image_height: config_object["image"]["image_height"].as_u64().unwrap_or(DEFAULT_CONFIG_OBJECT.image_height),
                            viewport_height: config_object["camera"]["viewport_height"].as_u64().unwrap_or(DEFAULT_CONFIG_OBJECT.viewport_height),
                            focal_length: config_object["camera"]["focal_length"].as_f64().unwrap_or(DEFAULT_CONFIG_OBJECT.focal_length),
                            origin: Vec3::from(
                                config_object["camera"]["origin"][0].as_f64().unwrap_or(DEFAULT_CONFIG_OBJECT.origin.x()),
                                config_object["camera"]["origin"][1].as_f64().unwrap_or(DEFAULT_CONFIG_OBJECT.origin.y()),
                                config_object["camera"]["origin"][2].as_f64().unwrap_or(DEFAULT_CONFIG_OBJECT.origin.z())
                            )
                        }
                    }
                    Err(_) => {
                        eprintln!("Error loading settings from file");
                        return DEFAULT_CONFIG_OBJECT;
                    }
                }
            },
            Err(_) => {
                eprintln!("Error opening settings file");
                return DEFAULT_CONFIG_OBJECT;
            }
        }
    }

    /************ Getters ************/

    pub fn image_width(&self) -> u64 {
        self.image_width
    }

    pub fn image_height(&self) -> u64 {
        self.image_height
    }

    pub fn aspect_ratio(&self) -> f64 {
        (self.image_width as f64) / (self.image_height as f64)
    }

    pub fn viewport_height(&self) -> u64 {
        self.viewport_height
    }

    pub fn viewport_width(&self) -> u64 {
        (self.viewport_height as f64 * self.aspect_ratio()) as u64
    }

    pub fn focal_length(&self) -> f64 {
        self.focal_length
    }

    pub fn origin(&self) -> &Vec3 {
        &self.origin
    }

}

const DEFAULT_CONFIG_OBJECT:Config = Config {
    image_width: 256,
    image_height: 256,
    viewport_height: 2,
    focal_length: 1.0,
    origin: Vec3::from(0.0, 0.0, 0.0)
};
