use crate::weather_api::{uv_index::UvIndex, weather::HourlyEntry};

#[derive(Debug)]
pub struct TimedRecommendation {
    pub recommendation: WeatherRecommendation,
    pub start_time: i64,
    pub end_time: i64,
}

#[derive(Debug, PartialEq)]
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
    Sunset(String), // TODO
    Sunrise(String),
}

impl TimedRecommendation {
    pub fn get_text(&self) -> String {
        let start_time = chrono::DateTime::from_timestamp_secs(self.start_time)
            .unwrap()
            .format("%l%P")
            .to_string();
        let end_time = chrono::DateTime::from_timestamp_secs(self.end_time)
            .unwrap()
            .format("%l%P")
            .to_string();
        match &self.recommendation {
            WeatherRecommendation::LowUvRisk => format!("Wear suncream: {start_time} - {end_time}"),
            WeatherRecommendation::HighUvRisk => {
                format!("Avoid direct sunlight: {start_time} - {end_time}")
            }
            WeatherRecommendation::ExpectRainStrongWinds => {
                format!("Expect rain and strong winds: {start_time} - {end_time}",)
            }
            WeatherRecommendation::ExpectRainLightWinds => {
                format!("Expect rain and light winds: {start_time} - {end_time}")
            }
            WeatherRecommendation::StrongWinds => {
                format!("Expect strong winds: {start_time} - {end_time}")
            }
            WeatherRecommendation::ExpectStorm => {
                format!("Expect a storm: {start_time} - {end_time}")
            }
            WeatherRecommendation::ExpectSnow => format!("Expect snow: {start_time} - {end_time}"),
            WeatherRecommendation::ExpectFog => format!("Expect fog: {start_time} - {end_time}"),
            WeatherRecommendation::WearJumper => {
                format!("Jumper weather: {start_time} - {end_time}")
            }
            WeatherRecommendation::WearShorts => {
                format!("Shorts weather: {start_time} - {end_time}")
            }
            WeatherRecommendation::Freezing => {
                format!("Freezing: {start_time} - {end_time}")
            }
            WeatherRecommendation::Sunset(time) => format!("Sunset at {time}"),
            WeatherRecommendation::Sunrise(time) => format!("Sunrise at {time}"),
        }
    }
}

#[derive(Debug)]
pub enum RecommendationTimespan {
    FourHour,
    EightHour,
    TwelveHour,
    TwentyFourHour,
}

impl RecommendationTimespan {
    pub fn to_name(&self) -> &'static str {
        match self {
            RecommendationTimespan::FourHour => "FourHour",
            RecommendationTimespan::EightHour => "EightHour",
            RecommendationTimespan::TwelveHour => "TwelveHour",
            RecommendationTimespan::TwentyFourHour => "TwentyFourHour",
        }
    }
    pub fn from_name(name: &str) -> Self {
        match name {
            "FourHour" => RecommendationTimespan::FourHour,
            "EightHour" => RecommendationTimespan::EightHour,
            "TwelveHour" => RecommendationTimespan::TwelveHour,
            "TwentyFourHour" => RecommendationTimespan::TwentyFourHour,
            _ => RecommendationTimespan::TwentyFourHour,
        }
    }
}

// Corresponds to beaufort scale 6 Strong Breeze, too much wind for an umbrella
const WINDS_UMBRELLA_THRESHOLD_METRIC: f64 = 39.0;
// Corresponds to beaufort scale 7 Near Gale
const HIGH_WINDS_THRESHOLD_METRIC: f64 = 50.0;
// Warm enough for shorts/summer clothes
const SHORTS_TEMP_THRESHOLD_METRIC: f64 = 20.0;
// Cold enough to need a jumper/jacket
const JUMPER_TEMP_THRESHOLD_METRIC: f64 = 15.0;

// TODO Handle imperial here
pub fn get_recommendations(
    weather_conditions: &[HourlyEntry],
    timespan: &RecommendationTimespan,
) -> Vec<TimedRecommendation> {
    let mut recommendations_list_with_times: Vec<(WeatherRecommendation, i64, i64)> = vec![];
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
    relevant_conditions.iter().for_each(|hour_entry| {
        let mut recommendations_list_raw = vec![];
        match hour_entry.uv_index {
            UvIndex::Low => {}
            UvIndex::Moderate => recommendations_list_raw.push(WeatherRecommendation::LowUvRisk),
            UvIndex::High => recommendations_list_raw.push(WeatherRecommendation::LowUvRisk),
            UvIndex::VeryHigh => recommendations_list_raw.push(WeatherRecommendation::HighUvRisk),
            UvIndex::Extreme => recommendations_list_raw.push(WeatherRecommendation::HighUvRisk),
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
        if hour_entry.weathercode.is_snow() {
            recommendations_list_raw.push(WeatherRecommendation::ExpectSnow);
        }
        if hour_entry.weathercode.is_storm() {
            recommendations_list_raw.push(WeatherRecommendation::ExpectStorm);
        }
        if hour_entry.weathercode.is_fog() {
            recommendations_list_raw.push(WeatherRecommendation::ExpectFog);
        }
        if hour_entry.apparent_temperature > SHORTS_TEMP_THRESHOLD_METRIC {
            recommendations_list_raw.push(WeatherRecommendation::WearShorts);
        } else if hour_entry.apparent_temperature < JUMPER_TEMP_THRESHOLD_METRIC {
            recommendations_list_raw.push(WeatherRecommendation::WearJumper);
        }
        if hour_entry.temperature_2m <= 0.0 {
            recommendations_list_raw.push(WeatherRecommendation::Freezing);
        }
        for recommendation in recommendations_list_raw {
            recommendations_list_with_times.push((
                recommendation,
                hour_entry.time,
                hour_entry.time + 60 * 60,
            ));
        }
    });

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
