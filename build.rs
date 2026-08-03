fn main() {
    relm4_icons_build::bundle_icons(
        // Name of the file that will be generated at `OUT_DIR`
        "icon_names.rs",
        None,
        None::<&str>,
        None::<&str>,
        [
            "thunderstorm",
            "clear-day",
            "partly-cloudy-day",
            "cloudy",
            "foggy",
            "rainy",
            "snowing",
            "clear-night",
            "partly-cloudy-night",
        ],
    );
}
