use serde::Deserialize;
use std::collections::HashMap;

pub struct WeatherApi {
    is_metric: bool,
}

const OPEN_METEO_BASE_URL: &str = "https://api.open-meteo.com/v1/forecast";
// --- Unit Conversion ---
const HPA_TO_INHG: f64 = 0.02953;

#[derive(Debug, Deserialize)]
pub struct WeatherResponse {
    pub latitude: f64,
    pub longitude: f64,
    pub elevation: f64,
    pub generationtime_ms: f64,
    pub utc_offset_seconds: i64,
    pub timezone: String,
    pub timezone_abbreviation: String,

    #[serde(default)]
    pub current: Option<CurrentWeather>,
    #[serde(default)]
    pub current_units: Option<HashMap<String, String>>,

    #[serde(default)]
    pub hourly: Option<HourlyWeather>,
    #[serde(default)]
    pub hourly_units: Option<HashMap<String, String>>,

    #[serde(default)]
    pub daily: Option<DailyWeather>,
    #[serde(default)]
    pub daily_units: Option<HashMap<String, String>>,
}

#[derive(Debug, Deserialize)]
pub struct CurrentWeather {
    pub time: i64,
    pub interval: i64,
    pub temperature_2m: f64,
    pub relativehumidity_2m: f64,
    pub apparent_temperature: f64,
    pub is_day: i64,
    pub uv_index: f64,
    pub precipitation: f64,
    pub weathercode: i64,
    pub surface_pressure: f64,
    pub windspeed_10m: f64,
    pub winddirection_10m: f64,
}

#[derive(Debug, Deserialize)]
pub struct DailyWeather {
    pub time: Vec<i64>,
    pub weathercode: Vec<i64>,
    pub temperature_2m_max: Vec<f64>,
    pub temperature_2m_min: Vec<f64>,
    pub sunrise: Vec<i64>,
    pub sunset: Vec<i64>,
    pub uv_index_max: Vec<f64>,
    pub precipitation_sum: Vec<f64>,
    pub windspeed_10m_max: Vec<f64>,
}

#[derive(Debug, Deserialize)]
pub struct HourlyWeather {
    pub time: Vec<i64>,
    pub temperature_2m: Vec<f64>,
    pub apparent_temperature: Vec<f64>,
    pub weathercode: Vec<i64>,
    pub precipitation: Vec<f64>,
    pub precipitation_probability: Vec<f64>,
    pub visibility: Vec<f64>,
    pub windspeed_10m: Vec<f64>,
    pub wind_direction_10m: Vec<f64>,
    pub uv_index: Vec<f64>,
    pub is_day: Vec<i64>,
}

#[derive(Debug, Clone)]
pub struct HourlyEntry {
    pub time: i64,
    pub temperature_2m: f64,
    pub apparent_temperature: f64,
    pub weathercode: i64,
    pub precipitation: f64,
    pub precipitation_probability: f64,
    pub visibility: f64,
    pub windspeed_10m: f64,
    pub wind_direction_10m: f64,
    pub uv_index: f64,
    pub is_day: i64,
}

#[derive(Debug, Clone)]
pub struct DayEntry {
    pub time: i64,
    pub weathercode: i64,
    pub temperature_2m_max: f64,
    pub temperature_2m_min: f64,
    pub sunrise: i64,
    pub sunset: i64,
    pub uv_index_max: f64,
    pub precipitation_sum: f64,
    pub windspeed_10m_max: f64,
}

impl HourlyWeather {
    pub fn to_entries(&self) -> Vec<HourlyEntry> {
        self.time
            .iter()
            .enumerate()
            .map(|(i, &time)| HourlyEntry {
                time,
                temperature_2m: self.temperature_2m[i],
                apparent_temperature: self.apparent_temperature[i],
                weathercode: self.weathercode[i],
                precipitation: self.precipitation[i],
                precipitation_probability: self.precipitation_probability[i],
                visibility: self.visibility[i],
                windspeed_10m: self.windspeed_10m[i],
                wind_direction_10m: self.wind_direction_10m[i],
                uv_index: self.uv_index[i],
                is_day: self.is_day[i],
            })
            .collect()
    }
}

impl DailyWeather {
    pub fn to_entries(&self) -> Vec<DayEntry> {
        self.time
            .iter()
            .enumerate()
            .map(|(i, &time)| DayEntry {
                time,
                weathercode: self.weathercode[i],
                temperature_2m_max: self.temperature_2m_max[i],
                temperature_2m_min: self.temperature_2m_min[i],
                sunrise: self.sunrise[i],
                sunset: self.sunset[i],
                uv_index_max: self.uv_index_max[i],
                precipitation_sum: self.precipitation_sum[i],
                windspeed_10m_max: self.windspeed_10m_max[i],
            })
            .collect()
    }
}

pub struct CityCoordinates {
    pub latitude: f64,
    pub longitude: f64,
}

pub enum ForecastTimeframe {
    Current,
    Hourly,
    Daily,
}

const CURRENT_METRICS_LIST: [&str; 8] = [
    "temperature_2m",
    "apparent_temperature",
    "is_day",
    "uv_index",
    "precipitation",
    "weathercode",
    "windspeed_10m",
    "winddirection_10m",
];

const DAILY_METRICS_LIST: [&str; 8] = [
    "weathercode",
    "temperature_2m_max",
    "temperature_2m_min",
    "sunrise",
    "sunset",
    "uv_index_max",
    "precipitation_sum",
    "windspeed_10m_max",
];

const HOURLY_METRICS_LIST: [&str; 10] = [
    "temperature_2m",
    "apparent_temperature",
    "weathercode",
    "precipitation",
    "precipitation_probability",
    "visibility",
    "windspeed_10m",
    "wind_direction_10m",
    "uv_index",
    "is_day",
];

impl WeatherApi {
    pub fn init(is_metric: bool) -> Self {
        Self { is_metric }
    }

    pub async fn get_weather(
        &self,
        city_coordinates: CityCoordinates,
        forecast_timeframe: ForecastTimeframe,
    ) -> Result<WeatherResponse, Box<dyn std::error::Error>> {
        let mut weather_url = format!(
            "{}?timeformat=unixtime&timezone=auto&latitude={}&longitude={}",
            OPEN_METEO_BASE_URL, city_coordinates.latitude, city_coordinates.longitude,
        );
        match forecast_timeframe {
            ForecastTimeframe::Current => {
                weather_url.push_str(&format!("&current={}", CURRENT_METRICS_LIST.join(",")))
            }
            ForecastTimeframe::Hourly => weather_url.push_str(&format!(
                "&hourly={}&forecast_days=2",
                HOURLY_METRICS_LIST.join(",")
            )),
            ForecastTimeframe::Daily => weather_url.push_str(&format!(
                "&daily={}&forecast_days=14",
                DAILY_METRICS_LIST.join(",")
            )),
        }
        if !self.is_metric {
            weather_url.push_str("&temperature_unit=fahrenheit&wind_speed_unit=mph");
        }
        let mut weather_data = reqwest::get(weather_url)
            .await?
            .json::<WeatherResponse>()
            .await?;

        if !self.is_metric {
            if let Some(current) = weather_data.current.as_mut() {
                current.surface_pressure *= HPA_TO_INHG;
            }
            if let Some(current_units) = weather_data.current_units.as_mut() {
                current_units.insert("surface_pressure".to_string(), "inHg".to_string());
            }
        }
        Ok(weather_data)
    }
}
