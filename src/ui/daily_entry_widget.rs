use relm4::{
    gtk::prelude::{BoxExt, OrientableExt, WidgetExt},
    prelude::*,
};

use crate::{
    app::AppMsg,
    weather_api::weather::{DayEntry, WeatherCode},
};

pub struct DayEntryWidget {
    pub forecast_data: DayEntry,
}

#[relm4::factory(pub)]
impl FactoryComponent for DayEntryWidget {
    type Init = DayEntry;
    type Input = ();
    type Output = AppMsg;
    type CommandOutput = ();
    type ParentWidget = gtk::Box;

    view! {
        gtk::Box{
            set_orientation: gtk::Orientation::Vertical,
            add_css_class: "card",
            add_css_class: "bg-transparency",
            set_spacing: 5,
            gtk::Label {
                add_css_class: "title-4",
                set_label: &chrono::DateTime::from_timestamp_secs(self.forecast_data.time).unwrap().format("%a %d/%m").to_string(),
                set_margin_horizontal: 5,
                set_margin_top: 5,
            },
            gtk::Image {
                set_icon_name: Some(WeatherCode::try_from(self.forecast_data.weathercode).unwrap().get_icon_name(true)),
                set_icon_size: gtk::IconSize::Large,
                set_margin_horizontal: 5,
            },
            gtk::Label {
                set_label: &format!("{}℃ / {}℃", self.forecast_data.temperature_2m_max, self.forecast_data.temperature_2m_min),
                set_margin_horizontal: 5,
            },
            gtk::Label {
                set_label: &format!("Sunrise: {}", chrono::DateTime::from_timestamp_secs(self.forecast_data.sunrise).unwrap().format("%H:%M")),
                set_margin_horizontal: 5,
            },
            gtk::Label {
                set_label: &format!("Sunset: {}", chrono::DateTime::from_timestamp_secs(self.forecast_data.sunset).unwrap().format("%H:%M")),
                set_margin_horizontal: 5,
            },
            gtk::Label {
                set_label: &format!("Rain: {}mm / {}%", self.forecast_data.precipitation_sum, self.forecast_data.precipitation_probability_max),
                set_margin_horizontal: 5,
            },
            gtk::Label {
                set_label: &format!("Wind: {}km/h", self.forecast_data.windspeed_10m_max),
                set_margin_horizontal: 5,
            },
            gtk::Label {
                set_label: &format!("UV Index: {}", self.forecast_data.uv_index_max),
                set_margin_horizontal: 5,
            },
        }
    }

    fn init_model(init: Self::Init, _index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
        Self {
            forecast_data: init,
        }
    }
}
