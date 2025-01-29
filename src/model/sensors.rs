// pub struct TemperatureData {
//     pub id: i32,
//     pub timestamp: String,
//     pub temperature: f32,
// }

// pub struct HumidityData {
//     pub id: i32,
//     pub timestamp: String,
//     pub humidity: f32,
// }

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Deserialize, Serialize, Debug, FromRow)]
pub struct SensorsData {
    #[serde(default)]
    pub id: i32,
    pub device_id: String,
    pub temperature: f64,
    pub humidity: f64,
    pub timestamp: NaiveDateTime,
    pub weight: f64,
}
