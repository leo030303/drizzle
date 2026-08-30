use adw::prelude::AdwDialogExt;
use gtk::prelude::GtkApplicationExt;
use relm4::{
    ComponentParts, ComponentSender, RelmWidgetExt, SimpleComponent,
    adw::{
        self,
        prelude::{ActionRowExt, PreferencesDialogExt, PreferencesGroupExt, PreferencesPageExt},
    },
    gtk::{
        self,
        gio::{self, prelude::SettingsExt},
    },
};

use crate::{app::AppMsg, config::APP_ID};

pub struct PreferencesDialog {}

impl SimpleComponent for PreferencesDialog {
    type Init = ();
    type Widgets = adw::PreferencesDialog;
    type Input = ();
    type Output = AppMsg;
    type Root = adw::PreferencesDialog;

    fn init_root() -> Self::Root {
        adw::PreferencesDialog::new()
    }

    fn init(
        _: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self {};

        let settings = gio::Settings::new(APP_ID);
        let preferences_page = adw::PreferencesPage::new();
        let preferences_group = adw::PreferencesGroup::new();
        preferences_group.add(&init_unit_row(&settings, sender));
        preferences_page.add(&preferences_group);
        root.add(&preferences_page);

        let widgets = root.clone();
        widgets.present(Some(&relm4::main_application().windows()[0]));

        ComponentParts { model, widgets }
    }

    fn update_view(&self, _dialog: &mut Self::Widgets, _sender: ComponentSender<Self>) {}
}

fn init_unit_row(
    settings: &gio::Settings,
    sender: ComponentSender<PreferencesDialog>,
) -> adw::ActionRow {
    let row = adw::ActionRow::builder()
        .title("Units")
        .subtitle("Which units to use")
        .build();
    let toggle_group = adw::ToggleGroup::new();
    toggle_group.add(
        adw::Toggle::builder()
            .label("Metric")
            .name("metric")
            .build(),
    );
    toggle_group.add(
        adw::Toggle::builder()
            .label("Imperial")
            .name("imperial")
            .build(),
    );
    toggle_group.set_margin_all(2);
    row.add_suffix(&toggle_group);
    toggle_group.set_active_name(if settings.boolean("use-metric") {
        Some("metric")
    } else {
        Some("imperial")
    });
    let settings_handle = settings.clone();

    toggle_group.connect_active_name_notify(move |group| {
        if group.active_name() == Some("metric".into()) {
            settings_handle.set_boolean("use-metric", true).unwrap();
        } else {
            settings_handle.set_boolean("use-metric", false).unwrap();
        }
        sender.output(AppMsg::RefreshWeatherData).unwrap();
    });
    row
}
