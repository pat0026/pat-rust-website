use crate::app::components::home::{HeroSection, about_section::AboutSection, skills_section::SkillsSection};
use leptos::prelude::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div>
            <HeroSection />
            <div class=" p-4 mt-4 mx-auto lg:max-w-screen-lg md:max-w-screen-md  sm:max-w-screen-sm flex flex-col gap-2 ">
                <AboutSection />
                <div class="divider"></div>
                <SkillsSection />
                <div class="divider"></div>
            // <BackgroundSection />
            // <div class="divider"></div>
            // <PortfolioSection />
            // <div class="divider"></div>
            // <ServicesSection />
            // <div class="divider"></div>
            // <ContactSection />
            </div>
        </div>
    }
}
