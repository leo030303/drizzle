use relm4::{
    gtk::prelude::{BoxExt, OrientableExt, WidgetExt},
    prelude::*,
};

use crate::{
    app::AppMsg,
    weather_api::weather::{HourlyEntry, WeatherCode},
};

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
            add_css_class: "bg-transparency",
            set_spacing: 5,
            gtk::Label {
                add_css_class: "title-4",
                set_label: &chrono::DateTime::from_timestamp_secs(self.forecast_data.time).unwrap().format("%H:%M").to_string(),
                set_margin_top: 5,
            },
            gtk::Image {
                set_icon_name: Some(WeatherCode::try_from(self.forecast_data.weathercode).unwrap().get_icon_name(self.forecast_data.is_day == 1)),
                set_icon_size: gtk::IconSize::Large,
                set_margin_horizontal: 5,
            },
            gtk::Label {
                set_label: &format!("{}℃ / Feels like {}℃", self.forecast_data.temperature_2m, self.forecast_data.apparent_temperature),
                set_margin_horizontal: 5,
            },
            gtk::Label {
                set_label: &format!("Rain: {}mm / {}%", self.forecast_data.precipitation, self.forecast_data.precipitation_probability),
                set_margin_horizontal: 5,
            },
            gtk::Label {
                set_label: &format!("Wind: {}km/h", self.forecast_data.windspeed_10m),
                set_margin_horizontal: 5,
            },
            gtk::Label {
                set_label: &format!("UV Index: {}", self.forecast_data.uv_index),
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
