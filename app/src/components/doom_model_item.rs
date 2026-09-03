// components/doom_model_item.rs
use yew::prelude::*;
use crate::components::model_viewer::ModelViewer;
use crate::components::card_shell::CardShell;
use crate::components::cta_button::{CtaButton, ButtonVariant, ButtonSize};
use crate::components::modal_shell::ModalShell;

#[derive(Properties, PartialEq)]
pub struct DoomModelItemProps {
    pub title: String,
    pub description: String,
    pub preview_image: String,
    pub model_name: String,
    #[prop_or_default]
    pub download_url: Option<String>,
    #[prop_or_default]
    pub file_size: Option<String>,
    #[prop_or_default]
    pub credits: Option<String>,
    pub border: String,
}

#[function_component(DoomModelItem)]
pub fn doom_model_item(props: &DoomModelItemProps) -> Html {
    let modal_open = use_state(|| false);

    let open_modal = {
        let modal_open = modal_open.clone();
        Callback::from(move |_| {
            modal_open.set(true);
        })
    };

    let close_modal = {
        let modal_open = modal_open.clone();
        Callback::from(move |_| {
            modal_open.set(false);
        })
    };

    let download_click = {
        let download_url = props.download_url.clone();
        Callback::from(move |_| {
            if let Some(url) = &download_url {
                if let Some(window) = web_sys::window() {
                    let _ = window.open_with_url_and_target(url, "_blank");
                }
            }
        })
    };

    let header = html! {
        <h3 class="text-xl font-bold text-doom-red font-mono mb-3">
            {&props.title}
        </h3>
    };

    html! {
        <>
            <CardShell
                border={props.border.clone()}
                image_src={props.preview_image.clone()}
                image_alt={props.title.clone()}
                description={props.description.clone()}
                header={header}
            >
                // more info
                <CtaButton onclick={open_modal} variant={ButtonVariant::Primary}>
                    <div class="flex items-center justify-center gap-2">
                        <span>{"VIEW MODEL"}</span>
                        <span class="text-xs group-hover:translate-x-1 transition-transform duration-200">{"→"}</span>
                    </div>
                </CtaButton>
                // download
                if props.download_url.is_some() {
                    <CtaButton onclick={download_click.clone()} variant={ButtonVariant::Success}>
                        <div class="flex items-center justify-center gap-2">
                            <span>{"DOWNLOAD"}</span>
                            <span class="text-xs group-hover:translate-x-1 transition-transform duration-200">{"↓"}</span>
                        </div>
                    </CtaButton>
                } else {
                    <CtaButton variant={ButtonVariant::Secondary}>
                        <div class="flex items-center justify-center gap-2">
                            <span class="group-hover:hidden">{"DOWNLOAD"}</span>
                            <span class="hidden group-hover:inline">{"COMING SOON"}</span>
                            <span class="text-xs">{"↓"}</span>
                        </div>
                    </CtaButton>
                }
            </CardShell>

            // modal with 3D viewer
            if *modal_open {
                <ModalShell
                    title={props.title.clone()}
                    onclick_backdrop={close_modal.clone()}
                    close_button={html! {
                        <button
                            onclick={close_modal}
                            class="text-doom-gray-dark hover:text-doom-red text-4xl font-bold transition-colors duration-200 cursor-pointer z-20 bg-black/50 rounded-full w-12 h-12 flex items-center justify-center leading-none"
                        >
                            <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
                            </svg>
                        </button>
                    }}
                >
                    // 3d model viewer
                    <div class="relative flex justify-center items-center bg-doom-panel-button rounded-lg p-4">
                        <div class="cursor-grab hover:cursor-grab active:cursor-grabbing">
                            <ModelViewer
                                model_name={props.model_name.clone()}
                                width=800
                                height=400
                                front_cam=false
                            />
                        </div>

                        // overlay instructions (attached to outer viewer area)
                        <div class="absolute bottom-3 right-4 bg-black/60 text-doom-gray-light text-xs font-mono px-2 py-1 rounded-lg pointer-events-none select-none">
                            {"Drag to move, scroll to zoom"}
                        </div>
                    </div>

                    // model info
                    <div class="grid grid-cols-4 gap-4">
                        <div class="bg-doom-panel-button p-4 rounded-lg col-span-1">
                            <h4 class="text-sm font-mono text-doom-red mb-2">{"MODEL INFO"}</h4>
                            <div class="space-y-2 text-doom-gray-light text-sm">
                                if let Some(file_size) = &props.file_size {
                                    <div class="flex flex-col">
                                        <span class="text-doom-gray-dark text-xs">{"File Size:"}</span>
                                        <span class="font-mono">{file_size}</span>
                                    </div>
                                }
                                <div class="flex flex-col">
                                    <span class="text-doom-gray-dark text-xs">{"Format:"}</span>
                                    <span class="font-mono">{"OBJ"}</span>
                                </div>
                            </div>
                        </div>

                        <div class="bg-doom-panel-button p-4 rounded-lg col-span-3">
                            <h4 class="text-sm font-mono text-doom-red mb-2">{"CREDITS"}</h4>
                            <div class="space-y-1 text-doom-gray-light text-sm">
                                if let Some(credit) = &props.credits {
                                    <div class="flex flex-col">
                                        <span class="font-mono whitespace-pre-line">{credit}</span> // whitespace-pre-line makes /n be respected
                                    </div>
                                }
                            </div>
                        </div>
                    </div>

                    // download button
                    <div class="pt-4 border-t border-gray-700">
                        if props.download_url.is_some() {
                            <CtaButton onclick={download_click} variant={ButtonVariant::Success} size={ButtonSize::Lg} class="w-full">
                                {"DOWNLOAD MODEL"}
                            </CtaButton>
                        } else {
                            <CtaButton variant={ButtonVariant::Secondary} size={ButtonSize::Lg} class="w-full">
                                {"DOWNLOAD COMING SOON"}
                            </CtaButton>
                        }
                    </div>
                </ModalShell>
            }
        </>
    }
}