use yew::prelude::*;
use yew_router::prelude::*; 
use crate::router::Route;
use crate::pages::home::Home;
use crate::pages::projects::Projects;
use crate::pages::doom_projects::DoomProjects;
use crate::components::header::Header;
use crate::components::footer::Footer;

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Projects => html! { <Projects /> },
        //Route::About => html! { <About /> },
        //Route::Avatar => html! { <Avatar /> },
        Route::DoomProjects => html! { <DoomProjects /> },
        //Route::Contact => html! { <Contact /> },
        Route::NotFound => html! { <div>{"404 - Page not found"}</div> },
    }
}

// separate component to use use_route hook, which only works inside router context i.e. must be child component of BrowserRouter
// contrast with previous state: use_route was called directly in app(), always returning none
#[function_component(AppContent)]
pub fn app_content() -> Html {
    let route = use_route::<Route>().unwrap_or(Route::Home);
    let is_doom_projects = matches!(route, Route::DoomProjects);
    
    // TODO: remove debug logs
    // web_sys::console::log_1(&format!("Current route: {:?}", route).into());
    // web_sys::console::log_1(&format!("Is doom projects: {}", is_doom_projects).into());

    html! {
        <>
            // header visible in all pages except doom projects
            <Header 
                show={true}
                is_doom_projects_page={is_doom_projects}
            />
            
            <main class={if is_doom_projects { "" } else { "pt-20 bg-black" }}>
                <Switch<Route> render={switch} />
            </main>

            // footer only visible in doom projects page
            <Footer show={is_doom_projects} />
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

