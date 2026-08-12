use relm4::{
    FactorySender, RelmWidgetExt,
    gtk::{
        self,
        prelude::{BoxExt, ButtonExt, OrientableExt, WidgetExt},
    },
    prelude::{DynamicIndex, FactoryComponent},
};

use crate::{modals::city_picker::CityPickerDialogMsg, weather_api::find_city::GeoResponse};

pub struct CitySearchResultRow {
    city: GeoResponse,
}

#[derive(Debug)]
pub enum CityRowMsg {
    Clicked,
}

#[relm4::factory(pub)]
impl FactoryComponent for CitySearchResultRow {
    type Init = GeoResponse;
    type Input = CityRowMsg;
    type Output = CityPickerDialogMsg;
    type CommandOutput = ();
    type ParentWidget = gtk::ListBox;

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Horizontal,
            set_spacing: 10,
            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_hexpand: true,
                set_margin_all: 8,

                gtk::Label {
                    set_halign: gtk::Align::Start,
                    #[watch]
                    set_label: &self.city.name,
                    add_css_class: "heading",
                },

                gtk::Label {
                    set_halign: gtk::Align::Start,
                    #[watch]
                    set_label: &format!(
                        "{}, {}",
                        self.city.admin1,
                        self.city.country
                    ),
                },
            },
            gtk::Button {
                set_margin_all: 5,
                set_vexpand: false,
                set_icon_name: "object-select-symbolic",
                connect_clicked => CityRowMsg::Clicked,
            },
        },
    }

    fn update(&mut self, message: Self::Input, sender: FactorySender<Self>) {
        match message {
            CityRowMsg::Clicked => {
                sender
                    .output(CityPickerDialogMsg::SelectCity(self.city.clone()))
                    .unwrap();
            }
        }
    }
    fn init_model(init: Self::Init, _index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
        Self { city: init }
    }
}
