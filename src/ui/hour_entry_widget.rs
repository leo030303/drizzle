use relm4::{
    gtk::prelude::{BoxExt, OrientableExt, WidgetExt},
    prelude::*,
};

use crate::{app::AppMsg, weather_api::weather::HourlyEntry};

pub struct HourEntryWidget {
    pub forecast_data: HourlyEntry,
}

// TODO Handle imperial units
#[relm4::factory(pub)]
impl FactoryComponent for HourEntryWidget {
    type Init = HourlyEntry;
    type Input = ();
    type Output = AppMsg;
    type CommandOutput = ();
    type ParentWidget = gtk::Box;

    view! {
        gtk::Box{
            set_orientation: gtk::Orientation::Vertical,
            add_css_class: "card",
            add_css_class: "weather-card",
            add_css_class: self.forecast_data.weathercode.get_background_css_class(self.forecast_data.is_day),
            set_spacing: 5,
            gtk::Box{
                set_orientation: gtk::Orientation::Horizontal,
                set_spacing: 10,
                set_margin_top: 10,
                set_margin_horizontal: 10,
                set_hexpand: true,
                set_halign: gtk::Align::Center,
                gtk::Image {
                    set_icon_name: Some(self.forecast_data.weathercode.get_icon_name(self.forecast_data.is_day)),
                    set_icon_size: gtk::IconSize::Large,
                },
                gtk::Label {
                    add_css_class: "title-2",
                    set_label: &chrono::DateTime::from_timestamp_secs(self.forecast_data.time).unwrap().format("%H:%M").to_string(),
                },
            },
            gtk::Label {
                add_css_class: "title-4",
                set_label: &format!("{}℃", self.forecast_data.temperature_2m),
                set_margin_horizontal: 5,
            },
            gtk::Label {
                set_label: &format!("Rain: {}mm / {}%", self.forecast_data.precipitation, self.forecast_data.precipitation_probability),
                set_margin_horizontal: 5,
            },
            gtk::Label {
                set_label: &format!("Wind: {} km/h", self.forecast_data.windspeed_10m),
                set_margin_horizontal: 5,
            },
            gtk::Label {
                set_label: &format!("UV Index: {}", self.forecast_data.uv_index),
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
