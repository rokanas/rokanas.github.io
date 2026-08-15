// components/hud_section.rs
use yew::prelude::*;
use crate::router::Route;

#[derive(Properties, PartialEq)]
pub struct HudSectionProps {
    pub children: Children,
    pub background_image: String,
    pub background_width: u32,
    pub text_color: String,
    #[prop_or(None)]
    pub route: Option<Route>,           // optional route to check for active state
}

#[function_component(HudSection)]
pub fn hud_section(props: &HudSectionProps) -> Html {
    let flex_style = format!("flex: {};", props.background_width);

    html! {
        <div
            class={format!(
                "relative {} flex items-center justify-center text-center bg-pixel-panel",
                props.text_color,
            )}
            style={format!(
                "background-image: url('{}'); height: 9vw; {};",
                props.background_image,
                flex_style
            )}
        >
            <div class="z-10">
                { for props.children.iter() }   
            </div>
        </div>
    }
}