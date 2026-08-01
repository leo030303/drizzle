use relm4::{
    gtk::prelude::{BoxExt, OrientableExt},
    prelude::*,
};

use crate::{app::AppMsg, weather_api::weather::HourlyEntry};

pub struct HourEntryWidget {
    pub forecast_data: HourlyEntry,
}

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
            set_spacing: 5,
            gtk::Label {
                set_label: &self.forecast_data.time.to_string(),
            },
            gtk::Label {
                set_label: &self.forecast_data.temperature_2m.to_string(),
            },
            gtk::Label {
                set_label: &format!("Feels like {}", self.forecast_data.apparent_temperature),
            },
            gtk::Label {
                set_label: &self.forecast_data.weathercode.to_string(),
            },
            gtk::Label {
                set_label: &self.forecast_data.precipitation.to_string(),
            },
            gtk::Label {
                set_label: &self.forecast_data.precipitation_probability.to_string(),
            },
            gtk::Label {
                set_label: &self.forecast_data.visibility.to_string(),
            },
            gtk::Label {
                set_label: &self.forecast_data.windspeed_10m.to_string(),
            },
            gtk::Label {
                set_label: &self.forecast_data.wind_direction_10m.to_string(),
            },
            gtk::Label {
                set_label: &self.forecast_data.uv_index.to_string(),
            },
            gtk::Label {
                set_label: &self.forecast_data.is_day.to_string(),
            }
        }
    }

    fn init_model(init: Self::Init, _index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
        Self {
            forecast_data: init,
        }
    }
}
