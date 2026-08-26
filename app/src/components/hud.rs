// components/hud.rs
use yew::prelude::*;                    // import everything from yew prelude (html macros, hooks, components, etc)

use crate::router::Route;               // import route enum for page navigation
use crate::components::hud_section::HudSection;
use crate::components::hud_button::HudButton;
use crate::components::hud_avatar::HudAvatar;
use crate::hooks::use_slide_visibility;

// props to control footer visibility and animation
#[derive(Properties, PartialEq)]
pub struct HudProps {
    #[prop_or(false)]
    pub show: bool,
}

#[function_component(Hud)]   // declare function as footer component
pub fn hud(props: &HudProps) -> Html {
    let (is_visible, should_render) = use_slide_visibility(props.show);

    // don't render if should_render is false
    if !should_render {
        return html! {};
    }

    let footer_classes = if is_visible {
        "fixed bottom-0 left-0 right-0 w-full z-40 transform translate-y-0 transition-transform duration-500 ease-out hidden sm:block"
    } else {
        "fixed bottom-0 left-0 right-0 w-full z-40 transform translate-y-full transition-transform duration-500 ease-out hidden sm:block"
    };

    html! { // macro to create html structure                       // start html block
        <footer class={footer_classes}>  // fixed position at bottom with high z-index (so it appears above other content)
            <div class="flex w-full">                               // flex container div taking full horizontal width
                
                // home
                <HudSection 
                    background_image="/static/hud/section/STBAR1.webp"
                    background_width=48
                    text_color="text-red-600"
                    route={Route::Home}>
                    <HudButton
                        src="/static/hud/button/HOME_SR"
                        alt_text="Home"
                        route={Route::Home}
                        disabled=false>
                    </HudButton>
                </HudSection>

                // about
                <HudSection
                    background_image="/static/hud/section/STBAR2B.webp"
                    background_width=36
                    text_color="text-red-600"
                    route={Route::Projects}>
                    <HudButton
                        src="/static/hud/button/ABOUT_SR"
                        alt_text="About"
                        route={Route::About}
                        disabled=false>
                    </HudButton>
                </HudSection>

                // projects
                <HudSection
                    background_image="/static/hud/section/STBAR3B.webp"
                    background_width=58
                    text_color="text-yellow-600"
                    route={Route::About}>
                    <HudButton
                        src="/static/hud/button/PROJECTS_SR"
                        alt_text="Projects"
                        route={Route::Projects}
                        disabled=false>
                    </HudButton>
                </HudSection>

                // avatar
                <HudSection
                    background_image="/static/hud/section/STBAR4.webp"
                    background_width=37
                    text_color="text-white">
                    <HudAvatar />
                </HudSection>

                // doom projects
                <HudSection
                    background_image="/static/hud/section/STBAR5.webp"
                    background_width=57
                    text_color="text-red-600"
                    route={Route::DoomProjects}>
                    <HudButton
                        src="/static/hud/button/DOOM_PROJECTS_SR"
                        alt_text="Doom Projects"
                        route={Route::DoomProjects}
                        disabled=false>
                    </HudButton>
                </HudSection>

                // keys section
                <HudSection
                    background_image="/static/hud/section/STBAR6.webp"
                    background_width=13
                    text_color="text-red-600">
                    <div class="flex flex-col">
                    </div>
                </HudSection>

                // contact
                <HudSection
                    background_image="/static/hud/section/STBAR7.webp"
                    background_width=71
                    text_color="text-blue-600"
                    route={Route::Contact}>
                    <HudButton
                        src="/static/hud/button/CONTACT_SR"
                        alt_text="Contact"
                        route={Route::Contact}>
                    </HudButton>
                </HudSection>
            </div>
        </footer>
    }
}