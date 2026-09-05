use relm4::gtk::glib;
use relm4::gtk::glib::subclass::prelude::*;
use std::cell::RefCell;

use crate::model::{uv_index::UvIndex, weather_code::WeatherCode};

#[derive(Debug, Clone)]
pub struct DailyEntry {
    pub time: i64,
    pub weathercode: WeatherCode,
    pub temperature_2m_max: f64,
    pub temperature_2m_min: f64,
    pub sunrise: i64,
    pub sunset: i64,
    pub uv_index_max: UvIndex,
    pub precipitation_sum: f64,
    pub precipitation_probability_max: f64,
    pub windspeed_10m_max: f64,
    pub is_metric: bool,
}

mod imp {
    use super::{DailyEntry, ObjectImpl, ObjectSubclass, RefCell, glib};

    #[derive(Default)]
    pub struct DailyEntryObject {
        pub entry: RefCell<Option<DailyEntry>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for DailyEntryObject {
        const NAME: &'static str = "DailyEntryObject";
        type Type = super::DailyEntryObject;
    }

    impl ObjectImpl for DailyEntryObject {}
}

glib::wrapper! {
    pub struct DailyEntryObject(ObjectSubclass<imp::DailyEntryObject>);
}

impl DailyEntryObject {
    pub fn new(entry: DailyEntry) -> Self {
        let object: Self = glib::Object::new();

        object.imp().entry.replace(Some(entry));

        object
    }

    pub fn entry(&self) -> DailyEntry {
        self.imp()
            .entry
            .borrow()
            .as_ref()
            .expect("DailyEntryObject has no DailyEntry")
            .clone()
    }
}
