use yew::prelude::*;
use yew_router::prelude::*;
use crate::router::Route;
use crate::components::model_viewer::ModelViewer;
use crate::app::NavbarContext;

#[hook]
fn use_navigation() -> Callback<Route> {
    let navigator = use_navigator().unwrap();
    
    Callback::from(move |route: Route| {
        navigator.push(&route);
    })
}

#[function_component(Home)]
pub fn home() -> Html {
    let navbar_context = use_context::<NavbarContext>().expect("NavbarContext not found");

    let window_width = use_state(|| {
        web_sys::window()
            .and_then(|w| w.inner_width().ok())
            .and_then(|w| w.as_f64())
            .unwrap_or(800.0) as u32
    });

    let window_height = use_state(|| {
    web_sys::window()
        .and_then(|w| w.inner_height().ok())
        .and_then(|w| w.as_f64())
        .unwrap_or(600.0) as u32
    });

    html! {
        <div class={format!(
            "h-screen flex items-center justify-center transition-all duration-500 ease-in-out {}",
            if navbar_context.is_default_navbar { "pt-40" } else { "pb-15" }
        )}>

            // model canvas is button to doom projects
            <div
                title = "Unholy Cathedral"
                class = "cursor-grab hover:cursor-grab active:cursor-grabbing">
                <ModelViewer 
                    model_name="unholy_cathedral"
                    width={*window_width}
                    height={*window_height}
                    front_cam=true
                />
            </div>

            // model made text anchored to top/bottom left depending on navbar type
            <img 
                src="/static/models/unholy_cathedral/MODEL_MADE.png" 
                alt="Model made using Ultimate Doom Builder + Blender"
                class={format!(
                    "absolute {} left-2 w-[25vw] h-auto  text-red-600", // max-w-32 max-h-32 TODO: use max if necessary for larger screens
                    if navbar_context.is_default_navbar { "bottom-2" } else { "top-2" }
            )}/>
        </div>
    }
}