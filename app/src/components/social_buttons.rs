// components/social_buttons.rs
use yew::prelude::*;

use crate::data::social_buttons::{social_buttons_data, SocialButtonTag};

#[derive(Properties, PartialEq)]
pub struct SocialButtonsProps {
    pub button_size: u8,
    pub svg_size: u8,
    // shown if a button has any of these tags; ignored when `only` is set
    #[prop_or_default]
    pub tags: Vec<SocialButtonTag>,
    // when set, show only the button with this exact title instead of filtering by tag
    #[prop_or_default]
    pub only: Option<&'static str>,
    #[prop_or_else(|| "flex justify-center items-center gap-4 mt-8 w-full".to_string())]
    pub wrapper_class: String,
}

#[function_component(SocialButtons)]
pub fn social_buttons(props: &SocialButtonsProps) -> Html {
    let buttons: Vec<_> = social_buttons_data()
        .into_iter()
        .filter(|button| match props.only {
            Some(title) => button.title == title,
            None => button.tags.iter().any(|tag| props.tags.contains(tag)),
        })
        .collect();

    html! {
        <div class={props.wrapper_class.clone()}>
            { for buttons.iter().map(|button| html! {
                <a
                    href={button.url}
                    target="_blank"
                    class={format!("w-{} h-{} {} {} rounded-lg flex items-center justify-center transition-all duration-300 hover:scale-110 border-2 {} {}", props.button_size, props.button_size, button.bg, button.hover_bg, button.border, button.hover_border)}
                    title={button.title}
                >
                    <img
                        src={button.icon}
                        alt={button.title}
                        class={format!("w-{} h-{} object-contain", props.svg_size, props.svg_size)}
                    />
                </a>
            })}
        </div>
    }
}
