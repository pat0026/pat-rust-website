use chrono::{Datelike, Local, NaiveDate};
use leptos::prelude::*;

fn caclculate_age(birthdate: &str) -> i32 {
    let birthdate = NaiveDate::parse_from_str(birthdate, "%B %d, %Y").expect("Invalid date");

    let today = Local::now().date_naive();
    let mut age = today.year() - birthdate.year();

    let month_diff = today.month() as i32 - birthdate.month() as i32;

    if month_diff < 0 || (month_diff == 0 && today.day() < birthdate.day()) {
        age -= 1;
    }

    age
}

#[component]
pub fn AboutSection() -> impl IntoView {
    let birthdate = "February 26,1997";
    let age = caclculate_age(birthdate);

    view! {
        <div id="about_section" class="scroll-mt-20 flex flex-col gap-8">
            <h1 class="text-3xl font-extrabold underline underline-offset-8">About</h1>
            <p class="text-justify">
                "I am a researcher of new technologies and a programming language
                enthusiast which fires my drive to learn any programming language. What
                I learn, I apply in my personal projects and work-related tasks which is
                an asset to any company. I aspire to become a software developer which
                is proficient creating any program, may it be low-level programs,
                web-apps or local apps. I am fond of both high and low-level programming
                languages/tools for building applications or servers. Also, I am
                currently enhancing my skills both in the front and backend side so that
                I could apply it in my daily life."
            </p>
            <div class="flex max-md:flex-col gap-4 items-center">
                <div class="avatar">
                    <div class="size-52 rounded-xl shadow-xl">
                        <img src="images/RESUME.jpg" alt="Resume 2x2" />
                    </div>
                </div>
                <div class="space-y-4">
                    <p class="text-2xl font-bold max-md:text-center">
                        CI Engineer & Software Developer
                    </p>
                    <p class="italic max-md:text-center">
                        "Don't regret what you did but regret what you didn't do."
                    </p>
                    <div class="flex  max-xl:flex-col text-sm">
                        <ul class="list-disc pl-6 flex-grow">
                            <li>
                                <span class="font-semibold">Birthday:</span>
                                {birthdate}
                            </li>
                            <li>
                                <span class="font-semibold">Website:</span>
                                <a href="https://pat0026.github.io/WebsitePortfolio/">
                                    pat0026.github.io/ <wbr />pat-portfolio
                                </a>
                            </li>
                            <li>
                                <span class="font-semibold">Phone:</span>
                                +63 9458297391
                            </li>
                            <li>
                                <span class="font-semibold">City:</span>
                                Tayabas, Quezon
                            </li>
                        </ul>
                        <ul class="list-disc pl-6 flex-grow">
                            <li>
                                <span class="font-semibold">Age:</span>
                                {age}
                            </li>
                            <li>
                                <span class="font-semibold">Degree:</span>
                                Vocational and BS
                                Degree
                            </li>
                            <li class="text-clip">
                                <span class="font-semibold">Email:</span>
                                patrick.caparros026
                                <wbr />
                                @gmail.com
                            </li>
                            <li>
                                <span class="font-semibold">Status:</span>
                                Open to work
                            </li>
                        </ul>
                    </div>
                </div>
            </div>
            <p class="text-justify">
                "If I don't know the solution to the problem I will not hesitate to tell
                you that I don't know the answer but I assure you that I will find a way
                to know what it is in the future. I'm aspiring to join a company that
                values personal growth, work-life balance and have a healthy environment
                which I could bond with diverse peers. A company that will show me to
                become a better software developer. That is why I strive to enhancing my
                skills towards any company I work with."
            </p>
        </div>
    }
}
