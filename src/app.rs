pub mod components;
pub mod pages;
pub mod router;

use crate::app::router::AppRouter;
use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    view! { <AppRouter /> }
}
