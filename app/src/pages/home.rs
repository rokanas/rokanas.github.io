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
    let navigate = use_navigation();

    let navbar_context = use_context::<NavbarContext>().expect("NavbarContext not found");

    html! {
        <div class={format!(
            "h-screen flex items-center justify-center {}",
            if navbar_context.is_default_navbar { "pt-40" } else { "pb-15" }
        )}>

            // model canvas is button to doom projects
            <button
                title = "Cathedral of Charybdis"
                onclick={Callback::from(move |_| navigate.emit(Route::DoomProjects))}
                class = "cursor-pointer">
                <ModelViewer 
                    obj_path="/static/cathedral/cathedral.obj"
                    width={650}
                    height={650}
                />
            </button>

            // model made text anchored to top/bottom left depending on navbar type
            <img 
                src="/static/cathedral/MODEL_MADE.png" 
                alt="Model made using Ultimate Doom Builder + Blender"
                class={format!(
                    "absolute {} left-2 w-[25vw] h-auto  text-red-600", // max-w-32 max-h-32 TODO: use max if necessary for larger screens
                    if navbar_context.is_default_navbar { "bottom-2" } else { "top-2" }
            )}/>
        </div>
    }
}