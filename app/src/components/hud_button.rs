// components/hud_button.rs
use yew::prelude::*;
use yew_router::prelude::*;
use crate::router::Route;
use crate::hooks::use_navigation;
use crate::utils::nav_button_helpers::{is_route_active, nav_image_paths};

#[derive(Properties, PartialEq)]
pub struct HudButtonProps {
    pub src: String,                    // base filepath
    pub alt_text: String,
    pub route: Route,
    #[prop_or(false)]
    pub disabled: bool,                 // for disabled buttons
}

#[function_component(HudButton)]
pub fn hud_button(props: &HudButtonProps) -> Html {
    let navigate = use_navigation();
    let current_route = use_route::<Route>();

    // helper function to check if this button's route is currently active
    let is_active = is_route_active(&current_route, &props.route);

    // construct image paths
    let (normal_img, active_img) = nav_image_paths(&props.src);
    let disabled_img = format!("{}_G.webp", props.src);

    html! {
        if props.disabled {
            // disabled state
            <button 
                class="group w-full h-full flex items-center justify-center bg-transparent border-none"
                disabled={true}>
                <img 
                    src={disabled_img} 
                    alt={props.alt_text.clone()}
                    class="w-4/5 h-auto block absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 transition-opacity duration-0 ease-in-out cursor-not-allowed"
                />
            </button>

        } else if is_active {
            // active state - red
            <button 
                onclick={navigate.reform({let route = props.route.clone(); move |_| route.clone()})}
                class="group w-full h-full flex items-center justify-center cursor-pointer bg-transparent border-none">
                <img 
                    src={active_img} 
                    alt={props.alt_text.clone()}
                    class="w-4/5 h-auto block absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2"
                />
            </button>
            
        } else {
            // normal state - show hover effect
            <button 
                onclick={navigate.reform({let route = props.route.clone(); move |_| route.clone()})}
                class="group w-full h-full flex items-center justify-center cursor-pointer bg-transparent border-none">
                <img 
                    src={normal_img} 
                    alt={props.alt_text.clone()}
                    class="w-4/5 h-auto block absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 transition-opacity duration-0 ease-in-out group-hover:opacity-0"
                />
                <img
                    src={active_img}
                    alt={props.alt_text.clone()}
                    class="w-4/5 h-auto block absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 opacity-0 transition-opacity duration-0 ease-in-out group-hover:opacity-100 z-10"
                />
            </button>
        }
    }
}