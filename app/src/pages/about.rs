// pages/about.rs
use yew::prelude::*;
use web_sys::window;

use crate::components::social_buttons::{SocialButtons};
use crate::components::heading::{Heading};
use crate::components::education::{Education};
use crate::components::experience::{Experience};
use crate::components::technologies::{Technologies};
use crate::data::about::{BioParagraph, bio_paragraphs};

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
            class="min-h-screen text-white pt-4 pb-10" 
        >
            <div class="container mx-auto px-4 max-w-7xl">
                
                // about section
                <div class="mb-16">
                    <div class="flex flex-col lg:flex-row items-center gap-8 lg:gap-16 max-w-6xl mx-auto">

                        // portrait
                        <div class="flex-shrink-0 w-full lg:w-auto flex flex-col items-center">
                            <div class="w-64 h-64 lg:w-80 lg:h-80 rounded-full bg-[#1a1a1a] flex items-center justify-center text-6xl lg:text-8xl shadow-2xl border-8 border-[#0b0b0a] hover:scale-105 transition-transform duration-300">
                                <img 
                                    src="/static/about/AVATAR.png" 
                                    alt="Avatar" 
                                    class="w-full h-full object-cover rounded-full"
                                />
                            </div>

                            // social media button links
                            <SocialButtons 
                                button_size={12}
                                svg_size={8}
                                professional=true
                            />
                        </div>
                            
                // description
                <div class="flex-1 text-center lg:text-left">
                    <div class="text-center mb-10 mt-10">
                        <img 
                            src="/static/about/KONSTANTINOS_ROKANAS_1.png" 
                            alt="Konstantinos Rokanas"
                            class="w-auto h-auto mx-auto"
                        />
                    </div>
                            { for bio_paragraphs().iter().enumerate().map(|(index, paragraph)| {
                                let class = if index == 0 {
                                    "text-gray-300 text-lg lg:text-lg leading-relaxed"
                                } else {
                                    "text-gray-300 text-lg lg:text-lg leading-relaxed mt-5"
                                };
                                match paragraph {
                                    BioParagraph::Plain(text) => html! {
                                        <p class={class}>{text}</p>
                                    },
                                    BioParagraph::WithEmphasis { before, emphasis, after } => html! {
                                        <p class={class}>{before} <strong>{emphasis}</strong> {after}</p>
                                    },
                                }
                            })}
                        </div>
                    </div>
                </div>

                // work experience section
                <div class="mb-16">
                    // heading
                    <Heading 
                        src="/static/about/EXPERIENCE_1.png" 
                        alt="Experience"
                        sub_heading=""             
                    />
                    // component
                    <Experience/>
                </div>

                // education
                <div class="mb-16">
                    // heading
                    <Heading 
                        src="/static/about/EDUCATION_1.png" 
                        alt="Education"
                        sub_heading=""             
                    />
                    // component
                    <Education/>
                </div>

                // technologies section
                <div class="mb-16">
                    // heading
                    <Heading 
                        src="/static/about/TECHNOLOGIES_1.png" 
                        alt="Technologies"
                        sub_heading=""             
                    />
                    <Technologies/>
                </div>

                // social media button links footer
                <SocialButtons 
                    button_size={12 }
                    svg_size={8}
                    professional=true
                />

            </div>
        </main>
    }
}