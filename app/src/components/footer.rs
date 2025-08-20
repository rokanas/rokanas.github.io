// components/footer.rs
use yew::prelude::*;
use web_sys::{window};
use gloo_events::EventListener;
use wasm_bindgen::JsCast;

#[derive(Properties, PartialEq)]
pub struct HudSectionProps {
    pub children: Children,
    pub background_image: String,
    pub background_width: u32,
    pub background_height: u32,
    pub text_color: String,
    #[prop_or_default]
    pub border_style: Option<String>,
    #[prop_or_default]
    pub width_class: Option<String>,
}

#[function_component(HudSection)]
pub fn hud_section(props: &HudSectionProps) -> Html {
    let flex_style = format!("flex: {};", props.background_width);

    html! {
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
                 height: 10vw; {};",
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

// hook to track mouse position and convert to grid position
#[hook]
fn use_mouse_grid() -> (i32, i32) {
    let grid_pos = use_state(|| (2, 1)); // start at center-left (default)
    
    {
        let grid_pos = grid_pos.clone();
        use_effect_with((), move |_| {
            let window = window().unwrap();
            let document = window.document().unwrap();
            let body = document.body().unwrap();
            
            let listener = EventListener::new(&body, "mousemove", move |event| {
                // use event target to get mouse coords
                if let Some(mouse_event) = event.dyn_ref::<web_sys::MouseEvent>() {
                    let x = mouse_event.client_x() as f64;
                    let y = mouse_event.client_y() as f64;
                    
                    // get viewport dimensions
                    if let (Ok(viewport_width), Ok(viewport_height)) = (
                        window.inner_width(),
                        window.inner_height()
                    ) {
                        let viewport_width = viewport_width.as_f64().unwrap_or(1920.0);
                        let viewport_height = viewport_height.as_f64().unwrap_or(1080.0);
                        
                        // calculate grid position (5 columns, 2 rows)
                        let col = ((x / viewport_width) * 5.0).floor() as i32;
                        let row = ((y / viewport_height) * 2.0).floor() as i32;
                        
                        // clamp values to valid ranges
                        let col = col.clamp(0, 4);
                        let row = row.clamp(0, 1);
                        
                        grid_pos.set((col, row));
                    }
                }
            });
            
            // return cleanup function
            move || drop(listener)
        });
    }
    
    (*grid_pos).clone()
}

// get avatar image based on grid position
fn get_avatar_image(col: i32, row: i32, is_hover: bool) -> String {
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
        _ => "/static/AVATAR_1.png".to_string(),
    }
}

#[function_component(Footer)]
pub fn footer() -> Html {
    let (mouse_col, mouse_row) = use_mouse_grid();
    
    let button_click = Callback::from(|_| {
        log::info!("Button clicked");
    });

    html! {
        <footer class="fixed bottom-0 left-0 right-0 w-full z-50">
            <div class="flex w-full">
                
                // home
                <HudSection 
                    background_image="/static/STBAR1.png"
                    background_width=48
                    background_height=32
                    text_color="text-red-400"
                    width_class="flex-1"
                    border_style="border-r-2 border-gray-600">
                    <button 
                        onclick={button_click.clone()} 
                        class="group w-full h-full flex items-center justify-center cursor-pointer bg-transparent border-none">
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
                    text_color="text-red-400"
                    width_class="flex-1"
                    border_style="border-r-2 border-gray-600">
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
                    text_color="text-yellow-400"
                    width_class="flex-1"
                    border_style="border-r-2 border-gray-600">
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
                    text_color="text-white"
                    width_class="flex-1"
                    border_style="border-r-2 border-gray-600">
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
                    text_color="text-red-400"
                    width_class="flex-1"
                    border_style="border-r-2 border-gray-600">
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
                    text_color="text-red-400"
                    width_class="flex-1"
                    border_style="border-r-2 border-gray-600">
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
                    text_color="text-blue-400"
                    width_class="flex-1"
                    border_style="border-r-2 border-gray-600">
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