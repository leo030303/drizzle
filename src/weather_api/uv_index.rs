use std::fmt::Display;

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
            UvIndex::Low => f.write_str("Low"),
            UvIndex::Moderate => f.write_str("Moderate"),
            UvIndex::High => f.write_str("High"),
            UvIndex::VeryHigh => f.write_str("Very High"),
            UvIndex::Extreme => f.write_str("Extreme"),
        }
    }
}
