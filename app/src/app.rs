use yew::prelude::*;
use yew_router::prelude::*; 
use crate::router::Route;
use crate::pages::home::Home;

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
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

