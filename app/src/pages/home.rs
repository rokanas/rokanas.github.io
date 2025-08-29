use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div class="min-h-screen flex flex-col">
            <main class="flex-grow flex flex-col items-center justify-center">
                <img class="logo" src="https://yew.rs/img/logo.svg" alt="Yew logo" />
                <h1 class="text-white">{ "Hello World!" }</h1>
            </main>
        </div>
    }
}