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
    }

    #[function_component(ProjectItem)]
    pub fn project_item(props: &ProjectItemProps) -> Html {
        let github_click = {
            let github_url = props.github_url.clone();
            Callback::from(move |_| {
                if let Some(window) = window() {
                    let _ = window.open_with_url_and_target(&github_url, "_blank");
                }
            })
        };

        let alt_text = props.image_alt.as_ref()
            .unwrap_or(&props.title)
            .clone();

        html! {
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
                    
                    // github button
                    <button 
                        onclick={github_click}
                        class="group w-full bg-gray-800 hover:bg-red-600 border-2 border-red-600 hover:border-red-600 text-red-600 hover:text-white font-bold py-2 px-4 rounded transition-all duration-200 cursor-pointer font-mono text-sm"
                    >
                        <div class="flex items-center justify-center gap-2">    
                            <span>{"GITHUB"}</span>
                            <span class="text-xs group-hover:translate-x-1 transition-transform duration-200">{"→"}</span>
                        </div>
                    </button>
                </div>
            </div>
        }
    }