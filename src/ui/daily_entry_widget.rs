use gettextrs::gettext;
use relm4::{
    gtk::{
        accessible,
        prelude::{AccessibleExt, AccessibleExtManual, BoxExt, OrientableExt, WidgetExt},
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
            set_accessible_role: gtk::AccessibleRole::Group,
            update_property: &[accessible::Property::Label(&accessible_label(&model.forecast_data))],
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
                    set_label: &time_label(&model.forecast_data),
                    set_accessible_role: gtk::AccessibleRole::Presentation,
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
                    set_label: &max_temp_label(&model.forecast_data),
                    set_margin_end: 10,
                    set_accessible_role: gtk::AccessibleRole::Presentation,
                },
                gtk::Image {
                    set_icon_name: Some("thermometer-loss"),
                    set_icon_size: gtk::IconSize::Normal,
                },
                gtk::Label {
                    set_label: &min_temp_label(&model.forecast_data),
                    set_accessible_role: gtk::AccessibleRole::Presentation,
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
                    set_label: &sunrise_label(&model.forecast_data),
                    set_margin_end: 10,
                    set_accessible_role: gtk::AccessibleRole::Presentation,
                },
                gtk::Image {
                    set_icon_name: Some("daytime-sunset"),
                    set_icon_size: gtk::IconSize::Normal,
                },
                gtk::Label {
                    set_label: &sunset_label(&model.forecast_data),
                    set_accessible_role: gtk::AccessibleRole::Presentation,
                },
            },
            gtk::Label {
                set_label: &rain_label(&model.forecast_data),
                set_margin_horizontal: 5,
                set_accessible_role: gtk::AccessibleRole::Presentation,
            },
            gtk::Label {
                set_label: &wind_label(&model.forecast_data),
                set_margin_horizontal: 5,
                set_accessible_role: gtk::AccessibleRole::Presentation,
            },
            gtk::Label {
                set_label: &uv_label(&model.forecast_data),
                set_margin_horizontal: 5,
                set_margin_bottom: 10,
                set_accessible_role: gtk::AccessibleRole::Presentation,
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

fn time_label(forecast_data: &DailyEntry) -> String {
    chrono::DateTime::from_timestamp_secs(forecast_data.time).map_or_else(
        || gettext("Invalid Timestamp"),
        |time| time.format("%a %d/%m").to_string(),
    )
}

fn max_temp_label(forecast_data: &DailyEntry) -> String {
    format!(
        "{}{}",
        forecast_data.temperature_2m_max,
        if forecast_data.is_metric {
            "℃"
        } else {
            "℉"
        }
    )
}

fn min_temp_label(forecast_data: &DailyEntry) -> String {
    format!(
        "{}{}",
        forecast_data.temperature_2m_min,
        if forecast_data.is_metric {
            "℃"
        } else {
            "℉"
        }
    )
}

fn sunrise_label(forecast_data: &DailyEntry) -> String {
    chrono::DateTime::from_timestamp_secs(forecast_data.sunrise).map_or_else(
        || gettext("Invalid Timestamp"),
        |time| time.format("%H:%M").to_string(),
    )
}

fn sunset_label(forecast_data: &DailyEntry) -> String {
    chrono::DateTime::from_timestamp_secs(forecast_data.sunset).map_or_else(
        || gettext("Invalid Timestamp"),
        |time| time.format("%H:%M").to_string(),
    )
}

fn rain_label(forecast_data: &DailyEntry) -> String {
    format!(
        "{}: {}{} / {}%",
        gettext("Rain"),
        forecast_data.precipitation_sum,
        if forecast_data.is_metric { "mm" } else { "in" },
        forecast_data.precipitation_probability_max
    )
}

fn wind_label(forecast_data: &DailyEntry) -> String {
    format!(
        "{}: {} {}",
        gettext("Wind"),
        forecast_data.windspeed_10m_max,
        if forecast_data.is_metric {
            "km/h"
        } else {
            "mph"
        }
    )
}

fn uv_label(forecast_data: &DailyEntry) -> String {
    format!("{}: {}", gettext("UV Index"), forecast_data.uv_index_max)
}

fn accessible_label(forecast_data: &DailyEntry) -> String {
    format!(
        "{}\n{}\n{} {}\n{} {}\n{} {}\n{} {}\n{}\n{}\n{}",
        time_label(forecast_data),
        forecast_data.weathercode.get_accessible_label(),
        gettext("Maximum Temperature"),
        max_temp_label(forecast_data),
        gettext("Minimum Temperature"),
        min_temp_label(forecast_data),
        gettext("Sunrise"),
        sunrise_label(forecast_data),
        gettext("Sunset"),
        sunset_label(forecast_data),
        rain_label(forecast_data),
        wind_label(forecast_data),
        uv_label(forecast_data)
    )
}
