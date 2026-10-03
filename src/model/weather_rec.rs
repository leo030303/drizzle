use chrono::DurationRound;
use gettextrs::gettext;

use crate::model::{daily_entry::DailyEntry, hourly_entry::HourlyEntry, uv_index::UvIndex};

#[derive(Debug)]
pub struct TimedRecommendation {
    pub recommendation: WeatherRecommendation,
    pub start_time: i64,
    pub end_time: i64,
}

#[derive(Debug, PartialEq, Eq)]
pub enum WeatherRecommendation {
    LowUvRisk,
    HighUvRisk,
    ExpectRainStrongWinds,
    ExpectRainLightWinds,
    StrongWinds,
    ExpectStorm,
    ExpectSnow,
    ExpectFog,
    WearJumper,
    WearShorts,
    Freezing,
    Sunset(String),
    Sunrise(String),
}

impl TimedRecommendation {
    pub fn get_text(&self) -> String {
        let start_time = chrono::DateTime::from_timestamp_secs(self.start_time).map_or_else(
            || gettext("Invalid Timestamp"),
            |time| time.format("%l%P").to_string(),
        );
        let end_time = chrono::DateTime::from_timestamp_secs(self.end_time).map_or_else(
            || gettext("Invalid Timestamp"),
            |time| time.format("%l%P").to_string(),
        );
        match &self.recommendation {
            WeatherRecommendation::LowUvRisk => {
                format!("{}: {start_time} - {end_time}", gettext("Wear suncream"))
            }
            WeatherRecommendation::HighUvRisk => {
                format!(
                    "{}: {start_time} - {end_time}",
                    gettext("Avoid direct sunlight")
                )
            }
            WeatherRecommendation::ExpectRainStrongWinds => {
                format!(
                    "{}: {start_time} - {end_time}",
                    gettext("Expect rain and strong winds")
                )
            }
            WeatherRecommendation::ExpectRainLightWinds => {
                format!(
                    "{}: {start_time} - {end_time}",
                    gettext("Expect rain and light winds")
                )
            }
            WeatherRecommendation::StrongWinds => {
                format!(
                    "{}: {start_time} - {end_time}",
                    gettext("Expect strong winds")
                )
            }
            WeatherRecommendation::ExpectStorm => {
                format!("{}: {start_time} - {end_time}", gettext("Expect a storm"))
            }
            WeatherRecommendation::ExpectSnow => {
                format!("{}: {start_time} - {end_time}", gettext("Expect snow"))
            }
            WeatherRecommendation::ExpectFog => {
                format!("{}: {start_time} - {end_time}", gettext("Expect fog"))
            }
            WeatherRecommendation::WearJumper => {
                format!("{}: {start_time} - {end_time}", gettext("Jumper weather"))
            }
            WeatherRecommendation::WearShorts => {
                format!("{}: {start_time} - {end_time}", gettext("Shorts weather"))
            }
            WeatherRecommendation::Freezing => {
                format!("{}: {start_time} - {end_time}", gettext("Freezing"))
            }
            WeatherRecommendation::Sunset(time) => format!("{} {time}", gettext("Sunset at")),
            WeatherRecommendation::Sunrise(time) => format!("{} {time}", gettext("Sunrise at")),
        }
    }
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug)]
pub enum RecommendationTimespan {
    FourHour,
    EightHour,
    TwelveHour,
    TwentyFourHour,
}

impl RecommendationTimespan {
    pub const fn to_name(&self) -> &'static str {
        match self {
            Self::FourHour => "FourHour",
            Self::EightHour => "EightHour",
            Self::TwelveHour => "TwelveHour",
            Self::TwentyFourHour => "TwentyFourHour",
        }
    }
    pub const fn to_int(&self) -> i64 {
        match self {
            Self::FourHour => 4,
            Self::EightHour => 8,
            Self::TwelveHour => 12,
            Self::TwentyFourHour => 24,
        }
    }
    pub fn from_name(name: &str) -> Self {
        match name {
            "FourHour" => Self::FourHour,
            "EightHour" => Self::EightHour,
            "TwelveHour" => Self::TwelveHour,
            _ => Self::TwentyFourHour,
        }
    }
}

// Corresponds to beaufort scale 6 Strong Breeze, too much wind for an umbrella
const WINDS_UMBRELLA_THRESHOLD_METRIC: f64 = 39.0;
const WINDS_UMBRELLA_THRESHOLD_IMPERIAL: f64 = 24.0;
// Corresponds to beaufort scale 7 Near Gale
const HIGH_WINDS_THRESHOLD_METRIC: f64 = 50.0;
const HIGH_WINDS_THRESHOLD_IMPERIAL: f64 = 31.0;
// Warm enough for shorts/summer clothes
const SHORTS_TEMP_THRESHOLD_METRIC: f64 = 20.0;
const SHORTS_TEMP_THRESHOLD_IMPERIAL: f64 = 68.0;
// Cold enough to need a jumper/jacket
const JUMPER_TEMP_THRESHOLD_METRIC: f64 = 12.0;
const JUMPER_TEMP_THRESHOLD_IMPERIAL: f64 = 54.0;
const FREEZING_TEMP_THRESHOLD_METRIC: f64 = 0.0;
const FREEZING_TEMP_THRESHOLD_IMPERIAL: f64 = 32.0;

pub fn get_recommendations(
    weather_conditions: &[HourlyEntry],
    todays_entry: &DailyEntry,
    tomorrows_entry: &DailyEntry,
    timespan: &RecommendationTimespan,
) -> Vec<TimedRecommendation> {
    let mut recommendations_list_with_times: Vec<(WeatherRecommendation, i64, i64)> = vec![];

    // Get general weather recs
    let relevant_conditions = match timespan {
        RecommendationTimespan::FourHour => weather_conditions
            .split_at_checked(4)
            .map(|item| item.0)
            .unwrap_or_default(),
        RecommendationTimespan::EightHour => weather_conditions
            .split_at_checked(8)
            .map(|item| item.0)
            .unwrap_or_default(),
        RecommendationTimespan::TwelveHour => weather_conditions
            .split_at_checked(12)
            .map(|item| item.0)
            .unwrap_or_default(),
        RecommendationTimespan::TwentyFourHour => weather_conditions
            .split_at_checked(24)
            .map(|item| item.0)
            .unwrap_or_default(),
    };
    for hour_entry in relevant_conditions {
        let mut recommendations_list_raw = vec![];
        match hour_entry.uv_index {
            UvIndex::Low => {}
            UvIndex::Moderate | UvIndex::High => {
                recommendations_list_raw.push(WeatherRecommendation::LowUvRisk);
            }
            UvIndex::VeryHigh | UvIndex::Extreme => {
                recommendations_list_raw.push(WeatherRecommendation::HighUvRisk);
            }
        }
        if hour_entry.weathercode.is_snow() {
            recommendations_list_raw.push(WeatherRecommendation::ExpectSnow);
        }
        if hour_entry.weathercode.is_storm() {
            recommendations_list_raw.push(WeatherRecommendation::ExpectStorm);
        }
        if hour_entry.weathercode.is_fog() {
            recommendations_list_raw.push(WeatherRecommendation::ExpectFog);
        }
        if hour_entry.is_metric {
            if hour_entry.apparent_temperature > SHORTS_TEMP_THRESHOLD_METRIC {
                recommendations_list_raw.push(WeatherRecommendation::WearShorts);
            } else if hour_entry.apparent_temperature < JUMPER_TEMP_THRESHOLD_METRIC {
                recommendations_list_raw.push(WeatherRecommendation::WearJumper);
            }
            if hour_entry.temperature_2m <= FREEZING_TEMP_THRESHOLD_METRIC {
                recommendations_list_raw.push(WeatherRecommendation::Freezing);
            }
            if hour_entry.windspeed_10m > HIGH_WINDS_THRESHOLD_METRIC {
                recommendations_list_raw.push(WeatherRecommendation::StrongWinds);
            }
            // If theres rain bring umbrella, unless its too windy then bring a coat
            if hour_entry.weathercode.is_rain() {
                if hour_entry.windspeed_10m > WINDS_UMBRELLA_THRESHOLD_METRIC {
                    recommendations_list_raw.push(WeatherRecommendation::ExpectRainStrongWinds);
                } else {
                    recommendations_list_raw.push(WeatherRecommendation::ExpectRainLightWinds);
                }
            }
        } else {
            if hour_entry.apparent_temperature > SHORTS_TEMP_THRESHOLD_IMPERIAL {
                recommendations_list_raw.push(WeatherRecommendation::WearShorts);
            } else if hour_entry.apparent_temperature < JUMPER_TEMP_THRESHOLD_IMPERIAL {
                recommendations_list_raw.push(WeatherRecommendation::WearJumper);
            }
            if hour_entry.temperature_2m <= FREEZING_TEMP_THRESHOLD_IMPERIAL {
                recommendations_list_raw.push(WeatherRecommendation::Freezing);
            }
            if hour_entry.windspeed_10m > HIGH_WINDS_THRESHOLD_IMPERIAL {
                recommendations_list_raw.push(WeatherRecommendation::StrongWinds);
            }
            // If theres rain bring umbrella, unless its too windy then bring a coat
            if hour_entry.weathercode.is_rain() {
                if hour_entry.windspeed_10m > WINDS_UMBRELLA_THRESHOLD_IMPERIAL {
                    recommendations_list_raw.push(WeatherRecommendation::ExpectRainStrongWinds);
                } else {
                    recommendations_list_raw.push(WeatherRecommendation::ExpectRainLightWinds);
                }
            }
        }
        for recommendation in recommendations_list_raw {
            recommendations_list_with_times.push((
                recommendation,
                hour_entry.time,
                hour_entry.time.saturating_add(60 * 60),
            ));
        }
    }

    // Get sunset and sunrise recs

    let current_time_rounded = chrono::Utc::now()
        .duration_trunc(chrono::TimeDelta::hours(1))
        .expect("Can't fail");
    let end_time = current_time_rounded
        .checked_add_signed(chrono::TimeDelta::hours(timespan.to_int()))
        .expect("Can't fail");

    let todays_sunrise =
        chrono::DateTime::from_timestamp_secs(todays_entry.sunrise).expect("Never None");
    let todays_sunset =
        chrono::DateTime::from_timestamp_secs(todays_entry.sunset).expect("Never None");
    let tomorrows_sunrise =
        chrono::DateTime::from_timestamp_secs(tomorrows_entry.sunrise).expect("Never None");
    let tomorrows_sunset =
        chrono::DateTime::from_timestamp_secs(tomorrows_entry.sunset).expect("Never None");

    if todays_sunrise > current_time_rounded && todays_sunrise < end_time {
        recommendations_list_with_times.push((
            WeatherRecommendation::Sunrise(todays_sunrise.format("%H:%M").to_string()),
            0,
            0,
        ));
    }

    if todays_sunset > current_time_rounded && todays_sunset < end_time {
        recommendations_list_with_times.push((
            WeatherRecommendation::Sunset(todays_sunset.format("%H:%M").to_string()),
            0,
            0,
        ));
    }

    if tomorrows_sunrise > current_time_rounded && tomorrows_sunrise < end_time {
        recommendations_list_with_times.push((
            WeatherRecommendation::Sunrise(tomorrows_sunrise.format("%H:%M").to_string()),
            0,
            0,
        ));
    }

    if tomorrows_sunset > current_time_rounded && tomorrows_sunset < end_time {
        recommendations_list_with_times.push((
            WeatherRecommendation::Sunset(tomorrows_sunset.format("%H:%M").to_string()),
            0,
            0,
        ));
    }

    // Group recs
    let mut grouped_recommendations: Vec<TimedRecommendation> = vec![];
    for (recommendation, start_time, end_time) in recommendations_list_with_times {
        if let Some(previous) = grouped_recommendations.iter_mut().find(|previous| {
            previous.recommendation == recommendation && previous.end_time == start_time
        }) {
            previous.end_time = end_time;
            continue;
        }

        grouped_recommendations.push(TimedRecommendation {
            recommendation,
            start_time,
            end_time,
        });
    }
    grouped_recommendations
}
