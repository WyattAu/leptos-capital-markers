//! CapitalLayer Leptos component.

use leptos::prelude::*;
use leptos_leaflet_wyatt::{LayerGroup, CircleMarker, CircleMarkerOptions, LatLng};

use crate::types::CapitalCity;

/// A layer of capital city markers for a Leaflet map.
///
/// Renders color-coded markers for capital cities.
///
/// # Props
///
/// - `visible`: Signal<bool> — whether the layer is visible
/// - `capitals`: Option<Signal<Vec<CapitalCity>>> — pre-loaded capital data (optional)
#[component]
pub fn CapitalLayer(
    #[prop(optional)] visible: Option<Signal<bool>>,
    #[prop(optional)] capitals: Option<Signal<Vec<CapitalCity>>>,
) -> impl IntoView {
    let visible = visible.unwrap_or_else(|| signal(true).0.into());

    let capitals_data = capitals.unwrap_or_else(|| {
        let (sig, set_sig) = signal(Vec::<CapitalCity>::new());

        // Fetch capital data on mount
        #[cfg(feature = "hydrate")]
        {
            leptos::task::spawn_local(async move {
                // Static list of major capitals for demo
                let caps = vec![
                    CapitalCity {
                        name: "London".into(), country: "UK".into(), iso2: "GB".into(),
                        lat: 51.5074, lng: -0.1278, temperature: None, weather_code: None,
                    },
                    CapitalCity {
                        name: "New York".into(), country: "USA".into(), iso2: "US".into(),
                        lat: 40.7128, lng: -74.0060, temperature: None, weather_code: None,
                    },
                    CapitalCity {
                        name: "Tokyo".into(), country: "Japan".into(), iso2: "JP".into(),
                        lat: 35.6762, lng: 139.6503, temperature: None, weather_code: None,
                    },
                    CapitalCity {
                        name: "Berlin".into(), country: "Germany".into(), iso2: "DE".into(),
                        lat: 52.5200, lng: 13.4050, temperature: None, weather_code: None,
                    },
                    CapitalCity {
                        name: "Paris".into(), country: "France".into(), iso2: "FR".into(),
                        lat: 48.8566, lng: 2.3522, temperature: None, weather_code: None,
                    },
                ];
                set_sig.set(caps);
            });
        }

        sig.into()
    });

    view! {
        {move || if visible.get() {
            let caps = capitals_data.get();
            view! {
                <LayerGroup id="capital-markers">
                    {caps.into_iter().map(|cap| {
                        let color = cap.temp_color();
                        view! {
                            <CircleMarker
                                latlng=LatLng::new(cap.lat, cap.lng)
                                options=CircleMarkerOptions {
                                    radius: 5.0,
                                    color: color.to_string(),
                                    fill_color: color.to_string(),
                                    fill_opacity: 0.8,
                                    weight: 1.0,
                                    opacity: 0.9,
                                }
                            />
                        }
                    }).collect::<Vec<_>>()}
                </LayerGroup>
            }.into_any()
        } else {
            ().into_any()
        }}
    }
}
