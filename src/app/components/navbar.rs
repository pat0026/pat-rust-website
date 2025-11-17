use leptos::{ev::MouseEvent, html::Details, logging::log, prelude::*};
use leptos_router::components::A;

#[component]
pub fn Navbar() -> impl IntoView {
    let (is_open, set_is_open) = signal(false);
    let details_ref = NodeRef::<Details>::new();

    let toogle_details = move |ev: MouseEvent| {
        ev.prevent_default();
        log!("Toggle button clicked. Before: {}", is_open.get());
        set_is_open.update(|prev| *prev = !(*prev));
    };

    Effect::new(move || {
        if let Some(details) = details_ref.get() {
            details.set_open(is_open.get());
        }
    });

    let handle_menu_item_click = move |_| set_is_open.update(|prev| *prev = !*prev);

    view! {
        <div class="navbar bg-base-300 w-full fixed z-10">
            <div class="flex-none lg:hidden">
                <label for="my-drawer-3" aria-label="open sidebar" class="btn btn-square btn-ghost">
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        fill="none"
                        viewBox="0 0 24 24"
                        class="inline-block size-6 stroke-current"
                    >
                        <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            stroke-width="2"
                            d="M4 6h16M4 12h16M4 18h16"
                        ></path>
                    </svg>
                </label>
            </div>
            <div class="mx-2 flex-1 px-2">
                <A href="#" {..} class="btn btn-ghost no-animation text-xl hover:bg-transparent ">
                    "Patrick's Website"
                </A>
            </div>
            <div class="hidden flex-none lg:block">
                <ul class="menu menu-horizontal">
                    {} <li>
                        <details node_ref=details_ref>
                            <summary on:click=toogle_details>Sections</summary>
                            <ul>
                                <li>
                                    <A href="home#about_section" on:click=handle_menu_item_click>
                                        About
                                    </A>
                                </li>
                                <li>
                                    <A href="home#skill_section" on:click=handle_menu_item_click>
                                        Skills
                                    </A>
                                </li>
                                <li>
                                    <A
                                        href="home#background_section"
                                        on:click=handle_menu_item_click
                                    >
                                        Background
                                    </A>
                                </li>
                                <li>
                                    <A
                                        href="home#portfolio_section"
                                        on:click=handle_menu_item_click
                                    >
                                        Portfolio
                                    </A>
                                </li>
                                <li>
                                    <A href="home#services_section" on:click=handle_menu_item_click>
                                        Services
                                    </A>
                                </li>
                                <li>
                                    <A href="home#contact_section" on:click=handle_menu_item_click>
                                        Contacts
                                    </A>
                                </li>
                            </ul>
                        </details>
                    </li> <li>
                        <A href="/projects">Projects</A>
                    </li> <li>
                        <A href="/blogs">Blogs</A>
                    </li>
                </ul>
            </div>
        </div>
    }
}
