    // components/doom_project_item.rs
    use yew::prelude::*;

    #[derive(Properties, PartialEq)]
    pub struct ProjectItemProps {
        pub image_src: String,
        pub title: String,
        pub description: String,
        #[prop_or_default]
        pub image_alt: Option<String>,
        #[prop_or_default]
        pub additional_images: Vec<String>, 
    }

    #[function_component(DoomProjectItem)]
    pub fn project_item(props: &ProjectItemProps) -> Html {
        let alt_text = props.image_alt.as_ref()
            .unwrap_or(&props.title)
            .clone();

        let lightbox_open = use_state(|| false);
        let current_image_index = use_state(|| 0usize);

        // create combined list of all images (main image + additional images)
        let all_images = {
            let mut images = vec![props.image_src.clone()];
            images.extend(props.additional_images.iter().cloned());
            images
        };

        let gallery_click = {
            let lightbox_open = lightbox_open.clone();
            let current_image_index = current_image_index.clone();
            Callback::from(move |_| {
                lightbox_open.set(true);
                current_image_index.set(0); // reset to first image when opening lightbox
            })
        };

        let close_lightbox = {
            let lightbox_open = lightbox_open.clone();
            Callback::from(move |_| {
                lightbox_open.set(false);
            })
        };

        let prev_image = {
            let current_image_index = current_image_index.clone();
            let total_images = all_images.len();
            Callback::from(move |e: MouseEvent| {
                e.stop_propagation(); // prevent lightbox from closing
                let current = *current_image_index;
                let new_index = if current == 0 {
                    total_images - 1
                } else {
                    current - 1
                };
                current_image_index.set(new_index);
            })
        };

        let next_image = {
            let current_image_index = current_image_index.clone();
            let total_images = all_images.len();
            Callback::from(move |e: MouseEvent| {
                e.stop_propagation(); // prevent lightbox from closing
                let current = *current_image_index;
                let new_index = (current + 1) % total_images;
                current_image_index.set(new_index);
            })
        };
        
        let _download_click = {
            // TODO: IMPLEMENT
        };

        let current_image_src = all_images.get(*current_image_index)
            .unwrap_or(&props.image_src)
            .clone();

        html! {
            <>  // fragment to group project item and lightbox
                <div class="bg-gray-900 border-2 border-gray-700 rounded-lg overflow-hidden shadow-lg hover:shadow-xl transition-all duration-300 hover:scale-105 max-w-sm">
                    // project image
                    <div class="aspect-video bg-gray-800 overflow-hidden">
                        <img 
                            src={props.image_src.clone()}
                            alt={alt_text}
                            class="w-full h-full object-contain hover:scale-110 transition-transform duration-500 image-rendering-pixelated"
                        />
                    </div>
                    
                    // project content
                    <div class="p-6">
                        // title
                        <h3 class="text-xl font-bold text-red-600 font-mono mb-3">
                            {&props.title}
                        </h3>
                        
                        // description
                        <p class="text-gray-300 mb-4 text-sm leading-relaxed">
                            {&props.description}
                        </p>
                        
                        // buttons row
                        <div class="flex justify-between items-start mb-3 gap-3">
                            // more info
                            <button 
                                onclick={gallery_click}
                                class="group w-full bg-gray-800 hover:bg-red-600 border-2 border-red-600 hover:border-red-600 text-red-600 hover:text-white font-bold py-2 px-4 rounded transition-all duration-200 cursor-pointer font-mono text-sm">
                                <div class="flex items-center justify-center gap-2">    
                                    <span>{"VIEW GALLERY"}</span>
                                    <span class="text-xs group-hover:translate-x-1 transition-transform duration-200">{"→"}</span>
                                </div>
                            </button>
                            // download
                            <button 
                                // onclick={download_click.clone()}
                                class="group w-full bg-gray-700 hover:bg-gray-600 border-2 border-gray-500 hover:border-gray-400 text-gray-400 hover:text-gray-300 font-bold py-2 px-4 rounded transition-all duration-200 cursor-not-allowed font-mono text-sm">
                                <div class="flex items-center justify-center gap-2">    
                                    <span class="group-hover:hidden">{"DOWNLOAD"}</span>
                                    <span class="hidden group-hover:inline">{"COMING SOON"}</span>
                                    <span class="text-xs group-hover:translate-x-1 transition-transform duration-200">{"→"}</span>
                                </div>
                            </button>
                        </div>
                    </div>
                </div>

                // lightbox
                if *lightbox_open {
                    <div 
                        class="fixed inset-0 backdrop-blur-lg bg-black/75 flex items-center justify-center z-50"
                        onclick={close_lightbox.clone()}
                    >
                        // close button (top-right)
                        <button
                            onclick={close_lightbox.clone()}
                            class="absolute top-4 right-4 text-white hover:text-red-600 text-4xl font-bold transition-colors duration-200 cursor-pointer z-20 bg-black/50 rounded-full w-12 h-12 flex items-center justify-center"
                        >
                            {"×"}
                        </button>

                        // image container
                        <div 
                            class="relative w-full h-full flex items-center justify-center p-4">
                            
                            
                            // main image display
                            <img
                                src={current_image_src}
                                alt={format!("{} - Image {}", props.title, *current_image_index + 1)}
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
                                    <div class="text-sm">{format!("{} / {}", *current_image_index + 1, all_images.len())}</div>
                                </div>
                            </div>
                        </div>
                    </div>
                }
            </> // end fragment
        }
    }