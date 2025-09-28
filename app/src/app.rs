// app/src/app.rs
use yew::prelude::*;
use yew_router::prelude::*; 
use web_sys::{window}; 
use gloo_events::EventListener;
use crate::router::Route;
use crate::pages::home::Home;
use crate::pages::about::About;
use crate::pages::projects::Projects;
use crate::pages::doom_projects::DoomProjects;
use crate::pages::contact::Contact;
use crate::components::header::Header;
use crate::components::hud::Hud;
use crate::components::navbar_toggle::NavbarToggle;
use crate::components::fade_wrapper::FadeWrapper;

// context for navbar style
#[derive(Clone, PartialEq)]
pub struct NavbarContext {
    pub is_default_navbar: bool,
    pub toggle: Callback<()>,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { 
            <FadeWrapper>
                <Home /> 
            </FadeWrapper>
        },
        Route::Projects => html! { 
            <FadeWrapper>
                <Projects /> 
            </FadeWrapper>
        },
        Route::About => html! { 
            <FadeWrapper>
                <About /> 
            </FadeWrapper>
        },
        //Route::Avatar => html! { <Avatar /> },
        Route::DoomProjects => html! { 
            <FadeWrapper>
                <DoomProjects /> 
            </FadeWrapper>
        },
        Route::Contact => html! { 
            <FadeWrapper>
                <Contact /> 
            </FadeWrapper>
         },
        Route::NotFound => html! { 
            <FadeWrapper>
                <div>{"404 - Page not found"}</div> 
            </FadeWrapper>
        },
    }
}

// separate component to use use_route hook, which only works inside router context i.e. must be child component of BrowserRouter
// contrast with previous state: use_route was called directly in app(), always returning none
#[function_component(AppContent)]
pub fn app_content() -> Html {
    let route = use_route::<Route>().unwrap_or(Route::Home);
    let is_home = matches!(route, Route::Home);

    // state tracking navbar style
    let is_default_navbar = use_state(||true);
    
    // toggle navbar function
    let toggle_navbar = {
        let is_default_navbar = is_default_navbar.clone();
        Callback::from(move |_| {
            is_default_navbar.set(!*is_default_navbar);
        })
    };

    // create context value
    let navbar_context = NavbarContext {
        is_default_navbar: *is_default_navbar,
        toggle: toggle_navbar.clone(),
    };

    // special styles for specific pages
    let mut main_classes = String::new();

    if is_home {
        main_classes.push_str("h-screen overflow-hidden ");
    } else {
        // smooth transition when navbar changes
        main_classes.push_str("transition-all duration-500 ease-in-out ");

        // padding top/bottom depending on navbar style
        if *is_default_navbar {
            main_classes.push_str("pt-20");
        } else {
            main_classes.push_str("pb-35");
        }
    }

    // detect if mobile / small screen size
    let is_mobile = use_state(|| {
        window()
            .and_then(|w| w.inner_width().ok())
            .and_then(|w| w.as_f64())
            .map(|w| w < 640.0)
            .unwrap_or(false)
    });

    // Listen for screen resize
    {
        let is_mobile = is_mobile.clone();
        use_effect_with((), move |_| {
            let window_obj = window().unwrap();
            let listener = EventListener::new(&window_obj, "resize", move |_| {
                let width = window() // This is the function call
                    .and_then(|w| w.inner_width().ok())
                    .and_then(|w| w.as_f64())
                    .unwrap_or(1024.0);
                is_mobile.set(width < 640.0);
            });
            move || drop(listener)
        });
    }

    html! {
        <ContextProvider<NavbarContext> context={navbar_context}>
            <Header show={*is_mobile || *is_default_navbar} />  // header always shown if mobile
            <Hud show={!*is_default_navbar}   />                

            <NavbarToggle 
                is_default_navbar={*is_default_navbar} 
                on_toggle={toggle_navbar} 
            />
            
            <main 
                class={main_classes} 
                style="background-image: url('/static/common/FLOOR4_9.png'); background-repeat: repeat; background-size: 290px; image-rendering: pixelated;"
            >
                <div key={format!("{:?}", route)}>      // key forces remount on route change, triggering use_effect in pages (yew doesn't unmount/remount on route change by default)
                    <Switch<Route> render={switch} />
                </div>
            </main>
        </ContextProvider<NavbarContext>>
        }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <AppContent />
        </BrowserRouter>
    }
}

