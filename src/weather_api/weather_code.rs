use serde::Deserialize;

/// WMO Weather interpretation codes (WW)
/// See <https://open-meteo.com/en/docs#weather_variable_documentation>
#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
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
    pub const fn get_icon_name(&self, is_day: bool) -> &'static str {
        match self {
            Self::ClearSky | Self::MainlyClear => {
                if is_day {
                    "clear-day"
                } else {
                    "clear-night"
                }
            }
            Self::PartlyCloudy => {
                if is_day {
                    "partly-cloudy-day"
                } else {
                    "partly-cloudy-night"
                }
            }
            Self::Overcast => "cloudy",
            Self::Fog | Self::DepositingRimeFog => "foggy",
            Self::LightRainShowers
            | Self::ModerateRainShowers
            | Self::ViolentRainShowers
            | Self::LightDrizzle
            | Self::ModerateDrizzle
            | Self::DenseDrizzle
            | Self::LightFreezingDrizzle
            | Self::DenseFreezingDrizzle
            | Self::LightRain
            | Self::ModerateRain
            | Self::HeavyRain
            | Self::LightFreezingRain
            | Self::HeavyFreezingRain => "rainy",
            Self::LightSnowShowers
            | Self::HeavySnowShowers
            | Self::LightSnowFall
            | Self::ModerateSnowFall
            | Self::HeavySnowFall
            | Self::SnowGrains => "snowing",
            Self::Thunderstorm | Self::ThunderstormLightHail | Self::ThunderstormHeavyHail => {
                "thunderstorm"
            }
            Self::NoMatch => "",
        }
    }

    pub const fn get_status_image_resource(&self, is_day: bool) -> &'static str {
        match self {
            Self::ClearSky | Self::MainlyClear => {
                if is_day {
                    "/com/github/leo030303/drizzle/weather_status_icons/drizzle-clear.svg"
                } else {
                    "/com/github/leo030303/drizzle/weather_status_icons/drizzle-clear-night.svg"
                }
            }
            Self::PartlyCloudy => {
                if is_day {
                    "/com/github/leo030303/drizzle/weather_status_icons/drizzle-few-clouds.svg"
                } else {
                    "/com/github/leo030303/drizzle/weather_status_icons/drizzle-few-clouds-night.svg"
                }
            }
            Self::Overcast => {
                "/com/github/leo030303/drizzle/weather_status_icons/drizzle-overcast.svg"
            }
            Self::Fog | Self::DepositingRimeFog => {
                "/com/github/leo030303/drizzle/weather_status_icons/drizzle-fog.svg"
            }
            Self::LightRainShowers
            | Self::LightDrizzle
            | Self::ModerateDrizzle
            | Self::LightFreezingDrizzle
            | Self::LightRain
            | Self::LightFreezingRain => {
                "/com/github/leo030303/drizzle/weather_status_icons/drizzle-showers.svg"
            }
            Self::ModerateRain
            | Self::HeavyRain
            | Self::DenseFreezingDrizzle
            | Self::DenseDrizzle
            | Self::ModerateRainShowers
            | Self::ViolentRainShowers
            | Self::HeavyFreezingRain => {
                "/com/github/leo030303/drizzle/weather_status_icons/drizzle-showers-scattered.svg"
            }
            Self::LightSnowShowers
            | Self::HeavySnowShowers
            | Self::LightSnowFall
            | Self::ModerateSnowFall
            | Self::HeavySnowFall
            | Self::SnowGrains => {
                "/com/github/leo030303/drizzle/weather_status_icons/drizzle-snow.svg"
            }
            Self::Thunderstorm | Self::ThunderstormLightHail | Self::ThunderstormHeavyHail => {
                "/com/github/leo030303/drizzle/weather_status_icons/drizzle-storm.svg"
            }
            Self::NoMatch => "",
        }
    }

    pub const fn get_background_css_class(&self, is_day: bool) -> &'static str {
        match self {
            Self::ClearSky | Self::MainlyClear => {
                if is_day {
                    "bg-weather-clear-sky"
                } else {
                    "bg-weather-clear-sky-night"
                }
            }
            Self::PartlyCloudy => {
                if is_day {
                    "bg-weather-few-clouds"
                } else {
                    "bg-weather-few-clouds-night"
                }
            }
            Self::Overcast => {
                if is_day {
                    "bg-weather-overcast"
                } else {
                    "bg-weather-overcast-night"
                }
            }
            Self::Fog | Self::DepositingRimeFog => {
                if is_day {
                    "bg-weather-fog"
                } else {
                    "bg-weather-fog-night"
                }
            }
            Self::LightRainShowers
            | Self::LightDrizzle
            | Self::ModerateDrizzle
            | Self::LightFreezingDrizzle
            | Self::LightRain
            | Self::LightFreezingRain => {
                if is_day {
                    "bg-weather-showers-scattered"
                } else {
                    "bg-weather-showers-scattered-night"
                }
            }
            Self::ModerateRain
            | Self::HeavyRain
            | Self::DenseFreezingDrizzle
            | Self::DenseDrizzle
            | Self::ModerateRainShowers
            | Self::ViolentRainShowers
            | Self::HeavyFreezingRain => {
                if is_day {
                    "bg-weather-showers-large"
                } else {
                    "bg-weather-showers-large-night"
                }
            }
            Self::LightSnowShowers
            | Self::HeavySnowShowers
            | Self::LightSnowFall
            | Self::ModerateSnowFall
            | Self::HeavySnowFall
            | Self::SnowGrains => {
                if is_day {
                    "bg-weather-snow"
                } else {
                    "bg-weather-snow-night"
                }
            }
            Self::Thunderstorm | Self::ThunderstormLightHail | Self::ThunderstormHeavyHail => {
                if is_day {
                    "bg-weather-storm"
                } else {
                    "bg-weather-storm-night"
                }
            }
            Self::NoMatch => "",
        }
    }

    pub fn is_rain(&self) -> bool {
        [
            Self::LightDrizzle,
            Self::ModerateDrizzle,
            Self::DenseDrizzle,
            Self::LightFreezingDrizzle,
            Self::DenseFreezingDrizzle,
            Self::LightRain,
            Self::ModerateRain,
            Self::HeavyRain,
            Self::LightFreezingRain,
            Self::HeavyFreezingRain,
            Self::LightRainShowers,
            Self::ModerateRainShowers,
            Self::ViolentRainShowers,
            Self::Thunderstorm,
        ]
        .contains(self)
    }

    pub fn is_snow(&self) -> bool {
        [
            Self::LightSnowFall,
            Self::ModerateSnowFall,
            Self::HeavySnowFall,
            Self::LightSnowShowers,
            Self::HeavySnowShowers,
            Self::SnowGrains,
        ]
        .contains(self)
    }

    pub fn is_fog(&self) -> bool {
        [Self::Fog, Self::DepositingRimeFog].contains(self)
    }

    pub fn is_storm(&self) -> bool {
        [
            Self::Thunderstorm,
            Self::ThunderstormLightHail,
            Self::ThunderstormHeavyHail,
        ]
        .contains(self)
    }
}

impl From<i64> for WeatherCode {
    fn from(value: i64) -> Self {
        match value {
            0 => Self::ClearSky,
            1 => Self::MainlyClear,
            2 => Self::PartlyCloudy,
            3 => Self::Overcast,
            45 => Self::Fog,
            48 => Self::DepositingRimeFog,
            51 => Self::LightDrizzle,
            53 => Self::ModerateDrizzle,
            55 => Self::DenseDrizzle,
            56 => Self::LightFreezingDrizzle,
            57 => Self::DenseFreezingDrizzle,
            61 => Self::LightRain,
            63 => Self::ModerateRain,
            65 => Self::HeavyRain,
            66 => Self::LightFreezingRain,
            67 => Self::HeavyFreezingRain,
            71 => Self::LightSnowFall,
            73 => Self::ModerateSnowFall,
            75 => Self::HeavySnowFall,
            77 => Self::SnowGrains,
            80 => Self::LightRainShowers,
            81 => Self::ModerateRainShowers,
            82 => Self::ViolentRainShowers,
            85 => Self::LightSnowShowers,
            86 => Self::HeavySnowShowers,
            95 => Self::Thunderstorm,
            96 => Self::ThunderstormLightHail,
            99 => Self::ThunderstormHeavyHail,
            _ => Self::NoMatch,
        }
    }
}
