use yew::prelude::*;
use yew_router::prelude::*;
use crate::router::Route;

// TODO: merge hud_button and header_button components

#[derive(Properties, PartialEq)]
pub struct HeaderButtonProps {
    pub src: String,        // base filepath
    pub alt_text: String, 
    pub route: Route,       
    #[prop_or(false)]
    pub disabled: bool,     // for disabled buttons
}

#[hook]
fn use_navigation() -> Callback<Route> {
    let navigator = use_navigator().unwrap();
    
    Callback::from(move |route: Route| {
        navigator.push(&route);
    })
}

#[function_component(HeaderButton)]
pub fn header_button(props: &HeaderButtonProps) -> Html {
    let navigate = use_navigation();
    let current_route = use_route::<Route>();

    let is_active = if let Some(current) = &current_route {
        *current == props.route
    } else {
        false
    };

    let normal_img = format!("{}_W.png", props.src);
    let active_img = format!("{}_R.png", props.src);

    html! {
        if props.disabled {
            // disabled state
            <button 
                class="relative group px-2 py-1.5 flex items-center justify-center rounded-md text-gray-500 cursor-not-allowed opacity-50 transition-all duration-200"
                disabled={true}>
                <img 
                    src={normal_img} 
                    alt={props.alt_text.clone()}
                    class="h-5 sm:h-6 lg:h-7"
                />
            </button>

        } else if is_active {
            // active state - red
            <button 
                onclick={navigate.reform({let route = props.route.clone(); move |_| route.clone()})}
                class="relative group px-2 py-1.5 flex items-center justify-center rounded-md cursor-pointer transition-all duration-200 bg-black/50 border-2 border-red-600/50">
                <img 
                    src={active_img} 
                    alt={props.alt_text.clone()}
                    class="h-5 sm:h-6 lg:h-7"
                />
            </button>
            
        } else {
            // normal state - show hover effect
            <button 
                onclick={navigate.reform({let route = props.route.clone(); move |_| route.clone()})}
                class="relative group px-2 py-1.5 flex items-center justify-center rounded-md cursor-pointer transition-all duration-200 hover:bg-black/30 border-2 border-transparent hover:border-white/30">
                <img 
                    src={normal_img.clone()} 
                    alt={props.alt_text.clone()}
                    class="h-5 sm:h-6 lg:h-7 transition-opacity duration-200 ease-in-out group-hover:opacity-0"
                />
                <img 
                    src={normal_img.clone()} 
                    alt={props.alt_text.clone()}
                    class="h-5 sm:h-6 lg:h-7 absolute inset-0 m-auto opacity-0 transition-opacity duration-200 ease-in-out group-hover:opacity-100"
                />
            </button>
        }
    }
}