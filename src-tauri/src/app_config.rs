use serde::{Deserialize, Serialize};

use crate::constants::{
    BREE_TO_RIVENDELL, LOTHLORIEN_TO_PARTH_GALEN, MINAS_MORGUL_TO_MOUNT_DOOM,
    PARTH_GALEN_TO_THE_BLACK_GATES, RIVENDELL_TO_LOTHLORIEN, THE_BLACK_GATES_TO_MINAS_MORGUL,
    THE_SHIRE_TO_BREE, TOTAL_WALKING_DISTANCE,
};

pub const APP_NAME: &str = "com.judethings/mouse-to-mordor";
pub const CONFIG_NAME: &str = "mouse-to-mordor-config";

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DistanceTraveled {
    pub total_pixels_traveled: f64,
    pub total_inches_traveled: f64,
    pub total_feet_traveled: f64,
    pub total_miles_traveled: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Landmarks {
    pub the_shire_to_bree: f64,
    pub bree_to_rivendell: f64,
    pub rivendell_to_lothlorien: f64,
    pub lothlorien_to_parth_galen: f64,
    pub parth_galen_to_the_black_gates: f64,
    pub the_black_gates_to_minas_morgul: f64,
    pub minas_morgul_to_mount_doom: f64,
    pub total_walking_distance: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Progress {
    pub distance_traveled: DistanceTraveled,
    pub landmarks: Vec<(String, f64)>,
}

impl Default for Progress {
    fn default() -> Self {
        Self {
            distance_traveled: Default::default(),
            landmarks: vec![
                ("THE_SHIRE_TO_BREE".to_string(), THE_SHIRE_TO_BREE),
                ("BREE_TO_RIVENDELL".to_string(), BREE_TO_RIVENDELL),
                (
                    "RIVENDELL_TO_LOTHLORIEN".to_string(),
                    RIVENDELL_TO_LOTHLORIEN,
                ),
                (
                    "LOTHLORIEN_TO_PARTH_GALEN".to_string(),
                    LOTHLORIEN_TO_PARTH_GALEN,
                ),
                (
                    "PARTH_GALEN_TO_THE_BLACK_GATES".to_string(),
                    PARTH_GALEN_TO_THE_BLACK_GATES,
                ),
                (
                    "THE_BLACK_GATES_TO_MINAS_MORGUL".to_string(),
                    THE_BLACK_GATES_TO_MINAS_MORGUL,
                ),
                (
                    "MINAS_MORGUL_TO_MOUNT_DOOM".to_string(),
                    MINAS_MORGUL_TO_MOUNT_DOOM,
                ),
                ("TOTAL_WALKING_DISTANCE".to_string(), TOTAL_WALKING_DISTANCE),
            ],
        }
    }
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
