// pages/portfolio.rs
use yew::prelude::*;
use web_sys::window;

use crate::components::social_buttons::{SocialButtons};
use crate::components::heading::{Heading};
use crate::components::education::{Education};
use crate::components::experience::{Experience};
use crate::components::skills::{Skills};

// struct to hold education and experience data
#[derive(Clone, PartialEq)]
pub struct ExperienceItem {
    pub title: String,
    pub institution: String,
    pub date: String,
    pub description: Option<Vec<String>>,
    pub icon: String,
}

#[function_component(About)]
pub fn about() -> Html {

    use_effect_with((), |_| {
        // scroll to top when component mounts
        if let Some(window) = window() {
            window.scroll_to_with_x_and_y(0.0, 0.0);
        }
        || {}
    });

    html! {
        <main 
            class="min-h-screen text-white pt-8 pb-10" 
        >
            <div class="container mx-auto px-4 max-w-7xl">
                
                // about section
                <div class="mb-16">
                    <div class="flex flex-col lg:flex-row items-center gap-8 lg:gap-16 max-w-6xl mx-auto">

                        // portrait
                        <div class="flex-shrink-0 w-full lg:w-auto flex flex-col items-center">
                            <div class="w-64 h-64 lg:w-80 lg:h-80 rounded-full bg-[#1a1a1a] flex items-center justify-center text-6xl lg:text-8xl shadow-2xl border-8 border-[#0b0b0a] hover:scale-105 transition-transform duration-300">
                                <img 
                                    src="/static/AVATAR_1.png" 
                                    alt="Avatar" 
                                    class="w-full h-full object-contain"
                                />
                            </div>

                            // social media button links
                            <SocialButtons 
                                button_size={12}
                                svg_size={8}
                            />
                        </div>
                            
                // description
                <div class="flex-1 text-center lg:text-left">
                    <div class="text-center mb-10 mt-10">
                        <img 
                            src="/static/KONSTANTINOS_ROKANAS_1.png" 
                            alt="Konstantinos Rokanas"
                            class="w-auto h-auto mx-auto"
                        />
                    </div>
                            <p class="text-gray-300 text-lg lg:text-lg leading-relaxed">
                                {"🎓 I'm a recent software engineering graduate eager to begin a career in tech."}
                            </p>
                            <p class="text-gray-300 text-lg lg:text-lg leading-relaxed mt-5">
                                {"🛠 I want to build intuitive software solutions that make your jobs and lives easier, saving you time and effort. Currently learning Rust and about AI agent workflow automation."}
                            </p>
                            <p class="text-gray-300 text-lg lg:text-lg leading-relaxed mt-5">
                                {"⚖️ Formerly a legal professional with a focus on personal data and cybersecurity policy compliance and experience both in international organizations and in the private sector."}
                            </p>
                            <p class="text-gray-300 text-lg lg:text-lg leading-relaxed mt-5">
                                {"⛧ I'm also a lifelong "} <strong>{"Doom"}</strong> {" enthusiast, mapmaker and content creator."}
                            </p>
                            <p class="text-gray-300 text-lg lg:text-lg leading-relaxed mt-5">
                                {"🤼‍♂️ When away from the computer, I enjoy practicing mixed-martial arts, submission wrestling, rock-climbing and playing electric guitar."}
                            </p>
                        </div>
                    </div>
                </div>

                // education
                <div class="mb-16">
                    // heading
                    <Heading 
                        src="/static/EDUCATION_1.png" 
                        alt="Education"
                        sub_heading=""             
                    />
                    // component
                    <Education/>
                </div>

                // work experience section
                <div class="mb-16">
                    // heading
                    <Heading 
                        src="/static/EXPERIENCE_1.png" 
                        alt="Experience"
                        sub_heading=""             
                    />
                    // component
                    <Experience/>
                </div>

                // skills section
                <div class="mb-16">
                    // heading
                    <Heading 
                        src="/static/SKILLS_1.png" 
                        alt="Skills"
                        sub_heading=""             
                    />
                    <Skills/>
                </div>

                // social media button links footer
                <SocialButtons 
                    button_size={12}
                    svg_size={8}
                />

            </div>
        </main>
    }
}