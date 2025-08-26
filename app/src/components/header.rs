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
        let is_visible = use_state(|| !props.is_doom_projects_page); // start visible by default, hidden on doom projects page

        // animate header entrance when show prop changes
        {
            let is_visible = is_visible.clone();
            use_effect_with(props.is_doom_projects_page, move |is_doom_page| {
                if *is_doom_page {
                    // small delay for smooth animation
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

        let header_style = if *is_visible {
            "fixed top-0 left-0 right-0 w-full z-40 transform -translate-y-0 transition-transform duration-500 ease-out bg-black shadow-lg"
        } else {
            "fixed top-0 left-0 right-0 w-full z-40 transform -translate-y-full transition-transform duration-500 ease-out bg-black shadow-lg"
        };

        html! {
            <header class={header_style}>
                <div class="container mx-auto px-6 py-4">
                    <div class="flex items-center justify-between">
                        
                        // Logo/Brand section
                        <div class="flex items-center space-x-2">
                            <button 
                                onclick={navigate.reform(|_| Route::Home)}
                                class="text-2xl font-bold text-red-600 hover:text-blue-600 transition-colors duration-200"
                            >
                                {"KR"}
                            </button>
                        </div>

                        // Navigation links
                        <nav class="hidden md:flex items-center space-x-8">
                            
                            // Home
                            <button
                                onclick={navigate.reform(|_| Route::Home)}
                                class="text-red-600 hover:text-white font-medium transition-colors duration-200 px-3 py-2 rounded-md hover:bg-red-600"
                            >
                                {"Home"}
                            </button>

                            // Projects
                            <button
                                onclick={navigate.reform(|_| Route::Projects)}
                                class="text-red-600 hover:text-white font-medium transition-colors duration-200 px-3 py-2 rounded-md hover:bg-red-600"
                            >
                                {"Projects"}
                            </button>

                            // Doom Projects
                            <button
                                onclick={navigate.reform(|_| Route::DoomProjects)}
                                class="text-red-600 hover:text-white font-medium transition-colors duration-200 px-3 py-2 rounded-md hover:bg-red-600"
                            >
                                {"Doom Projects"}
                            </button>

                            // About (commented out like in footer)
                            <button
                                // onclick={navigate.reform(|_| Route::About)}
                                class="text-gray-400 cursor-not-allowed font-medium px-3 py-2 rounded-md"
                                disabled={true}
                            >
                                {"About"}
                            </button>

                            // Contact (commented out like in footer)
                            <button
                                // onclick={navigate.reform(|_| Route::Contact)}
                                class="text-gray-400 cursor-not-allowed font-medium px-3 py-2 rounded-md"
                                disabled={true}
                            >
                                {"Contact"}
                            </button>
                        </nav>

                        // Mobile menu button
                        <div class="md:hidden">
                            <button
                                class="text-gray-700 hover:text-blue-600 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-inset p-2 rounded-md"
                                onclick={Callback::from(|_| {
                                    // You can implement mobile menu toggle logic here
                                    web_sys::console::log_1(&"Mobile menu clicked".into());
                                })}
                            >
                                <svg class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" />
                                </svg>
                            </button>
                        </div>
                    </div>

                    // Mobile menu (hidden by default, you can expand this with state management)
                    <div class="md:hidden mt-4 pt-4 border-t border-gray-200 hidden">
                        <div class="flex flex-col space-y-2">
                            
                            <button
                                onclick={navigate.reform(|_| Route::Home)}
                                class="text-left text-gray-700 hover:text-blue-600 font-medium transition-colors duration-200 px-3 py-2 rounded-md hover:bg-gray-50"
                            >
                                {"Home"}
                            </button>

                            <button
                                onclick={navigate.reform(|_| Route::Projects)}
                                class="text-left text-gray-700 hover:text-blue-600 font-medium transition-colors duration-200 px-3 py-2 rounded-md hover:bg-gray-50"
                            >
                                {"Projects"}
                            </button>

                            <button
                                onclick={navigate.reform(|_| Route::DoomProjects)}
                                class="text-left text-gray-700 hover:text-blue-600 font-medium transition-colors duration-200 px-3 py-2 rounded-md hover:bg-gray-50"
                            >
                                {"Doom Projects"}
                            </button>

                            <button
                                class="text-left text-gray-400 cursor-not-allowed font-medium px-3 py-2 rounded-md"
                                disabled={true}
                            >
                                {"About"}
                            </button>

                            <button
                                class="text-left text-gray-400 cursor-not-allowed font-medium px-3 py-2 rounded-md"
                                disabled={true}
                            >
                                {"Contact"}
                            </button>
                        </div>
                    </div>
                </div>
                <div 
                    class="absolute top-full left-0 right-0 overflow-hidden h-2.5"
                    style="background: url('/static/DIVIDER_3B.png') repeat-x top left; background-size: auto 100%;">
                </div>
            </header>
        }
    }