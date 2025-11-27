use serde::{Deserialize, Serialize};

pub const APP_NAME: &str = "com.judethings/mouse-to-mordor";
pub const CONFIG_NAME: &str = "mouse-to-mordor-config";

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DistanceTraveled {
    pub total_pixels_traveled: f64,
    pub total_inches_traveled: f64,
    pub total_feet_traveled: f64,
    pub total_miles_traveled: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    #[serde(default)]
    pub warning: String,
    pub distance_traveled: DistanceTraveled,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            warning: "WARNING: DO NOT MODIFY THIS FILE. IT COULD HAVE UNINTENDED CONSEQUENCES."
                .to_string(),
            distance_traveled: Default::default(),
        }
    }
}

impl AppConfig {
    pub fn load() -> Self {
        confy::load(APP_NAME, CONFIG_NAME).unwrap_or_default()
    }
}
