// pages/projects.rs
use yew::prelude::*;
use crate::components::project_item::{ProjectItem, Tag};
use crate::components::footer::Footer;

// struct to hold project data
#[derive(Clone, PartialEq)]
pub struct Project {
    pub title: String,
    pub tags: Vec<Tag>,
    pub description: String,
    pub image_src: String,
    pub image_alt: Option<String>,
    pub github_url: String,
}

#[function_component(Projects)]
pub fn projects() -> Html {
    // project item definitions (temporarily hardcoded, can be moved to db later)
    let projects = vec![
        Project {
            title: "Skinscan".to_string(),
            tags: vec![
                Tag { name: "Python".to_string(), color: "bg-blue-500".to_string(), text_color: None },
                Tag { name: "Typescript".to_string(), color: "bg-blue-800".to_string(), text_color: None },
            ],
            description: "A web app for classifying skin lesions using AI, made for DIT826 Software Engineering for Data-Intensive AI Applications course.".to_string(),
            image_src: "/static/projects/skinscan_logo.png".to_string(),
            image_alt: Some("Skinscan logo".to_string()),
            github_url: "https://github.com/rokanas/skinscan".to_string(),
        },
        Project {
            title: "Dentago".to_string(),
            tags: vec![
                Tag { name: "Javascript".to_string(), color: "bg-yellow-500".to_string(), text_color: None },
            ],
            description: "A distributed system for booking dentist appointments, made for DIT356 Distributed Systems Development course.".to_string(),
            image_src: "/static/projects/dentago_component_diagram.png".to_string(),
            github_url: "https://github.com/rokanas/dentago".to_string(),
            image_alt: Some("Dentago component diagram".to_string()),
        },
        Project {
            title: "Terminarium".to_string(),
            tags: vec![
                Tag { name: "C++".to_string(), color: "bg-pink-500".to_string(), text_color: None },
                Tag { name: "Javascript".to_string(), color: "bg-yellow-500".to_string(), text_color: None },
            ],
            description: "A monitoring system for terrariums using the Wio Terminal, made for DIT043 Object-Oriented Programming course. Link: https://terminarium.netlify.app/".to_string(),
            image_src: "/static/projects/terminarium_logo.png".to_string(),
            github_url: "https://github.com/rokanas/terminarium".to_string(),
            image_alt: Some("Terminarium logo".to_string()),
        },
        Project {
            title: "Zulubot".to_string(),
            tags: vec![
                Tag { name: "Python".to_string(), color: "bg-blue-500".to_string(), text_color: None },
            ],
            description: "A multi-purpose discord bot made for Zulu Empire server.".to_string(),
            image_src: "/static/projects/zulubot_logo.jpg".to_string(),
            github_url: "https://github.com/rokanas/zulubot".to_string(),
            image_alt: Some("Zulu empire logo".to_string()),
        },
        Project {
            title: "Wio Terminal Keyboard".to_string(),
            tags: vec![
                Tag { name: "C++".to_string(), color: "bg-pink-500".to_string(), text_color: None },
            ],
            description: "A fully-functioning onscreen keyboard for the Wio Terminal that registers user input".to_string(),
            image_src: "/static/projects/wio_terminal_keyboard_1.png".to_string(),
            github_url: "https://github.com/rokanas/wio-terminal-keyboard".to_string(),
            image_alt: Some("Wio terminal keyboard".to_string()),
        },
    ];

    html! {
        <main class="min-h-screen bg-black text-white pt-8 pb-24"> // pb-24 for footer space
            <div class="container mx-auto px-4 max-w-7xl">
                
                // page header
                <div class="text-center mb-12">
                    <img 
                        src="/static/PROJECTS_1.png" 
                        alt="Projects"
                        class="w-auto h-auto mx-auto"
                    />
                    <img 
                        src="/static/DIVIDER.png" 
                        alt="Projects"
                        class="w-auto h-auto mx-auto mb-4 -mt-6"
                    />
                    <p class="text-gray-300 text-lg max-w-2xl mx-auto leading-relaxed">
                        {"A collection of my software projects, both personal and academic."}
                    </p>
                </div>

                // projects grid
                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8 justify-items-center">
                    { for projects.iter().map(|project| html! {
                        <ProjectItem
                            title={project.title.clone()}
                            tags={project.tags.clone()}
                            description={project.description.clone()}
                            image_src={project.image_src.clone()}
                            github_url={project.github_url.clone()}
                            image_alt={project.image_alt.clone()}
                        />
                    })}
                </div>

                // footer note
                <div class="text-center mt-16 text-gray-500">
                    <p class="font-mono text-sm">
                        {"More projects available on my "} 
                        <a 
                            href="https://github.com/rokanas" 
                            target="_blank" 
                            class="text-red-400 hover:text-red-300 underline transition-colors duration-200"
                        >
                            {"GitHub profile"}
                        </a>
                    </p>
                </div>
            </div>
            <Footer />
        </main>
    }
}