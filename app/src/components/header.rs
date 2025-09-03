// components/header.rs
use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::Route;

#[derive(Properties, PartialEq)]
pub struct HeaderProps {
    #[prop_or(true)]  // header visible by default on all pages
    pub show: bool,
    #[prop_or(false)] // flag for doom projects page behavior
    pub is_doom_projects_page: bool,
}

#[hook]
fn use_navigation() -> Callback<Route> {
    let navigator = use_navigator().unwrap();
    
    Callback::from(move |route: Route| {
        navigator.push(&route);
    })
}

#[function_component(Header)]
pub fn header(props: &HeaderProps) -> Html {
    let navigate = use_navigation();
    let is_visible = use_state(|| !props.is_doom_projects_page);
    let current_route = use_route::<Route>();

    // animate header entrance when show prop changes
    {
        let is_visible = is_visible.clone();
        use_effect_with(props.is_doom_projects_page, move |is_doom_page| {
            if *is_doom_page {
                is_visible.set(false);
            } else {
                let is_visible_clone = is_visible.clone();
                gloo_timers::callback::Timeout::new(50, move || {
                    is_visible_clone.set(true);
                }).forget();
            }
            || {}
        });
    }

    // only render if show prop is true
    if props.is_doom_projects_page  {
        return html! {};
    }

    let header_class = if *is_visible {
        "fixed top-0 left-0 right-0 w-full z-40 transform -translate-y-0 transition-transform duration-500 ease-out"
    } else {
        "fixed top-0 left-0 right-0 w-full z-40 transform -translate-y-full transition-transform duration-500 ease-out"
    };

    // helper to get correct CSS class for a button
    let get_button_class = |route: Route| -> String {
        let base_class = "font-medium transition-colors duration-200 px-4 py-2 rounded-md cursor-pointer text-sm uppercase tracking-wide";
        let active_class = "text-red-600 bg-black/50 border-2 border-red-600/50";
        let inactive_class = "text-gray-300 hover:text-red-600 hover:bg-black/30 border-2 border-transparent hover:border-red-600/30";
        
        if let Some(current) = &current_route {
            if *current == route {
                return format!("{} {}", base_class, active_class);
            }
        }
        format!("{} {}", base_class, inactive_class)
    };

    html! {
        <header class={header_class} style="background-image: url('/static/SHAWN_2.png'); background-repeat: repeat; background-size: 60px;">
            <div class="relative container mx-auto px-6 py-4">
                <div class="flex items-center justify-between relative">
                    
                    // left navigation
                    <nav class="hidden md:flex items-center space-x-16 flex-1 justify-end pr-36">
                        <button
                            onclick={navigate.reform(|_| Route::Home)}
                            class={get_button_class(Route::Home)}
                        >
                            {"Home"}
                        </button>

                        <button
                            onclick={navigate.reform(|_| Route::About)}
                            class={get_button_class(Route::About)}
                        >
                            {"About"}
                        </button>

                        <button
                            onclick={navigate.reform(|_| Route::Projects)}
                            class={get_button_class(Route::Projects)}
                        >
                            {"Projects"}
                        </button>
                    </nav>

                    // center logo section with inverted trapezoid/pentagon shape
                    <div class="absolute left-1/2 transform -translate-x-1/2 z-10">
                        <div class="relative">
                            // inverted trapezoid shape - wide at top, angled sides, flat bottom
                            <div class="absolute top-0 left-1/2 transform -translate-x-1/2 w-56 h-32">
                            // border layer
                            <div 
                                class="absolute inset-0"
                                style="background: #c20000ff;
                                    clip-path: polygon(0% 0%, 100% 0%, 85% 100%, 15% 100%);">
                            </div>
                            // content layer
                            <div 
                                class="absolute inset-1"
                                style="background: linear-gradient(135deg, #1a1a1a 0%, #2a2a2a 50%, #1a1a1a 100%);
                                    clip-path: polygon(0% 0%, 100% 0%, 85% 100%, 15% 100%);
                                    box-shadow: 0 0 20px rgba(74, 222, 128, 0.3), 
                                                inset 0 0 20px rgba(0, 0, 0, 0.5);">
                            </div>
                        </div>
                            
                            // logo container
                            <button 
                                onclick={navigate.reform(|_| Route::Home)} 
                                class="relative z-20 w-56 h-32 flex items-center justify-center cursor-pointer bg-transparent border-none group">
                                <img 
                                    src="/static/KR_1.png" 
                                    alt="Home"
                                    class="transition-all duration-300 group-hover:scale-110 group-hover:brightness-125 drop-shadow-lg pt-5"
                                />
                            </button>

                            // divider below logo
                            // <div 
                            //     class="absolute bottom-0 left-1/2 transform -translate-x-1/2 overflow-hidden h-2.5"
                            //     style="background: url('/static/DIVIDER_3B.png') repeat-x top left; 
                            //            background-size: auto 100%;
                            //            width: 70%;">
                            // </div>
                        </div>
                    </div>

                    // right navigation
                    <nav class="hidden md:flex items-center space-x-20 flex-1 pl-36">
                        <button
                            onclick={navigate.reform(|_| Route::DoomProjects)}
                            class={get_button_class(Route::DoomProjects)}
                        >
                            {"Doom Projects"}
                        </button>

                        <button
                            class="text-gray-500 cursor-not-allowed font-medium px-4 py-2 rounded-md text-sm uppercase tracking-wide"
                            disabled={true}
                        >
                            {"Contact"}
                        </button>
                    </nav>

                    // mobile menu button
                    <div class="md:hidden ml-auto">
                        <button
                            class="text-gray-300 hover:text-red-600 focus:outline-none focus:ring-2 focus:ring-red-600 focus:ring-inset p-2 rounded-md"
                            onclick={Callback::from(|_| {
                                web_sys::console::log_1(&"Mobile menu clicked".into());
                            })}
                        >
                            <svg class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" />
                            </svg>
                        </button>
                    </div>
                </div>

                // mobile menu (hidden by default)
                <div class="md:hidden mt-4 pt-4 border-t border-gray-600 hidden">
                    <div class="flex flex-col space-y-2">
                        <button
                            onclick={navigate.reform(|_| Route::Home)}
                            class="text-left text-gray-300 hover:text-green-400 font-medium transition-colors duration-200 px-3 py-2 rounded-md hover:bg-black/30"
                        >
                            {"Home"}
                        </button>

                        <button
                            onclick={navigate.reform(|_| Route::About)}
                            class="text-left text-gray-300 hover:text-green-400 font-medium transition-colors duration-200 px-3 py-2 rounded-md hover:bg-black/30"
                        >
                            {"About"}
                        </button>

                        <button
                            onclick={navigate.reform(|_| Route::Projects)}
                            class="text-left text-gray-300 hover:text-green-400 font-medium transition-colors duration-200 px-3 py-2 rounded-md hover:bg-black/30"
                        >
                            {"Projects"}
                        </button>

                        <button
                            onclick={navigate.reform(|_| Route::DoomProjects)}
                            class="text-left text-gray-300 hover:text-green-400 font-medium transition-colors duration-200 px-3 py-2 rounded-md hover:bg-black/30"
                        >
                            {"Doom Projects"}
                        </button>

                        <button
                            class="text-left text-gray-500 cursor-not-allowed font-medium px-3 py-2 rounded-md"
                            disabled={true}
                        >
                            {"Contact"}
                        </button>
                    </div>
                </div>
            </div>
            
            // bottom divider line
            <div 
                class="absolute top-full left-0 right-0 overflow-hidden h-2.5"
                style="background: url('/static/DIVIDER_3B.png') repeat-x top left; background-size: auto 100%;">
            </div>
        </header>
    }
}