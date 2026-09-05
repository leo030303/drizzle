use crate::model::{uv_index::UvIndex, weather_code::WeatherCode};
use relm4::gtk::glib;
use relm4::gtk::glib::subclass::prelude::*;
use std::cell::RefCell;

#[derive(Debug, Clone)]
pub struct HourlyEntry {
    pub time: i64,
    pub temperature_2m: f64,
    pub apparent_temperature: f64,
    pub weathercode: WeatherCode,
    pub precipitation: f64,
    pub precipitation_probability: f64,
    pub windspeed_10m: f64,
    pub uv_index: UvIndex,
    pub is_day: bool,
    pub is_metric: bool,
}

mod imp {
    use super::{HourlyEntry, ObjectImpl, ObjectSubclass, RefCell, glib};

    #[derive(Default)]
    pub struct HourlyEntryObject {
        pub entry: RefCell<Option<HourlyEntry>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for HourlyEntryObject {
        const NAME: &'static str = "HourlyEntryObject";
        type Type = super::HourlyEntryObject;
    }

    impl ObjectImpl for HourlyEntryObject {}
}

glib::wrapper! {
    pub struct HourlyEntryObject(ObjectSubclass<imp::HourlyEntryObject>);
}

impl HourlyEntryObject {
    pub fn new(entry: HourlyEntry) -> Self {
        let object: Self = glib::Object::new();

        object.imp().entry.replace(Some(entry));

        object
    }

    pub fn entry(&self) -> HourlyEntry {
        self.imp()
            .entry
            .borrow()
            .as_ref()
            .expect("HourlyEntryObject has no HourlyEntry")
            .clone()
    }
}
