// components/doom_map_item.rs
use yew::prelude::*;
use crate::components::card_shell::CardShell;
use crate::hooks::use_image_carousel;

#[derive(Properties, PartialEq)]
pub struct ProjectItemProps {
    pub image_src: String,
    pub title: String,
    pub description: String,
    #[prop_or_default]
    pub image_alt: Option<String>,
    #[prop_or_default]
    pub additional_images: Vec<String>, 
    pub border: String,
}

#[function_component(DoomMapItem)]
pub fn project_item(props: &ProjectItemProps) -> Html {
    let alt_text = props.image_alt.as_ref()
        .unwrap_or(&props.title)
        .clone();

    let lightbox_open = use_state(|| false);

    // create combined list of all images (main image + additional images)
    let all_images = {
        let mut images = vec![props.image_src.clone()];
        images.extend(props.additional_images.iter().cloned());
        images
    };

    let (current_image_index, current_image_src, prev_image, next_image, reset_carousel) =
        use_image_carousel(all_images.clone());

    let gallery_click = {
        let lightbox_open = lightbox_open.clone();
        let reset_carousel = reset_carousel.clone();
        Callback::from(move |_| {
            lightbox_open.set(true);
            reset_carousel.emit(()); // reset to first image when opening lightbox
        })
    };

    let close_lightbox = {
        let lightbox_open = lightbox_open.clone();
        Callback::from(move |_| {
            lightbox_open.set(false);
        })
    };

    let header = html! {
        <h3 class="text-xl font-bold text-red-600 font-mono mb-3">
            {&props.title}
        </h3>
    };

    html! {
        <>  // fragment to group project item and lightbox
            <CardShell
                border={props.border.clone()}
                image_src={props.image_src.clone()}
                image_alt={alt_text}
                description={props.description.clone()}
                header={header}
            >
                // view gallery
                <button
                    onclick={gallery_click}
                    class="group w-full bg-[#2b2b2b] hover:bg-red-600 border-2 border-red-600 hover:border-red-600 text-red-600 hover:text-white font-bold py-2 px-4 rounded transition-all duration-200 cursor-pointer font-mono text-sm">
                    <div class="flex items-center justify-center gap-2">
                        <span>{"VIEW GALLERY"}</span>
                        <span class="text-xs group-hover:translate-x-1 transition-transform duration-200">{"→"}</span>
                    </div>
                </button>
                // download (coming soon)
                <button
                    // onclick={download_click.clone()}
                    class="group w-full bg-[#2b2b2b] hover:bg-gray-600 border-2 border-gray-500 hover:border-gray-400 text-gray-400 hover:text-gray-300 font-bold py-2 px-4 rounded transition-all duration-200 cursor-not-allowed font-mono text-sm">
                    <div class="flex items-center justify-center gap-2">
                        <span class="group-hover:hidden">{"DOWNLOAD"}</span>
                        <span class="hidden group-hover:inline">{"COMING SOON"}</span>
                        <span class="text-xs group-hover:translate-x-1 transition-transform duration-200">{"→"}</span>
                    </div>
                </button>
            </CardShell>

            // lightbox
            if *lightbox_open {
                <div 
                    class="fixed inset-0 backdrop-blur-lg bg-black/60 flex items-center justify-center z-50"
                    onclick={close_lightbox.clone()}
                >
                    // close button (top-right)
                        <button
                            onclick={close_lightbox.clone()}
                            class="absolute top-4 right-4 text-white hover:text-red-600 text-4xl font-bold transition-colors duration-200 cursor-pointer z-20 bg-black/50 rounded-full w-12 h-12 flex items-center justify-center leading-none"
                        >
                            // svg 'x'
                            <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
                            </svg>
                        </button>

                    // image container
                    <div 
                        class="relative w-full h-full flex items-center justify-center p-4">
                        
                        // main image display
                        <img
                            src={current_image_src}
                            alt={format!("{} - Image {}", props.title, current_image_index + 1)}
                            class="max-w-full max-h-full object-contain"
                            style="image-rendering: pixelated;"
                            onclick={Callback::from(|e: MouseEvent| e.stop_propagation())}
                        />

                        // navigation buttons (only show if there are multiple images)
                        if all_images.len() > 1 {
                            <>
                                // left arrow
                                <button
                                    onclick={prev_image}
                                    class="absolute left-4 top-1/2 transform -translate-y-1/2 bg-black/70 hover:bg-black/90 text-white rounded-full p-3 transition-all duration-200 cursor-pointer z-10"
                                >
                                    <svg class="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7"></path>
                                    </svg>
                                </button>

                                // right arrow  
                                <button
                                    onclick={next_image}
                                    class="absolute right-4 top-1/2 transform -translate-y-1/2 bg-black/70 hover:bg-black/90 text-white rounded-full p-3 transition-all duration-200 cursor-pointer z-10"
                                >
                                    <svg class="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"></path>
                                    </svg>
                                </button>

                            </>
                        }

                        // image counter and title
                        <div class="absolute bottom-6 left-1/2 transform -translate-x-1/2 bg-black/70 text-white px-4 py-2 rounded-lg">
                            <div class="text-center font-mono">
                                <div class="text-lg font-bold text-red-600">{&props.title}</div>
                                <div class="text-sm">{format!("{} / {}", current_image_index + 1, all_images.len())}</div>
                            </div>
                        </div>
                    </div>
                </div>
            }
        </> // end fragment
    }
}