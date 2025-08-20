// pages/projects.rs
use yew::prelude::*;
use crate::components::project_item::ProjectItem;
use crate::components::footer::Footer;

// Struct to hold project data
#[derive(Clone, PartialEq)]
pub struct Project {
    pub title: String,
    pub description: String,
    pub image_src: String,
    pub github_url: String,
    pub image_alt: Option<String>,
}

#[function_component(Projects)]
pub fn projects() -> Html {
    // Define your projects data here
    let projects = vec![
        Project {
            title: "Skinscan".to_string(),
            description: "A web app for classifying skin lesions using AI, made for DIT826 Software Engineering for Data-Intensive AI Applications course.".to_string(),
            image_src: "/static/project1.png".to_string(),
            github_url: "https://github.com/rokanas/skinscan".to_string(),
            image_alt: Some("Screenshot of Rust web framework".to_string()),
        },
        Project {
            title: "Dentago".to_string(),
            description: "A distributed system for booking dentist appointments, made for DIT356 Distributed Systems Development course.".to_string(),
            image_src: "/static/project2.png".to_string(),
            github_url: "https://github.com/rokanas/dentago".to_string(),
            image_alt: Some("Game engine interface screenshot".to_string()),
        },
        Project {
            title: "Terminarium".to_string(),
            description: "A monitoring system for terrariums using the Wio Terminal, made for DIT043 Object-Oriented Programming course.".to_string(),
            image_src: "/static/project3.png".to_string(),
            github_url: "https://github.com/rokanas/terminarium".to_string(),
            image_alt: Some("Terminal showing system monitor output".to_string()),
        },
        Project {
            title: "Zulubot".to_string(),
            description: "A multi-purpose discord bot made for Zulu Empire server.".to_string(),
            image_src: "/static/project4.png".to_string(),
            github_url: "https://github.com/rokanas/zulubot".to_string(),
            image_alt: Some("Blockchain network visualization".to_string()),
        },
        Project {
            title: "Wio Terminal Keyboard".to_string(),
            description: "A fully-functioning onscreen keyboard for the Wio Terminal that registers user input".to_string(),
            image_src: "/static/project4.png".to_string(),
            github_url: "https://github.com/rokanas/wio-terminal-keyboard".to_string(),
            image_alt: Some("Blockchain network visualization".to_string()),
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

                // Projects Grid
                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8 justify-items-center">
                    { for projects.iter().map(|project| html! {
                        <ProjectItem
                            title={project.title.clone()}
                            description={project.description.clone()}
                            image_src={project.image_src.clone()}
                            github_url={project.github_url.clone()}
                            image_alt={project.image_alt.clone()}
                        />
                    })}
                </div>

                // Footer Note
                <div class="text-center mt-16 text-gray-500">
                    <p class="font-mono text-sm">
                        {"More projects available on my "} 
                        <a 
                            href="https://github.com/yourusername" 
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