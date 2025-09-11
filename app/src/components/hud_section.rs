// components/hud_section.rs
use yew::prelude::*;
use yew_router::prelude::*;
use crate::router::Route;

#[derive(Properties, PartialEq)]        
pub struct HudSectionProps {            
    pub children: Children,             
    pub background_image: String,       
    pub background_width: u32,          
    pub background_height: u32,         
    pub text_color: String,
    #[prop_or(None)]
    pub route: Option<Route>,           // optional route to check for active state
}

#[function_component(HudSection)]       
pub fn hud_section(props: &HudSectionProps) -> Html {   
    let current_route = use_route::<Route>();
    let flex_style = format!("flex: {};", props.background_width);

    // check if this section's route is currently active
    let is_active = if let (Some(current), Some(section_route)) = (&current_route, &props.route) {
        *current == *section_route
    } else {
        false
    };

    html! {                   
        <div
            class={format!(
                "relative {} flex items-center justify-center text-center",
                props.text_color,
            )}
            style={format!(
                "background-image: url('{}'); \
                 background-repeat: no-repeat; \
                 background-size: 100% 100%; \
                 image-rendering: pixelated; \
                 height: 9vw; {};",
                props.background_image,
                flex_style
            )}
        >
            // darkening overlay when route is active
            if is_active {
                <div class="w-[90%] h-[80%] absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 bg-black/40 z-5
                              shadow-inner shadow-black-900/50
                              border-2 border-black/40">
                </div>
            }
            
            <div class="z-10">
                { for props.children.iter() }   
            </div>
        </div>
    }
}