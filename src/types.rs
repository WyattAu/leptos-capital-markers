//! Capital city types.

use serde::Deserialize;

/// A capital city with coordinates and optional weather data.
#[derive(Clone, Debug, Default, Deserialize, serde::Serialize)]
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

#[cfg(test)]
mod tests {
    use super::*;

    fn make_city(name: &str, temp: Option<f64>) -> CapitalCity {
        CapitalCity {
            name: name.into(),
            country: "Test".into(),
            iso2: "TT".into(),
            lat: 0.0,
            lng: 0.0,
            temperature: temp,
            weather_code: None,
        }
    }

    #[test]
    fn temp_color_cold() {
        assert_eq!(make_city("A", Some(0.0)).temp_color(), "#448aff");
        assert_eq!(make_city("A", Some(-10.0)).temp_color(), "#448aff");
        assert_eq!(make_city("A", Some(4.9)).temp_color(), "#448aff");
    }

    #[test]
    fn temp_color_cool() {
        assert_eq!(make_city("A", Some(5.0)).temp_color(), "#00e5ff");
        assert_eq!(make_city("A", Some(10.0)).temp_color(), "#00e5ff");
        assert_eq!(make_city("A", Some(14.9)).temp_color(), "#00e5ff");
    }

    #[test]
    fn temp_color_mild() {
        assert_eq!(make_city("A", Some(15.0)).temp_color(), "#69f0ae");
        assert_eq!(make_city("A", Some(20.0)).temp_color(), "#69f0ae");
        assert_eq!(make_city("A", Some(24.9)).temp_color(), "#69f0ae");
    }

    #[test]
    fn temp_color_warm() {
        assert_eq!(make_city("A", Some(25.0)).temp_color(), "#ffab00");
        assert_eq!(make_city("A", Some(30.0)).temp_color(), "#ffab00");
        assert_eq!(make_city("A", Some(34.9)).temp_color(), "#ffab00");
    }

    #[test]
    fn temp_color_hot() {
        assert_eq!(make_city("A", Some(35.0)).temp_color(), "#ff5252");
        assert_eq!(make_city("A", Some(50.0)).temp_color(), "#ff5252");
    }

    #[test]
    fn temp_color_none() {
        assert_eq!(make_city("A", None).temp_color(), "#888888");
    }

    #[test]
    fn tooltip_with_temp() {
        let city = make_city("London", Some(12.3));
        assert_eq!(city.tooltip_text(), "London: 12\u{00b0}C");
    }

    #[test]
    fn tooltip_without_temp() {
        let city = make_city("Paris", None);
        assert_eq!(city.tooltip_text(), "Paris");
    }

    #[test]
    fn tooltip_negative_temp() {
        let city = make_city("Reykjavik", Some(-5.0));
        assert_eq!(city.tooltip_text(), "Reykjavik: -5\u{00b0}C");
    }

    #[test]
    fn default_values() {
        let city = CapitalCity::default();
        assert!(city.name.is_empty());
        assert!(city.country.is_empty());
        assert!(city.temperature.is_none());
    }

    #[test]
    fn serde_roundtrip() {
        let city = make_city("Tokyo", Some(22.5));
        let json = serde_json::to_string(&city).unwrap();
        let deserialized: CapitalCity = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.name, "Tokyo");
        assert_eq!(deserialized.temperature, Some(22.5));
    }
}
