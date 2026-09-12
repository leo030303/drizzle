use crate::config::APP_ID;
use crate::modals::about::AboutDialog;
use crate::modals::city_picker::CityPickerDialog;
use crate::modals::city_picker::CityPickerDialogMsg;
use crate::modals::preferences::PreferencesDialog;
use crate::modals::shortcuts::ShortcutsDialog;
use crate::model::daily_entry::DailyEntry;
use crate::model::daily_entry::DailyEntryObject;
use crate::model::hourly_entry::HourlyEntry;
use crate::model::hourly_entry::HourlyEntryObject;
use crate::model::weather_rec::RecommendationTimespan;
use crate::model::weather_rec::get_recommendations;
use crate::ui::daily_entry_widget::DailyEntryWidget;
use crate::ui::hour_entry_widget::HourEntryWidget;
use crate::ui::weather_recommendation_widget::WeatherRecommendationWidget;
use crate::weather_api::find_city::GeoResponse;
use crate::weather_api::weather::CurrentWeather;
use crate::weather_api::weather::get_weather_current;
use crate::weather_api::weather::get_weather_daily;
use crate::weather_api::weather::get_weather_hourly;
use relm4::ComponentController;
use relm4::Controller;
use relm4::adw::prelude::AdwDialogExt;
use relm4::gtk::Accessible;
use relm4::gtk::ListItem;
use relm4::gtk::ListView;
use relm4::gtk::SignalListItemFactory;
use relm4::gtk::SingleSelection;
use relm4::gtk::accessible;
use relm4::gtk::gio::ListStore;
use relm4::gtk::gio::prelude::SettingsExtManual;
use relm4::gtk::glib::object::Cast;
use relm4::gtk::glib::object::CastNone;
use relm4::gtk::prelude::AccessibleExt;
use relm4::gtk::prelude::AccessibleExtManual;
use relm4::gtk::prelude::AdjustmentExt;
use relm4::gtk::prelude::ListItemExt;
use relm4::{
    Component, ComponentParts, ComponentSender, RelmWidgetExt,
    actions::{AccelsPlus, RelmAction, RelmActionGroup},
    adw,
    gtk::{
        self,
        glib::clone,
        prelude::{BoxExt, ButtonExt},
    },
    main_application,
    prelude::FactoryVecDeque,
};

use gtk::prelude::{ApplicationExt, GtkWindowExt, OrientableExt, SettingsExt, WidgetExt};
use gtk::{gio, glib};

pub struct App {
    is_loading: bool,
    show_no_wifi_error_message: bool,
    hourly_entries_store: ListStore,
    daily_entries_store: ListStore,
    weather_recommendations: FactoryVecDeque<WeatherRecommendationWidget>,
    recommendation_timespan_toggle: adw::ToggleGroup,
    current_weather: Option<CurrentWeather>,
    current_city: Option<GeoResponse>,
    city_search_dialog: Controller<CityPickerDialog>,
    recent_cities: Vec<GeoResponse>,
    hourly_scrolled_window: gtk::ScrolledWindow,
    daily_scrolled_window: gtk::ScrolledWindow,
}

#[derive(Debug)]
pub enum AppMsg {
    ShowCityPicker,
    SelectCity(GeoResponse),
    RefreshWeatherData,
    RefreshWeatherRecommendations,
    SetWeatherData(Vec<HourlyEntry>, Vec<DailyEntry>, CurrentWeather),
    ShowErrorPage,
    Quit,
}

relm4::new_action_group!(pub(super) WindowActionGroup, "win");
relm4::new_stateless_action!(PreferencesAction, WindowActionGroup, "preferences");
relm4::new_stateless_action!(pub(super) ShortcutsAction, WindowActionGroup, "show-help-overlay");
relm4::new_stateless_action!(AboutAction, WindowActionGroup, "about");
relm4::new_stateless_action!(QuitAction, WindowActionGroup, "quit");

#[relm4::component(pub)]
impl Component for App {
    type Init = ();
    type Input = AppMsg;
    type Output = ();
    type Widgets = AppWidgets;
    type CommandOutput = AppMsg;

    menu! {
        primary_menu: {
            section! {
                "_Preferences" => PreferencesAction,
                "_Keyboard" => ShortcutsAction,
                "_About Drizzle" => AboutAction,
            }
        }
    }

    view! {
        main_window = adw::ApplicationWindow::new(&main_application()) {
            set_visible: true,

            connect_close_request[sender] => move |_| {
                sender.input(AppMsg::Quit);
                glib::Propagation::Stop
            },

            if model.is_loading {

                adw::Spinner {
                    set_halign: gtk::Align::Center,
                    set_valign: gtk::Align::Center,
                    set_width_request: 64,
                    set_height_request: 64,
                }

            } else if model.show_no_wifi_error_message {
                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,

                    adw::HeaderBar {
                        pack_end = &gtk::MenuButton {
                            set_icon_name: "open-menu-symbolic",
                            update_property: &[accessible::Property::Label("Menu")],
                            set_menu_model: Some(&primary_menu),
                        }
                    },
                    adw::StatusPage {
                        set_icon_name: Some("radiowaves-none"),
                        set_title: "Error",
                        set_description: Some("Error retrieving weather data, check your internet connection"),
                        set_hexpand: true,
                        set_vexpand: true,
                        gtk::Button {
                            set_label: "Reload",
                            set_css_classes: &["pill", "suggested-action"],
                            set_halign: gtk::Align::Center,
                            connect_clicked => AppMsg::RefreshWeatherData,
                        }
                    }
                }
            } else if model.current_city.is_none() {
                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,

                    adw::HeaderBar {
                        pack_end = &gtk::MenuButton {
                            set_icon_name: "open-menu-symbolic",
                            update_property: &[accessible::Property::Label("Menu")],
                            set_menu_model: Some(&primary_menu),
                        }
                    },
                    adw::StatusPage {
                        set_icon_name: Some("system-search-symbolic"),
                        set_title: "No City Selected",
                        set_description: Some("Search to find your local city"),
                        set_hexpand: true,
                        set_vexpand: true,
                        #[local_ref]
                        none_selected_city_picker_button -> gtk::Button {
                            set_label: "Search",
                            set_css_classes: &["pill", "suggested-action"],
                            set_halign: gtk::Align::Center,
                            connect_clicked => AppMsg::ShowCityPicker,
                        }
                    }
                }
            } else {

                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,


                    adw::HeaderBar {
                        pack_start = &gtk::Button {
                            set_icon_name: "view-refresh-symbolic",
                            update_property: &[accessible::Property::Label("Refresh Weather")],
                            connect_clicked => AppMsg::RefreshWeatherData
                        },
                        pack_end = &gtk::MenuButton {
                            set_icon_name: "open-menu-symbolic",
                            update_property: &[accessible::Property::Label("Menu")],
                            set_menu_model: Some(&primary_menu),
                        }
                    },

                    gtk::ScrolledWindow {
                        set_vexpand: true,
                        set_policy: (gtk::PolicyType::Never, gtk::PolicyType::Automatic),
                        gtk::Box {
                            set_orientation: gtk::Orientation::Vertical,
                            gtk::Box {
                                #[watch]
                                set_css_classes: &[
                                    "card",
                                    "weather-card",
                                    model.current_weather.as_ref().map_or("", |current| current.weathercode.get_background_css_class(current.is_day))
                                ],
                                set_orientation: gtk::Orientation::Vertical,
                                set_margin_all: 10,
                                set_align: gtk::Align::Center,

                                gtk::Box {
                                    set_orientation: gtk::Orientation::Horizontal,
                                    gtk::Image {
                                        #[watch]
                                        set_resource: model.current_weather.as_ref().map(|current| current.weathercode.get_status_image_resource(current.is_day)),
                                        set_icon_size: gtk::IconSize::Inherit,
                                        set_pixel_size: 84,
                                        set_margin_all: 20,
                                    },
                                    gtk::Box {
                                        set_orientation: gtk::Orientation::Vertical,
                                        set_margin_top: 10,
                                        set_margin_end: 10,
                                        set_spacing: 10,
                                        #[local_ref]
                                        city_picker_button-> gtk::Button {
                                            set_align: gtk::Align::Center,
                                            connect_clicked[sender] => move |_| {
                                                sender.input(AppMsg::ShowCityPicker);
                                            },
                                            gtk::Box {
                                                set_orientation: gtk::Orientation::Horizontal,
                                                gtk::Image {
                                                    set_icon_name: Some("mark-location-symbolic"),
                                                    set_icon_size: gtk::IconSize::Normal,
                                                    set_margin_start: 5,
                                                    set_margin_end: 10,
                                                },
                                                gtk::Label {
                                                    #[watch]
                                                    set_label: &model.current_city.as_ref().map_or_else(|| String::from("Select A City"), |geo| geo.name.clone()),
                                                    set_margin_end: 5,
                                                    },

                                            },
                                        },
                                        gtk::Label {
                                            #[watch]
                                            set_label: &model.current_weather.as_ref().map(|current|
                                                format!(
                                                    "{}{}",
                                                    current.temperature_2m,
                                                    if current.is_metric {"℃"} else {"℉"}
                                                )
                                            ).unwrap_or_default(),
                                            set_css_classes: &["current-temp-label"],
                                        },
                                        gtk::Label {
                                            #[watch]
                                            set_label: &model.current_weather.as_ref().map(|current|
                                                format!(
                                                    "Feels like {}{}",
                                                    current.apparent_temperature,
                                                    if current.is_metric {"℃"} else {"℉"}
                                                )
                                            ).unwrap_or_default(),
                                            set_css_classes: &["current-apparent-temp-label"],
                                            set_margin_bottom: 10,
                                            set_margin_start: 30,
                                        },
                                    },
                                },
                                #[local_ref]
                                timespan_togglegroup -> adw::ToggleGroup {
                                    set_margin_horizontal: 5,
                                    connect_active_name_notify[sender] => move |_| {
                                        sender.input(AppMsg::RefreshWeatherRecommendations);
                                    },
                                    add = adw::Toggle {
                                        set_label: Some("4 Hour"),
                                        set_name: Some(RecommendationTimespan::FourHour.to_name())
                                    },
                                    add = adw::Toggle {
                                        set_label: Some("8 Hour"),
                                        set_name: Some(RecommendationTimespan::EightHour.to_name())
                                    },
                                    add = adw::Toggle {
                                        set_label: Some("12 Hour"),
                                        set_name: Some(RecommendationTimespan::TwelveHour.to_name())
                                    },
                                    add = adw::Toggle {
                                        set_label: Some("24 Hour"),
                                        set_name: Some(RecommendationTimespan::TwentyFourHour.to_name())
                                    },
                                },
                                #[local_ref]
                                weather_recommendations_box -> gtk::Box {
                                    set_orientation: gtk::Orientation::Vertical,
                                    set_margin_all: 10,
                                    set_spacing: 5,
                                }
                            },

                            #[name = "hourly_box"]
                            gtk::Box {
                                set_orientation: gtk::Orientation::Vertical,
                                set_focusable: true,
                                set_accessible_role: gtk::AccessibleRole::Group,

                                #[name = "hourly_label"]
                                gtk::Label {
                                    set_label: "Hourly Forecast",
                                    set_css_classes: &["title-1"],
                                },

                                #[local_ref]
                                hourly_scrolled_window -> gtk::ScrolledWindow {
                                    set_hexpand: true,
                                    set_policy: (gtk::PolicyType::Automatic, gtk::PolicyType::Never),

                                    #[local_ref]
                                    hourly_entry_list_view -> gtk::ListView {
                                        set_orientation: gtk::Orientation::Horizontal,
                                        set_margin_all: 10,
                                        set_tab_behavior: gtk::ListTabBehavior::Item,
                                    }
                                },
                            },

                            #[name = "daily_box"]
                            gtk::Box {
                                set_orientation: gtk::Orientation::Vertical,
                                set_focusable: true,
                                set_accessible_role: gtk::AccessibleRole::Group,

                                #[name = "daily_label"]
                                gtk::Label {
                                    set_label: "Daily Forecast",
                                    set_css_classes: &["title-1"],
                                },

                                #[local_ref]
                                daily_scrolled_window -> gtk::ScrolledWindow {
                                    set_hexpand: true,
                                    set_policy: (gtk::PolicyType::Automatic, gtk::PolicyType::Never),

                                    #[local_ref]
                                    daily_entry_list_view -> gtk::ListView {
                                        set_orientation: gtk::Orientation::Horizontal,
                                        set_margin_all: 10,
                                        set_tab_behavior: gtk::ListTabBehavior::Item,
                                    }
                                },
                            },

                            gtk::Label {
                                set_label: "Weather data from <a href='https://open-meteo.com/'>Open-Meteo</a>.",
                                set_wrap: true,
                                set_use_markup: true,
                                set_margin_vertical: 5,
                                set_css_classes: &["caption", "dim-label"],
                            },
                        }
                    }
                }
            }


        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let hourly_entries_store = gio::ListStore::new::<HourlyEntryObject>();
        let daily_entries_store = gio::ListStore::new::<DailyEntryObject>();
        let (hourly_entry_list_view, daily_entry_list_view) =
            init_list_views(hourly_entries_store.clone(), daily_entries_store.clone());
        let weather_recommendations: FactoryVecDeque<WeatherRecommendationWidget> =
            FactoryVecDeque::builder()
                .launch(gtk::Box::default())
                .detach();
        let mut model = Self {
            is_loading: false,
            show_no_wifi_error_message: false,
            hourly_entries_store,
            daily_entries_store,
            current_weather: None,
            current_city: None,
            city_search_dialog: CityPickerDialog::builder()
                .launch(())
                .forward(sender.input_sender(), |response| response),
            recent_cities: vec![],
            hourly_scrolled_window: gtk::ScrolledWindow::new(),
            daily_scrolled_window: gtk::ScrolledWindow::new(),
            recommendation_timespan_toggle: adw::ToggleGroup::new(),
            weather_recommendations,
        };
        let weather_recommendations_box = model.weather_recommendations.widget();
        let city_picker_button = gtk::Button::new();
        let none_selected_city_picker_button = gtk::Button::new();
        let hourly_scrolled_window = model.hourly_scrolled_window.clone();
        let daily_scrolled_window = model.daily_scrolled_window.clone();
        let timespan_togglegroup = model.recommendation_timespan_toggle.clone();

        model
            .recommendation_timespan_toggle
            .set_active_name(Some(RecommendationTimespan::FourHour.to_name()));
        let widgets = view_output!();
        widgets
            .hourly_box
            .update_relation(&[accessible::Relation::LabelledBy(&[widgets
                .hourly_label
                .upcast_ref::<Accessible>(
            )])]);
        widgets
            .daily_box
            .update_relation(&[accessible::Relation::LabelledBy(&[widgets
                .daily_label
                .upcast_ref::<Accessible>(
            )])]);

        let app = root.application().expect("Failed to get application");

        let actions = init_app_actions(&sender);
        // Connect action with hotkeys
        app.set_accelerators_for_action::<QuitAction>(&["<Control>q"]);
        actions.register_for_widget(&widgets.main_window);

        widgets.load_app_state(&mut model);

        sender.input(AppMsg::RefreshWeatherData);

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>, root: &Self::Root) {
        match message {
            AppMsg::RefreshWeatherData => {
                self.is_loading = true;
                self.show_no_wifi_error_message = false;
                if let Some(current_city) = self.current_city.clone() {
                    let settings = gio::Settings::new(APP_ID);
                    let is_metric = settings.boolean("use-metric");
                    sender.oneshot_command(async move {
                        let current_weather =
                            match get_weather_current(&current_city, is_metric).await {
                                Ok(weather) => weather,
                                Err(e) => {
                                    println!("Error loading weather data: {e}");
                                    return AppMsg::ShowErrorPage;
                                }
                            };
                        let hourly_entries =
                            match get_weather_hourly(&current_city, is_metric).await {
                                Ok(weather) => weather,
                                Err(e) => {
                                    println!("Error loading weather data: {e}");
                                    return AppMsg::ShowErrorPage;
                                }
                            };
                        let daily_entries = match get_weather_daily(&current_city, is_metric).await
                        {
                            Ok(weather) => weather,
                            Err(e) => {
                                println!("Error loading weather data: {e}");
                                return AppMsg::ShowErrorPage;
                            }
                        };

                        AppMsg::SetWeatherData(hourly_entries, daily_entries, current_weather)
                    });
                } else {
                    self.is_loading = false;
                }
            }
            AppMsg::ShowErrorPage => {
                self.is_loading = false;
                self.show_no_wifi_error_message = true;
            }
            AppMsg::Quit => main_application().quit(),
            AppMsg::RefreshWeatherRecommendations => {
                self.weather_recommendations.guard().clear();
                let hour_entries: Vec<HourlyEntry> = self
                    .hourly_entries_store
                    .into_iter()
                    .map(|item| {
                        item.expect("Never None")
                            .downcast::<HourlyEntryObject>()
                            .expect("Should be HourlyEntryObject")
                            .entry()
                    })
                    .collect();
                for rec in get_recommendations(
                    &hour_entries,
                    &RecommendationTimespan::from_name(
                        &self.recommendation_timespan_toggle.active_name()
                            .expect(
                                "No active name set on reccomendation timespan toggle, this shouldn't be possible",
                            ),
                    ),
                ) {
                    self.weather_recommendations.guard().push_back(rec);
                }
            }
            AppMsg::SetWeatherData(hour_entries, day_entries, current_weather) => {
                self.hourly_entries_store.remove_all();
                for entry in hour_entries {
                    self.hourly_entries_store
                        .append(&HourlyEntryObject::new(entry));
                }
                self.daily_entries_store.remove_all();
                for entry in day_entries {
                    self.daily_entries_store
                        .append(&DailyEntryObject::new(entry));
                }
                self.current_weather = Some(current_weather);
                self.is_loading = false;
                self.show_no_wifi_error_message = false;
                sender.input(AppMsg::RefreshWeatherRecommendations);
            }
            AppMsg::ShowCityPicker => {
                self.city_search_dialog
                    .emit(CityPickerDialogMsg::SetRecentCities(
                        self.recent_cities.clone(),
                    ));
                self.city_search_dialog.widget().present(Some(root));
            }
            AppMsg::SelectCity(selected_city) => {
                self.current_city = Some(selected_city.clone());
                self.hourly_scrolled_window
                    .hadjustment()
                    .set_value(self.hourly_scrolled_window.hadjustment().lower());
                self.daily_scrolled_window
                    .hadjustment()
                    .set_value(self.daily_scrolled_window.hadjustment().lower());
                if let Some(target_index) = self
                    .recent_cities
                    .iter()
                    .enumerate()
                    .find(|(_i, item)| **item == selected_city)
                {
                    self.recent_cities.remove(target_index.0);
                }
                self.recent_cities.insert(0, selected_city);
                if self.recent_cities.len() > 5 {
                    self.recent_cities.pop();
                }
                sender.input(AppMsg::RefreshWeatherData);
            }
        }
    }

    fn update_cmd(
        &mut self,
        message: Self::CommandOutput,
        sender: ComponentSender<Self>,
        root: &Self::Root,
    ) {
        self.update(message, sender, root);
    }

    fn shutdown(&mut self, widgets: &mut Self::Widgets, _output: relm4::Sender<Self::Output>) {
        widgets
            .save_app_state(self)
            .expect("A settings key has been set to readonly, please report this bug");
    }
}

fn init_app_actions(sender: &ComponentSender<App>) -> RelmActionGroup<WindowActionGroup> {
    let mut actions = RelmActionGroup::<WindowActionGroup>::new();

    let shortcuts_action = {
        RelmAction::<ShortcutsAction>::new_stateless(move |_| {
            ShortcutsDialog::builder().launch(()).detach();
        })
    };

    let about_action = {
        RelmAction::<AboutAction>::new_stateless(move |_| {
            AboutDialog::builder().launch(()).detach();
        })
    };

    let preferences_action = {
        RelmAction::<PreferencesAction>::new_stateless(clone!(
            #[strong]
            sender,
            move |_| {
                PreferencesDialog::builder()
                    .launch(())
                    .forward(sender.input_sender(), |response| response);
            },
        ))
    };

    let quit_action = {
        RelmAction::<QuitAction>::new_stateless(clone!(
            #[strong]
            sender,
            move |_| {
                sender.input(AppMsg::Quit);
            }
        ))
    };

    actions.add_action(shortcuts_action);
    actions.add_action(about_action);
    actions.add_action(preferences_action);
    actions.add_action(quit_action);

    actions
}

fn init_list_views(
    hourly_entries_store: ListStore,
    daily_entries_store: ListStore,
) -> (gtk::ListView, gtk::ListView) {
    let hourly_entry_factory = SignalListItemFactory::new();

    hourly_entry_factory.connect_bind(move |_, list_item| {
        let hourly_entry_object = list_item
            .downcast_ref::<ListItem>()
            .expect("Needs to be ListItem")
            .item()
            .and_downcast::<HourlyEntryObject>()
            .expect("The item has to be an `HourlyEntryObject`.");

        let hourly_entry_widget = HourEntryWidget::builder()
            .launch(hourly_entry_object.entry())
            .detach();
        list_item
            .downcast_ref::<ListItem>()
            .expect("Needs to be ListItem")
            .set_child(Some(&hourly_entry_widget.widget().clone()));
    });

    let hourly_selection_model = SingleSelection::new(Some(hourly_entries_store));

    let hourly_entry_list_view =
        ListView::new(Some(hourly_selection_model), Some(hourly_entry_factory));

    let daily_entry_factory = SignalListItemFactory::new();

    daily_entry_factory.connect_bind(move |_, list_item| {
        let daily_entry_object = list_item
            .downcast_ref::<ListItem>()
            .expect("Needs to be ListItem")
            .item()
            .and_downcast::<DailyEntryObject>()
            .expect("The item has to be an `DailyEntryObject`.");

        let daily_entry_widget = DailyEntryWidget::builder()
            .launch(daily_entry_object.entry())
            .detach();
        list_item
            .downcast_ref::<ListItem>()
            .expect("Needs to be ListItem")
            .set_child(Some(&daily_entry_widget.widget().clone()));
    });

    let daily_selection_model = SingleSelection::new(Some(daily_entries_store));

    let daily_entry_list_view =
        ListView::new(Some(daily_selection_model), Some(daily_entry_factory));

    (hourly_entry_list_view, daily_entry_list_view)
}

impl AppWidgets {
    fn save_app_state(&self, model: &App) -> Result<(), glib::BoolError> {
        let settings = gio::Settings::new(APP_ID);
        let (width, height) = self.main_window.default_size();

        settings.set_int("window-width", width)?;
        settings.set_int("window-height", height)?;

        settings.set_boolean("is-maximized", self.main_window.is_maximized())?;
        settings.set_string(
            "recommendation-timespan",
            &model.recommendation_timespan_toggle.active_name().expect(
                "No active name set on reccomendation timespan toggle, this shouldn't be possible",
            ),
        )?;
        settings.set_strv(
            "recent-cities",
            model
                .recent_cities
                .iter()
                .filter_map(|item| serde_json::to_string(item).ok())
                .collect::<Vec<String>>(),
        )?;

        Ok(())
    }

    fn load_app_state(&self, model: &mut App) {
        let settings = gio::Settings::new(APP_ID);
        let recent_cities: Vec<GeoResponse> = settings
            .strv("recent-cities")
            .iter()
            .filter_map(|item| {
                let deserialised: Option<GeoResponse> = serde_json::from_str(item).ok();
                deserialised
            })
            .collect();
        model.recent_cities = recent_cities;
        model.current_city = model.recent_cities.first().cloned();
        model
            .recommendation_timespan_toggle
            .set_active_name(Some(&settings.string("recommendation-timespan")));

        let width = settings.int("window-width");
        let height = settings.int("window-height");
        let is_maximized = settings.boolean("is-maximized");

        self.main_window.set_default_size(width, height);

        if is_maximized {
            self.main_window.maximize();
        }
    }
}
