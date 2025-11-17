use crate::app::components::home::HeroSection;
use leptos::prelude::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div>
            <HeroSection />
        </div>
    }
}
