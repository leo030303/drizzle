use crate::config::APP_ID;
use crate::modals::about::AboutDialog;
use crate::modals::city_picker::CityPickerDialog;
use crate::modals::city_picker::CityPickerDialogMsg;
use crate::modals::shortcuts::ShortcutsDialog;
use crate::ui::daily_entry_widget::DayEntryWidget;
use crate::ui::hour_entry_widget::HourEntryWidget;
use crate::weather_api::find_city::GeoResponse;
use crate::weather_api::weather::CurrentWeather;
use crate::weather_api::weather::DailyEntry;
use crate::weather_api::weather::HourlyEntry;
use crate::weather_api::weather::get_weather_current;
use crate::weather_api::weather::get_weather_daily;
use crate::weather_api::weather::get_weather_hourly;
use relm4::ComponentController;
use relm4::Controller;
use relm4::adw::prelude::AdwDialogExt;
use relm4::gtk::gio::prelude::SettingsExtManual;
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
    hourly_entries: FactoryVecDeque<HourEntryWidget>,
    daily_entries: FactoryVecDeque<DayEntryWidget>,
    current_weather: Option<CurrentWeather>,
    current_city: Option<GeoResponse>,
    city_search_dialog: Controller<CityPickerDialog>,
    recent_cities: Vec<GeoResponse>,
}

#[derive(Debug)]
pub enum AppMsg {
    ShowCityPicker,
    SelectCity(GeoResponse),
    RefreshWeatherData,
    SetWeatherData(Vec<HourlyEntry>, Vec<DailyEntry>, CurrentWeather),
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

            } else if model.current_city.is_none() {
                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,

                    adw::HeaderBar {
                        pack_end = &gtk::MenuButton {
                            set_icon_name: "open-menu-symbolic",
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
                            add_css_class: "pill",
                            add_css_class: "suggested-action",
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
                            connect_clicked => AppMsg::RefreshWeatherData
                        },
                        pack_end = &gtk::MenuButton {
                            set_icon_name: "open-menu-symbolic",
                            set_menu_model: Some(&primary_menu),
                        }
                    },

                    gtk::ScrolledWindow {
                        set_vexpand: true,
                        set_policy: (gtk::PolicyType::Never, gtk::PolicyType::Automatic),
                        gtk::Box {
                            set_orientation: gtk::Orientation::Vertical,
                            gtk::Box {
                                set_orientation: gtk::Orientation::Horizontal,
                                set_margin_top: 10,

                                gtk::Image {
                                    #[watch]
                                    set_icon_name: model.current_weather.as_ref().map(|current| current.weathercode.get_icon_name(current.is_day)),
                                    set_icon_size: gtk::IconSize::Inherit,
                                    set_pixel_size: 84,
                                    set_margin_all: 20,
                                },
                                gtk::Box {
                                    set_orientation: gtk::Orientation::Vertical,
                                    set_spacing: 10,
                                    #[local_ref]
                                    city_picker_button-> gtk::Button {
                                        connect_clicked[sender] => move |_| {
                                            sender.input(AppMsg::ShowCityPicker);
                                        },
                                        gtk::Box {
                                            set_orientation: gtk::Orientation::Horizontal,
                                            gtk::Image {
                                                set_icon_name: Some("mark-location-symbolic"),
                                                set_icon_size: gtk::IconSize::Normal,
                                                set_margin_horizontal: 10,
                                            },
                                            gtk::Label {
                                                #[watch]
                                                set_label: &model.current_city.as_ref().map(|geo| geo.name.clone()).unwrap_or(String::from("Select A City")),
                                            },

                                        },
                                    },
                                    gtk::Label {
                                        #[watch]
                                        set_label: &format!("{}℃", model.current_weather.as_ref().map(|current| current.temperature_2m.to_string()).unwrap_or_default()),
                                        add_css_class: "current-temp-label",
                                    },
                                    gtk::Label {
                                        #[watch]
                                        set_label: &format!("Feels like {}℃", model.current_weather.as_ref().map(|current| current.apparent_temperature.to_string()).unwrap_or_default()),
                                        add_css_class: "current-apparent-temp-label",
                                        set_margin_bottom: 10,
                                        set_margin_start: 30,
                                    },
                                },
                            },

                            gtk::Label {
                                set_label: "Hourly",
                                add_css_class: "title-1",
                            },

                            gtk::ScrolledWindow {
                                set_hexpand: true,
                                set_policy: (gtk::PolicyType::Automatic, gtk::PolicyType::Never),

                                #[local_ref]
                                hourly_box -> gtk::Box {
                                    set_orientation: gtk::Orientation::Horizontal,
                                    set_spacing: 5,
                                    set_margin_all: 10,
                                }
                            },

                            gtk::Label {
                                set_label: "Daily",
                                add_css_class: "title-1",
                            },

                            gtk::ScrolledWindow {
                                set_hexpand: true,
                                set_policy: (gtk::PolicyType::Automatic, gtk::PolicyType::Never),

                                #[local_ref]
                                daily_box -> gtk::Box {
                                    set_orientation: gtk::Orientation::Horizontal,
                                    set_spacing: 5,
                                    set_margin_all: 10,
                                }
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
        let hourly_entries: FactoryVecDeque<HourEntryWidget> = FactoryVecDeque::builder()
            .launch(gtk::Box::default())
            .forward(sender.input_sender(), |output| output);
        let daily_entries: FactoryVecDeque<DayEntryWidget> = FactoryVecDeque::builder()
            .launch(gtk::Box::default())
            .forward(sender.input_sender(), |output| output);
        let mut model = Self {
            is_loading: false,
            hourly_entries,
            daily_entries,
            current_weather: None,
            current_city: None,
            city_search_dialog: CityPickerDialog::builder()
                .launch(())
                .forward(sender.input_sender(), |response| response),
            recent_cities: vec![],
        };
        let hourly_box = model.hourly_entries.widget();
        let daily_box = model.daily_entries.widget();
        let city_picker_button = gtk::Button::new();
        let none_selected_city_picker_button = gtk::Button::new();
        let widgets = view_output!();

        let app = root.application().unwrap();
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

        let quit_action = {
            RelmAction::<QuitAction>::new_stateless(clone!(
                #[strong]
                sender,
                move |_| {
                    sender.input(AppMsg::Quit);
                }
            ))
        };

        // Connect action with hotkeys
        app.set_accelerators_for_action::<QuitAction>(&["<Control>q"]);

        actions.add_action(shortcuts_action);
        actions.add_action(about_action);
        actions.add_action(quit_action);
        actions.register_for_widget(&widgets.main_window);

        widgets.load_app_state(&mut model);

        sender.input(AppMsg::RefreshWeatherData);

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>, root: &Self::Root) {
        match message {
            AppMsg::RefreshWeatherData => {
                self.is_loading = true;
                if let Some(current_city) = self.current_city.clone() {
                    sender.oneshot_command(async move {
                        let is_metric = true;
                        let current_weather =
                            get_weather_current(&current_city, is_metric).await.unwrap();
                        let hourly_entries =
                            get_weather_hourly(&current_city, is_metric).await.unwrap();
                        let daily_entries =
                            get_weather_daily(&current_city, is_metric).await.unwrap();
                        AppMsg::SetWeatherData(hourly_entries, daily_entries, current_weather)
                    });
                } else {
                    self.is_loading = false;
                }
            }
            AppMsg::Quit => main_application().quit(),
            AppMsg::SetWeatherData(hour_entries, day_entries, current_weather) => {
                self.hourly_entries.guard().clear();
                for entry in hour_entries {
                    self.hourly_entries.guard().push_back(entry);
                }
                self.daily_entries.guard().clear();
                for entry in day_entries {
                    self.daily_entries.guard().push_back(entry);
                }
                self.current_weather = Some(current_weather);
                self.is_loading = false;
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
                if !self.recent_cities.contains(&selected_city) {
                    self.recent_cities.insert(0, selected_city);
                }
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
        widgets.save_app_state(self).unwrap();
    }
}

impl AppWidgets {
    fn save_app_state(&self, model: &App) -> Result<(), glib::BoolError> {
        let settings = gio::Settings::new(APP_ID);
        let (width, height) = self.main_window.default_size();

        settings.set_int("window-width", width)?;
        settings.set_int("window-height", height)?;

        settings.set_boolean("is-maximized", self.main_window.is_maximized())?;
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

        let width = settings.int("window-width");
        let height = settings.int("window-height");
        let is_maximized = settings.boolean("is-maximized");

        self.main_window.set_default_size(width, height);

        if is_maximized {
            self.main_window.maximize();
        }
    }
}
