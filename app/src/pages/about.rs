// pages/portfolio.rs
use yew::prelude::*;
use web_sys::window;

use crate::components::social_buttons::{SocialButtons};
use crate::components::heading::{Heading};
use crate::components::education::{Education};
use crate::components::experience::{Experience};

// struct to hold education and experience data
#[derive(Clone, PartialEq)]
pub struct ExperienceItem {
    pub title: String,
    pub institution: String,
    pub date: String,
    pub description: Option<Vec<String>>,
    pub icon: String,
}

// struct to hold skill data
#[derive(Clone, PartialEq)]
pub struct Skill {
    pub name: String,
    pub icon: String,
    pub color: String,
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

    // skills data
    let skills = vec![
        Skill { name: "Python".to_string(), icon: "/static/about/skills/PYTHON.svg".to_string(), color: "bg-orange-500".to_string() },
        Skill { name: "Java".to_string(), icon: "/static/about/skills/JAVA.svg".to_string(), color: "bg-blue-500".to_string() },
        Skill { name: "C++".to_string(), icon: "/static/about/skills/CPP.svg".to_string(), color: "bg-blue-600".to_string() },
        Skill { name: "Cmake".to_string(), icon: "/static/about/skills/CMAKE.svg".to_string(), color: "bg-yellow-600".to_string() },
        Skill { name: "Typescript".to_string(), icon: "/static/about/skills/TS.svg".to_string(), color: "bg-cyan-500".to_string() },
        Skill { name: "SQL".to_string(), icon: "/static/about/skills/SQL.svg".to_string(), color: "bg-purple-500".to_string() },
        Skill { name: "Docker".to_string(), icon: "/static/about/skills/DOCKER.svg".to_string(), color: "bg-yellow-600".to_string() },
        Skill { name: "Yew".to_string(), icon: "/static/about/skills/YEW.svg".to_string(), color: "bg-blue-700".to_string() },
        Skill { name: "Vue".to_string(), icon: "/static/about/skills/VUE.svg".to_string(), color: "bg-yellow-500".to_string() },
        Skill { name: "React".to_string(), icon: "/static/about/skills/REACT.svg".to_string(), color: "bg-yellow-600".to_string() },
        Skill { name: "Svelte".to_string(), icon: "/static/about/skills/SVELTE.svg".to_string(), color: "bg-blue-800".to_string() },
        Skill { name: "Express".to_string(), icon: "/static/about/skills/EXPRESS.svg".to_string(), color: "bg-yellow-600".to_string() },
        Skill { name: "Postman".to_string(), icon: "/static/about/skills/POSTMAN.svg".to_string(), color: "bg-yellow-600".to_string() },
        Skill { name: "TensorFlow".to_string(), icon: "/static/about/skills/TENSORFLOW.svg".to_string(), color: "bg-yellow-600".to_string() },
        Skill { name: "Arduino".to_string(), icon: "/static/about/skills/ARDUINO.svg".to_string(), color: "bg-yellow-600".to_string() },
    ];

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
                    

                    // skills grid
                    <div class="grid grid-cols-3 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-6 gap-6 max-w-6xl mx-auto">
                        { for skills.iter().map(|skill| html! {
                            <div 
                                class="relative text-white flex items-center justify-left text-left p-6 hover:scale-105 transition-transform duration-300 aspect-square"
                                style="background-image: url('/static/STBAR6_2.png'); 
                                        background-repeat: no-repeat; 
                                        background-size: 100% 100%; 
                                        image-rendering: pixelated;"
                            >
                            <div class="w-17 h-17 rounded-lg flex text-center items-center justify-center mx-auto mb-3 group-hover:scale-110 transition-transform duration-300 overflow-hidden">
                                    <img 
                                        src={skill.icon.clone()} 
                                        alt={skill.name.clone()} 
                                        title={skill.name.clone()} 
                                        class="object-contain w-full h-full drop-shadow-[2px_4px_6px_rgba(0,0,0,0.9)]"
                                    />
                                </div>
                            </div>
                        })}
                    </div>
                </div>

                // social media button links footer
                <SocialButtons 
                    button_size={14}
                    svg_size={10}
                />

            </div>
        </main>
    }
}