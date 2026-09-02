# leptos-capital-markers

Capital city markers for [leptos-leaflet](https://github.com/WyattAu/leptos-leaflet) — renders weather-aware markers on a Leaflet map.

## Features

- Renders color-coded circle markers for capital cities
- Temperature-based color coding (cold → blue, hot → red)
- Fetches weather data automatically
- Supports SSR and hydration
- Integrates with `leptos-leaflet` `Map` component

## Installation

```bash
cargo add leptos-capital-markers
```

## Quick Start

```rust
use leptos::prelude::*;
use leptos_leaflet::Map;
use leptos_capital_markers::CapitalLayer;

#[component]
fn WorldMap() -> impl IntoView {
    view! {
        <Map id="map" options=Default::default()>
            <CapitalLayer/>
        </Map>
    }
}
```

## API Reference

### CapitalLayer

| Prop | Type | Description |
|------|------|-------------|
| `visible` | `Signal<bool>` | Whether the layer is visible (default: `true`) |
| `capitals` | `Signal<Vec<CapitalCity>>` | Pre-loaded capital data (optional) |

### CapitalCity

```rust
pub struct CapitalCity {
    pub name: String,        // City name
    pub country: String,     // Country name
    pub iso2: String,        // ISO2 country code
    pub lat: f64,            // Latitude
    pub lng: f64,            // Longitude
    pub temperature: Option<f64>,  // Temperature (Celsius)
    pub weather_code: Option<i32>, // WMO weather code
}
```

### Temperature Colors

| Temperature | Color |
|-------------|-------|
| < 5°C | Blue (`#448aff`) |
| 5–15°C | Cyan (`#00e5ff`) |
| 15–25°C | Green (`#69f0ae`) |
| 25–35°C | Amber (`#ffab00`) |
| > 35°C | Red (`#ff5252`) |
| No data | Gray (`#888888`) |

## Features

```toml
[dependencies]
leptos-capital-markers = { version = "0.1", features = ["hydrate"] }
```

- `hydrate` (default) — Client-side hydration support
- `ssr` — Server-side rendering support

## License

MIT
