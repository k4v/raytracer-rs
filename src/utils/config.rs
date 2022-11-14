#![allow(dead_code)]

use serde::{Deserialize, Serialize};

use crate::types::vec3::Vec3;

#[derive(Debug, Serialize, Deserialize)]
pub struct MsaaConfig {
    samples_per_pixel: u64,
}

impl MsaaConfig {
    pub fn samples_per_pixel(&self) -> u64 {
        self.samples_per_pixel
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum AntialiasingMode {
    None,
    MSAA, // TODO: Figure out how to deserialize typed Enum variants in TOML
}

#[derive(Debug, Serialize, Deserialize)]
pub enum DiffuseScatterMode {
    ApproxLambert,
    TrueLambert,
    Hemispherical,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageConfig {
    image_width: u64,
    image_height: u64,
    gamma: Option<f64>,
    aa_mode: Option<AntialiasingMode>,
}

impl ImageConfig {
    pub fn image_width(&self) -> u64 {
        self.image_width
    }

    pub fn image_height(&self) -> u64 {
        self.image_height
    }

    pub fn gamma(&self) -> f64 {
        self.gamma.unwrap_or(1.0)
    }

    pub fn aa_mode(&self) -> &AntialiasingMode {
        self.aa_mode.as_ref().unwrap_or(&AntialiasingMode::None)
    }

    pub fn samples_per_pixel(&self) -> u64 {
        if let AntialiasingMode::MSAA = self.aa_mode() {
            return 50;
        } else {
            return 1;
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CameraConfig {
    viewport_width: u64,
    viewport_height: u64,
    origin: Option<Vec3>,
    focal_length: Option<f64>,
}

impl CameraConfig {
    pub fn viewport_width(&self) -> u64 {
        self.viewport_width
    }

    pub fn viewport_height(&self) -> u64 {
        self.viewport_height
    }

    pub fn origin(&self) -> &Vec3 {
        self.origin.as_ref().unwrap()
    }

    pub fn focal_length(&self) -> f64 {
        self.focal_length.unwrap()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RaysConfig {
    max_child_rays: Option<u64>,
    diffuse_scatter_mode: Option<DiffuseScatterMode>,
}

impl RaysConfig {
    pub fn max_child_rays(&self) -> u64 {
        self.max_child_rays.unwrap_or(50)
    }

    pub fn diffuse_scatter_mode(&self) -> &DiffuseScatterMode {
        self.diffuse_scatter_mode
            .as_ref()
            .unwrap_or(&DiffuseScatterMode::TrueLambert)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    image: ImageConfig,
    camera: CameraConfig,
    rays: RaysConfig,
}

impl Config {
    /********* Initializers *********/

    pub fn default() -> Self {
        DEFAULT_CONFIG_OBJECT
    }

    pub fn from_toml(config_toml_file: &str) -> Self {
        if let Ok(toml_content) = std::fs::read_to_string(config_toml_file) {
            let mut config_object =
                toml::from_str(toml_content.as_str()).unwrap_or_else(|toml_error| {
                    eprintln!(
                        "Unable to load config file: {}. Using default configuration",
                        toml_error.to_string()
                    );
                    DEFAULT_CONFIG_OBJECT
                });

            if config_object.image.aa_mode.is_none() {
                config_object.image.aa_mode = DEFAULT_CONFIG_OBJECT.image.aa_mode;
            }

            if config_object.image.gamma.is_none() {
                config_object.image.gamma = DEFAULT_CONFIG_OBJECT.image.gamma;
            }

            if config_object.camera.origin.is_none() {
                config_object.camera.origin = DEFAULT_CONFIG_OBJECT.camera.origin;
            }

            if config_object.camera.focal_length.is_none() {
                config_object.camera.focal_length =
                    DEFAULT_CONFIG_OBJECT.camera.focal_length;
            }

            if config_object.rays.max_child_rays.is_none() {
                config_object.rays.max_child_rays =
                    DEFAULT_CONFIG_OBJECT.rays.max_child_rays;
            }

            if config_object.rays.diffuse_scatter_mode.is_none() {
                config_object.rays.diffuse_scatter_mode =
                    DEFAULT_CONFIG_OBJECT.rays.diffuse_scatter_mode;
            }

            return config_object;
        }

        DEFAULT_CONFIG_OBJECT
    }

    /************ Getters ************/

    pub fn image_config(&self) -> &ImageConfig {
        &self.image
    }

    pub fn camera_config(&self) -> &CameraConfig {
        &self.camera
    }

    pub fn rays_config(&self) -> &RaysConfig {
        &self.rays
    }
}

const DEFAULT_CONFIG_OBJECT: Config = Config {
    image: ImageConfig {
        image_width: 256,
        image_height: 256,
        gamma: Some(1.0),
        aa_mode: Some(AntialiasingMode::None),
    },
    camera: CameraConfig {
        viewport_width: 2,
        viewport_height: 2,
        origin: Some(Vec3::new(0.0, 0.0, 0.0)),
        focal_length: Some(1.0),
    },
    rays: RaysConfig {
        max_child_rays: Some(50),
        diffuse_scatter_mode: Some(DiffuseScatterMode::TrueLambert),
    },
};
