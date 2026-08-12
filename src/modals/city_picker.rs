use relm4::{
    Component, ComponentParts, ComponentSender, RelmWidgetExt,
    gtk::{
        self,
        prelude::{BoxExt, ButtonExt, EditableExt, OrientableExt, PopoverExt, WidgetExt},
    },
    prelude::FactoryVecDeque,
};

use crate::{
    app::AppMsg,
    ui::city_search_result_row::CitySearchResultRow,
    weather_api::find_city::{GeoResponse, search_city_list},
};

pub struct CityPickerDialog {
    search_query: String,
    search_results: FactoryVecDeque<CitySearchResultRow>,
}

#[derive(Debug)]
pub enum CityPickerDialogMsg {
    SearchQueryChanged(String),
    SearchCities,
    SelectCity(GeoResponse),
    Show,
}

#[relm4::component(pub)]
impl Component for CityPickerDialog {
    type Init = ();
    type Input = CityPickerDialogMsg;
    type Output = AppMsg;
    type CommandOutput = Vec<GeoResponse>;
    type Widgets = CityPickerWidgets;

    view! {
        gtk::Popover {
            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 10,
                set_margin_all: 20,

                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_spacing: 5,
                    gtk::SearchEntry {
                        set_placeholder_text: Some("Search for a city"),
                        connect_activate => CityPickerDialogMsg::SearchCities,
                        connect_search_changed[sender] => move |entry| {
                            sender.input(CityPickerDialogMsg::SearchQueryChanged(entry.text().to_string()));
                        },
                    },
                    gtk::Button {
                        set_icon_name: "system-search-symbolic",
                        connect_clicked => CityPickerDialogMsg::SearchCities
                    },
                },

                #[local_ref]
                city_list -> gtk::ListBox {
                    set_selection_mode: gtk::SelectionMode::None,
                    add_css_class: "boxed-list",
                    set_margin_all: 10,
                },
            }
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let search_results: FactoryVecDeque<CitySearchResultRow> = FactoryVecDeque::builder()
            .launch(gtk::ListBox::default())
            .forward(sender.input_sender(), |output| output);

        let model = Self {
            search_query: String::new(),
            search_results,
        };
        let city_list = model.search_results.widget();

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>, root: &Self::Root) {
        match message {
            CityPickerDialogMsg::SearchQueryChanged(search) => {
                self.search_query = search;
            }
            CityPickerDialogMsg::SearchCities => {
                if self.search_query.trim().is_empty() {
                    self.search_results.guard().clear();
                    return;
                }
                let search_query = self.search_query.clone();
                sender
                    .oneshot_command(async move { search_city_list(&search_query).await.unwrap() });
            }
            CityPickerDialogMsg::SelectCity(city) => {
                sender.output(AppMsg::SelectCity(city)).unwrap();
            }

            CityPickerDialogMsg::Show => {
                root.popup();
            }
        }
    }

    fn update_cmd(
        &mut self,
        cities: Self::CommandOutput,
        _sender: ComponentSender<Self>,
        _root: &Self::Root,
    ) {
        self.search_results.guard().clear();

        for city in cities {
            self.search_results.guard().push_back(city);
        }
    }
}
