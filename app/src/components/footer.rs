// components/footer.rs
use yew::prelude::*;                    // import everything from yew prelude (html macros, hooks, components, etc)
use web_sys::{window};                  // import window function from web_sys (browser APIs for web assemnbly)
use gloo_events::EventListener;         // import for handling DOM effects (e.g. mouse movement)
use wasm_bindgen::JsCast;               // import trait for convering between Javascript types in web assembly

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
                 height: 10vw; {};",        // height is 10% of viewport width
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

#[function_component(Footer)]   // declare function as footer component
pub fn footer() -> Html {
    let (mouse_col, mouse_row) = use_mouse_grid();      // destructure tuple returned by hook to two variables (column, row)
    
    let button_click = Callback::from(|_| { // create callback for button click events
        log::info!("Button clicked");                             // |_| ignroes event parameters
    });

    html! { // macro to create html structure                       // start html block
        <footer class="fixed bottom-0 left-0 right-0 w-full z-50">  // fixed position at bottom with high z-index (so it appears above other content)
            <div class="flex w-full">                               // flex container div taking full horizontal width
                
                // home
                <HudSection 
                    background_image="/static/STBAR1.png"
                    background_width=48
                    background_height=32
                    text_color="text-red-400">
                    <button 
                        onclick={button_click.clone()} 
                        class="group w-full h-full flex items-center justify-center cursor-pointer bg-transparent border-none"> // group class allows child elements to react to hover state of parent
                        <img 
                            src="/static/HOME1.png" 
                            alt="Home"
                            class="block transition-opacity duration-200 ease-in-out group-hover:opacity-0"
                        />
                        <img 
                            src="/static/HOME2.png" 
                            alt="Home Hover"
                            class="block absolute opacity-0 transition-opacity duration-200 ease-in-out group-hover:opacity-100"
                        />
                    </button>
                </HudSection>

                // projects
                <HudSection
                    background_image="/static/STBAR2.png"
                    background_width=58
                    background_height=32
                    text_color="text-red-400">
                    <button 
                        onclick={button_click.clone()} 
                        class="group w-full h-full flex items-center justify-center cursor-pointer bg-transparent border-none">
                        <img 
                            src="/static/PROJECTS1.png" 
                            alt="Projects"
                            class="w-4/5 h-auto block transition-opacity duration-0 ease-in-out group-hover:opacity-0"
                        />
                        <img 
                            src="/static/PROJECTS2.png" 
                            alt="Projects Hover"
                            class="w-4/5 h-auto block absolute opacity-0 transition-opacity duration-0 ease-in-out group-hover:opacity-100"
                        />
                    </button>
                </HudSection>

                // about
                <HudSection
                    background_image="/static/STBAR3.png"
                    background_width=36
                    background_height=32
                    text_color="text-yellow-400">
                    <button 
                        onclick={button_click.clone()} 
                        class="group w-full h-full flex items-center justify-center cursor-pointer bg-transparent border-none">
                        <img 
                            src="/static/ABOUT1.png" 
                            alt="About"
                            class="w-4/5 h-auto block transition-opacity duration-0 ease-in-out group-hover:opacity-0"
                        />
                        <img 
                            src="/static/ABOUT2.png" 
                            alt="About Hover"
                            class="w-4/5 h-auto block absolute opacity-0 transition-opacity duration-0 ease-in-out group-hover:opacity-100"
                        />
                    </button>
                </HudSection>

                // avatar (now mouse-following)
                <HudSection
                    background_image="/static/STBAR4.png"
                    background_width=37
                    background_height=32
                    text_color="text-white">
                    <button 
                        onclick={button_click.clone()} 
                        class="group w-full h-full flex items-center justify-center cursor-pointer bg-transparent border-none">
                        <img 
                            src={get_avatar_image(mouse_col, mouse_row, false)}
                            alt="Avatar"
                            class="w-4/5 block absolute transition-opacity duration-200 ease-in-out group-hover:opacity-0"
                        />
                        <img 
                            src={get_avatar_image(mouse_col, mouse_row, true)}
                            alt="Avatar Hover"
                            class="w-4/5 block absolute opacity-0 transition-opacity duration-200 ease-in-out group-hover:opacity-100"
                        />
                    </button>
                </HudSection>

                // doom projects
                <HudSection
                    background_image="/static/STBAR5.png"
                    background_width=57
                    background_height=32
                    text_color="text-red-400">
                    <button 
                        onclick={button_click.clone()} 
                        class="group w-full h-full flex items-center justify-center cursor-pointer bg-transparent border-none">
                        <img 
                            src="/static/DOOM_PROJECTS1.png" 
                            alt="Doom Projects"
                            class="w-4/5 h-auto block transition-opacity duration-0 ease-in-out group-hover:opacity-0"
                        />
                        <img 
                            src="/static/DOOM_PROJECTS2.png" 
                            alt="Doom Projects Hover"
                            class="w-4/5 h-auto block absolute opacity-0 transition-opacity duration-0 ease-in-out group-hover:opacity-100"
                        />
                    </button>
                </HudSection>

                // keys section
                <HudSection
                    background_image="/static/STBAR6.png"
                    background_width=13 
                    background_height=32
                    text_color="text-red-400">
                    <div class="flex flex-col">
                        <span class="text-2xl font-bold">{format!("0%")}</span>
                        <span class="text-xs">{"KEYS"}</span>
                    </div>
                </HudSection>

                // contact
                <HudSection
                    background_image="/static/STBAR7.png"
                    background_width=71
                    background_height=32
                    text_color="text-blue-400">
                    <button 
                        onclick={button_click.clone()} 
                        class="group w-full h-full flex items-center justify-center cursor-pointer bg-transparent border-none">
                        <img 
                            src="/static/CONTACT1.png" 
                            alt="Contact"
                            class="block transition-opacity duration-0 ease-in-out group-hover:opacity-0"
                        />
                        <img 
                            src="/static/CONTACT2.png" 
                            alt="Contact Hover"
                            class="block absolute opacity-0 transition-opacity duration-0 ease-in-out group-hover:opacity-100"
                        />
                    </button>
                </HudSection>
            </div>
        </footer>
    }
}