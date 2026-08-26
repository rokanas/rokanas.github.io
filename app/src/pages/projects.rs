// pages/projects.rs
use yew::prelude::*;
use web_sys::window;
use crate::components::heading::{Heading};
use crate::components::project_item::ProjectItem;
use crate::data::projects::all_projects;

#[function_component(Projects)]
pub fn projects() -> Html {
    use_effect_with((), |_| {
        // scroll to top when component mounts
        if let Some(window) = window() {
            window.scroll_to_with_x_and_y(0.0, 0.0);
        }
        || {}
    });

    let projects = all_projects();

    html! {
        <main 
            class="min-h-screen text-white pt-8 pb-10" 
        >
            <div class="container mx-auto pt-4 px-4 max-w-7xl">
                
                // page heading
                <Heading 
                    src="/static/projects/PROJECTS_1.webp" 
                    alt="Projects"
                    sub_heading="A collection of my software projects, both personal and academic."
                />

                // projects grid
                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8 justify-items-center">
                    { for projects.iter().map(|project| html! {
                        <ProjectItem
                            title={project.title.clone()}
                            tags={project.tags.clone()}
                            description={project.description.clone()}
                            image_src={project.image_src.clone()}
                            image_alt={project.image_alt.clone()}
                            github_url={project.github_url.clone()}
                            detailed_description={project.detailed_description.clone()}
                            technologies_used={project.technologies_used.clone()}
                            key_features={project.key_features.clone()}
                            wiki_url={project.wiki_url.clone()}
                            additional_images={project.additional_images.clone()}
                        />
                    })}
                </div>

                // footer note                                       // TODO: add padding so footer doesn't overlap
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