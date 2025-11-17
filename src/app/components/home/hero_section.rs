use leptos::prelude::*;

#[component]
pub fn HeroSection() -> impl IntoView {
    view! {
        <div class="hero min-h-screen bg-[url(/images/Portfolio_bg.jpg)] bg-fixed bg-[right_-4rem_center]  md:bg-center">
            <div class="hero-overlay  bg-blend-multiply"></div>
            <div class="hero-content text-slate-100 text-center text-shadow-md text-shadow-slate-600 ">
                <div class="max-w-md ">
                    <h1 class="text-5xl font-bold">Hi There!</h1>
                    <h1 class="mb-5 text-5xl font-bold">Patrick Here!</h1>
                    <p>
                        I&apos;m a developer, devops engineer and a life long learner who is
                        aspiring to continue learning and developing new skills until the
                        end of time.
                    </p>
                </div>
            </div>
        </div>
    }
}
