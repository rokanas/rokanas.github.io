// pages/projects.rs
use yew::prelude::*;
use web_sys::window;
use crate::components::project_item::{ProjectItem, Tag};

// struct to hold project data
#[derive(Clone, PartialEq)]
pub struct Project {
    pub title: String,
    pub tags: Vec<Tag>,
    pub description: String,
    pub image_src: String,
    pub image_alt: Option<String>,
    pub github_url: String,
    pub detailed_description: Option<String>,
    pub technologies_used: Vec<String>,
    pub key_features: Vec<String>,
    pub wiki_url: Option<String>,
    pub additional_images: Vec<String>,
}

#[function_component(Projects)]
pub fn projects() -> Html {
    use_effect_with((), |_| {
        // scroll to top when component mounts
        if let Some(window) = window() {
            window.scroll_to_with_x_and_y(0.0, 0.0);
        }
        || {}
    });

    // project item definitions (temporarily hardcoded, can be moved to db later)
    let projects = vec![
        Project {
            title: "Skinscan".to_string(),
            tags: vec![
                Tag { name: "Python".to_string(), color: "bg-blue-500".to_string(), text_color: None },
                Tag { name: "Typescript".to_string(), color: "bg-blue-800".to_string(), text_color: None },
            ],
            description: "A web app for classifying skin lesions using custom-trained AI, made for DIT826 Software Engineering for Data-Intensive AI Applications course.".to_string(),
            image_src: "/static/projects/skinscan/skinscan_logo.png".to_string(),
            image_alt: Some("Skinscan logo".to_string()),
            github_url: "https://github.com/rokanas/skinscan".to_string(),
            detailed_description: Some("Skinscan is a comprehensive web application designed to assist in the early detection of skin cancer through AI-powered image analysis. Built as part of the DIT826 course, this project demonstrates the integration of machine learning models with modern web technologies to create a user-friendly medical screening tool. The application allows users to upload images of skin lesions and receive preliminary classifications based on trained neural network models.".to_string()),
            technologies_used: vec![
                "Python".to_string(),
                "TypeScript".to_string(),
                "TensorFlow".to_string(),
                "React".to_string(),
                "FastAPI".to_string(),
                "Docker".to_string(),
                "PostgreSQL".to_string(),
            ],
            key_features: vec![
                "AI-powered skin lesion classification".to_string(),
                "Secure image upload and processing".to_string(),
                "User authentication and history tracking".to_string(),
                "Responsive web interface".to_string(),
                "RESTful API with comprehensive documentation".to_string(),
                "Containerized deployment with Docker".to_string(),
            ],
            wiki_url: Some("https://github.com/rokanas/skinscan/wiki".to_string()),
            additional_images: vec![
                "/static/projects/skinscan/skinscan_homepage.png".to_string(),
                "/static/projects/skinscan/skinscan_admin_dashboard.png".to_string(),
            ],
        },
        Project {
            title: "Dentago".to_string(),
            tags: vec![
                Tag { name: "Javascript".to_string(), color: "bg-yellow-500".to_string(), text_color: None },
            ],
            description: "A distributed system for booking dentist appointments, made for DIT356 Distributed Systems Development course.".to_string(),
            image_src: "/static/projects/dentago/dentago_logo.png".to_string(),
            image_alt: Some("Dentago component diagram".to_string()),
            github_url: "https://github.com/rokanas/dentago".to_string(),
            detailed_description: Some("Dentago is a comprehensive distributed system designed for managing dental appointments across multiple clinics. The system demonstrates advanced distributed systems concepts including microservices architecture, message queuing, load balancing, and fault tolerance. Built for the DIT356 course, it showcases real-world application of distributed computing principles in a healthcare context.".to_string()),
            technologies_used: vec![
                "JavaScript".to_string(),
                "Node.js".to_string(),
                "MongoDB".to_string(),
                "RabbitMQ".to_string(),
                "Docker".to_string(),
                "MQTT".to_string(),
                "Vue.js".to_string(),
            ],
            key_features: vec![
                "Microservices architecture with independent scalability".to_string(),
                "Real-time appointment notifications via MQTT".to_string(),
                "Distributed data storage across multiple nodes".to_string(),
                "Fault-tolerant message queuing system".to_string(),
                "Load balancing for high availability".to_string(),
                "Multi-clinic support with centralized management".to_string(),
            ],
            wiki_url: Some("https://github.com/rokanas/dentago/wiki".to_string()),
            additional_images: vec![
                "/static/projects/dentago/dentago_component_diagram.png".to_string(),
            ],

        },
        Project {
            title: "Terminarium".to_string(),
            tags: vec![
                Tag { name: "C++".to_string(), color: "bg-pink-500".to_string(), text_color: None },
                Tag { name: "Javascript".to_string(), color: "bg-yellow-500".to_string(), text_color: None },
            ],
            description: "A monitoring system for terrariums using the Wio Terminal, made for DIT043 Object-Oriented Programming course.".to_string(),
            image_src: "/static/projects/terminarium/terminarium_logo.png".to_string(),
            image_alt: Some("Terminarium logo".to_string()),
            github_url: "https://github.com/rokanas/terminarium".to_string(),
            detailed_description: Some("Terminarium is an IoT-based terrarium monitoring system built using the Wio Terminal microcontroller. This project combines embedded systems programming with web technologies to create a comprehensive environment monitoring solution. The system tracks temperature, humidity, and light levels while providing both local display and remote web-based monitoring capabilities.".to_string()),
            technologies_used: vec![
                "C++".to_string(),
                "Arduino Framework".to_string(),
                "JavaScript".to_string(),
                "HTML/CSS".to_string(),
                "WiFi Communication".to_string(),
                "Sensor Integration".to_string(),
            ],
            key_features: vec![
                "Real-time environmental monitoring".to_string(),
                "LCD display with intuitive interface".to_string(),
                "WiFi-enabled data transmission".to_string(),
                "Web dashboard for remote monitoring".to_string(),
                "Automatic alert system for critical conditions".to_string(),
                "Data logging and historical tracking".to_string(),
            ],
            wiki_url: Some("https://github.com/rokanas/terminarium/wiki".to_string()),
            additional_images: vec![
                "/static/projects/terminarium/terminarium_system_design.jpg".to_string(),
            ],
        },
        Project {
            title: "Zulubot".to_string(),
            tags: vec![
                Tag { name: "Python".to_string(), color: "bg-blue-500".to_string(), text_color: None },
            ],
            description: "A multi-purpose discord bot made for Zulu Empire server.".to_string(),
            image_src: "/static/projects/zulubot/zulubot_logo.png".to_string(),
            image_alt: Some("Zulu empire logo".to_string()),
            github_url: "https://github.com/rokanas/zulubot".to_string(),
            detailed_description: Some("Zulubot is a feature-rich Discord bot developed specifically for the Zulu Empire gaming community. Built with Python and the discord.py library, it provides comprehensive server management tools, entertainment features, and community engagement utilities. The bot has been actively serving the community for over a year, handling thousands of daily interactions.".to_string()),
            technologies_used: vec![
                "Python".to_string(),
                "discord.py".to_string(),
                "SQLite".to_string(),
                "asyncio".to_string(),
                "Discord API".to_string(),
                "JSON".to_string(),
            ],
            key_features: vec![
                "Comprehensive moderation tools and automod".to_string(),
                "Music playback with queue management".to_string(),
                "Custom server economy system".to_string(),
                "Role management and reaction roles".to_string(),
                "Fun commands and mini-games".to_string(),
                "Automated welcome messages and logging".to_string(),
            ],
            wiki_url: None,
            additional_images: vec![],
            
        },
        Project {
            title: "Wio Terminal Keyboard".to_string(),
            tags: vec![
                Tag { name: "C++".to_string(), color: "bg-pink-500".to_string(), text_color: None },
            ],
            description: "A fully-functioning onscreen keyboard for the Wio Terminal that registers user input".to_string(),
            image_src: "/static/projects/wio_terminal_keyboard/wio_terminal_keyboard_1.png".to_string(),
            image_alt: Some("Wio terminal keyboard".to_string()),
            github_url: "https://github.com/rokanas/wio-terminal-keyboard".to_string(),
            detailed_description: Some("This project implements a fully functional on-screen keyboard interface for the Wio Terminal microcontroller. The keyboard provides a complete input solution for embedded applications, featuring multiple layouts, responsive touch input, and seamless integration with other Wio Terminal applications. It demonstrates advanced embedded programming techniques and user interface design for resource-constrained devices.".to_string()),
            technologies_used: vec![
                "C++".to_string(),
                "Arduino Framework".to_string(),
                "TFT LCD Programming".to_string(),
                "Touch Interface".to_string(),
                "Embedded Systems".to_string(),
            ],
            key_features: vec![
                "Full QWERTY layout with special characters".to_string(),
                "Responsive touch input with haptic feedback".to_string(),
                "Multiple keyboard layouts (letters, numbers, symbols)".to_string(),
                "Backspace and space key functionality".to_string(),
                "Visual feedback for key presses".to_string(),
                "Easy integration with other Wio Terminal projects".to_string(),
            ],
            wiki_url: None,
            additional_images: vec![],
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

                // footer note
                <div class="text-center mt-16 text-gray-500">
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