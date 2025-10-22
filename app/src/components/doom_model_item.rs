// components/doom_model_item.rs
use yew::prelude::*;
use crate::components::model_viewer::ModelViewer;

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

    html! {
        <>
            <div class="max-w-sm hover:scale-105 transition-all duration-300">
                <div 
                    class="relative overflow-hidden shadow-lg hover:shadow-xl transition-all duration-300"
                    style="background-image: url('/static/common/STBAR_MID.png'); 
                            background-repeat: no-repeat; 
                            background-size: 100% 100%; 
                            image-rendering: pixelated;
                            min-height: 400px;"
                >
                    // inner black overlay box
                    <div 
                        class="absolute inset-0 m-3 z-5 bg-[#1a1a1a] bg-opacity-60 border-4 border-[#0b0b0a]"
                    ></div>
                    
                    // content
                    <div class="relative z-10 p-6 h-full flex flex-col">
                        // preview image
                        <div class="aspect-video bg-[#2b2b2b] overflow-hidden rounded mb-4">
                            <img 
                                src={props.preview_image.clone()}
                                alt={props.title.clone()}
                                class="w-full h-full object-contain image-rendering-pixelated"
                            />
                        </div>
                        
                        // title
                        <h3 class="text-xl font-bold text-red-600 font-mono mb-3">
                            {&props.title}
                        </h3>
                        
                        // description
                        <p class="text-gray-300 mb-4 text-sm leading-relaxed flex-grow">
                            {&props.description}
                        </p>
                        
                        // buttons row
                        <div class="flex justify-between items-start mb-3 gap-3">
                            // more info
                            <button 
                                onclick={open_modal}
                                class="group w-full bg-[#2b2b2b] hover:bg-red-600 border-2 border-red-600 hover:border-red-600 text-red-600 hover:text-white font-bold py-2 px-4 rounded transition-all duration-200 cursor-pointer font-mono text-sm">
                                <div class="flex items-center justify-center gap-2">    
                                    <span>{"VIEW MODEL"}</span>
                                    <span class="text-xs group-hover:translate-x-1 transition-transform duration-200">{"→"}</span>
                                </div>
                            </button>
                            // download
                            if props.download_url.is_some() {
                                <button 
                                    onclick={download_click.clone()}
                                    class="group w-full bg-[#2b2b2b] hover:bg-green-600 border-2 border-green-600 hover:border-green-600 text-green-600 hover:text-white font-bold py-2 px-4 rounded transition-all duration-200 cursor-pointer font-mono text-sm">
                                    <div class="flex items-center justify-center gap-2">    
                                        <span>{"DOWNLOAD"}</span>
                                        <span class="text-xs group-hover:translate-x-1 transition-transform duration-200">{"↓"}</span>
                                    </div>
                                </button>
                            } else {
                                <button 
                                    class="group w-full bg-[#2b2b2b] hover:bg-gray-600 border-2 border-gray-500 hover:border-gray-400 text-gray-400 hover:text-gray-300 font-bold py-2 px-4 rounded transition-all duration-200 cursor-not-allowed font-mono text-sm">
                                    <div class="flex items-center justify-center gap-2">    
                                        <span class="group-hover:hidden">{"DOWNLOAD"}</span>
                                        <span class="hidden group-hover:inline">{"COMING SOON"}</span>
                                        <span class="text-xs">{"↓"}</span>
                                    </div>
                                </button>
                            }
                        </div>
                    </div>
                </div>
            </div>

            // modal with 3D viewer
            if *modal_open {
                <div 
                    class="fixed inset-0 backdrop-blur-lg bg-black/60 flex items-center justify-center z-50 p-4"
                    onclick={close_modal.clone()}
                >
                    <div 
                        class="bg-[#1a1a1a] border-3 border-red-600 rounded-lg max-w-4xl w-full max-h-[90vh] overflow-y-auto"
                        onclick={Callback::from(|e: MouseEvent| e.stop_propagation())}
                    >
                        // modal header
                        <div class="flex justify-between items-center p-6 border-b border-gray-400/50">
                            <h2 class="text-2xl font-bold text-red-600 font-mono">{&props.title}</h2>
                            <button 
                                onclick={close_modal}
                                class="text-gray-400 hover:text-red-600 text-4xl font-bold transition-colors duration-200 cursor-pointer z-20 bg-black/50 rounded-full w-12 h-12 flex items-center justify-center leading-none"
                            >
                                <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
                                </svg>
                            </button>
                        </div>

                        // modal content
                        <div class="p-6 space-y-6">
                            // 3d model viewer
                            <div class="relative flex justify-center items-center bg-[#2b2b2b] rounded-lg p-4">
                                <div class="cursor-grab hover:cursor-grab active:cursor-grabbing">
                                    <ModelViewer 
                                        model_name={props.model_name.clone()}
                                        width=800
                                        height=400
                                        front_cam=false
                                    />
                                </div>

                                // overlay instructions (attached to outer viewer area)
                                <div class="absolute bottom-3 right-4 bg-black/60 text-gray-300 text-xs font-mono px-2 py-1 rounded-lg pointer-events-none select-none">
                                    {"Drag to move, scroll to zoom"}
                                </div>
                            </div>

                            // // detailed description if available
                            // if let Some(credit) = &props.credits {
                            //     <div>
                            //         <h3 class="text-lg font-bold text-red-600 font-mono mb-2">{"CREDITS"}</h3>
                            //         <p class="text-gray-300 leading-relaxed whitespace-pre-line">{credit}</p>
                            //     </div>
                            // }

                            // model info
                            <div class="grid grid-cols-4 gap-4">
                                <div class="bg-[#2b2b2b] p-4 rounded-lg col-span-1">
                                    <h4 class="text-sm font-mono text-red-500 mb-2">{"MODEL INFO"}</h4>
                                    <div class="space-y-2 text-gray-300 text-sm">
                                        if let Some(file_size) = &props.file_size {
                                            <div class="flex flex-col">
                                                <span class="text-gray-400 text-xs">{"File Size:"}</span>
                                                <span class="font-mono">{file_size}</span>
                                            </div>
                                        }
                                        <div class="flex flex-col">
                                            <span class="text-gray-400 text-xs">{"Format:"}</span>
                                            <span class="font-mono">{"OBJ"}</span>
                                        </div>
                                    </div>
                                </div>

                                <div class="bg-[#2b2b2b] p-4 rounded-lg col-span-3">
                                    <h4 class="text-sm font-mono text-red-500 mb-2">{"CREDITS"}</h4>
                                    <div class="space-y-1 text-gray-300 text-sm">
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
                                    <button 
                                        onclick={download_click}
                                        class="w-full bg-[#2b2b2b] hover:bg-green-600 border-2 border-green-600 hover:border-green-600 text-green-600 hover:text-white font-bold py-3 px-6 rounded cursor-pointer transition-all duration-200 font-mono"
                                    >
                                        {"DOWNLOAD MODEL"}
                                    </button>
                                } else {
                                    <button 
                                        class="w-full bg-[#2b2b2b] border-2 border-gray-500 text-gray-400 font-bold py-3 px-6 rounded cursor-not-allowed font-mono"
                                    >
                                        {"DOWNLOAD COMING SOON"}
                                    </button>
                                }
                            </div>
                        </div>
                    </div>
                </div>
            }
        </>
    }
}