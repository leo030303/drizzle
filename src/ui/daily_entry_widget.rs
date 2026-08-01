use relm4::{
    gtk::prelude::{BoxExt, OrientableExt},
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
            set_spacing: 5,
            gtk::Label {
                set_label: &self.forecast_data.time.to_string(),
            },
            gtk::Label {
                set_label: &format!("{} / {}", self.forecast_data.temperature_2m_max, self.forecast_data.temperature_2m_min),
            },
            gtk::Label {
                set_label: &self.forecast_data.weathercode.to_string(),
            },
            gtk::Label {
                set_label: &self.forecast_data.sunrise.to_string(),
            },
            gtk::Label {
                set_label: &self.forecast_data.sunset.to_string(),
            },
            gtk::Label {
                set_label: &self.forecast_data.precipitation_sum.to_string(),
            },
            gtk::Label {
                set_label: &self.forecast_data.windspeed_10m_max.to_string(),
            },
            gtk::Label {
                set_label: &self.forecast_data.uv_index_max.to_string(),
            }
        }
    }

    fn init_model(init: Self::Init, _index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
        Self {
            forecast_data: init,
        }
    }
}
