use chrono::{Datelike, Timelike};
use relm4::{
    gtk::prelude::{BoxExt, OrientableExt, WidgetExt},
    prelude::*,
};

use crate::{app::AppMsg, weather_api::weather::DayEntry};

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
            set_spacing: 5,
            gtk::Label {
                add_css_class: "title-4",
                set_label: &chrono::DateTime::from_timestamp_secs(self.forecast_data.time).unwrap().format("%a %d/%m").to_string(),
                set_margin_horizontal: 5,
                set_margin_top: 5,
            },
            gtk::Label {
                set_label: &format!("{} / {}", self.forecast_data.temperature_2m_max, self.forecast_data.temperature_2m_min),
                set_margin_horizontal: 5,
            },
            gtk::Label {
                set_label: &self.forecast_data.weathercode.to_string(),
                set_margin_horizontal: 5,
            },
            gtk::Label {
                set_label: &self.forecast_data.sunrise.to_string(),
                set_margin_horizontal: 5,
            },
            gtk::Label {
                set_label: &self.forecast_data.sunset.to_string(),
                set_margin_horizontal: 5,
            },
            gtk::Label {
                set_label: &self.forecast_data.precipitation_sum.to_string(),
                set_margin_horizontal: 5,
            },
            gtk::Label {
                set_label: &self.forecast_data.windspeed_10m_max.to_string(),
                set_margin_horizontal: 5,
            },
            gtk::Label {
                set_label: &self.forecast_data.uv_index_max.to_string(),
                set_margin_bottom: 5,
            }
        }
    }

    fn init_model(init: Self::Init, _index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
        Self {
            forecast_data: init,
        }
    }
}
