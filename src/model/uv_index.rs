use std::fmt::Display;

use gettextrs::gettext;

#[derive(Debug, Clone)]
pub enum UvIndex {
    Low,
    Moderate,
    High,
    VeryHigh,
    Extreme,
}

impl From<f64> for UvIndex {
    fn from(value: f64) -> Self {
        if value < 3.0 {
            Self::Low
        } else if value < 6.0 {
            Self::Moderate
        } else if value < 8.0 {
            Self::High
        } else if value < 11.0 {
            Self::VeryHigh
        } else {
            Self::Extreme
        }
    }
}

impl Display for UvIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Low => f.write_str(&gettext("Low")),
            Self::Moderate => f.write_str(&gettext("Moderate")),
            Self::High => f.write_str(&gettext("High")),
            Self::VeryHigh => f.write_str(&gettext("Very High")),
            Self::Extreme => f.write_str(&gettext("Extreme")),
        }
    }
}
