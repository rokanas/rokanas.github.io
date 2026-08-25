// pages/doom_projects.rs
use yew::prelude::*;
use web_sys::window;
use crate::components::doom_map_item::{DoomMapItem};
use crate::components::doom_model_item::{DoomModelItem};
use crate::components::heading::{Heading};
use crate::data::doom_projects::{all_maps, all_models};

// TODO: make smaller thumbnails if page loads slowly (involves refactoring image_src to thumbnail_src)
#[function_component(DoomProjects)]
pub fn doom_projects() -> Html {
    use_effect_with((), {
        move |_| {
            // scroll to top when component mounts
            if let Some(window) = window() {
                window.scroll_to_with_x_and_y(0.0, 0.0);
            }
            || {}
        }
    });

    let maps = all_maps();
    let models = all_models();

    html! {
        <main class="min-h-screen text-white pt-8 pb-10">
            <div class="container mx-auto pt-4 px-4 max-w-7xl">
                
                // page heading
                <Heading 
                    src="/static/doom_projects/DOOM_MAPS_1.webp" 
                    alt="Doom Maps"
                    sub_heading="All maps made for limit-removing source ports and tested in GZDoom."
                />

                // projects grid
                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8 justify-items-center pb-14">
                    { for maps.iter().map(|map| html! {
                        <DoomMapItem
                            title={map.title.clone()}
                            description={map.description.clone()}
                            image_src={map.image_src.clone()}
                            image_alt={map.image_alt.clone()}
                            additional_images={map.additional_images.clone()}
                            border={map.border.clone()}
                        />
                    })}
                </div>

                <Heading 
                    src="/static/doom_projects/DOOM_MODELS_1.webp" 
                    alt="Doom Models"
                    sub_heading="All models made using Ultimate Doom Builder and Blender."
                />

                // models grid
                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8 justify-items-center">
                    { for models.iter().map(|model| html! {
                        <DoomModelItem
                            title={model.title.clone()}
                            description={model.description.clone()}
                            preview_image={model.preview_image.clone()}
                            model_name={model.model_name.clone()}
                            download_url={model.download_url.clone()}
                            file_size={model.file_size.clone()}
                            credits={model.credits.clone()}
                            border={model.border.clone()}
                        />
                    })}
                </div>

                // footer note
                <div class="text-center mt-10 text-gray-500">
                    <p class="font-mono text-sm">
                        {"More projects available on my "} 
                        <a 
                            href="https://github.com/rokanas" 
                            target="_blank" 
                            class="text-red-600 hover:text-red-300 underline transition-colors duration-200"
                        >
                            {"GitHub profile"}
                        </a>
                    </p>
                </div>
            </div>
        </main>
    }
}