// components/footer.rs
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct HudSectionProps {
    pub children: Children,
    pub background_image: String,     // path to hud section png
    pub background_width: u32,        // pixel width of hud section
    pub background_height: u32,       // pixel height of hud section
    pub text_color: String,
    #[prop_or_default]
    pub border_style: Option<String>,
    #[prop_or_default]
    pub width_class: Option<String>,
}

#[function_component(HudSection)]
pub fn hud_section(props: &HudSectionProps) -> Html {
    // flex width ratio based on hud section images
    let flex_style = format!("flex: {};", props.background_width);

    html! {
        <div
            class={format!(
                "relative {} flex items-center justify-center text-center",
                props.text_color,
            )}
            style={format!(
                "background-image: url('{}'); \
                 background-repeat: no-repeat; \
                 background-size: 100% 100%; \
                 image-rendering: pixelated; \
                 height: 10vw; {};",
                props.background_image,
                flex_style
            )}
        >
            <div class="z-10">
                { for props.children.iter() }
            </div>
        </div>
    }
}

#[function_component(Footer)]
pub fn footer() -> Html {

    let button_click = Callback::from(|_| {
        // do something
        log::info!("Button clicked");
    });

    html! {
        <footer class="fixed bottom-0 left-0 right-0 w-full z-50">
            <div class="flex w-full">
                
                // home
                <HudSection 
                    background_image="/static/STBAR1.png"
                    background_width=48
                    background_height=32
                    text_color="text-red-400"
                    width_class="flex-1"
                    border_style="border-r-2 border-gray-600">
                    <button 
                        onclick={button_click.clone()} 
                        class="group w-full h-full flex items-center justify-center cursor-pointer bg-transparent border-none">
                        <img 
                            src="/static/HOME1.png" 
                            alt="Home"
                            class="block transition-opacity duration-200 ease-in-out group-hover:opacity-0"
                        />
                        <img 
                            src="/static/HOME2.png" 
                            alt="Home Hover"
                            class="block absolute opacity-0 transition-opacity duration-200 ease-in-out group-hover:opacity-100"
                        />
                    </button>
                </HudSection>

                // projects
                <HudSection
                    background_image="/static/STBAR2.png"
                    background_width=58
                    background_height=32
                    text_color="text-red-400"
                    width_class="flex-1"
                    border_style="border-r-2 border-gray-600">
                    <button 
                        onclick={button_click.clone()} 
                        class="group w-full h-full flex items-center justify-center cursor-pointer bg-transparent border-none">
                        <img 
                            src="/static/PROJECTS1.png" 
                            alt="Home"
                            class="w-4/5 h-auto block transition-opacity duration-0 ease-in-out group-hover:opacity-0"
                        />
                        <img 
                            src="/static/PROJECTS2.png" 
                            alt="Home Hover"
                            class="w-4/5 h-auto block absolute opacity-0 transition-opacity duration-0 ease-in-out group-hover:opacity-100"
                        />
                    </button>
                </HudSection>

                // about
                <HudSection
                    background_image="/static/STBAR3.png"
                    background_width=36
                    background_height=32
                    text_color="text-yellow-400"
                    width_class="flex-1"
                    border_style="border-r-2 border-gray-600">
                    <button 
                        onclick={button_click.clone()} 
                        class="group w-full h-full flex items-center justify-center cursor-pointer bg-transparent border-none">
                        <img 
                            src="/static/ABOUT1.png" 
                            alt="About"
                            class="w-4/5 h-auto block transition-opacity duration-0 ease-in-out group-hover:opacity-0"
                        />
                        <img 
                            src="/static/ABOUT2.png" 
                            alt="About"
                            class="w-4/5 h-auto block absolute opacity-0 transition-opacity duration-0 ease-in-out group-hover:opacity-100"
                        />
                    </button>
                </HudSection>

                // avatar
                <HudSection
                    background_image="/static/STBAR4.png"
                    background_width=37
                    background_height=32
                    text_color="text-white"
                    width_class="flex-1"
                    border_style="border-r-2 border-gray-600">
                    <button 
                        onclick={button_click.clone()} 
                        class="group w-full h-full flex items-center justify-center cursor-pointer bg-transparent border-none">
                        <img 
                            src="/static/AVATAR_1.png" 
                            alt="Avatar"
                            class="w-4/5 block absolute transition-opacity duration-0 ease-in-out group-hover:opacity-0"
                        />
                        <img 
                            src="/static/AVATAR_2.png" 
                            alt="Avatar"
                            class="w-4/5 block absolute opacity-0 transition-opacity duration-0 ease-in-out group-hover:opacity-100"
                        />
                    </button>
                </HudSection>

                // doom projects
                <HudSection
                    background_image="/static/STBAR5.png"
                    background_width=57
                    background_height=32
                    text_color="text-red-400"
                    width_class="flex-1"
                    border_style="border-r-2 border-gray-600">
                    <button 
                        onclick={button_click.clone()} 
                        class="group w-full h-full flex items-center justify-center cursor-pointer bg-transparent border-none">
                        <img 
                            src="/static/DOOM_PROJECTS1.png" 
                            alt="Doom Projects"
                            class="w-4/5 h-auto block transition-opacity duration-0 ease-in-out group-hover:opacity-0"
                        />
                        <img 
                            src="/static/DOOM_PROJECTS2.png" 
                            alt="Doom Projects"
                            class="w-4/5 h-auto block absolute opacity-0 transition-opacity duration-0 ease-in-out group-hover:opacity-100"
                        />
                    </button>
                </HudSection>

                // keys section
                <HudSection
                    background_image="/static/STBAR6.png"
                    background_width=13 
                    background_height=32
                    text_color="text-red-400"
                    width_class="flex-1"
                    border_style="border-r-2 border-gray-600">
                    <div class="flex flex-col">
                        <span class="text-2xl font-bold">{format!("0%")}</span>
                        <span class="text-xs">{"KEYS"}</span>
                    </div>
                </HudSection>

                // contact
                <HudSection
                    background_image="/static/STBAR7.png"
                    background_width=71
                    background_height=32
                    text_color="text-blue-400"
                    width_class="flex-1"
                    border_style="border-r-2 border-gray-600">
                    <button 
                        onclick={button_click.clone()} 
                        class="group w-full h-full flex items-center justify-center cursor-pointer bg-transparent border-none">
                        <img 
                            src="/static/CONTACT1.png" 
                            alt="Home"
                            class="block transition-opacity duration-0 ease-in-out group-hover:opacity-0"
                        />
                        <img 
                            src="/static/CONTACT2.png" 
                            alt="Home Hover"
                            class="block absolute opacity-0 transition-opacity duration-0 ease-in-out group-hover:opacity-100"
                        />
                    </button>
                </HudSection>
            </div>
        </footer>
    }
}