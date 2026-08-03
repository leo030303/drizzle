use serde::Deserialize;

pub struct WeatherApi {
    is_metric: bool,
}

const OPEN_METEO_BASE_URL: &str = "https://api.open-meteo.com/v1/forecast";

#[derive(Debug, Deserialize)]
pub struct WeatherResponse {
    pub utc_offset_seconds: i64,

    #[serde(default)]
    pub current: Option<CurrentWeather>,

    #[serde(default)]
    pub hourly: Option<HourlyWeather>,

    #[serde(default)]
    pub daily: Option<DailyWeather>,
}

impl WeatherResponse {
    /// Applies the UTC offset directly to all timestamp fields in current, hourly, and daily forecasts.
    pub fn apply_utc_offset(&mut self) {
        let offset = self.utc_offset_seconds;

        if let Some(current) = self.current.as_mut() {
            current.time += offset;
        }
        if let Some(hourly) = self.hourly.as_mut() {
            for time in hourly.time.iter_mut() {
                *time += offset;
            }
        }

        if let Some(daily) = self.daily.as_mut() {
            for time in daily.time.iter_mut() {
                *time += offset;
            }
            for time in daily.sunrise.iter_mut() {
                *time += offset;
            }
            for time in daily.sunset.iter_mut() {
                *time += offset;
            }
        }
    }
}

/// WMO Weather interpretation codes (WW)
/// See https://open-meteo.com/en/docs#weather_variable_documentation
pub enum WeatherCode {
    ClearSky,
    MainlyClear,
    PartlyCloudy,
    Overcast,
    Fog,
    DepositingRimeFog,
    LightDrizzle,
    ModerateDrizzle,
    DenseDrizzle,
    LightFreezingDrizzle,
    DenseFreezingDrizzle,
    LightRain,
    ModerateRain,
    HeavyRain,
    LightFreezingRain,
    HeavyFreezingRain,
    LightSnowFall,
    ModerateSnowFall,
    HeavySnowFall,
    SnowGrains,
    LightRainShowers,
    ModerateRainShowers,
    ViolentRainShowers,
    LightSnowShowers,
    HeavySnowShowers,
    Thunderstorm,
    ThunderstormLightHail,
    ThunderstormHeavyHail,
}

#[derive(Debug)]
pub enum WeatherCodeError {
    NoMatch,
}

impl WeatherCode {
    pub fn get_icon_name(&self, is_day: bool) -> &'static str {
        match self {
            WeatherCode::ClearSky | WeatherCode::MainlyClear => {
                if is_day {
                    "clear-day"
                } else {
                    "clear-night"
                }
            }
            WeatherCode::PartlyCloudy => {
                if is_day {
                    "partly-cloudy-day"
                } else {
                    "partly-cloudy-night"
                }
            }
            WeatherCode::Overcast => "cloudy",
            WeatherCode::Fog | WeatherCode::DepositingRimeFog => "foggy",
            WeatherCode::LightRainShowers
            | WeatherCode::ModerateRainShowers
            | WeatherCode::ViolentRainShowers
            | WeatherCode::LightDrizzle
            | WeatherCode::ModerateDrizzle
            | WeatherCode::DenseDrizzle
            | WeatherCode::LightFreezingDrizzle
            | WeatherCode::DenseFreezingDrizzle
            | WeatherCode::LightRain
            | WeatherCode::ModerateRain
            | WeatherCode::HeavyRain
            | WeatherCode::LightFreezingRain
            | WeatherCode::HeavyFreezingRain => "rainy",
            WeatherCode::LightSnowShowers
            | WeatherCode::HeavySnowShowers
            | WeatherCode::LightSnowFall
            | WeatherCode::ModerateSnowFall
            | WeatherCode::HeavySnowFall
            | WeatherCode::SnowGrains => "snowing",
            WeatherCode::Thunderstorm
            | WeatherCode::ThunderstormLightHail
            | WeatherCode::ThunderstormHeavyHail => "thunderstorm",
        }
    }
    pub fn get_background_css_class(&self) -> &'static str {
        match self {
            WeatherCode::ClearSky | WeatherCode::MainlyClear => "bg-weather-clear-sky",
            WeatherCode::PartlyCloudy => "bg-weather-few-clouds",
            WeatherCode::Overcast => "bg-weather-overcast",
            WeatherCode::Fog | WeatherCode::DepositingRimeFog => "bg-weather-fog",
            WeatherCode::LightRainShowers
            | WeatherCode::LightDrizzle
            | WeatherCode::ModerateDrizzle
            | WeatherCode::LightFreezingDrizzle
            | WeatherCode::LightRain
            | WeatherCode::LightFreezingRain => "bg-weather-showers-scattered",
            WeatherCode::ModerateRain
            | WeatherCode::HeavyRain
            | WeatherCode::DenseFreezingDrizzle
            | WeatherCode::DenseDrizzle
            | WeatherCode::ModerateRainShowers
            | WeatherCode::ViolentRainShowers
            | WeatherCode::HeavyFreezingRain => "bg-weather-showers-large",
            WeatherCode::LightSnowShowers
            | WeatherCode::HeavySnowShowers
            | WeatherCode::LightSnowFall
            | WeatherCode::ModerateSnowFall
            | WeatherCode::HeavySnowFall
            | WeatherCode::SnowGrains => "bg-weather-snow",
            WeatherCode::Thunderstorm
            | WeatherCode::ThunderstormLightHail
            | WeatherCode::ThunderstormHeavyHail => "bg-weather-storm",
        }
    }
}

impl TryFrom<i64> for WeatherCode {
    type Error = WeatherCodeError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(WeatherCode::ClearSky),
            1 => Ok(WeatherCode::MainlyClear),
            2 => Ok(WeatherCode::PartlyCloudy),
            3 => Ok(WeatherCode::Overcast),
            45 => Ok(WeatherCode::Fog),
            48 => Ok(WeatherCode::DepositingRimeFog),
            51 => Ok(WeatherCode::LightDrizzle),
            53 => Ok(WeatherCode::ModerateDrizzle),
            55 => Ok(WeatherCode::DenseDrizzle),
            56 => Ok(WeatherCode::LightFreezingDrizzle),
            57 => Ok(WeatherCode::DenseFreezingDrizzle),
            61 => Ok(WeatherCode::LightRain),
            63 => Ok(WeatherCode::ModerateRain),
            65 => Ok(WeatherCode::HeavyRain),
            66 => Ok(WeatherCode::LightFreezingRain),
            67 => Ok(WeatherCode::HeavyFreezingRain),
            71 => Ok(WeatherCode::LightSnowFall),
            73 => Ok(WeatherCode::ModerateSnowFall),
            75 => Ok(WeatherCode::HeavySnowFall),
            77 => Ok(WeatherCode::SnowGrains),
            80 => Ok(WeatherCode::LightRainShowers),
            81 => Ok(WeatherCode::ModerateRainShowers),
            82 => Ok(WeatherCode::ViolentRainShowers),
            85 => Ok(WeatherCode::LightSnowShowers),
            86 => Ok(WeatherCode::HeavySnowShowers),
            95 => Ok(WeatherCode::Thunderstorm),
            96 => Ok(WeatherCode::ThunderstormLightHail),
            99 => Ok(WeatherCode::ThunderstormHeavyHail),
            _ => Err(WeatherCodeError::NoMatch),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CurrentWeather {
    pub time: i64,
    pub interval: i64,
    pub temperature_2m: f64,
    pub apparent_temperature: f64,
    pub is_day: i64,
    pub uv_index: f64,
    pub precipitation: f64,
    pub weathercode: i64,
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
    pub precipitation_probability_max: Vec<f64>,
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
    pub windspeed_10m: Vec<f64>,
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
    pub windspeed_10m: f64,
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
    pub precipitation_probability_max: f64,
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
                windspeed_10m: self.windspeed_10m[i],
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
                precipitation_probability_max: self.precipitation_probability_max[i],
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
                "&hourly={}&forecast_hours=48",
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

        weather_data.apply_utc_offset();
        Ok(weather_data)
    }
}
