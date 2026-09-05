use crate::model::{uv_index::UvIndex, weather_code::WeatherCode};

#[derive(Debug, Clone)]
pub struct HourlyEntry {
    pub time: i64,
    pub temperature_2m: f64,
    pub apparent_temperature: f64,
    pub weathercode: WeatherCode,
    pub precipitation: f64,
    pub precipitation_probability: f64,
    pub windspeed_10m: f64,
    pub uv_index: UvIndex,
    pub is_day: bool,
    pub is_metric: bool,
}
