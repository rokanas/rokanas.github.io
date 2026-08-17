// components/technologies.rs
use yew::prelude::*;

use crate::data::technologies::technologies_data;

#[function_component(Technologies)]
pub fn technologies() -> Html {
    let skills = technologies_data();

    html! {
        // technologies grid
        <div class="grid grid-cols-3 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-7 gap-6 max-w-6xl mx-auto justify-items-center">
            { for skills.iter().map(|skill| html! {
                <div
                    class="relative text-white flex items-center justify-left text-left p-6 hover:scale-105 transition-transform duration-300 aspect-square max-w-35 max-h-35 bg-pixel-panel"
                    style="background-image: url('/static/hud/section/STBAR6_2.png');"
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
    }
}