use leptos::prelude::*;
use leptos_router::{
    components::{ParentRoute, Redirect, Route, Router, Routes},
    path,
};

use super::components::ProgressBar;
use super::pages::home_page::HomePage;

use crate::app::{
    components::{under_development::UnderDevelopment, Layout},
    not_found::NotFound,
};

#[component]
pub fn AppRouter() -> impl IntoView {
    view! {
        <Router>
            <Routes fallback=NotFound>
                <ParentRoute path=path!("/") view=Layout>
                    <Route path=path!("") view=|| view! { <Redirect path="/home" /> } />
                    <Route path=path!("/home") view=HomePage />
                    <Route path=path!("/blogs") view=UnderDevelopment />
                    <Route path=path!("/projects") view=UnderDevelopment />
                </ParentRoute>
                <Route path=path!("/*") view=NotFound />
            </Routes>
        </Router>
    }
}
