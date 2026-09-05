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
    pub temperature_2m: f64,
    pub apparent_temperature: f64,
    pub is_day: bool,
    pub is_metric: bool,
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
    pub fn process(&self, is_metric: bool) -> CurrentWeather {
        CurrentWeather {
            temperature_2m: self.temperature_2m,
            apparent_temperature: self.apparent_temperature,
            is_day: self.is_day == 1,
            is_metric,
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
    pub apparent_temperature: Vec<f64>,
    pub weathercode: Vec<i64>,
    pub precipitation: Vec<f64>,
    pub precipitation_probability: Vec<f64>,
    pub windspeed_10m: Vec<f64>,
    pub uv_index: Vec<f64>,
    pub is_day: Vec<i64>,
}

impl HourlyWeatherRaw {
    pub fn to_entries(&self, utc_offset: i64, is_metric: bool) -> Vec<HourlyEntry> {
        self.time
            .iter()
            .enumerate()
            .filter_map(|(i, &time)| {
                Some(HourlyEntry {
                    time: time.saturating_add(utc_offset),
                    temperature_2m: if let Some(temp) = self.temperature_2m.get(i) {
                        *temp
                    } else {
                        return None;
                    },
                    apparent_temperature: if let Some(apparent_temp) =
                        self.apparent_temperature.get(i)
                    {
                        *apparent_temp
                    } else {
                        return None;
                    },
                    weathercode: if let Some(code) = self.weathercode.get(i) {
                        WeatherCode::from(*code)
                    } else {
                        return None;
                    },
                    precipitation: if let Some(prec) = self.precipitation.get(i) {
                        *prec
                    } else {
                        return None;
                    },
                    precipitation_probability: if let Some(prec_prob) =
                        self.precipitation_probability.get(i)
                    {
                        *prec_prob
                    } else {
                        return None;
                    },
                    windspeed_10m: if let Some(wind) = self.windspeed_10m.get(i) {
                        *wind
                    } else {
                        return None;
                    },
                    uv_index: if let Some(uv) = self.uv_index.get(i) {
                        UvIndex::from(*uv)
                    } else {
                        return None;
                    },
                    is_day: if let Some(day) = self.is_day.get(i) {
                        *day == 1
                    } else {
                        return None;
                    },
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
                    time: time.saturating_add(utc_offset),
                    weathercode: if let Some(code) = self.weathercode.get(i) {
                        WeatherCode::from(*code)
                    } else {
                        return None;
                    },
                    temperature_2m_max: if let Some(temp_max) = self.temperature_2m_max.get(i) {
                        *temp_max
                    } else {
                        return None;
                    },
                    temperature_2m_min: if let Some(temp_min) = self.temperature_2m_min.get(i) {
                        *temp_min
                    } else {
                        return None;
                    },
                    sunrise: if let Some(sunrise) = self.sunrise.get(i) {
                        sunrise.saturating_add(utc_offset)
                    } else {
                        return None;
                    },
                    sunset: if let Some(sunset) = self.sunset.get(i) {
                        sunset.saturating_add(utc_offset)
                    } else {
                        return None;
                    },
                    uv_index_max: if let Some(uv) = self.uv_index_max.get(i) {
                        UvIndex::from(*uv)
                    } else {
                        return None;
                    },
                    precipitation_sum: if let Some(prec_sum) = self.precipitation_sum.get(i) {
                        *prec_sum
                    } else {
                        return None;
                    },
                    precipitation_probability_max: if let Some(prec_prob) =
                        self.precipitation_probability_max.get(i)
                    {
                        *prec_prob
                    } else {
                        return None;
                    },
                    windspeed_10m_max: if let Some(wind) = self.windspeed_10m_max.get(i) {
                        *wind
                    } else {
                        return None;
                    },
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
