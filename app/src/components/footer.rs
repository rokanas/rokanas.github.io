// components/footer.rs
use yew::prelude::*;

// #[function_component(Footer)]
// pub fn footer() -> Html {
//     html! {
//         <footer class="bg-gray-800 text-white text-center p-4 fixed bottom-0 left-0 right-0 w-full">
//             <div class="container mx-auto">
//                 <p class="text-sm">{ "© 2025 K. Rokanas. All rights reserved." }</p>
//             </div>
//         </footer>
//     }
// }

// #[function_component(Footer)]
// pub fn footer() -> Html {
//     html! {
//         <footer class="fixed bottom-0 left-0 right-0 w-full">
//             <img 
//                 src="/static/footer.jpg" 
//                 alt="Footer" 
//                 class="w-full h-auto"
//             />
//         </footer>
//     }
// }

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
                 // height: clamp(60px, 15vw, 150px); {};", // min 60px, pref 15% viewport, max 150px
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

#[derive(Properties, PartialEq)]
pub struct FooterProps {
    #[prop_or_default]
    pub ammo_count: i32,
    #[prop_or_default] 
    pub health_percent: i32,
    #[prop_or_default]
    pub armor_percent: i32,
    #[prop_or_default]
    pub weapon_slots: Vec<i32>,
}

#[function_component(Footer)]
pub fn footer(props: &FooterProps) -> Html {
    html! {
        <footer class="fixed bottom-0 left-0 right-0 w-full z-50">
            <div class="flex w-full bg-gray-800 border-t-4 border-gray-600">
                
                // Ammo Section
                <HudSection 
                    background_image="/static/STBAR1.png"
                    background_width=48
                    background_height=32
                    text_color="text-red-400"
                    // width_class="w-32"    <-- this is static width
                    width_class="flex-1"
                    border_style="border-r-2 border-gray-600">
                    <div class="flex flex-col">
                        <span class="text-2xl font-bold">{props.ammo_count}</span>
                        <span class="text-xs">{"AMMO"}</span>
                    </div>
                </HudSection>

                // Health Section  
                <HudSection
                    background_image="/static/STBAR2.png"
                    background_width=58
                    background_height=32
                    text_color="text-red-400"
                    width_class="flex-1"
                    border_style="border-r-2 border-gray-600">
                    <div class="flex flex-col">
                        <span class="text-2xl font-bold">{format!("{}%", props.health_percent)}</span>
                        <span class="text-xs">{"HEALTH"}</span>
                    </div>
                </HudSection>

                // Weapon Slots Section
                <HudSection
                    background_image="/static/STBAR3.png"
                    background_width=36
                    background_height=32
                    text_color="text-yellow-400"
                    width_class="flex-1"
                    border_style="border-r-2 border-gray-600">
                    <div class="grid grid-cols-3 gap-1 text-xs font-bold">
                        {
                            (1..=9).map(|i| {
                                let has_weapon = props.weapon_slots.contains(&i);
                                html! {
                                    <span class={if has_weapon { "text-yellow-400" } else { "text-gray-500" }}>
                                        {i}
                                    </span>
                                }
                            }).collect::<Html>()
                        }
                    </div>
                </HudSection>

                // Character Portrait Section
                <HudSection
                    background_image="/static/STBAR4.png"
                    background_width=37
                    background_height=32
                    text_color="text-white"
                    width_class="flex-1"
                    border_style="border-r-2 border-gray-600">
                    <div class="w-12 h-12 bg-gray-600 border border-gray-500 flex items-center justify-center">
                        // placeholder for character portrait
                        <span class="text-xs">{"FACE"}</span>
                    </div>
                </HudSection>

                // Armor Section
                <HudSection
                    background_image="/static/STBAR5.png"
                    background_width=57
                    background_height=32
                    text_color="text-red-400"
                    width_class="flex-1"
                    border_style="border-r-2 border-gray-600">
                    <div class="flex flex-col">
                        <span class="text-2xl font-bold">{format!("{}%", props.armor_percent)}</span>
                        <span class="text-xs">{"ARMOR"}</span>
                    </div>
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
                        <span class="text-2xl font-bold">{format!("{}%", props.armor_percent)}</span>
                        <span class="text-xs">{"KEYS"}</span>
                    </div>
                </HudSection>

                // Stats Section (expandable)
                <HudSection
                    background_image="/static/STBAR7.png"
                    background_width=71
                    background_height=32
                    text_color="text-blue-400"
                    width_class="flex-1"
                    border_style="border-r-2 border-gray-600">
                    <div class="grid grid-cols-2 gap-4 text-sm">
                        <div class="flex justify-between">
                            <span class="text-blue-400">{"BULL"}</span>
                            <span class="text-yellow-400">{"195"}</span>
                        </div>
                        <div class="flex justify-between">
                            <span class="text-blue-400">{"SHEL"}</span>
                            <span class="text-yellow-400">{"51"}</span>
                        </div>
                        <div class="flex justify-between">
                            <span class="text-blue-400">{"ROKT"}</span>
                            <span class="text-yellow-400">{"51"}</span>
                        </div>
                        <div class="flex justify-between">
                            <span class="text-blue-400">{"CELL"}</span>
                            <span class="text-yellow-400">{"340"}</span>
                        </div>
                    </div>
                </HudSection>
            </div>
        </footer>
    }
}

// Usage example with default values
impl Default for FooterProps {
    fn default() -> Self {
        Self {
            ammo_count: 145,
            health_percent: 40,
            armor_percent: 107,
            weapon_slots: vec![2, 3, 4, 5, 6, 7],
        }
    }
}