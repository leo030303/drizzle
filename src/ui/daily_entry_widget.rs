use relm4::{
    gtk::{
        accessible,
        prelude::{AccessibleExtManual, BoxExt, OrientableExt, WidgetExt},
    },
    prelude::*,
};

use crate::{app::AppMsg, model::daily_entry::DailyEntry};

pub struct DailyEntryWidget {
    pub forecast_data: DailyEntry,
}

#[relm4::component(pub)]
impl Component for DailyEntryWidget {
    type Init = DailyEntry;
    type Input = ();
    type Output = AppMsg;
    type Widgets = DailyEntryWidgets;
    type CommandOutput = ();

    view! {
        gtk::Box{
            set_orientation: gtk::Orientation::Vertical,
            set_css_classes: &[
                "card",
                "weather-card",
                model.forecast_data.weathercode.get_background_css_class(true)
            ],
            set_spacing: 5,
            set_width_request: 180,
            set_margin_horizontal: 2,
            gtk::Box{
                set_orientation: gtk::Orientation::Horizontal,
                set_spacing: 10,
                set_margin_top: 10,
                set_margin_horizontal: 10,
                set_hexpand: true,
                set_halign: gtk::Align::Center,
                gtk::Image {
                    set_icon_name: Some(model.forecast_data.weathercode.get_icon_name(true)),
                    set_icon_size: gtk::IconSize::Large,
                },
                gtk::Label {
                    set_css_classes: &["title-2"],
                    set_label: &chrono::DateTime::from_timestamp_secs(model.forecast_data.time).map_or_else(|| String::from("Invalid Timestamp"), |time| time.format("%a %d/%m").to_string()),
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
                    set_label: &format!(
                                    "{}{}",
                                    model.forecast_data.temperature_2m_max,
                                    if model.forecast_data.is_metric {"℃"} else {"℉"}
                                ),
                    update_property: &[accessible::Property::Label(&format!(
                                    "Maximum Temperature {}{}",
                                    model.forecast_data.temperature_2m_max,
                                    if model.forecast_data.is_metric {"℃"} else {"℉"}
                                ))],
                    set_margin_end: 10,
                },
                gtk::Image {
                    set_icon_name: Some("thermometer-loss"),
                    set_icon_size: gtk::IconSize::Normal,
                },
                gtk::Label {
                    set_label: &format!(
                                    "{}{}",
                                    model.forecast_data.temperature_2m_min,
                                    if model.forecast_data.is_metric {"℃"} else {"℉"}
                                ),
                    update_property: &[accessible::Property::Label(&format!(
                                    "Minimum Temperature {}{}",
                                    model.forecast_data.temperature_2m_min,
                                    if model.forecast_data.is_metric {"℃"} else {"℉"}
                                ))],
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
                    set_label: &chrono::DateTime::from_timestamp_secs(model.forecast_data.sunrise).map_or_else(|| String::from("Invalid Timestamp"), |time| time.format("%H:%M").to_string()),
                    update_property: &[accessible::Property::Label(&chrono::DateTime::from_timestamp_secs(model.forecast_data.sunrise).map_or_else(|| String::from("Invalid Timestamp"), |time| time.format("Sunrise %H:%M").to_string()))],
                    set_margin_end: 10,
                },
                gtk::Image {
                    set_icon_name: Some("daytime-sunset"),
                    set_icon_size: gtk::IconSize::Normal,
                },
                gtk::Label {
                    set_label: &chrono::DateTime::from_timestamp_secs(model.forecast_data.sunset).map_or_else(|| String::from("Invalid Timestamp"), |time| time.format("%H:%M").to_string()),
                    update_property: &[accessible::Property::Label(&chrono::DateTime::from_timestamp_secs(model.forecast_data.sunset).map_or_else(|| String::from("Invalid Timestamp"), |time| time.format("Sunset %H:%M").to_string()))],
                },
            },
            gtk::Label {
                set_label: &format!(
                                "Rain: {}{} / {}%",
                                model.forecast_data.precipitation_sum,
                                if model.forecast_data.is_metric {"mm"} else {"in"},
                                model.forecast_data.precipitation_probability_max
                            ),
                set_margin_horizontal: 5,
            },
            gtk::Label {
                set_label: &format!(
                                "Wind: {} {}",
                                model.forecast_data.windspeed_10m_max,
                                if model.forecast_data.is_metric {"km/h"} else {"mph"}
                            ),
                set_margin_horizontal: 5,
            },
            gtk::Label {
                set_label: &format!("UV Index: {}", model.forecast_data.uv_index_max),
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
