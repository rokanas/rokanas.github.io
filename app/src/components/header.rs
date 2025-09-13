use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::Route;
use crate::components::header_button::HeaderButton;

#[derive(Properties, PartialEq)]
pub struct HeaderProps {
    #[prop_or(true)]
    pub show: bool,
}

#[hook]
fn use_navigation() -> Callback<Route> {
    let navigator = use_navigator().unwrap();
    Callback::from(move |route: Route| navigator.push(&route))
}

#[function_component(Header)]
pub fn header(props: &HeaderProps) -> Html {
    let navigate = use_navigation();
    let is_visible = use_state(|| false);

    {
        let is_visible = is_visible.clone();
        use_effect_with(props.show, move |show| {
            if *show {
                let is_visible_clone = is_visible.clone();
                gloo_timers::callback::Timeout::new(50, move || is_visible_clone.set(true)).forget();
            } else {
                is_visible.set(false);
            }
            || {}
        });
    }

    if !props.show {
        return html! {};
    }

    // header slides in
    let header_class = if *is_visible {
        "fixed top-0 left-0 right-0 w-full z-40 transform -translate-y-0 transition-transform duration-500 ease-out overflow-visible"
    } else {
        "fixed top-0 left-0 right-0 w-full z-40 transform -translate-y-full transition-transform duration-500 ease-out overflow-visible"
    };

    html! {
        <header class={header_class}
            style="background-image:url('/static/SHAWN_2.png');background-repeat:repeat;background-size:60px;">
            <div class="w-full px-2 sm:px-4 lg:px-8 pt-0 pb-1">
                <div class="w-full max-w-screen-2xl mx-auto">
                    <div class="flex items-center justify-between w-full relative min-h-[56px] sm:min-h-[62px]">

                        // left nav
                        <div class="hidden md:flex items-center gap-2 lg:gap-8 flex-1 justify-start">
                            <HeaderButton src="/static/header/HOME_SRB" alt_text="Home" route={Route::Home} />
                            <HeaderButton src="/static/header/ABOUT_SRB" alt_text="About" route={Route::About} />
                            <HeaderButton src="/static/header/PROJECTS_SRB" alt_text="Projects" route={Route::Projects} />
                        </div>

                        // center logo
                         <div class="flex-1 md:flex-0 flex justify-center items-start z-50 -mt-1 -mb-4 sm:-mb-5 pl-3 pr-3">
                            <div class="relative w-40 sm:w-44 lg:w-52 h-20 sm:h-22 lg:h-26">
                                // border
                                <div class="absolute inset-0"
                                    style="background:#c20000ff;
                                            clip-path:polygon(0% 0%,100% 0%,85% 100%,15% 100%);">
                                </div>
                                // background
                                <div class="absolute inset-1"
                                    style="background:linear-gradient(135deg,#1a1a1a 0%,#2a2a2a 50%,#1a1a1a 100%);
                                            clip-path:polygon(0% 0%,100% 0%,85% 100%,15% 100%);
                                            box-shadow:0 0 20px rgba(74,222,128,.3),
                                                    inset 0 0 20px rgba(0,0,0,.5);">
                                </div>
                                // logo image
                                <button onclick={navigate.reform(|_| Route::Home)}
                                        class="absolute inset-0 flex items-center justify-center cursor-pointer">
                                    <img src="/static/KR_1.png" alt="Home"
                                        class="max-h-14 sm:max-h-18 lg:max-h-22 transition-transform duration-300 drop-shadow-lg hover:scale-110 hover:brightness-125"/>
                                </button>
                            </div>
                        </div>

                        // right nav
                        <div class="hidden md:flex items-center gap-2 lg:gap-4 flex-1 justify-end">
                            <HeaderButton src="/static/header/DOOM_PROJECTS_SRB" alt_text="Doom Projects" route={Route::DoomProjects} />
                            <HeaderButton src="/static/header/CONTACT_SRB" alt_text="Contact" route={Route::Home} disabled={true} />
                        </div>

                        // mobile menu button
                        <div class="md:hidden absolute right-4">
                            <button
                                class="text-gray-300 hover:text-red-600 focus:outline-none focus:ring-2 focus:ring-red-600 focus:ring-inset p-2 rounded-md"
                                onclick={Callback::from(|_| web_sys::console::log_1(&"Mobile menu clicked".into()))}>
                                <svg class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" />
                                </svg>
                            </button>
                        </div>
                    </div>
                </div>

                // mobile menu
                <div class="md:hidden mt-3 pt-3 border-t border-gray-600 hidden">
                    <div class="flex flex-col space-y-2">
                        <button onclick={navigate.reform(|_| Route::Home)} class="text-left text-gray-300 hover:text-green-400 font-medium transition-colors duration-200 px-3 py-2 rounded-md hover:bg-black/30">{"Home"}</button>
                        <button onclick={navigate.reform(|_| Route::About)} class="text-left text-gray-300 hover:text-green-400 font-medium transition-colors duration-200 px-3 py-2 rounded-md hover:bg-black/30">{"About"}</button>
                        <button onclick={navigate.reform(|_| Route::Projects)} class="text-left text-gray-300 hover:text-green-400 font-medium transition-colors duration-200 px-3 py-2 rounded-md hover:bg-black/30">{"Projects"}</button>
                        <button onclick={navigate.reform(|_| Route::DoomProjects)} class="text-left text-gray-300 hover:text-green-400 font-medium transition-colors duration-200 px-3 py-2 rounded-md hover:bg-black/30">{"Doom Projects"}</button>
                        <button class="text-left text-gray-500 cursor-not-allowed font-medium px-3 py-2 rounded-md" disabled=true>{"Contact"}</button>
                    </div>
                </div>
            </div>

            // bottom divider
            <div class="absolute top-full left-0 right-0 overflow-hidden h-2.5 z-10"
                 style="background:url('/static/DIVIDER_3B.png') repeat-x top left; background-size:auto 100%;">
            </div>
        </header>
    }
}