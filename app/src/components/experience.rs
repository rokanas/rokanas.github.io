// components/experience.rs
use yew::prelude::*;

use crate::components::social_buttons::SocialButtons;
use crate::data::experience::experience_data;
use crate::utils::markdown_links::render_inline_links;

#[function_component(Experience)]
pub fn experience() -> Html {
    let selected_job = use_state(|| 0usize);    // track which job is selected
    let experience = experience_data();

    html! {
        // education component
        <div class="max-w-6xl mx-auto">
            <div class="flex flex-col lg:flex-row gap-8">
        
                // left sidebar (job titles)
                <div class="lg:w-1/3 space-y-3">
                    { for experience.iter().enumerate().map(|(index, exp)| {
                        let is_selected = *selected_job == index; // first item selected by default
                        let selected_job_clone = selected_job.clone();
                        let onclick = Callback::from(move |_| {
                            selected_job_clone.set(index);
                        });
                        html! {
                            <div class={format!("flex items-center p-4 cursor-pointer transition-all duration-300 {}", 
                                if is_selected { 
                                    "bg-doom-red/30 border-3 border-doom-red/40" 
                                } else { 
                                    "bg-gray-500/25 border-3 border-gray-900 hover:border-gray-600" 
                                }
                            )}
                            onclick={onclick}>
                                <div class="flex-shrink-0 mr-4">
                                    <div class="w-12 h-12 bg-gray-800 rounded-full flex items-center justify-center">
                                        <div class={format!("w-10 h-10 rounded-full overflow-hidden flex items-center justify-center {}", exp.icon_bg)}>
                                            <img
                                                src={exp.icon.clone()}
                                                alt="Institution logo"
                                                class="w-full h-full object-contain"
                                            />
                                        </div>
                                    </div>
                                </div>
                                <div class="flex-1">
                                    <h3 class="text-doom-white font-semibold text-lg mb-1">{&exp.title}</h3>
                                    <p class="text-doom-gray-dark text-sm">{&exp.institution}</p>
                                </div>
                            </div>
                        }
                    })}
                </div>

                // right content (job details)
                <div class="lg:w-2/3">
                    <div
                        class="relative p-8 bg-pixel-panel"
                        style="background-image: url('/static/common/STBAR_BIG.webp'); min-height: 120px; transparency: 0.9;"
                    >
                        // inner box
                        <div
                            class="absolute inset-0 m-4 z-5 bg-doom-panel-inner bg-opacity-60 border-4 border-doom-panel-outer"
                        ></div>
                        
                        // social button
                        if let Some(title) = experience[*selected_job].social_button {
                            <SocialButtons
                                button_size={12}
                                svg_size={8}
                                only={Some(title)}
                                wrapper_class={"absolute top-9 right-9 z-20".to_string()}
                            />
                        }

                        <div class="relative z-10">
                            <div class="flex items-center mb-6">
                                <div class="flex-shrink-0 mr-4">
                                    <div class="w-16 h-16 bg-gray-800 rounded-full flex items-center justify-center">
                                        <div class={format!("w-13 h-13 rounded-full overflow-hidden flex items-center justify-center {}", experience[*selected_job].icon_bg)}>
                                            <img
                                                src={experience[*selected_job].icon.clone()}
                                                alt="Company logo"
                                                class="w-full h-full object-contain"
                                            />
                                        </div>
                                    </div>
                                </div>
                                <div>
                                    <h2 class="text-2xl font-bold text-doom-white mb-2">{&experience[*selected_job].title}</h2>
                                    <p class="text-doom-red text-lg mb-1">{&experience[*selected_job].institution}</p>
                                    <p class="text-doom-gray-dark text-sm">{&experience[*selected_job].date}</p>
                                </div>
                            </div>

                            <div class="space-y-4">
                                if let Some(description) = &experience[*selected_job].description {
                                    { for description.iter().map(|point| {
                                        html! {
                                            <li class="flex items-start">
                                                <div class="flex-shrink-0 w-2 h-2 bg-doom-red rounded-full mt-2 mr-3"></div>
                                                <p class="text-doom-gray-light leading-relaxed mb-2">{render_inline_links(point)}</p>
                                            </li>
                                        }
                                    })}
                                }
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}