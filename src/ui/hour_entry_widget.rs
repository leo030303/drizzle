use relm4::{
    gtk::prelude::{BoxExt, OrientableExt, WidgetExt},
    prelude::*,
};

use crate::{app::AppMsg, model::hourly_entry::HourlyEntry};

pub struct HourEntryWidget {
    pub forecast_data: HourlyEntry,
}

#[relm4::component(pub)]
impl Component for HourEntryWidget {
    type Init = HourlyEntry;
    type Input = ();
    type Output = AppMsg;
    type Widgets = HourlyEntryWidgets;
    type CommandOutput = ();

    view! {
        gtk::Box{
            set_orientation: gtk::Orientation::Vertical,
            set_css_classes: &[
                "card",
                "weather-card",
                model.forecast_data.weathercode.get_background_css_class(model.forecast_data.is_day)
            ],
            set_spacing: 5,
            gtk::Box{
                set_orientation: gtk::Orientation::Horizontal,
                set_spacing: 10,
                set_margin_top: 10,
                set_margin_horizontal: 10,
                set_hexpand: true,
                set_halign: gtk::Align::Center,
                gtk::Image {
                    set_icon_name: Some(model.forecast_data.weathercode.get_icon_name(model.forecast_data.is_day)),
                    set_icon_size: gtk::IconSize::Large,
                },
                gtk::Label {
                    set_css_classes: &["title-2"],
                    set_label: &chrono::DateTime::from_timestamp_secs(model.forecast_data.time).map_or_else(|| String::from("Invalid Timestamp"), |time| time.format("%H:%M").to_string()),
                },
            },
            gtk::Label {
                set_css_classes: &["title-4"],
                set_label: &format!(
                                "{}{}",
                                model.forecast_data.temperature_2m,
                                if model.forecast_data.is_metric {"℃"} else {"℉"}
                            ),
                set_margin_horizontal: 5,
            },
            gtk::Label {
                set_label: &format!(
                                "Rain: {}{} / {}%",
                                model.forecast_data.precipitation,
                                if model.forecast_data.is_metric {"mm"} else {"in"},
                                model.forecast_data.precipitation_probability
                            ),
                set_margin_horizontal: 5,
            },
            gtk::Label {
                set_label: &format!(
                                "Wind: {} {}",
                                model.forecast_data.windspeed_10m,
                                if model.forecast_data.is_metric {"km/h"} else {"mph"}
                            ),
                set_margin_horizontal: 5,
            },
            gtk::Label {
                set_label: &format!("UV Index: {}", model.forecast_data.uv_index),
                set_margin_horizontal: 5,
                set_margin_bottom: 10,
            },
        }
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self {
            forecast_data: init,
        };
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
}
