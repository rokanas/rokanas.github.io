// pages/projects.rs
use yew::prelude::*;
use web_sys::window;
use crate::components::heading::{Heading};
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
            detailed_description: Some(
                "Skinscan is a user-friendly web application designed to assist users in the early detection of skin cancer through AI-powered image analysis. It allows users to upload images of skin lesions and receive preliminary classifications using our custom-trained AI model. It also enables admins to upload new training data to remotely re-train the model, swap between model versions and view model performance analytics.\n
                Note that Skinscan is designed as a complement to professional dermatological advice, not a substitute.".to_string()),
            technologies_used: vec![
                "Python".to_string(),
                "TensorFlow".to_string(),
                "Nvidia cuDNN".to_string(),
                "Jupyter-notebook".to_string(),
                "Django".to_string(),
                "SQLite".to_string(),
                "TypeScript".to_string(),
                "Svelte".to_string(),
                "HTML/CSS".to_string(),
                "Tailwind".to_string(),
                "Docker".to_string(),
            ],
            key_features: vec![
                "Upload photos to receive AI-powered skin lesion classification".to_string(),
                "Explainable AI for classification decisions".to_string(),
                "Responsive desktop and mobile web interface".to_string(),
                "User profiles and classification history tracking".to_string(),
                "Easy uploading of new training data for GPU-accelerated model re-training".to_string(),
                "Model version hotswapping and performance analytics".to_string(),
                "Extensive unit tests".to_string(),
                "Automated, containerized deployment".to_string(),
                "Flexible and configurable CI/CD pipeline".to_string(),
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
            detailed_description: Some(
                "Dentago is a distributed system designed to centralize dentist appointment bookings across Sweden. Patients interact via a web-based interface accessible on both computers and mobile devices, while dentists manage appointments through a dedicated GUI.\n
                The system relies on MQTT for inter-component communication to achieve scalability, and reliability. Fault tolerance is achieved through diverse mechanisms such as circuit breakers and load balancers.".to_string()),
            technologies_used: vec![
                "JavaScript".to_string(),
                "MQTT".to_string(),
                "MongoDB".to_string(),
                "Node.js".to_string(),
                "Vue.js".to_string(),
                "Express".to_string(),
                "HTML/CSS".to_string(),
                "JWT".to_string(),
            ],
            key_features: vec![
                "Web app for patients to book available appointment slots".to_string(),
                "Dedicated GUI for dentists to submit available slots and view appointments".to_string(),
                "Real-time appointment notifications".to_string(),
                "Service-oriented architecture with decoupled components".to_string(),
                "Inter-component communication via MQTT and REST API".to_string(),
                "User authentication service".to_string(),
                "Extensive event logging".to_string(),
                "Continuous component health monitoring".to_string(),
                "Fault-tolerant message queuing system".to_string(),
                "Reverse proxies, load balancers & circuit breakers for high-traffic services".to_string(),
                "Unit & integration testing".to_string(),
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
            detailed_description: Some(
                "Terminarium is an IoT-based monitoring system built using the Wio Terminal microcontroller and multiple sensor peripherals. The system tracks temperature, humidity, moisture, loudness, vibration and light levels.\n
                Users can monitor environmental conditions both on location via the Wio Terminal's LCD display, or remotely from a web application. From this, they can also access historical and statistical data and can receive alerts when conditions become critical.".to_string()),
            technologies_used: vec![
                "C++".to_string(),
                "Arduino".to_string(),
                "Wio Terminal".to_string(),
                "MQTT".to_string(),
                "JavaScript".to_string(),
                "Firebase DB".to_string(),
                "Node.js".to_string(),
                "Vue.js".to_string(),
                "Express".to_string(),
                "HTML/CSS".to_string(),
            ],
            key_features: vec![
                "Real-time, non-blocking environmental monitoring".to_string(),
                "LCD display with intuitive interface".to_string(),
                "Updateable network info and persistent storage on microcontroller".to_string(),
                "Continuous data transmission via MQTT".to_string(),
                "Web app for remote monitoring and multiple device management".to_string(),
                "Display and storage of historical and statistical data".to_string(),
                "Automatic alert system for critical conditions".to_string(),
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
            description: "A multi-purpose discord bot made for the Zulu Empire community server.".to_string(),
            image_src: "/static/projects/zulubot/zulubot_logo.png".to_string(),
            image_alt: Some("Zulu empire logo".to_string()),
            github_url: "https://github.com/rokanas/zulubot".to_string(),
            detailed_description: Some(
                "Zulubot is a multi-purpose Discord bot developed specifically for the Zulu Empire discord community.
                Built with Python and the discord.py library, it leverages multiple speech generation and recognition toolkits to provide amusing voice chat entertainment using multiple configurable personas. It also includes other light-hearted features like a music player and AI image generation.".to_string()),
            technologies_used: vec![
                "Python".to_string(),
                "discord.py".to_string(),
                "asyncio".to_string(),
                "Gemini API".to_string(),
                "JSON".to_string(),
                "Elevenlabs TTS".to_string(),
                "Vosk".to_string(),
            ],
            key_features: vec![
                "Chatting functionality with text and voice generation".to_string(),
                "Speech recognition for user voice commands".to_string(),
                "Different configurable personas for generated voices and text".to_string(),
                "Youtube music player".to_string(),
                "Queueing system for all audio (music and voice)".to_string(),
                "Image generation".to_string(),
                "Fetching up-to-date cryptocurrency information".to_string(),
            ],
            wiki_url: None,
            additional_images: vec![],
            
        },
        Project {
            title: "Wio Terminal Keyboard".to_string(),
            tags: vec![
                Tag { name: "C++".to_string(), color: "bg-pink-500".to_string(), text_color: None },
            ],
            description: "A fully-functioning onscreen keyboard for the Wio Terminal that registers and stores user input".to_string(),
            image_src: "/static/projects/wio_terminal_keyboard/wio_terminal_keyboard_1.png".to_string(),
            image_alt: Some("Wio terminal keyboard".to_string()),
            github_url: "https://github.com/rokanas/wio-terminal-keyboard".to_string(),
            detailed_description: Some(
                "A fully functional on-screen keyboard interface for the Wio Terminal microcontroller. The keyboard provides a complete input solution for embedded applications.\n
                A notable use case is the ability to update stored variables without having to modify and reupload a sketch to your device. For example, if connecting to wifi or an mqtt broker, changes to an address can be made and stored directly using the onscreen keyboard, rather than changing the hardcoded address in the source code.".to_string()
            ),
            technologies_used: vec![
                "C++".to_string(),
                "Arduino".to_string(),
                "Wio Terminal".to_string(),
            ],
            key_features: vec![
                "Full alphabenumeric layout plus special characters".to_string(),
                "Responsive navigation using Wio Terminal buttons".to_string(),
                "Persistent storage to flash memory".to_string(),
                "Easy integration of library with other Wio Terminal projects".to_string(),
            ],
            wiki_url: None,
            additional_images: vec![],
        },
    ];

    html! {
        <main 
            class="min-h-screen text-white pt-8 pb-10" 
        >
            <div class="container mx-auto pt-4 px-4 max-w-7xl">
                
                // page heading
                <Heading 
                    src="/static/PROJECTS_1.png" 
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