use crate::model::{uv_index::UvIndex, weather_code::WeatherCode};

#[derive(Debug, Clone)]
pub struct DailyEntry {
    pub time: i64,
    pub weathercode: WeatherCode,
    pub temperature_2m_max: f64,
    pub temperature_2m_min: f64,
    pub sunrise: i64,
    pub sunset: i64,
    pub uv_index_max: UvIndex,
    pub precipitation_sum: f64,
    pub precipitation_probability_max: f64,
    pub windspeed_10m_max: f64,
    pub is_metric: bool,
}
