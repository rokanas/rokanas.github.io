// components/hud_section.rs
use yew::prelude::*;

#[derive(Properties, PartialEq)]        // macro automatically implementing properties trait (for yew component props) and partialeq (for comparing prop changes)
pub struct HudSectionProps {            // struct defining hud component properties
    pub children: Children,             // holds child elements rendered inside the component
    pub background_image: String,       // path to background image
    pub background_width: u32,          // width of background image in pixels
    pub background_height: u32,         // height of background image in pixels
    pub text_color: String,             // CSS class for text color
}

#[function_component(HudSection)]       // macro to define functional component in yew
pub fn hud_section(props: &HudSectionProps) -> Html {   // signature taking props by reference, returning html
    let flex_style = format!("flex: {};", props.background_width);  // create CSS flex property string using background_width

    html! {                     // macro allowing html-like syntax in rust   
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
                 height: 9vw; {};",        // height is 10% of viewport width
                props.background_image,
                flex_style
            )}
        >
            <div class="z-10">
                { for props.children.iter() }   // iterate through and render all child components
            </div>
        </div>
    }
}