use leptos::{html, prelude::*};
use leptos_use::{UseIntersectionObserverOptions, use_intersection_observer_with_options};

// pub struct SkillProps {}

#[component]
pub fn Skill(icon: String, skill_name: String) -> impl IntoView {
    let visible = RwSignal::new(false);

    // Element reference
    let elem = NodeRef::<html::Div>::new();

    // Use intersection observer from leptos_use
    use_intersection_observer_with_options(
        elem,
        move |entries, _observer| {
            if !visible.get() && !entries.is_empty() {
                visible.set(entries.first().unwrap().is_intersecting());
            }
        },
        UseIntersectionObserverOptions::default().thresholds(vec![0.8]),
    );

    view! {
        <div
            node_ref=elem
            class="flex flex-col p-4 items-center"
        >
            <i class={
                move || format!(
                    "{} text-8xl p-2 {}",
                    icon,
                    if visible.get() {
                        "animate__animated animate__fadeInUp"
                    } else {
                        "opacity-0"
                    }
                )
            }>
            </i>

            <p>{ skill_name }</p>
        </div>
    }
}
