use leptos::prelude::*;
use leptos_router::{
    components::{ParentRoute, Redirect, Route, Router, Routes},
    path,
};

use super::components::ProgressBar;
use super::pages::home_page::HomePage;

use crate::app::components::Layout;

#[component]
pub fn AppRouter() -> impl IntoView {
    view! {
        <Router>
            <Routes fallback=|| "Not found.">
                <ParentRoute path=path!("/") view=Layout>
                    <Route path=path!("") view=|| view! { <Redirect path="/home" /> } />
                    <Route path=path!("/home") view=HomePage />
                </ParentRoute>
            </Routes>
        </Router>
    }
}
