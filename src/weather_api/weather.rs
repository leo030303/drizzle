use serde::Deserialize;

use crate::{
    model::{
        daily_entry::DailyEntry, hourly_entry::HourlyEntry, uv_index::UvIndex,
        weather_code::WeatherCode,
    },
    weather_api::find_city::GeoResponse,
};

const OPEN_METEO_BASE_URL: &str = "https://api.open-meteo.com/v1/forecast";

#[derive(Debug, Deserialize)]
pub struct WeatherResponseCurrent {
    pub current: CurrentWeatherRaw,
}

#[derive(Debug, Deserialize)]
pub struct WeatherResponseHourly {
    pub utc_offset_seconds: i64,
    pub hourly: HourlyWeatherRaw,
}

#[derive(Debug, Deserialize)]
pub struct WeatherResponseDaily {
    pub utc_offset_seconds: i64,
    pub daily: DailyWeatherRaw,
}

#[derive(Debug)]
pub struct CurrentWeather {
    pub temperature_2m: Option<f64>,
    pub apparent_temperature: Option<f64>,
    pub is_day: bool,
    pub is_metric: bool,
    pub weathercode: WeatherCode,
}

#[derive(Debug, Deserialize)]
pub struct CurrentWeatherRaw {
    pub temperature_2m: Option<f64>,
    pub apparent_temperature: Option<f64>,
    pub is_day: Option<i64>,
    pub weathercode: Option<i64>,
}

impl CurrentWeatherRaw {
    pub fn process(&self, is_metric: bool) -> CurrentWeather {
        CurrentWeather {
            temperature_2m: self.temperature_2m,
            apparent_temperature: self.apparent_temperature,
            is_day: self.is_day.unwrap_or(1) == 1,
            is_metric,
            weathercode: WeatherCode::from(self.weathercode.unwrap_or(-1)),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct DailyWeatherRaw {
    pub time: Vec<Option<i64>>,
    pub weathercode: Vec<Option<i64>>,
    pub temperature_2m_max: Vec<Option<f64>>,
    pub temperature_2m_min: Vec<Option<f64>>,
    pub sunrise: Vec<Option<i64>>,
    pub sunset: Vec<Option<i64>>,
    pub uv_index_max: Vec<Option<f64>>,
    pub precipitation_sum: Vec<Option<f64>>,
    pub precipitation_probability_max: Vec<Option<f64>>,
    pub windspeed_10m_max: Vec<Option<f64>>,
}

#[derive(Debug, Deserialize)]
pub struct HourlyWeatherRaw {
    pub time: Vec<Option<i64>>,
    pub temperature_2m: Vec<Option<f64>>,
    pub apparent_temperature: Vec<Option<f64>>,
    pub weathercode: Vec<Option<i64>>,
    pub precipitation: Vec<Option<f64>>,
    pub precipitation_probability: Vec<Option<f64>>,
    pub windspeed_10m: Vec<Option<f64>>,
    pub uv_index: Vec<Option<f64>>,
    pub is_day: Vec<Option<i64>>,
}

impl HourlyWeatherRaw {
    pub fn to_entries(&self, utc_offset: i64, is_metric: bool) -> Vec<HourlyEntry> {
        self.time
            .iter()
            .enumerate()
            .filter_map(|(i, &time)| {
                Some(HourlyEntry {
                    time: time.map(|time| time.saturating_add(utc_offset))?,
                    temperature_2m: self.temperature_2m.get(i).and_then(|temp| *temp)?,
                    apparent_temperature: self
                        .apparent_temperature
                        .get(i)
                        .and_then(|apparent_temp| *apparent_temp)?,
                    weathercode: WeatherCode::from(self.weathercode.get(i)?.unwrap_or(-1)),
                    precipitation: self.precipitation.get(i).and_then(|prec| *prec)?,
                    precipitation_probability: self
                        .precipitation_probability
                        .get(i)
                        .and_then(|prec_prob| *prec_prob)?,
                    windspeed_10m: self.windspeed_10m.get(i).and_then(|wind| *wind)?,
                    uv_index: self
                        .uv_index
                        .get(i)
                        .map(|uv| UvIndex::from(uv.unwrap_or(-1.0)))?,
                    is_day: self.is_day.get(i)?.map(|is_day| is_day == 1)?,
                    is_metric,
                })
            })
            .collect()
    }
}

impl DailyWeatherRaw {
    pub fn to_entries(&self, utc_offset: i64, is_metric: bool) -> Vec<DailyEntry> {
        self.time
            .iter()
            .enumerate()
            .filter_map(|(i, &time)| {
                Some(DailyEntry {
                    time: time.map(|time| time.saturating_add(utc_offset))?,
                    weathercode: WeatherCode::from(self.weathercode.get(i)?.unwrap_or(-1)),
                    temperature_2m_max: self
                        .temperature_2m_max
                        .get(i)
                        .and_then(|temp_max| *temp_max)?,
                    temperature_2m_min: self
                        .temperature_2m_min
                        .get(i)
                        .and_then(|temp_min| *temp_min)?,
                    sunrise: self
                        .sunrise
                        .get(i)?
                        .map(|sunrise| sunrise.saturating_add(utc_offset))?,
                    sunset: self
                        .sunset
                        .get(i)?
                        .map(|sunset| sunset.saturating_add(utc_offset))?,
                    uv_index_max: self
                        .uv_index_max
                        .get(i)
                        .map(|uv| UvIndex::from(uv.unwrap_or(-1.0)))?,
                    precipitation_sum: self
                        .precipitation_sum
                        .get(i)
                        .and_then(|prec_sum| *prec_sum)?,
                    precipitation_probability_max: self
                        .precipitation_probability_max
                        .get(i)
                        .and_then(|prec_prob| *prec_prob)?,
                    windspeed_10m_max: self.windspeed_10m_max.get(i).and_then(|wind| *wind)?,
                    is_metric,
                })
            })
            .collect()
    }
}

pub async fn get_weather_current(
    city_details: &GeoResponse,
    is_metric: bool,
) -> Result<CurrentWeather, String> {
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
        weather_url
            .push_str("&temperature_unit=fahrenheit&wind_speed_unit=mph&precipitation_unit=inch");
    }
    let weather_data = reqwest::get(weather_url)
        .await
        .map_err(|e| e.to_string())?
        .json::<WeatherResponseCurrent>()
        .await
        .map_err(|e| e.to_string())?;

    Ok(weather_data.current.process(is_metric))
}

pub async fn get_weather_hourly(
    city_details: &GeoResponse,
    is_metric: bool,
) -> Result<Vec<HourlyEntry>, String> {
    const HOURLY_METRICS_LIST: [&str; 8] = [
        "temperature_2m",
        "apparent_temperature",
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
        weather_url
            .push_str("&temperature_unit=fahrenheit&wind_speed_unit=mph&precipitation_unit=inch");
    }
    let weather_data = reqwest::get(weather_url)
        .await
        .map_err(|e| e.to_string())?
        .json::<WeatherResponseHourly>()
        .await
        .map_err(|e| e.to_string())?;

    Ok(weather_data
        .hourly
        .to_entries(weather_data.utc_offset_seconds, is_metric))
}

pub async fn get_weather_daily(
    city_details: &GeoResponse,
    is_metric: bool,
) -> Result<Vec<DailyEntry>, String> {
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
        weather_url
            .push_str("&temperature_unit=fahrenheit&wind_speed_unit=mph&precipitation_unit=inch");
    }
    let weather_data = reqwest::get(weather_url)
        .await
        .map_err(|e| e.to_string())?
        .json::<WeatherResponseDaily>()
        .await
        .map_err(|e| e.to_string())?;

    Ok(weather_data
        .daily
        .to_entries(weather_data.utc_offset_seconds, is_metric))
}
