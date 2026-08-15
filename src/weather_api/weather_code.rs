use serde::Deserialize;

/// WMO Weather interpretation codes (WW)
/// See https://open-meteo.com/en/docs#weather_variable_documentation
#[derive(Debug, Deserialize, Clone)]
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
            WeatherCode::NoMatch => "",
        }
    }

    pub fn get_status_image_resource(&self, is_day: bool) -> &'static str {
        match self {
            WeatherCode::ClearSky | WeatherCode::MainlyClear => {
                if is_day {
                    "/com/github/leo030303/drizzle/weather_status_icons/drizzle-clear.svg"
                } else {
                    "/com/github/leo030303/drizzle/weather_status_icons/drizzle-clear-night.svg"
                }
            }
            WeatherCode::PartlyCloudy => {
                if is_day {
                    "/com/github/leo030303/drizzle/weather_status_icons/drizzle-few-clouds.svg"
                } else {
                    "/com/github/leo030303/drizzle/weather_status_icons/drizzle-few-clouds-night.svg"
                }
            }
            WeatherCode::Overcast => {
                "/com/github/leo030303/drizzle/weather_status_icons/drizzle-overcast.svg"
            }
            WeatherCode::Fog | WeatherCode::DepositingRimeFog => {
                "/com/github/leo030303/drizzle/weather_status_icons/drizzle-fog.svg"
            }
            WeatherCode::LightRainShowers
            | WeatherCode::LightDrizzle
            | WeatherCode::ModerateDrizzle
            | WeatherCode::LightFreezingDrizzle
            | WeatherCode::LightRain
            | WeatherCode::LightFreezingRain => {
                "/com/github/leo030303/drizzle/weather_status_icons/drizzle-showers.svg"
            }
            WeatherCode::ModerateRain
            | WeatherCode::HeavyRain
            | WeatherCode::DenseFreezingDrizzle
            | WeatherCode::DenseDrizzle
            | WeatherCode::ModerateRainShowers
            | WeatherCode::ViolentRainShowers
            | WeatherCode::HeavyFreezingRain => {
                "/com/github/leo030303/drizzle/weather_status_icons/drizzle-showers-scattered.svg"
            }
            WeatherCode::LightSnowShowers
            | WeatherCode::HeavySnowShowers
            | WeatherCode::LightSnowFall
            | WeatherCode::ModerateSnowFall
            | WeatherCode::HeavySnowFall
            | WeatherCode::SnowGrains => {
                "/com/github/leo030303/drizzle/weather_status_icons/drizzle-snow.svg"
            }
            WeatherCode::Thunderstorm
            | WeatherCode::ThunderstormLightHail
            | WeatherCode::ThunderstormHeavyHail => {
                "/com/github/leo030303/drizzle/weather_status_icons/drizzle-storm.svg"
            }
            WeatherCode::NoMatch => "",
        }
    }

    pub fn get_background_css_class(&self, is_day: bool) -> &'static str {
        match self {
            WeatherCode::ClearSky | WeatherCode::MainlyClear => {
                if is_day {
                    "bg-weather-clear-sky"
                } else {
                    "bg-weather-clear-sky-night"
                }
            }
            WeatherCode::PartlyCloudy => {
                if is_day {
                    "bg-weather-few-clouds"
                } else {
                    "bg-weather-few-clouds-night"
                }
            }
            WeatherCode::Overcast => {
                if is_day {
                    "bg-weather-overcast"
                } else {
                    "bg-weather-overcast-night"
                }
            }
            WeatherCode::Fog | WeatherCode::DepositingRimeFog => {
                if is_day {
                    "bg-weather-fog"
                } else {
                    "bg-weather-fog-night"
                }
            }
            WeatherCode::LightRainShowers
            | WeatherCode::LightDrizzle
            | WeatherCode::ModerateDrizzle
            | WeatherCode::LightFreezingDrizzle
            | WeatherCode::LightRain
            | WeatherCode::LightFreezingRain => {
                if is_day {
                    "bg-weather-showers-scattered"
                } else {
                    "bg-weather-showers-scattered-night"
                }
            }
            WeatherCode::ModerateRain
            | WeatherCode::HeavyRain
            | WeatherCode::DenseFreezingDrizzle
            | WeatherCode::DenseDrizzle
            | WeatherCode::ModerateRainShowers
            | WeatherCode::ViolentRainShowers
            | WeatherCode::HeavyFreezingRain => {
                if is_day {
                    "bg-weather-showers-large"
                } else {
                    "bg-weather-showers-large-night"
                }
            }
            WeatherCode::LightSnowShowers
            | WeatherCode::HeavySnowShowers
            | WeatherCode::LightSnowFall
            | WeatherCode::ModerateSnowFall
            | WeatherCode::HeavySnowFall
            | WeatherCode::SnowGrains => {
                if is_day {
                    "bg-weather-snow"
                } else {
                    "bg-weather-snow-night"
                }
            }
            WeatherCode::Thunderstorm
            | WeatherCode::ThunderstormLightHail
            | WeatherCode::ThunderstormHeavyHail => {
                if is_day {
                    "bg-weather-storm"
                } else {
                    "bg-weather-storm-night"
                }
            }
            WeatherCode::NoMatch => "",
        }
    }
}

impl From<i64> for WeatherCode {
    fn from(value: i64) -> Self {
        match value {
            0 => WeatherCode::ClearSky,
            1 => WeatherCode::MainlyClear,
            2 => WeatherCode::PartlyCloudy,
            3 => WeatherCode::Overcast,
            45 => WeatherCode::Fog,
            48 => WeatherCode::DepositingRimeFog,
            51 => WeatherCode::LightDrizzle,
            53 => WeatherCode::ModerateDrizzle,
            55 => WeatherCode::DenseDrizzle,
            56 => WeatherCode::LightFreezingDrizzle,
            57 => WeatherCode::DenseFreezingDrizzle,
            61 => WeatherCode::LightRain,
            63 => WeatherCode::ModerateRain,
            65 => WeatherCode::HeavyRain,
            66 => WeatherCode::LightFreezingRain,
            67 => WeatherCode::HeavyFreezingRain,
            71 => WeatherCode::LightSnowFall,
            73 => WeatherCode::ModerateSnowFall,
            75 => WeatherCode::HeavySnowFall,
            77 => WeatherCode::SnowGrains,
            80 => WeatherCode::LightRainShowers,
            81 => WeatherCode::ModerateRainShowers,
            82 => WeatherCode::ViolentRainShowers,
            85 => WeatherCode::LightSnowShowers,
            86 => WeatherCode::HeavySnowShowers,
            95 => WeatherCode::Thunderstorm,
            96 => WeatherCode::ThunderstormLightHail,
            99 => WeatherCode::ThunderstormHeavyHail,
            _ => WeatherCode::NoMatch,
        }
    }
}
