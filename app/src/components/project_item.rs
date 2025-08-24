    // components/project_item.rs
    use yew::prelude::*;
    use web_sys::window;

    #[derive(Clone, PartialEq)]
    pub struct Tag {
        pub name: String,
        pub color: String,                  // css class e.g. bg-blue-500"
        pub text_color: Option<String>,     // optional text color override
    }

    #[derive(Properties, PartialEq)]
    pub struct ProjectItemProps {
        pub image_src: String,
        pub title: String,
        pub description: String,
        pub github_url: String,
        pub tags: Vec<Tag>,
        #[prop_or_default]
        pub image_alt: Option<String>,
        #[prop_or_default]
        pub detailed_description: Option<String>,
        #[prop_or_default]
        pub technologies_used: Vec<String>,
        #[prop_or_default]
        pub key_features: Vec<String>,
        #[prop_or_default]
        pub wiki_url: Option<String>,
        #[prop_or_default]
        pub additional_images: Vec<String>, 
    }

    #[function_component(ProjectItem)]
    pub fn project_item(props: &ProjectItemProps) -> Html {
        let alt_text = props.image_alt.as_ref()
            .unwrap_or(&props.title)
            .clone();

        let modal_open = use_state(|| false);
        let current_image_index = use_state(|| 0usize);

        // create combined list of images (based on availability)
        let all_images = if props.additional_images.is_empty() {
            // only main image if no additional images
            vec![props.image_src.clone()]
        } else {
            // only additional images if they exist
            props.additional_images.clone()
        };

        let more_info_click = {
            let modal_open = modal_open.clone();
            let current_image_index = current_image_index.clone();
            Callback::from(move |_| {
                modal_open.set(true);
                current_image_index.set(0); // reset to first image when opening modal
            })
        };

        let close_modal = {
            let modal_open = modal_open.clone();
            Callback::from(move |_| {
                modal_open.set(false);
            })
        };

        let prev_image = {
            let current_image_index = current_image_index.clone();
            let total_images = all_images.len();
            Callback::from(move |e: MouseEvent| {
                e.stop_propagation(); // prevent modal from closing
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
                e.stop_propagation(); // prevent modal from closing
                let current = *current_image_index;
                let new_index = (current + 1) % total_images;
                current_image_index.set(new_index);
            })
        };

        let wiki_click = {
            let demo_url = props.wiki_url.clone();
            Callback::from(move |_| {
                if let Some(url) = &demo_url {      // wiki url is optional, hence extra condition
                    if let Some(window) = window() {
                        let _ = window.open_with_url_and_target(url, "_blank");
                    }
                }
            })
        };
        
        let github_click = {
            let github_url = props.github_url.clone();
            Callback::from(move |_| {
                if let Some(window) = window() {
                    let _ = window.open_with_url_and_target(&github_url, "_blank");
                }
            })
        };

        let current_image_src = all_images.get(*current_image_index)
            .unwrap_or(&props.image_src)
            .clone();

        html! {
            <>  // fragment to group project item and modal
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
                        // title + tags row
                        <div class="flex justify-between items-start mb-3 gap-3">
                            // title (left)
                            <h3 class="text-xl font-bold text-red-600 font-mono flex-shrink-0">
                                {&props.title}
                            </h3>
                            
                            // tags (right)
                            <div class="flex flex-wrap gap-1 justify-end">
                                { for props.tags.iter().map(|tag| {
                                    let tag_classes = format!("px-2 py-1 rounded text-xs font-mono font-bold {} {}", 
                                        tag.color,
                                        tag.text_color.as_ref().unwrap_or(&"text-white".to_string())
                                    );
                                    
                                    html! {
                                        <span class={tag_classes}>
                                            {&tag.name}
                                        </span>
                                    }
                                })}
                            </div>
                        </div>
                        
                        // description
                        <p class="text-gray-300 mb-4 text-sm leading-relaxed">
                            {&props.description}
                        </p>
                        
                        // buttons row
                        <div class="flex justify-between items-start mb-3 gap-3">
                            // more info
                            <button 
                                onclick={more_info_click}
                                class="group w-full bg-gray-800 hover:bg-red-600 border-2 border-red-600 hover:border-red-600 text-red-600 hover:text-white font-bold py-2 px-4 rounded transition-all duration-200 cursor-pointer font-mono text-sm">
                                <div class="flex items-center justify-center gap-2">    
                                    <span>{"MORE INFO"}</span>
                                    <span class="text-xs group-hover:translate-x-1 transition-transform duration-200">{"→"}</span>
                                </div>
                            </button>
                            // github
                            <button 
                                onclick={github_click.clone()}
                                class="group w-full bg-gray-800 hover:bg-red-600 border-2 border-red-600 hover:border-red-600 text-red-600 hover:text-white font-bold py-2 px-4 rounded transition-all duration-200 cursor-pointer font-mono text-sm">
                                <div class="flex items-center justify-center gap-2">    
                                    <span>{"GITHUB"}</span>
                                    <span class="text-xs group-hover:translate-x-1 transition-transform duration-200">{"→"}</span>
                                </div>
                            </button>
                        </div>
                    </div>
                </div>

                // modal
                if *modal_open {
                    <div class="fixed inset-0 backdrop-blur-lg bg-black/50 bg-opacity-75 flex items-center justify-center p-4 z-50" onclick={close_modal.clone()}>
                        <div 
                            class="bg-gray-900 border-2 border-red-600 rounded-lg max-w-2xl w-full max-h-[90vh] overflow-y-auto"
                            onclick={Callback::from(|e: MouseEvent| e.stop_propagation())}
                        >
                            // modal header
                            <div class="flex justify-between items-center p-6 border-b border-gray-700">
                                <h2 class="text-2xl font-bold text-red-600 font-mono">{&props.title}</h2>
                                <button 
                                    onclick={close_modal}
                                    class="text-gray-400 hover:text-red-400 text-2xl font-bold transition-colors duration-200 cursor-pointer">
                                    {"×"}
                                </button>
                            </div>

                            // modal content
                            <div class="p-6 space-y-6">
                                // project image with navigation
                                <div class="relative">
                                    <div class="aspect-video bg-gray-800 rounded-lg overflow-hidden">
                                        <img 
                                            src={current_image_src}
                                            // alt={alt_text.clone()} TODO: FIX
                                            class="w-full h-full object-contain"
                                        />
                                    </div>
                                    
                                    // navigation buttons (only show if there are multiple images)
                                    if all_images.len() > 1 {
                                        <>
                                            // left arrow
                                            <button
                                                onclick={prev_image}
                                                class="absolute left-2 top-1/2 transform -translate-y-1/2 bg-black/70 hover:bg-black/90 text-white rounded-full p-2 transition-all duration-200 cursor-pointer z-10"
                                            >
                                                <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7"></path>
                                                </svg>
                                            </button>
                                            
                                            // right arrow
                                            <button
                                                onclick={next_image}
                                                class="absolute right-2 top-1/2 transform -translate-y-1/2 bg-black/70 hover:bg-black/90 text-white rounded-full p-2 transition-all duration-200 cursor-pointer z-10"
                                            >
                                                <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"></path>
                                                </svg>
                                            </button>
                                            
                                            // image counter
                                            <div class="absolute bottom-2 right-2 bg-black/70 text-white px-2 py-1 rounded text-sm font-mono">
                                                {format!("{}/{}", *current_image_index + 1, all_images.len())}
                                            </div>
                                        </>
                                    }
                                </div>

                                // detailed description
                                if let Some(detailed_desc) = &props.detailed_description {
                                    <div>
                                        <h3 class="text-lg font-bold text-red-600 font-mono mb-2">{"DESCRIPTION"}</h3>
                                        <p class="text-gray-300 leading-relaxed">{detailed_desc}</p>
                                    </div>
                                }

                                // technologies used
                                if !props.technologies_used.is_empty() {
                                    <div>
                                        <h3 class="text-lg font-bold text-red-600 font-mono mb-2">{"TECHNOLOGIES"}</h3>
                                        <div class="flex flex-wrap gap-2">
                                            { for props.technologies_used.iter().map(|tech| html! {
                                                <span class="px-3 py-1 bg-gray-800 border border-red-600 text-red-600 rounded font-mono text-sm">
                                                    {tech}
                                                </span>
                                            })}
                                        </div>
                                    </div>
                                }

                                // key features
                                if !props.key_features.is_empty() {
                                    <div>
                                        <h3 class="text-lg font-bold text-red-500 font-mono mb-2">{"KEY FEATURES"}</h3>
                                        <ul class="space-y-2">
                                            { for props.key_features.iter().map(|feature| html! {
                                                <li class="text-gray-300 flex items-start">
                                                    <span class="text-red-500 mr-2 font-mono">{"•"}</span>
                                                    <span>{feature}</span>
                                                </li>
                                            })}
                                        </ul>
                                    </div>
                                }

                                // links / buttons
                                <div class="flex gap-4 pt-4 border-t border-gray-700">
                                    <button 
                                        onclick={github_click.clone()}
                                        class="flex-1 bg-gray-800 hover:bg-red-600 border-2 border-red-600 hover:border-red-600 text-red-600 hover:text-white font-bold py-3 px-6 rounded cursor-pointer transition-all duration-200 font-mono"
                                    >
                                        {"GITHUB REPO"}
                                    </button>
                                    if props.wiki_url.is_some() {
                                        <button 
                                            onclick={wiki_click}
                                            class="flex-1 bg-red-600 hover:bg-red-700 text-white font-bold py-3 px-6 rounded cursor-pointer transition-all duration-200 font-mono"
                                        >
                                            {"PROJECT WIKI"}
                                        </button>
                                    }
                                </div>
                            </div>
                        </div>
                    </div>
                }
            </> // end fragment
        }
    }