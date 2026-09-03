// components/cta_button.rs
use yew::prelude::*;

// shared CTA-button look used across project items
// (inside CardShell's buttons row, and in their modal footers)
#[derive(Clone, Copy, PartialEq)]
pub enum ButtonVariant {
    Primary,   // red, interactive
    Success,   // green, interactive
    Secondary, // gray, non-interactive ("coming soon" / disabled states)
}

#[derive(Clone, Copy, PartialEq)]
pub enum ButtonSize {
    Sm, // used inside CardShell's buttons row
    Lg, // used in modal footers
}

impl Default for ButtonSize {
    fn default() -> Self {
        ButtonSize::Sm
    }
}

#[derive(Properties, PartialEq)]
pub struct CtaButtonProps {
    pub variant: ButtonVariant,
    #[prop_or_default]
    pub size: ButtonSize,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    // extra classes the caller controls, e.g. width (`w-full` vs `flex-1`)
    #[prop_or_default]
    pub class: Classes,
    pub children: Children,
}

#[function_component(CtaButton)]
pub fn cta_button(props: &CtaButtonProps) -> Html {
    let shape = match props.size {
        ButtonSize::Sm => "group w-full font-bold py-2 px-4 rounded font-mono text-sm",
        ButtonSize::Lg => "font-bold py-3 px-6 rounded font-mono",
    };

    let state = match (props.variant, props.size) {
        (ButtonVariant::Primary, _) =>
            "bg-doom-panel-button hover:bg-doom-red border-2 border-doom-red hover:border-doom-red text-doom-red hover:text-doom-white cursor-pointer transition-all duration-200",
        (ButtonVariant::Success, _) =>
            "bg-doom-panel-button hover:bg-green-600 border-2 border-green-600 hover:border-green-600 text-green-600 hover:text-doom-white cursor-pointer transition-all duration-200",
        (ButtonVariant::Secondary, ButtonSize::Sm) =>
            "bg-doom-panel-button hover:bg-gray-600 border-2 border-gray-500 hover:border-doom-gray-dark text-doom-gray-dark hover:text-doom-gray-light cursor-not-allowed transition-all duration-200",
        (ButtonVariant::Secondary, ButtonSize::Lg) =>
            "bg-doom-panel-button border-2 border-gray-500 text-doom-gray-dark cursor-not-allowed",
    };

    html! {
        <button onclick={props.onclick.clone()} class={classes!(shape, state, props.class.clone())}>
            { for props.children.iter() }
        </button>
    }
}
