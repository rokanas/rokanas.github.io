// components/card_shell.rs
use yew::prelude::*;

// shared visual shell for project/doom-map/doom-model cards: bordered panel,
// dark overlay, image, then a caller-supplied header (title, or title+tags)
// and description, with children rendered into the buttons row.
#[derive(Properties, PartialEq)]
pub struct CardShellProps {
    pub border: String,
    pub image_src: String,
    pub image_alt: String,
    pub description: String,
    pub header: Html,
    pub children: Children,
}

#[function_component(CardShell)]
pub fn card_shell(props: &CardShellProps) -> Html {
    html! {
        <div class="max-w-sm hover:scale-105 transition-all duration-300">
            <div
                class="relative overflow-hidden shadow-lg hover:shadow-xl transition-all duration-300 bg-pixel-panel"
                style={format!("background-image: url({}); min-height: 400px;", props.border)}
            >
                // inner black overlay box
                <div
                    class="absolute inset-0 m-3 z-5 bg-[#1a1a1a] bg-opacity-60 border-4 border-[#0b0b0a]"
                ></div>

                // content
                <div class="relative z-10 p-6 h-full flex flex-col">
                    // card image
                    <div class="aspect-video bg-[#2b2b2b] overflow-hidden rounded mb-4">
                        <img
                            src={props.image_src.clone()}
                            alt={props.image_alt.clone()}
                            class="w-full h-full object-contain image-rendering-pixelated"
                        />
                    </div>

                    { props.header.clone() }

                    // description
                    <p class="text-gray-300 mb-4 text-sm leading-relaxed flex-grow">
                        {&props.description}
                    </p>

                    // buttons row
                    <div class="flex justify-between items-start mb-3 gap-3">
                        { for props.children.iter() }
                    </div>
                </div>
            </div>
        </div>
    }
}
