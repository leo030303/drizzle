use crate::config::APP_ID;
use crate::modals::about::AboutDialog;
use crate::modals::shortcuts::ShortcutsDialog;
use crate::ui::daily_entry_widget::DayEntryWidget;
use crate::ui::hour_entry_widget::HourEntryWidget;
use crate::weather_api::weather::CurrentWeather;
use crate::weather_api::weather::DayEntry;
use crate::weather_api::weather::HourlyEntry;
use crate::weather_api::weather::WeatherCode;
use crate::weather_api::{self, weather::WeatherApi};
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
}

#[derive(Debug)]
pub enum AppMsg {
    RefreshWeatherData,
    SetWeatherData(Vec<HourlyEntry>, Vec<DayEntry>, CurrentWeather),
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

            } else {

                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    #[watch]
                    add_css_class?: model.current_weather.as_ref().map(|current_weather| WeatherCode::try_from(current_weather.weathercode).unwrap().get_background_css_class()),


                    adw::HeaderBar {
                        add_css_class: "bg-transparency",
                        pack_start = &gtk::Button {
                            set_icon_name: "view-refresh-symbolic",
                            connect_clicked => AppMsg::RefreshWeatherData
                        },
                        pack_end = &gtk::MenuButton {
                            set_icon_name: "open-menu-symbolic",
                            set_menu_model: Some(&primary_menu),
                        }
                    },

                    gtk::Box {
                        set_orientation: gtk::Orientation::Horizontal,
                        set_margin_top: 10,

                        gtk::Image {
                            #[watch]
                            set_icon_name: model.current_weather.as_ref().and_then(|current| WeatherCode::try_from(current.weathercode).ok().map(|weather_code| weather_code.get_icon_name(current.is_day == 1))),
                            set_icon_size: gtk::IconSize::Inherit,
                            set_pixel_size: 84,
                            set_margin_all: 20,
                        },
                        gtk::Box {
                            set_orientation: gtk::Orientation::Vertical,
                            set_spacing: 10,
                            gtk::ToggleButton {
                                add_css_class: "text-button",
                                add_css_class: "arrow-button",
                                gtk::Box {
                                    set_orientation: gtk::Orientation::Horizontal,
                                    gtk::Label {
                                        #[watch]
                                        set_label: "Cork, Ireland",
                                    },
                                    gtk::Image {
                                        set_icon_name: Some("down"),
                                        set_icon_size: gtk::IconSize::Normal,
                                        set_margin_horizontal: 10,
                                    },

                                },
                            },
                            gtk::Label {
                                #[watch]
                                set_label: &format!("{}℃", model.current_weather.as_ref().map(|current| current.temperature_2m.to_string()).unwrap_or_default()),
                                add_css_class: "current-temp-label",
                            },
                        },
                    },

                    gtk::Label {
                        set_label: "Hourly",
                        add_css_class: "title-1",
                    },

                    gtk::ScrolledWindow {
                        set_hexpand: true,
                        set_propagate_natural_width: false,
                        set_policy: (gtk::PolicyType::Automatic, gtk::PolicyType::Never),

                        #[local_ref]
                        hourly_box -> gtk::Box {
                            set_orientation: gtk::Orientation::Horizontal,
                            set_spacing: 5,
                            set_margin_all: 5,
                        }
                    },

                    gtk::Label {
                        set_label: "Daily",
                        add_css_class: "title-1",
                    },

                    gtk::ScrolledWindow {
                        set_hexpand: true,
                        set_propagate_natural_width: false,
                        set_policy: (gtk::PolicyType::Automatic, gtk::PolicyType::Never),

                        #[local_ref]
                        daily_box -> gtk::Box {
                            set_orientation: gtk::Orientation::Horizontal,
                            set_spacing: 5,
                            set_margin_all: 5,
                        }
                    },
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
        let model = Self {
            is_loading: false,
            hourly_entries,
            daily_entries,
            current_weather: None,
        };
        let hourly_box = model.hourly_entries.widget();
        let daily_box = model.daily_entries.widget();
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

        widgets.load_window_size();

        sender.input(AppMsg::RefreshWeatherData);

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>, _root: &Self::Root) {
        match message {
            AppMsg::RefreshWeatherData => {
                self.is_loading = true;
                sender.oneshot_command(async move {
                    let weather_api = WeatherApi::init(true);
                    let weather_results = weather_api
                        .get_weather(
                            weather_api::weather::CityCoordinates {
                                latitude: 51.908481,
                                longitude: -8.475720,
                            },
                            weather_api::weather::ForecastTimeframe::Daily,
                        )
                        .await;
                    let daily_entries = weather_results.unwrap().daily.unwrap().to_entries();
                    let weather_results = weather_api
                        .get_weather(
                            weather_api::weather::CityCoordinates {
                                latitude: 51.908481,
                                longitude: -8.475720,
                            },
                            weather_api::weather::ForecastTimeframe::Hourly,
                        )
                        .await;
                    let hourly_entries = weather_results.unwrap().hourly.unwrap().to_entries();
                    let weather_results = weather_api
                        .get_weather(
                            weather_api::weather::CityCoordinates {
                                latitude: 51.908481,
                                longitude: -8.475720,
                            },
                            weather_api::weather::ForecastTimeframe::Current,
                        )
                        .await;
                    let current_weather = weather_results.unwrap().current.unwrap();
                    AppMsg::SetWeatherData(hourly_entries, daily_entries, current_weather)
                });
            }
            AppMsg::Quit => main_application().quit(),
            AppMsg::SetWeatherData(hour_entries, day_entries, current_weather) => {
                for entry in hour_entries {
                    self.hourly_entries.guard().push_back(entry);
                }
                for entry in day_entries {
                    self.daily_entries.guard().push_back(entry);
                }
                self.current_weather = Some(current_weather);
                self.is_loading = false;
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
        widgets.save_window_size().unwrap();
    }
}

impl AppWidgets {
    fn save_window_size(&self) -> Result<(), glib::BoolError> {
        let settings = gio::Settings::new(APP_ID);
        let (width, height) = self.main_window.default_size();

        settings.set_int("window-width", width)?;
        settings.set_int("window-height", height)?;

        settings.set_boolean("is-maximized", self.main_window.is_maximized())?;

        Ok(())
    }

    fn load_window_size(&self) {
        let settings = gio::Settings::new(APP_ID);

        let width = settings.int("window-width");
        let height = settings.int("window-height");
        let is_maximized = settings.boolean("is-maximized");

        self.main_window.set_default_size(width, height);

        if is_maximized {
            self.main_window.maximize();
        }
    }
}
