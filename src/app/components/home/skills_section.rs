use leptos::prelude::*;

use crate::app::{components::home::skills::Skill, data::skills};

#[component]
pub fn SkillsSection() -> impl IntoView {
    let logos = skills()
        .into_iter()
        .map(|(skill_name, icon)| {
            view! { <Skill icon=icon.to_string() skill_name=skill_name.to_string() /> }
        })
        .collect::<Vec<_>>();

    view! {
        <div id="skill_section" class="scroll-mt-20 flex flex-col gap-8">
            <h1 class="text-3xl font-extrabold underline underline-offset-8">"Skills"</h1>

            <p class="text-justify">
                "I have acquired the set of skills for IC Layout. I'm knowledgeable both
                in hardware and software which is an asset for embedded systems or
                servers. Expert in diverse CI Platforms. Proficient in different
                compiled languages and skillful in different scripting languages."
            </p>

            <div class="grid grid-cols-2 md:grid-cols-4">{logos}</div>
        </div>
    }
}
