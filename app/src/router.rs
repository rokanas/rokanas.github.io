use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]        // default page
    Home,
    #[at("/projects")]
    Projects,
    #[not_found]
    #[at("/404")]
    NotFound
}