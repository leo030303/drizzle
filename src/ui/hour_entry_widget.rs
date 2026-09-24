use relm4::{
    gtk::{
        accessible,
        prelude::{AccessibleExt, AccessibleExtManual, BoxExt, OrientableExt, WidgetExt},
    },
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
            set_width_request: 160,
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
                    set_icon_name: Some(model.forecast_data.weathercode.get_icon_name(model.forecast_data.is_day)),
                    set_icon_size: gtk::IconSize::Large,
                },
                gtk::Label {
                    set_css_classes: &["title-2"],
                    set_accessible_role: gtk::AccessibleRole::Presentation,
                    set_label: &time_label(&model.forecast_data),
                },
            },
            gtk::Label {
                set_css_classes: &["title-4"],
                set_accessible_role: gtk::AccessibleRole::Presentation,
                set_label: &temp_label(&model.forecast_data),
                set_margin_horizontal: 5,
            },
            gtk::Label {
                set_accessible_role: gtk::AccessibleRole::Presentation,
                set_label: &rain_label(&model.forecast_data),
                set_margin_horizontal: 5,
            },
            gtk::Label {
                set_accessible_role: gtk::AccessibleRole::Presentation,
                set_label: &wind_label(&model.forecast_data),
                set_margin_horizontal: 5,
            },
            gtk::Label {
                set_accessible_role: gtk::AccessibleRole::Presentation,
                set_label: &uv_label(&model.forecast_data),
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

fn accessible_label(forecast_data: &HourlyEntry) -> String {
    format!(
        "{}\n{}\n{}\n{}\n{}\n{}",
        time_label(forecast_data),
        forecast_data.weathercode.get_accessible_label(),
        temp_label(forecast_data),
        rain_label(forecast_data),
        wind_label(forecast_data),
        uv_label(forecast_data)
    )
}

fn uv_label(forecast_data: &HourlyEntry) -> String {
    format!("UV Index: {}", forecast_data.uv_index)
}

fn wind_label(forecast_data: &HourlyEntry) -> String {
    format!(
        "Wind: {} {}",
        forecast_data.windspeed_10m,
        if forecast_data.is_metric {
            "km/h"
        } else {
            "mph"
        }
    )
}

fn rain_label(forecast_data: &HourlyEntry) -> String {
    format!(
        "Rain: {}{} / {}%",
        forecast_data.precipitation,
        if forecast_data.is_metric { "mm" } else { "in" },
        forecast_data.precipitation_probability
    )
}

fn temp_label(forecast_data: &HourlyEntry) -> String {
    format!(
        "{}{}",
        forecast_data.temperature_2m,
        if forecast_data.is_metric {
            "℃"
        } else {
            "℉"
        }
    )
}

fn time_label(forecast_data: &HourlyEntry) -> String {
    chrono::DateTime::from_timestamp_secs(forecast_data.time).map_or_else(
        || String::from("Invalid Timestamp"),
        |time| time.format("%H:%M").to_string(),
    )
}
