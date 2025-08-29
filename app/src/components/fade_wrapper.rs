// components/fade_wrapper.rs
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct FadeWrapperProps {
    pub children: Children,
}

// wrapper component for page fade-in effect using tailwind classes
// exists to avoid stacking context issue where header/footer will appear above overlays
#[function_component(FadeWrapper)]
pub fn tailwind_fade_wrapper(props: &FadeWrapperProps) -> Html {
    let is_visible = use_state(|| false);
    
    use_effect_with((), {
        let is_visible = is_visible.clone();
        move |_| {
            gloo_timers::callback::Timeout::new(50, move || {
                is_visible.set(true);
            }).forget();
            || {}
        }
    });
    
    let opacity_class = if *is_visible { 
        "opacity-100" 
    } else { 
        "opacity-0" 
    };
    
    html! {
        <div class={format!("transition-opacity duration-500 ease-in-out {}", opacity_class)}>
            { for props.children.iter() }
        </div>
    }
}