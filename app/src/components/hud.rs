// components/footer.rs
use yew::prelude::*;                    // import everything from yew prelude (html macros, hooks, components, etc)
use yew_router::prelude::*;             // import everything from yew router prelude (routing macros, hooks, etc)
use web_sys::{window};                  // import window function from web_sys (browser APIs for web assemnbly)
use gloo_events::EventListener;         // import for handling DOM effects (e.g. mouse movement)
use wasm_bindgen::JsCast;               // import trait for convering between Javascript types in web assembly

use crate::router::Route;               // import route enum for page navigation
use crate::components::hud_section::HudSection;
use crate::components::hud_button::HudButton;

// props to control footer visibility and animation
#[derive(Properties, PartialEq)]
pub struct HudProps {
    #[prop_or(false)]
    pub show: bool,
}

// hook to track mouse position and convert to grid position
#[hook] // macro for yew hook (reusable stateful logic)
fn use_mouse_grid() -> (i32, i32) {         // return tuple of signed integers (column, row)
    let grid_pos = use_state(|| (2, 1));    // hook creates reactive state, initialized at center-left (default)
    
    {
        let grid_pos = grid_pos.clone();    // clone state handle to move into effect closure (anonymous function)
        use_effect_with((), move |_| {      // runs side effects when component mounts or updates (empty tuple means run once)
            let window = window().unwrap(); // gets browser window object, panics if not available
            let document = window.document().unwrap();  // gets document from window
            let body = document.body().unwrap();        // gets body element from document
            
            let listener = EventListener::new(&body, "mousemove", move |event| {    // create a listener on body for mousemove events
                // use event target to get mouse coords
                if let Some(mouse_event) = event.dyn_ref::<web_sys::MouseEvent>() { // pattern matching to safely cast generic event to MouseEvent
                    let x = mouse_event.client_x() as f64;  // get x coords of mouse, convert to 64-bit float
                    let y = mouse_event.client_y() as f64;  // get y coords of mouse, convert to 64-bit float
                    
                    // get viewport dimensions
                    if let (Ok(viewport_width), Ok(viewport_height)) = (    // pattern matching on tuple to get viewport dimensions, only proceeds if both are ok
                        window.inner_width(),
                        window.inner_height()
                    ) {
                        let viewport_width = viewport_width.as_f64().unwrap_or(1920.0);     // convert viewport width to f64, default to 1920.0 if conversion fails
                        let viewport_height = viewport_height.as_f64().unwrap_or(1080.0);   // convert viewport height to f64, default to 1080.0
                        
                        // calculate grid position (5 columns, 2 rows)
                        let col = ((x / viewport_width) * 5.0).floor() as i32;  // calculate column (0-4) by dividing mouse x by width, multiplying by 5 and flooring
                        let row = ((y / viewport_height) * 2.0).floor() as i32; // calculate row (0-1) by dividing mouse y by height, multiplying by 2 and flooring
                        
                        // clamp values to valid ranges
                        let col = col.clamp(0, 4);  // ensure column between 0...4
                        let row = row.clamp(0, 1);  // ensure row between 0, 1
                        
                        grid_pos.set((col, row));   // update state with new grid position
                    }
                }
            });
            
            // return cleanup function
            move || drop(listener)  // return closure that drops event listener when component unmounts
        });
    }
    
    (*grid_pos).clone() // dereference state handle and clone current value to return it
}

// get avatar image based on grid position
fn get_avatar_image(col: i32, row: i32, is_hover: bool) -> String { // take grid position and hover state, return image path
    if is_hover {
        // single hover image regardless of grid position
        return "/static/AVATAR_2.png".to_string();
    }
    
    // map grid positions to corresponding avatar image
    match (col, row) {
        // top row (row 0)
        (0, 0) => "/static/AVATAR_TOP_LEFT.png".to_string(),
        (1, 0) => "/static/AVATAR_TOP_CENTER_LEFT.png".to_string(),
        (2, 0) => "/static/AVATAR_TOP_CENTER.png".to_string(),
        (3, 0) => "/static/AVATAR_TOP_CENTER_RIGHT.png".to_string(),
        (4, 0) => "/static/AVATAR_TOP_RIGHT.png".to_string(),
        
        // bottom row (row 1)
        (0, 1) => "/static/AVATAR_BOTTOM_LEFT.png".to_string(),
        (1, 1) => "/static/AVATAR_BOTTOM_CENTER_LEFT.png".to_string(),
        (2, 1) => "/static/AVATAR_BOTTOM_CENTER.png".to_string(),
        (3, 1) => "/static/AVATAR_BOTTOM_CENTER_RIGHT.png".to_string(),
        (4, 1) => "/static/AVATAR_BOTTOM_RIGHT.png".to_string(),
        
        // fallback
        _ => "/static/AVATAR_1.png".to_string(),    // _ is catch-all pattern for any other value
    }
}

#[hook]         // macro for yew hook (reusable stateful logic)
fn use_navigation() -> Callback<Route> {        // returns callback that takes a Route enum
    let navigator = use_navigator().unwrap();   // get navigator object from yew router, panics if not available
    
    Callback::from(move |route: Route| {        // create callback that takes a route
        navigator.push(&route);                 // use navigator to push new route
    })
}

#[function_component(Hud)]   // declare function as footer component
pub fn hud(props: &HudProps) -> Html {
    let (mouse_col, mouse_row) = use_mouse_grid();     // destructure tuple returned by hook to two variables (column, row)
    let is_visible = use_state(|| false);

    // animate footer entrance when show prop changes
    {
        let is_visible = is_visible.clone();
        use_effect_with(props.show, move |show| {
            if *show {                                              // when true, footer renders but is off-screen
                // small delay for smooth animation
                let is_visible_clone = is_visible.clone();
                gloo_timers::callback::Timeout::new(50, move || {   // after 50ms delay, is_visible becomes true (controls CSS animation)
                    is_visible_clone.set(true);
                }).forget();
            } else {
                is_visible.set(false);                              // when false, footer slides offscreen
            }
            || {}
        });
    }

    // only render if show prop is true
    if !props.show {
        return html! {};
    }

    let footer_classes = if *is_visible {
        "fixed bottom-0 left-0 right-0 w-full z-40 transform translate-y-0 transition-transform duration-500 ease-out"
    } else {
        "fixed bottom-0 left-0 right-0 w-full z-40 transform translate-y-full transition-transform duration-500 ease-out"
    };

    html! { // macro to create html structure                       // start html block
        <footer class={footer_classes}>  // fixed position at bottom with high z-index (so it appears above other content)
            <div class="flex w-full">                               // flex container div taking full horizontal width
                
                // home
                <HudSection 
                    background_image="/static/STBAR1.png"
                    background_width=48
                    background_height=32
                    text_color="text-red-600"
                    route={Route::Home}>
                    <HudButton
                        src="/static/hud/HOME_SR"
                        alt_text="Home"
                        route={Route::Home}
                        disabled=false>
                    </HudButton>
                </HudSection>

                // about
                <HudSection
                    background_image="/static/STBAR2B.png"
                    background_width=36
                    background_height=32
                    text_color="text-red-600"
                    route={Route::Projects}>
                    <HudButton
                        src="/static/hud/ABOUT_SR"
                        alt_text="Projects"
                        route={Route::Projects}
                        disabled=false>
                    </HudButton>
                </HudSection>

                // projects
                <HudSection
                    background_image="/static/STBAR3B.png"
                    background_width=58
                    background_height=32
                    text_color="text-yellow-600"
                    route={Route::About}>
                    <HudButton
                        src="/static/hud/PROJECTS_SR"
                        alt_text="About"
                        route={Route::About}
                        disabled=false>
                    </HudButton>
                </HudSection>

                // avatar
                <HudSection
                    background_image="/static/STBAR4.png"
                    background_width=37
                    background_height=32
                    text_color="text-white">
                    <button 
                        //onclick={navigate.reform(|_| Route::Avatar)}
                        class="group w-full h-full flex items-center justify-center cursor-pointer bg-transparent border-none">
                        <img 
                            src={get_avatar_image(mouse_col, mouse_row, false)}
                            alt="Avatar"
                            class="w-4/5 block absolute transition-opacity duration-200 ease-in-out group-hover:opacity-0"
                        />
                        <img 
                            src={get_avatar_image(mouse_col, mouse_row, true)}
                            alt="Avatar"
                            class="w-4/5 block absolute opacity-0 transition-opacity duration-200 ease-in-out group-hover:opacity-100"
                        />
                    </button>
                </HudSection>

                // doom projects
                <HudSection
                    background_image="/static/STBAR5.png"
                    background_width=57
                    background_height=32
                    text_color="text-red-600"
                    route={Route::DoomProjects}>
                    <HudButton
                        src="/static/hud/DOOM_PROJECTS_SR"
                        alt_text="Doom Projects"
                        route={Route::DoomProjects}
                        disabled=false>
                    </HudButton>
                </HudSection>

                // keys section
                <HudSection
                    background_image="/static/STBAR6.png"
                    background_width=13 
                    background_height=32
                    text_color="text-red-600">
                    <div class="flex flex-col">
                    </div>
                </HudSection>

                // contact
                <HudSection
                    background_image="/static/STBAR7.png"
                    background_width=71
                    background_height=32
                    text_color="text-blue-600"
                    route={Route::Home}>
                    <HudButton
                        src="/static/hud/CONTACT_SR"
                        alt_text="Contact"
                        route={Route::Home}
                        disabled=true>
                    </HudButton>
                </HudSection>
            </div>
        </footer>
    }
}