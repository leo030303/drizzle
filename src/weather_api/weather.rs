use serde::Deserialize;

use crate::weather_api::{find_city::GeoResponse, weather_code::WeatherCode};

const OPEN_METEO_BASE_URL: &str = "https://api.open-meteo.com/v1/forecast";

#[derive(Debug, Deserialize)]
pub struct WeatherResponse {
    pub utc_offset_seconds: i64,

    #[serde(default)]
    pub current: Option<CurrentWeatherRaw>,

    #[serde(default)]
    pub hourly: Option<HourlyWeatherRaw>,

    #[serde(default)]
    pub daily: Option<DailyWeatherRaw>,
}

#[derive(Debug)]
pub struct CurrentWeather {
    pub temperature_2m: f64,
    pub apparent_temperature: f64,
    pub is_day: bool,
    pub weathercode: WeatherCode,
}

#[derive(Debug, Deserialize)]
pub struct CurrentWeatherRaw {
    pub temperature_2m: f64,
    pub apparent_temperature: f64,
    pub is_day: i64,
    pub weathercode: i64,
}

impl CurrentWeatherRaw {
    pub fn process(&self) -> CurrentWeather {
        CurrentWeather {
            temperature_2m: self.temperature_2m,
            apparent_temperature: self.apparent_temperature,
            is_day: self.is_day == 1,
            weathercode: WeatherCode::from(self.weathercode),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct DailyWeatherRaw {
    pub time: Vec<i64>,
    pub weathercode: Vec<i64>,
    pub temperature_2m_max: Vec<f64>,
    pub temperature_2m_min: Vec<f64>,
    pub sunrise: Vec<i64>,
    pub sunset: Vec<i64>,
    pub uv_index_max: Vec<f64>,
    pub precipitation_sum: Vec<f64>,
    pub precipitation_probability_max: Vec<f64>,
    pub windspeed_10m_max: Vec<f64>,
}

#[derive(Debug, Deserialize)]
pub struct HourlyWeatherRaw {
    pub time: Vec<i64>,
    pub temperature_2m: Vec<f64>,
    pub weathercode: Vec<i64>,
    pub precipitation: Vec<f64>,
    pub precipitation_probability: Vec<f64>,
    pub windspeed_10m: Vec<f64>,
    pub uv_index: Vec<f64>,
    pub is_day: Vec<i64>,
}

#[derive(Debug, Clone)]
pub struct HourlyEntry {
    pub time: i64,
    pub temperature_2m: f64,
    pub weathercode: WeatherCode,
    pub precipitation: f64,
    pub precipitation_probability: f64,
    pub windspeed_10m: f64,
    pub uv_index: f64,
    pub is_day: bool,
}

#[derive(Debug, Clone)]
pub struct DailyEntry {
    pub time: i64,
    pub weathercode: WeatherCode,
    pub temperature_2m_max: f64,
    pub temperature_2m_min: f64,
    pub sunrise: i64,
    pub sunset: i64,
    pub uv_index_max: f64,
    pub precipitation_sum: f64,
    pub precipitation_probability_max: f64,
    pub windspeed_10m_max: f64,
}

impl HourlyWeatherRaw {
    pub fn to_entries(&self, utc_offset: i64) -> Vec<HourlyEntry> {
        self.time
            .iter()
            .enumerate()
            .map(|(i, &time)| HourlyEntry {
                time: time + utc_offset,
                temperature_2m: self.temperature_2m[i],
                weathercode: WeatherCode::from(self.weathercode[i]),
                precipitation: self.precipitation[i],
                precipitation_probability: self.precipitation_probability[i],
                windspeed_10m: self.windspeed_10m[i],
                uv_index: self.uv_index[i],
                is_day: self.is_day[i] == 1,
            })
            .collect()
    }
}

impl DailyWeatherRaw {
    pub fn to_entries(&self, utc_offset: i64) -> Vec<DailyEntry> {
        self.time
            .iter()
            .enumerate()
            .map(|(i, &time)| DailyEntry {
                time: time + utc_offset,
                weathercode: WeatherCode::from(self.weathercode[i]),
                temperature_2m_max: self.temperature_2m_max[i],
                temperature_2m_min: self.temperature_2m_min[i],
                sunrise: self.sunrise[i] + utc_offset,
                sunset: self.sunset[i] + utc_offset,
                uv_index_max: self.uv_index_max[i],
                precipitation_sum: self.precipitation_sum[i],
                precipitation_probability_max: self.precipitation_probability_max[i],
                windspeed_10m_max: self.windspeed_10m_max[i],
            })
            .collect()
    }
}

pub async fn get_weather_current(
    city_details: &GeoResponse,
    is_metric: bool,
) -> Result<CurrentWeather, Box<dyn std::error::Error>> {
    const CURRENT_METRICS_LIST: [&str; 4] = [
        "temperature_2m",
        "is_day",
        "apparent_temperature",
        "weathercode",
    ];
    let mut weather_url = format!(
        "{}?timeformat=unixtime&timezone=auto&latitude={}&longitude={}&current={}",
        OPEN_METEO_BASE_URL,
        city_details.latitude,
        city_details.longitude,
        CURRENT_METRICS_LIST.join(",")
    );
    if !is_metric {
        weather_url.push_str("&temperature_unit=fahrenheit&wind_speed_unit=mph");
    }
    let weather_data = reqwest::get(weather_url)
        .await?
        .json::<WeatherResponse>()
        .await?;

    Ok(weather_data.current.unwrap().process())
}

pub async fn get_weather_hourly(
    city_details: &GeoResponse,
    is_metric: bool,
) -> Result<Vec<HourlyEntry>, Box<dyn std::error::Error>> {
    const HOURLY_METRICS_LIST: [&str; 7] = [
        "temperature_2m",
        "weathercode",
        "precipitation",
        "precipitation_probability",
        "windspeed_10m",
        "uv_index",
        "is_day",
    ];
    let mut weather_url = format!(
        "{}?timeformat=unixtime&timezone=auto&latitude={}&longitude={}&hourly={}&forecast_hours=48",
        OPEN_METEO_BASE_URL,
        city_details.latitude,
        city_details.longitude,
        HOURLY_METRICS_LIST.join(",")
    );
    if !is_metric {
        weather_url.push_str("&temperature_unit=fahrenheit&wind_speed_unit=mph");
    }
    let weather_data = reqwest::get(weather_url)
        .await?
        .json::<WeatherResponse>()
        .await?;

    Ok(weather_data
        .hourly
        .unwrap()
        .to_entries(weather_data.utc_offset_seconds))
}

pub async fn get_weather_daily(
    city_details: &GeoResponse,
    is_metric: bool,
) -> Result<Vec<DailyEntry>, Box<dyn std::error::Error>> {
    const DAILY_METRICS_LIST: [&str; 9] = [
        "weathercode",
        "temperature_2m_max",
        "temperature_2m_min",
        "sunrise",
        "sunset",
        "uv_index_max",
        "precipitation_sum",
        "precipitation_probability_max",
        "windspeed_10m_max",
    ];
    let mut weather_url = format!(
        "{}?timeformat=unixtime&timezone=auto&latitude={}&longitude={}&daily={}&forecast_days=14",
        OPEN_METEO_BASE_URL,
        city_details.latitude,
        city_details.longitude,
        DAILY_METRICS_LIST.join(",")
    );
    if !is_metric {
        weather_url.push_str("&temperature_unit=fahrenheit&wind_speed_unit=mph");
    }
    let weather_data = reqwest::get(weather_url)
        .await?
        .json::<WeatherResponse>()
        .await?;

    Ok(weather_data
        .daily
        .unwrap()
        .to_entries(weather_data.utc_offset_seconds))
}
