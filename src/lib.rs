//! # leptos-capital-markers
//!
//! Capital city markers for leptos-leaflet.
//!
//! Renders weather-aware markers on a Leaflet map for capital cities.
//! Fetches weather data and displays temperature with color coding.
//!
//! ## Usage
//!
//! ```rust,no_run
//! use leptos::prelude::*;
//! use leptos_leaflet::Map;
//! use leptos_capital_markers::CapitalLayer;
//!
//! #[component]
//! fn WorldMap() -> impl IntoView {
//!     view! {
//!         <Map id="map" options=Default::default()>
//!             <CapitalLayer/>
//!         </Map>
//!     }
//! }
//! ```

#![deny(missing_docs)]

mod types;
mod component;

pub use types::CapitalCity;
pub use component::CapitalLayer;
