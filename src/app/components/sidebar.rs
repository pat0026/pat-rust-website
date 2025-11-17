use leptos::{ev::MouseEvent, prelude::*};
use leptos_router::components::A;

#[component]
pub fn Sidebar(on_close: impl FnMut(MouseEvent) + Clone + 'static) -> impl IntoView {
    view! {
        <ul class="menu bg-base-200 min-h-full w-72 space-y-6 ">
            <div class="avatar mt-12 self-center">
                <div class="ring-primary ring-offset-base-100 size-36 rounded-full ring-offset-2 ring-4">
                    <img src="images/RESUME.jpg" alt="Resume 2x2" />
                </div>
            </div>
            <div class="flex flex-col mx-auto gap-2">
                <h1 class="mx-auto">Patrick Christain Caparros</h1>
                <div class="flex mx-auto gap-4 items-center">
                    <a href="https://www.facebook.com/pat00026 " target="_blank">
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            viewBox="0 0 320 512"
                            class="rounded-full border-2 size-8 p-1 border-primary fill-current"
                        >
                            <path d="M80 299.3V512H196V299.3h86.5l18-97.8H196V166.9c0-51.7 20.3-71.5 72.7-71.5c16.3 0 29.4 .4 37 1.2V7.9C291.4 4 256.4 0 236.2 0C129.3 0 80 50.5 80 159.4v42.1H14v97.8H80z" />
                        </svg>
                    </a>
                    <a href="https://www.linkedin.com/in/patrick-caparros" target="_blank">
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            viewBox="0 0 448 512"
                            class="rounded-full border-2 size-8  p-1 border-primary fill-current"
                        >
                            <path d="M100.3 448H7.4V148.9h92.9zM53.8 108.1C24.1 108.1 0 83.5 0 53.8a53.8 53.8 0 0 1 107.6 0c0 29.7-24.1 54.3-53.8 54.3zM447.9 448h-92.7V302.4c0-34.7-.7-79.2-48.3-79.2-48.3 0-55.7 37.7-55.7 76.7V448h-92.8V148.9h89.1v40.8h1.3c12.4-23.5 42.7-48.3 87.9-48.3 94 0 111.3 61.9 111.3 142.3V448z" />
                        </svg>
                    </a>
                </div>
            </div>
            <div>
                <li>
                    <h2 class="menu-title">Sections</h2>
                    <ul>
                        <li>
                            <A href="home#about_section" on:click=on_close.clone()>
                                About
                            </A>
                        </li>
                        <li>
                            <A href="home#skill_section" on:click=on_close.clone()>
                                Skills
                            </A>
                        </li>
                        <li>
                            <A href="home#background_section" on:click=on_close.clone()>
                                Background
                            </A>
                        </li>
                        <li>
                            <A href="home#portfolio_section" on:click=on_close.clone()>
                                Portfolio
                            </A>
                        </li>
                        <li>
                            <A href="home#services_section" on:click=on_close.clone()>
                                Services
                            </A>
                        </li>
                        <li>
                            <A href="home#contact_section" on:click=on_close.clone()>
                                Contacts
                            </A>
                        </li>
                    </ul>
                </li>

                <li>
                    <A href="projects" on:click=on_close.clone()>
                        Projects
                    </A>
                </li>
                <li>
                    <A href="blogs" on:click=on_close.clone()>
                        Blogs
                    </A>
                </li>
            </div>
        </ul>
    }
}
