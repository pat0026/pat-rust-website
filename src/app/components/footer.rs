use chrono::{Datelike, Local};
use leptos::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        // <footer class="footer footer-center bg-base-300 text-base-content p-4 mt-4">
        <footer class="footer footer-center bg-base-300 text-base-content p-4">
            <aside>
                <p>
                    {format!(
                        "Copyright © {} - All rights reserved by Patrick Christian Caparros",
                        Local::now().year(),
                    )}
                </p>
            </aside>
        </footer>
    }
}
