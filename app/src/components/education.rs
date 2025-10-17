// components/education.rs
use yew::prelude::*;

use crate::pages::about::{ExperienceItem};

#[function_component(Education)]
pub fn education() -> Html {
    // education data
    let education: Vec<ExperienceItem> = vec![
        ExperienceItem {
            title: "Software Engineering & Management (BSc)".to_string(),
            institution: "University of Gothenburg, SE".to_string(),
            date: "2022 - 2025".to_string(),
            description: None,
            icon: "/static/about/education/U_GOTH.png".to_string(),
        },
        ExperienceItem {
            title: "European Law (LLM)".to_string(),
            institution: "Leiden University, NL".to_string(),
            date: "2015 - 2016".to_string(),
            description: None,
            icon: "/static/about/education/U_LEID.png".to_string(),
        },
        ExperienceItem {
            title: "Law (LLB)".to_string(),
            institution: "University of Reading, UK".to_string(),
            date: "2012 - 2015".to_string(),
            description: None,
            icon: "/static/about/education/U_READ.png".to_string(),
        },
    ];

    html! {
        // timeline
        <div class="max-w-4xl mx-auto">
            <div class="relative">
                // vertical line
                <div class="absolute left-8 md:left-1/2 w-1 bg-red-600 transform md:-translate-x-1/2 top-14 bottom-12 md:top-12 md:bottom-12"></div>

                { for education.iter().enumerate().map(|(index, exp)| {
                    let is_even = index % 2 == 0;
                    html! {
                        <div class={format!("relative flex items-center mb-12 {}", 
                            if is_even { "md:flex-row" } else { "md:flex-row-reverse" }
                        )}>
                            // timeline node
                            <div class="absolute left-8 md:left-1/2 w-16 h-16 bg-white border-4 border-red-600 rounded-full flex items-center justify-center transform -translate-x-1/2 md:-translate-x-1/2 z-20">
                                <img 
                                    src={exp.icon.clone()} 
                                    alt="Avatar" 
                                    class="w-12 h-12 object-contain rounded-full"
                                />
                            </div>

                                // date on opposite side
                                <div class={format!("hidden md:block absolute top-1/2 transform -translate-y-1/2 {} md:w-5/12", 
                                    if is_even { "md:ml-8 right-10 pl-5" } else { "md:mr-8 left-10 pr-5 text-right" }
                                )}>
                                    <div class="text-red-600 font-semibold text-l">{&exp.date}</div>
                            </div>
                            
                            // content with HUD-style background
                            <div class={format!("ml-24 md:ml-0 {} md:w-5/12", 
                                if is_even { "md:mr-8 md:text-right" } else { "md:ml-8" }
                            )}>
                                <div 
                                    class="relative text-white flex items-center justify-left text-left p-6 hover:scale-105 transition-transform duration-300"
                                    style="background-image: url('/static/hud/section/STBAR7.png'); 
                                            background-repeat: no-repeat; 
                                            background-size: 100% 100%; 
                                            image-rendering: pixelated; 
                                            min-height: 120px;"
                                >

                                    // inner box
                                    <div 
                                        class="absolute inset-0 m-4 z-5 bg-[#1a1a1a] bg-opacity-60 border-4 border-[#0b0b0a]"
                                    ></div>

                                    <div class="z-10">
                                        <h3 class="text-xl font-bold text-white mb-2 pl-3">{&exp.title}</h3>
                                        <div class="text-red-600 font-semibold mb-2 pl-3">{&exp.institution}</div>
                                        // date shown on mobile only
                                        <div class="text-gray-400 text-sm mb-3 md:hidden pl-3">{&exp.date}</div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    }
                })}
            </div>
        </div>
    }
}