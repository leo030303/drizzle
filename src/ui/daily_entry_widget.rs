use relm4::{
    gtk::prelude::{BoxExt, OrientableExt, WidgetExt},
    prelude::*,
};

use crate::{app::AppMsg, weather_api::weather::DailyEntry};

pub struct DayEntryWidget {
    pub forecast_data: DailyEntry,
}

#[relm4::factory(pub)]
impl FactoryComponent for DayEntryWidget {
    type Init = DailyEntry;
    type Input = ();
    type Output = AppMsg;
    type CommandOutput = ();
    type ParentWidget = gtk::Box;

    view! {
        gtk::Box{
            set_orientation: gtk::Orientation::Vertical,
            add_css_class: "card",
            add_css_class: "weather-card",
            add_css_class: self.forecast_data.weathercode.get_background_css_class(true),
            set_spacing: 5,
            gtk::Box{
                set_orientation: gtk::Orientation::Horizontal,
                set_spacing: 10,
                set_margin_top: 10,
                set_margin_horizontal: 10,
                gtk::Image {
                    set_icon_name: Some(self.forecast_data.weathercode.get_icon_name(true)),
                    set_icon_size: gtk::IconSize::Large,
                },
                gtk::Label {
                    add_css_class: "title-2",
                    set_label: &chrono::DateTime::from_timestamp_secs(self.forecast_data.time).unwrap().format("%a %d/%m").to_string(),
                },
            },
            gtk::Box{
                set_orientation: gtk::Orientation::Horizontal,
                set_spacing: 5,
                set_margin_top: 5,
                set_margin_horizontal: 10,
                set_hexpand: true,
                set_halign: gtk::Align::Center,
                gtk::Image {
                    set_icon_name: Some("thermometer-gain"),
                    set_icon_size: gtk::IconSize::Normal,
                },
                gtk::Label {
                    set_label: &format!("{}℃", self.forecast_data.temperature_2m_max),
                    set_margin_end: 10,
                },
                gtk::Image {
                    set_icon_name: Some("thermometer-loss"),
                    set_icon_size: gtk::IconSize::Normal,
                },
                gtk::Label {
                    set_label: &format!("{}℃", self.forecast_data.temperature_2m_min),
                },
            },
            gtk::Box{
                set_orientation: gtk::Orientation::Horizontal,
                set_spacing: 5,
                set_margin_top: 5,
                set_margin_horizontal: 10,
                set_hexpand: true,
                set_halign: gtk::Align::Center,
                gtk::Image {
                    set_icon_name: Some("daytime-sunrise"),
                    set_icon_size: gtk::IconSize::Normal,
                },
                gtk::Label {
                    set_label: &chrono::DateTime::from_timestamp_secs(self.forecast_data.sunrise).unwrap().format("%H:%M").to_string(),
                    set_margin_end: 10,
                },
                gtk::Image {
                    set_icon_name: Some("daytime-sunset"),
                    set_icon_size: gtk::IconSize::Normal,
                },
                gtk::Label {
                    set_label: &chrono::DateTime::from_timestamp_secs(self.forecast_data.sunset).unwrap().format("%H:%M").to_string(),
                },
            },
            gtk::Label {
                set_label: &format!("Rain: {}mm / {}%", self.forecast_data.precipitation_sum, self.forecast_data.precipitation_probability_max),
                set_margin_horizontal: 5,
            },
            gtk::Label {
                set_label: &format!("Wind: {} km/h", self.forecast_data.windspeed_10m_max),
                set_margin_horizontal: 5,
            },
            gtk::Label {
                set_label: &format!("UV Index: {}", self.forecast_data.uv_index_max),
                set_margin_horizontal: 5,
                set_margin_bottom: 10,
            },
        }
    }

    fn init_model(init: Self::Init, _index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
        Self {
            forecast_data: init,
        }
    }
}
