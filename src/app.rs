pub mod components;
pub mod pages;
pub mod router;
pub mod not_found;
pub mod data;

use crate::app::router::AppRouter;
use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    view! { <AppRouter /> }
}
