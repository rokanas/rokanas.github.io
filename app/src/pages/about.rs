// pages/portfolio.rs
use yew::prelude::*;
use web_sys::window;

// Struct to hold experience data
#[derive(Clone, PartialEq)]
pub struct Experience {
    pub title: String,
    pub company: String,
    pub date: String,
    pub description: String,
    pub icon: String,
}

// Struct to hold skill data
#[derive(Clone, PartialEq)]
pub struct Skill {
    pub name: String,
    pub icon: String,
    pub color: String,
}

#[function_component(About)]
pub fn about() -> Html {
    use_effect_with((), |_| {
        // scroll to top when component mounts
        if let Some(window) = window() {
            window.scroll_to_with_x_and_y(0.0, 0.0);
        }
        || {}
    });

    // experience data
    let experiences = vec![
        Experience {
            title: "Senior Rust Developer".to_string(),
            company: "Amazing Tech Co.".to_string(),
            date: "2022 - Present".to_string(),
            description: "Placeholder.".to_string(),
            icon: "🏢".to_string(),
        },
        Experience {
            title: "Full Stack Developer".to_string(),
            company: "Innovative Solutions Ltd.".to_string(),
            date: "2020 - 2022".to_string(),
            description: "Placeholder".to_string(),
            icon: "💻".to_string(),
        },
        Experience {
            title: "Software Engineering Intern".to_string(),
            company: "StartUp Inc.".to_string(),
            date: "2019 - 2020".to_string(),
            description: "Placeholder.".to_string(),
            icon: "🚀".to_string(),
        },
    ];

    // skills data
    let skills = vec![
        Skill { name: "Rust".to_string(), icon: "🦀".to_string(), color: "bg-orange-500".to_string() },
        Skill { name: "Yew".to_string(), icon: "🕸️".to_string(), color: "bg-green-500".to_string() },
        Skill { name: "Python".to_string(), icon: "🐍".to_string(), color: "bg-blue-500".to_string() },
        Skill { name: "Docker".to_string(), icon: "🐳".to_string(), color: "bg-blue-600".to_string() },
        Skill { name: "React".to_string(), icon: "⚛️".to_string(), color: "bg-cyan-500".to_string() },
        Skill { name: "WebAssembly".to_string(), icon: "🌐".to_string(), color: "bg-purple-500".to_string() },
        Skill { name: "PostgreSQL".to_string(), icon: "🗃️".to_string(), color: "bg-blue-700".to_string() },
        Skill { name: "AWS".to_string(), icon: "☁️".to_string(), color: "bg-yellow-500".to_string() },
        Skill { name: "TypeScript".to_string(), icon: "📘".to_string(), color: "bg-blue-800".to_string() },
        Skill { name: "JavaScript".to_string(), icon: "📜".to_string(), color: "bg-yellow-600".to_string() },
        Skill { name: "C++".to_string(), icon: "⚡".to_string(), color: "bg-pink-500".to_string() },
        Skill { name: "Git".to_string(), icon: "🔧".to_string(), color: "bg-red-500".to_string() },
    ];

    html! {
        <main 
            class="min-h-screen text-white pt-8 pb-10" 
        >
            <div class="container mx-auto px-4 max-w-7xl">
                
                // about section
                <div class="mb-16">
                    <div class="text-center mb-12">
                        <img 
                            src="/static/ABOUT_ME.png" 
                            alt="About"
                            class="w-auto h-auto mx-auto"
                        />
                        <img 
                            src="/static/DIVIDER_2.png" 
                            alt="Divider"
                            class="w-auto h-auto mx-auto mb-4 -mt-6"
                        />
                    </div>
                    
                    <div class="flex flex-col lg:flex-row items-center gap-8 lg:gap-16 max-w-6xl mx-auto">
                        // portrait
                        <div class="flex-shrink-0">
                            <div class="w-64 h-64 lg:w-80 lg:h-80 rounded-full bg-gradient-to-br from-red-600 to-purple-700 flex items-center justify-center text-6xl lg:text-8xl shadow-2xl border-4 border-gray-700 hover:scale-105 transition-transform duration-300">
                                {"👤"}
                            </div>
                        </div>
                        
                        // description
                        <div class="flex-1 text-center lg:text-left">
                            <h1 class="text-4xl lg:text-6xl font-bold mb-6 bg-gradient-to-r from-red-500 to-purple-600 bg-clip-text text-transparent">
                                {"Your Name"}
                            </h1>
                            <p class="text-gray-300 text-lg lg:text-xl leading-relaxed mb-4">
                                {"Welcome to my portfolio! I'm a passionate Rust developer with expertise in building high-performance web applications using Yew and other modern technologies."}
                            </p>
                            <p class="text-gray-300 text-lg lg:text-xl leading-relaxed">
                                {"I love creating efficient, scalable solutions and am always eager to tackle new challenges in systems programming and web development."}
                            </p>
                        </div>
                    </div>
                </div>

                // work experience section
                <div class="mb-16">
                    <div class="text-center mb-12">
                        <img 
                            src="/static/EXPERIENCE.png" 
                            alt="Experience"
                            class="w-auto h-auto mx-auto"
                        />
                        <img 
                            src="/static/DIVIDER_2.png" 
                            alt="Divider"
                            class="w-auto h-auto mx-auto mb-4 -mt-6"
                        />
                    </div>

                    // timeline
                    <div class="max-w-4xl mx-auto">
                        <div class="relative">
                            // vertical line
                            <div class="absolute left-4 md:left-1/2 top-0 bottom-0 w-1 bg-gradient-to-b from-red-600 to-purple-700 transform md:-translate-x-1/2"></div>
                            
                            { for experiences.iter().enumerate().map(|(index, exp)| {
                                let is_even = index % 2 == 0;
                                html! {
                                    <div class={format!("relative flex items-center mb-12 {}", 
                                        if is_even { "md:flex-row" } else { "md:flex-row-reverse" }
                                    )}>
                                        // timeline node
                                        <div class="absolute left-4 md:left-1/2 w-12 h-12 bg-gray-900 border-4 border-red-600 rounded-full flex items-center justify-center transform md:-translate-x-1/2 z-10">
                                            <span class="text-xl">{&exp.icon}</span>
                                        </div>
                                        
                                        // content
                                        <div class={format!("ml-20 md:ml-0 {} md:w-5/12", 
                                            if is_even { "md:mr-8 md:text-right" } else { "md:ml-8" }
                                        )}>
                                            <div class="bg-gray-900 p-6 rounded-lg shadow-xl border border-gray-700 hover:border-red-600 transition-colors duration-300">
                                                <h3 class="text-xl font-bold text-white mb-2">{&exp.title}</h3>
                                                <div class="text-red-400 font-semibold mb-2">{&exp.company}</div>
                                                <div class="text-gray-400 text-sm mb-3">{&exp.date}</div>
                                                <p class="text-gray-300 leading-relaxed">{&exp.description}</p>
                                            </div>
                                        </div>
                                    </div>
                                }
                            })}
                        </div>
                    </div>
                </div>

                // skills section
                <div class="mb-16">
                    <div class="text-center mb-12">
                        <img 
                            src="/static/SKILLS.png" 
                            alt="Skills"
                            class="w-auto h-auto mx-auto"
                        />
                        <img 
                            src="/static/DIVIDER_2.png" 
                            alt="Divider"
                            class="w-auto h-auto mx-auto mb-4 -mt-6"
                        />
                    </div>

                    // skills grid
                    <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-6 gap-6 max-w-6xl mx-auto">
                        { for skills.iter().map(|skill| html! {
                            <div class="bg-gray-900 p-6 rounded-lg text-center hover:scale-105 transform transition-all duration-300 border border-gray-700 hover:border-red-600 group">
                                <div class={format!("w-12 h-12 {} rounded-lg flex items-center justify-center mx-auto mb-3 group-hover:scale-110 transition-transform duration-300", skill.color)}>
                                    <span class="text-white text-xl">{&skill.icon}</span>
                                </div>
                                <span class="text-gray-300 text-sm font-medium">{&skill.name}</span>
                            </div>
                        })}
                    </div>
                </div>

                // hobbies section
                <div class="mb-16">
                    <div class="text-center mb-12">
                        <img 
                            src="/static/HOBBIES.png" 
                            alt="Hobbies"
                            class="w-auto h-auto mx-auto"
                        />
                        <img 
                            src="/static/DIVIDER_2.png" 
                            alt="Divider"
                            class="w-auto h-auto mx-auto mb-4 -mt-6"
                        />
                    </div>

                    <div class="max-w-4xl mx-auto">
                        <div class="bg-gray-900 p-8 rounded-lg shadow-xl border border-gray-700">
                            <p class="text-gray-300 text-lg leading-relaxed mb-6">
                                {"When I'm not coding, you can find me exploring the great outdoors through hiking and photography. I'm passionate about capturing the beauty of nature and sharing it with others."}
                            </p>
                            <p class="text-gray-300 text-lg leading-relaxed mb-6">
                                {"I'm also an avid reader of science fiction novels and enjoy staying up-to-date with the latest developments in technology and programming languages. In my spare time, I contribute to open-source projects and enjoy experimenting with new frameworks and tools."}
                            </p>
                            <p class="text-gray-300 text-lg leading-relaxed">
                                {"Music is another big part of my life - I play guitar and enjoy attending live concerts whenever possible. There's something magical about the intersection of creativity and technical skill that appeals to me in both programming and music."}
                            </p>
                        </div>
                    </div>
                </div>

                // projects link
                <div class="text-center mt-16">
                    <a 
                        href="/projects" 
                        class="inline-block bg-gradient-to-r from-red-600 to-purple-700 text-white font-bold py-4 px-8 rounded-lg hover:from-red-700 hover:to-purple-800 transform hover:scale-105 transition-all duration-300 shadow-xl text-lg"
                    >
                        {"View My Projects →"}
                    </a>
                </div>

                // footer note
                <div class="text-center mt-16 text-gray-500">
                    <p class="font-mono text-sm">
                        {"More about me available on my "} 
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