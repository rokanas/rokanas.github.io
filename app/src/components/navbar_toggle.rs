// components/navbar_toggle.rs
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct NavbarToggleProps {
    pub is_default_navbar: bool,
    pub on_toggle: Callback<()>,
}

#[function_component(NavbarToggle)]
pub fn navbar_toggle(props: &NavbarToggleProps) -> Html {
    let on_click = {
        let on_toggle = props.on_toggle.clone();
        Callback::from(move |_: MouseEvent| {
            on_toggle.emit(());
        })
    };

    html! {
        <button
            onclick={on_click}
            class={format!(
                "fixed z-50 bg-gray-800 hover:bg-gray-700 text-white p-3 rounded-full shadow-lg transition-all duration-300 {}",
                if props.is_default_navbar {
                    "bottom-4 right-4"  // bottom right when header is at top
                } else {
                    "top-4 right-4"     // top right when HUD is at bottom
                }
            )}
            title={if props.is_default_navbar { "Switch to HUD Navbar" } else { "Switch to Header Navbar" }}
        >
            // icon or text for button
            <span class="text-sm font-bold">
                {if props.is_default_navbar { "🔝" } else { "🎮" }}
            </span>
        </button>
    }
}