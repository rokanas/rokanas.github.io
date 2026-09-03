// components/project_item.rs
use yew::prelude::*;
use web_sys::window;
use crate::components::card_shell::CardShell;
use crate::components::cta_button::{CtaButton, ButtonVariant, ButtonSize};
use crate::components::modal_shell::ModalShell;
use crate::hooks::use_image_carousel;

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

    // create combined list of images (based on availability)
    let all_images = if props.additional_images.is_empty() {
        // only main image if no additional images
        vec![props.image_src.clone()]
    } else {
        // only additional images if they exist
        props.additional_images.clone()
    };

    let (current_image_index, current_image_src, prev_image, next_image, reset_carousel) =
        use_image_carousel(all_images.clone());

    let more_info_click = {
        let modal_open = modal_open.clone();
        let reset_carousel = reset_carousel.clone();
        Callback::from(move |_| {
            modal_open.set(true);
            reset_carousel.emit(()); // reset to first image when opening modal
        })
    };

    let close_modal = {
        let modal_open = modal_open.clone();
        Callback::from(move |_| {
            modal_open.set(false);
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

    let header = html! {
        <div class="flex justify-between items-start mb-3 gap-3">
            // title (left)
            <h3 class="text-xl font-bold text-doom-red font-mono flex-shrink-0">
                {&props.title}
            </h3>

            // tags (right)
            <div class="flex flex-wrap gap-1 justify-end">
                { for props.tags.iter().map(|tag| {
                    let tag_classes = format!("px-2 py-1 rounded text-xs font-mono font-bold {} {}",
                        tag.color,
                        tag.text_color.as_ref().unwrap_or(&"text-doom-white".to_string())
                    );

                    html! {
                        <span class={tag_classes}>
                            {&tag.name}
                        </span>
                    }
                })}
            </div>
        </div>
    };

    html! {
        <>  // fragment to group project item and modal
            <CardShell
                border="/static/common/STBAR_MID.webp"
                image_src={props.image_src.clone()}
                image_alt={alt_text}
                description={props.description.clone()}
                header={header}
            >
                // more info
                <CtaButton onclick={more_info_click} variant={ButtonVariant::Primary}>
                    <div class="flex items-center justify-center gap-2">
                        <span>{"MORE INFO"}</span>
                        <span class="text-xs group-hover:translate-x-1 transition-transform duration-200">{"→"}</span>
                    </div>
                </CtaButton>
                // github
                <CtaButton onclick={github_click.clone()} variant={ButtonVariant::Primary}>
                    <div class="flex items-center justify-center gap-2">
                        <span>{"GITHUB"}</span>
                        <span class="text-xs group-hover:translate-x-1 transition-transform duration-200">{"→"}</span>
                    </div>
                </CtaButton>
            </CardShell>

            // modal
            if *modal_open {
                <ModalShell
                    title={props.title.clone()}
                    onclick_backdrop={close_modal.clone()}
                    close_button={html! {
                        <button
                            onclick={close_modal}
                            class="text-doom-gray-dark hover:text-doom-red text-2xl font-bold transition-colors duration-200 cursor-pointer">
                            {"×"}
                        </button>
                    }}
                >
                    // detailed description
                    if let Some(detailed_desc) = &props.detailed_description {
                        <div>
                            // <h3 class="text-lg font-bold text-doom-red font-mono mb-2">{"DESCRIPTION"}</h3>
                            <p class="text-doom-gray-light leading-relaxed whitespace-pre-line">{detailed_desc}</p>
                        </div>
                    }

                    // project image with navigation
                    <div class="relative">
                        <div class="aspect-video bg-doom-panel-button rounded-lg overflow-hidden">
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
                                    class="absolute left-2 top-1/2 transform -translate-y-1/2 bg-black/70 hover:bg-black/90 text-doom-white rounded-full p-2 transition-all duration-200 cursor-pointer z-10"
                                >
                                    <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7"></path>
                                    </svg>
                                </button>

                                // right arrow
                                <button
                                    onclick={next_image}
                                    class="absolute right-2 top-1/2 transform -translate-y-1/2 bg-black/70 hover:bg-black/90 text-doom-white rounded-full p-2 transition-all duration-200 cursor-pointer z-10"
                                >
                                    <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"></path>
                                    </svg>
                                </button>

                                // image counter
                                <div class="absolute bottom-2 right-2 bg-black/70 text-doom-white px-2 py-1 rounded text-sm font-mono">
                                    {format!("{}/{}", current_image_index + 1, all_images.len())}
                                </div>
                            </>
                        }
                    </div>

                    // technologies used
                    if !props.technologies_used.is_empty() {
                        <div>
                            <h3 class="text-lg font-bold text-doom-red font-mono mb-2">{"TECHNOLOGIES"}</h3>
                            <div class="flex flex-wrap gap-2">
                                { for props.technologies_used.iter().map(|tech| html! {
                                    <span class="px-3 py-1 bg-doom-panel-button border border-doom-red text-doom-red rounded font-mono text-sm">
                                        {tech}
                                    </span>
                                })}
                            </div>
                        </div>
                    }

                    // key features
                    if !props.key_features.is_empty() {
                        <div>
                            <h3 class="text-lg font-bold text-doom-red font-mono mb-2">{"KEY FEATURES"}</h3>
                            <ul class="space-y-2">
                                { for props.key_features.iter().map(|feature| html! {
                                    <li class="text-doom-gray-light flex items-start">
                                        <span class="text-doom-red mr-2 font-mono">{"•"}</span>
                                        <span>{feature}</span>
                                    </li>
                                })}
                            </ul>
                        </div>
                    }

                    // links / buttons
                    <div class="flex gap-4 pt-4 border-t border-gray-700">
                        <CtaButton onclick={github_click.clone()} variant={ButtonVariant::Primary} size={ButtonSize::Lg} class="flex-1">
                            {"GITHUB REPO"}
                        </CtaButton>
                        if props.wiki_url.is_some() {
                            <CtaButton onclick={wiki_click} variant={ButtonVariant::Primary} size={ButtonSize::Lg} class="flex-1">
                                {"PROJECT WIKI"}
                            </CtaButton>
                        }
                    </div>
                </ModalShell>
            }
        </> // end fragment
    }
}