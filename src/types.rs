//! Capital city types.

use serde::Deserialize;

/// A capital city with coordinates and optional weather data.
#[derive(Clone, Debug, Default, Deserialize)]
pub struct CapitalCity {
    /// City name
    pub name: String,
    /// Country name
    pub country: String,
    /// ISO2 country code
    pub iso2: String,
    /// Latitude
    pub lat: f64,
    /// Longitude
    pub lng: f64,
    /// Current temperature (Celsius)
    pub temperature: Option<f64>,
    /// Weather code (WMO)
    pub weather_code: Option<i32>,
}

impl CapitalCity {
    /// Get a color based on temperature.
    pub fn temp_color(&self) -> &'static str {
        match self.temperature {
            Some(t) if t < 5.0 => "#448aff",   // blue - cold
            Some(t) if t < 15.0 => "#00e5ff",  // cyan - cool
            Some(t) if t < 25.0 => "#69f0ae",  // green - mild
            Some(t) if t < 35.0 => "#ffab00",  // amber - warm
            Some(_) => "#ff5252",               // red - hot
            None => "#888888",                   // gray - no data
        }
    }

    /// Get a display string for the marker tooltip.
    pub fn tooltip_text(&self) -> String {
        match self.temperature {
            Some(t) => format!("{}: {:.0}\u{00b0}C", self.name, t),
            None => self.name.clone(),
        }
    }
}
