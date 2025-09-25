// app/src/app.rs
use yew::prelude::*;
use yew_router::prelude::*; 
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

    // special styles for specific pages
    let mut main_classes = String::new();

    if *is_default_navbar {
        main_classes.push_str("pt-20");
    } else {
        main_classes.push_str("pb-35");
    }
    
    if is_home {
        main_classes = "h-screen overflow-hidden".to_string();
    }

    html! {
        <>
            // header visible in all pages except doom projects
            <Header show={*is_default_navbar} />

            <NavbarToggle 
                is_default_navbar={*is_default_navbar} 
                on_toggle={toggle_navbar} 
            />
            
            // add padding to the top to compensate for header (unless home or doom projects pages)
            <main 
                class={main_classes} 
                style="background-image: url('/static/FLOOR4_9.png'); background-repeat: repeat; background-size: 290px; image-rendering: pixelated;"
            >
                <div key={format!("{:?}", route)}>      // key forces remount on route change, triggering use_effect in pages (yew doesn't unmount/remount on route change by default)
                    <Switch<Route> render={switch} />
                </div>
            </main>

            // footer only visible in doom projects page
            <Hud show={!*is_default_navbar} />
        </>
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

