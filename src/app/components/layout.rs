use leptos::prelude::*;
use leptos_router::components::Outlet;

use super::footer::Footer;
use super::navbar::Navbar;

#[component]
pub fn Layout() -> impl IntoView {
    let (is_sidebar_open, set_is_sidebar_open) = signal(false);

    let toggle_sidebar = move || set_is_sidebar_open.update(|prev| *prev = !*prev);
    let close_sidebar = move || set_is_sidebar_open.set(false);

    view! {
        <div>
            <div class="drawer ">
                // <input
                // id="my-drawer-3"
                // type="checkbox"
                // class="drawer-toggle"
                // checked=is_sidebar_open
                // on:change=toggle_sidebar
                // />
                <div class="drawer-content flex flex-col min-h-screen">
                    <Navbar />
                    <div class="pt-16 flex-1">
                        <Outlet />
                    </div>
                    <Footer />
                </div>
            // <div class="drawer-side z-20 ">
            // <label
            // for="my-drawer-3"
            // aria-label="close sidebar"
            // class="drawer-overlay "
            // ></label>
            // <Sidebar closeSidebar=closeSidebar />
            // </div>
            </div>
        </div>
    }
}
