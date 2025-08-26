use yew::prelude::*;
use yew_router::prelude::*; 
use crate::router::Route;
use crate::pages::home::Home;
use crate::pages::projects::Projects;
use crate::pages::doom_projects::DoomProjects;

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

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            // <Layout>
                <Switch<Route> render={switch} />
            // </Layout>
        </BrowserRouter>
    }
}

