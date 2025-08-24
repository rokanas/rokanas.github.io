use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]        // default page
    Home,
    #[at("/projects")]
    Projects,
    //#[at("/about")]
    //About,
    //#[at("/avatar")]
    //Avatar,
    #[at("/doom-projects")]
    DoomProjects,
    //#[at("/contact")]
    //Contact,
    #[not_found]
    #[at("/404")]
    NotFound
}