use relm4::{
    Component, ComponentParts, ComponentSender, RelmWidgetExt,
    adw::{self, prelude::AdwDialogExt},
    gtk::{
        self,
        prelude::{BoxExt, ButtonExt, EditableExt, OrientableExt, WidgetExt},
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
    recent_cities_list: Vec<GeoResponse>,
    search_entry_widget: gtk::SearchEntry,
}

#[derive(Debug)]
pub enum CityPickerDialogMsg {
    SearchQueryChanged(String),
    SearchCities,
    SelectCity(GeoResponse),
    SetRecentCities(Vec<GeoResponse>),
}

#[relm4::component(pub)]
impl Component for CityPickerDialog {
    type Init = ();
    type Input = CityPickerDialogMsg;
    type Output = AppMsg;
    type CommandOutput = Vec<GeoResponse>;
    type Widgets = CityPickerWidgets;

    view! {
        adw::Dialog {
            #[wrap(Some)]
            set_child = &adw::ToolbarView {
                add_top_bar = &adw::HeaderBar {},
                #[wrap(Some)]
                set_content = &gtk::Box{
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 10,
                    set_margin_all: 20,


                    gtk::Box {
                        set_orientation: gtk::Orientation::Horizontal,
                        set_spacing: 5,
                        set_halign: gtk::Align::Center,
                        #[name = "search_entry"]
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
                    cities_list_widget -> gtk::ListBox {
                        set_selection_mode: gtk::SelectionMode::None,
                        set_css_classes: &["boxed-list"],
                        set_margin_top: 10,
                    }
                },

                },
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
        let cities_list_widget = search_results.widget();
        let widgets = view_output!();
        let model = Self {
            search_query: String::new(),
            search_results,
            recent_cities_list: vec![],
            search_entry_widget: widgets.search_entry.clone(),
        };

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>, root: &Self::Root) {
        match message {
            CityPickerDialogMsg::SearchQueryChanged(search) => {
                self.search_query = search;
                if self.search_query.trim().is_empty() {
                    self.search_results.guard().clear();
                    for city in self.recent_cities_list.clone() {
                        self.search_results.guard().push_back(city);
                    }
                }
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
                self.search_entry_widget.set_text("");
                root.close();
            }
            CityPickerDialogMsg::SetRecentCities(recent_cities) => {
                self.recent_cities_list = recent_cities;
                if self.search_query.trim().is_empty() {
                    self.search_results.guard().clear();
                    for city in self.recent_cities_list.clone() {
                        self.search_results.guard().push_back(city);
                    }
                }
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
