// components/hud_button.rs
use yew::prelude::*;
use yew_router::prelude::*;
use crate::router::Route;

#[derive(Properties, PartialEq)]
pub struct HudButtonProps {
    pub src: String,                    // base filepath
    pub alt_text: String,               
    pub route: Route,                   
    #[prop_or(false)]
    pub disabled: bool,                 // for disabled buttons
}

#[hook]
fn use_navigation() -> Callback<Route> {
    let navigator = use_navigator().unwrap();
    
    Callback::from(move |route: Route| {
        navigator.push(&route);
    })
}

#[function_component(HudButton)]
pub fn hud_button(props: &HudButtonProps) -> Html {
    let navigate = use_navigation();
    let current_route = use_route::<Route>();

    // helper function to check if this button's route is currently active
    let is_active = if let Some(current) = &current_route {
        *current == props.route
    } else {
        false
    };

    // construct image paths
    let normal_img = format!("{}_W.png", props.src);
    let active_img = format!("{}_R.png", props.src);
    let disabled_img = format!("{}_G.png", props.src);

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
                // // darkening overlay when route is active
                // <div class="w-[90%] h-[80%] absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 bg-black/30 z-5
                //                 shadow-inner shadow-black-900/50
                //                 border-white/40
                //                 opacity-0 group-hover:opacity-100 z-5">
                // </div>
            </button>
        }
    }
}