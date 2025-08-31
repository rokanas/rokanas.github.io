// pages/portfolio.rs
use yew::prelude::*;
use web_sys::window;

// struct to hold education and experience data
#[derive(Clone, PartialEq)]
pub struct Experience {
    pub title: String,
    pub company: String,
    pub date: String,
    pub description: Option<String>,
    pub icon: String,
}

// struct to hold skill data
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

    // education data
    let education: Vec<Experience> = vec![
        Experience {
            title: "Software Engineering & Management (BSc)".to_string(),
            company: "University of Gothenburg, SE".to_string(),
            date: "Aug 2022 - Jun 2025".to_string(),
            description: None,
            icon: "/static/U_GOTH.png".to_string(),
        },
        Experience {
            title: "European Law (LLM)".to_string(),
            company: "Leiden University, NL".to_string(),
            date: "Sep 2015 - Jun 2016".to_string(),
            description: None,
            icon: "/static/U_LEID.png".to_string(),
        },
        Experience {
            title: "Law (LLB)".to_string(),
            company: "University of Reading".to_string(),
            date: "Sep 2012 - Jun 2015".to_string(),
            description: None,
            icon: "/static/U_READ.png".to_string(),
        },
    ];

    // experience data
    let experiences = vec![
        Experience {
            title: "Teaching Assistant".to_string(),
            company: "University of Gothenburg".to_string(),
            date: "Aug 2024 - Jun 2025".to_string(),
            description: Some("TA for Software Architecture, Requirements Engineering and Systems Development.
            \n Led TA meetings and workshops with students, promoted in-person and remote guidance.".to_string()),
            icon: "/static/AVATAR_1.png".to_string(),
        },
        Experience {
            title: "Full Stack Developer".to_string(),
            company: "Innovative Solutions Ltd.".to_string(),
            date: "2020 - 2022".to_string(),
            description: Some("Placeholder".to_string()),
            icon: "/static/AVATAR_1.png".to_string(),
        },
        Experience {
            title: "Software Engineering Intern".to_string(),
            company: "StartUp Inc.".to_string(),
            date: "2019 - 2020".to_string(),
            description: Some("Placeholder".to_string()),
            icon: "/static/AVATAR_1.png".to_string(),
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
                    
                    <div class="flex flex-col lg:flex-row items-center gap-8 lg:gap-16 max-w-6xl mx-auto">
                        // portrait
                        <div class="flex-shrink-0">
                            <div class="w-64 h-64 lg:w-80 lg:h-80 rounded-full bg-[#1a1a1a] flex items-center justify-center text-6xl lg:text-8xl shadow-2xl border-8 border-[#0b0b0a] hover:scale-105 transition-transform duration-300">
                                <img 
                                    src="/static/AVATAR_1.png" 
                                    alt="Avatar" 
                                    class="w-full h-full object-contain"
                                />
                            </div>
                        </div>
                        
                        // description
                        <div class="flex-1 text-center lg:text-left">
                    <div class="text-center mb-12">
                        <img 
                            src="/static/KONSTANTINOS_ROKANAS_1.png" 
                            alt="Konstantinos Rokanas"
                            class="w-auto h-auto mx-auto"
                        />
                    </div>
                            <p class="text-gray-300 text-lg lg:text-xl leading-relaxed mb-4">
                                {"Welcome to my portfolio! I'm a passionate Rust developer with expertise in building high-performance web applications using Yew and other modern technologies."}
                            </p>
                            <p class="text-gray-300 text-lg lg:text-xl leading-relaxed">
                                {"I love creating efficient, scalable solutions and am always eager to tackle new challenges in systems programming and web development."}
                            </p>
                        </div>
                    </div>
                </div>

                // education section
                <div class="mb-16">
                    <div class="text-center mb-12">
                        <img 
                            src="/static/EDUCATION_1.png" 
                            alt="Experience"
                            class="max-w-xs md:max-w-md lg:max-w-lg xl:max-w-xl mx-auto"
                        />
                        <img 
                            src="/static/DIVIDER_2.png" 
                            alt="Divider"
                            class="max-w-xs md:max-w-md lg:max-w-lg xl:max-w-xl mx-auto mb-4 -mt-6"
                        />
                    </div>

                    // timeline
                    <div class="max-w-4xl mx-auto">
                        <div class="relative">
                            // vertical line
                            <div class="absolute left-4 md:left-1/2 top-0 bottom-0 w-1 bg-red-600 transform md:-translate-x-1/2"></div>
                            
                            { for education.iter().enumerate().map(|(index, exp)| {
                                let is_even = index % 2 == 0;
                                html! {
                                    <div class={format!("relative flex items-start mb-12 {}", 
                                        if is_even { "md:flex-row" } else { "md:flex-row-reverse" }
                                    )}>
                                        // timeline node
                                        <div class="absolute left-4 md:left-1/2 w-12 h-12 bg-white border-4 border-red-600 rounded-full flex items-center justify-center transform md:-translate-x-1/2 z-20">
                                            <img 
                                                src={exp.icon.clone()} 
                                                alt="Avatar" 
                                                class="w-8 h-8 object-contain rounded-full"
                                            />
                                        </div>

                                        // date on opposite side
                                        <div class={format!("hidden md:block absolute top-4 {} md:w-5/12", 
                                            if is_even { "md:ml-8 right-0" } else { "md:mr-8 left-0 text-right" }
                                        )}>
                                            <div class="text-red-600 font-semibold text-sm">{&exp.date}</div>
                                        </div>
                                        
                                        // content with HUD-style background
                                        <div class={format!("ml-20 md:ml-0 {} md:w-5/12", 
                                            if is_even { "md:mr-8 md:text-right" } else { "md:ml-8" }
                                        )}>
                                            <div 
                                                class="relative text-white flex items-center justify-center text-center p-6 hover:scale-105 transition-transform duration-300"
                                                style="background-image: url('/static/STBAR7.png'); 
                                                       background-repeat: no-repeat; 
                                                       background-size: 100% 100%; 
                                                       image-rendering: pixelated; 
                                                       min-height: 120px;"
                                            >

                                                // inner box
                                                <div 
                                                    class="absolute inset-0 m-4 z-5 bg-[#1a1a1a] bg-opacity-60 border-4 border-[#0b0b0a]"
                                                ></div>

                                                <div class="z-10">
                                                    <h3 class="text-xl font-bold text-white mb-2">{&exp.title}</h3>
                                                    <div class="text-red-600 font-semibold mb-2">{&exp.company}</div>
                                                    // date shown on mobile only
                                                    <div class="text-gray-300 text-sm mb-3 md:hidden">{&exp.date}</div>
                                                        <p class="text-gray-200 text-sm leading-relaxed">{&exp.description}</p>
                                                </div>
                                            </div>
                                        </div>
                                    </div>
                                }
                            })}
                        </div>
                    </div>
                </div>

                // work experience section
                <div class="mb-16">
                    <div class="text-center mb-12">
                        <img 
                            src="/static/EXPERIENCE_1.png" 
                            alt="Experience"
                            class="max-w-xs md:max-w-md lg:max-w-lg xl:max-w-xl mx-auto"
                        />
                        <img 
                            src="/static/DIVIDER_2.png" 
                            alt="Divider"
                            class="max-w-xs md:max-w-md lg:max-w-lg xl:max-w-xl mx-auto mb-4 -mt-6"
                        />
                    </div>

                    // timeline
                    <div class="max-w-4xl mx-auto">
                        <div class="relative">
                            // vertical line
                            <div class="absolute left-4 md:left-1/2 top-0 bottom-0 w-1 bg-red-600 transform md:-translate-x-1/2"></div>
                            
                            { for experiences.iter().enumerate().map(|(index, exp)| {
                                let is_even = index % 2 == 0;
                                html! {
                                    <div class={format!("relative flex items-start mb-12 {}", 
                                        if is_even { "md:flex-row" } else { "md:flex-row-reverse" }
                                    )}>
                                        // timeline node
                                        <div class="absolute left-4 md:left-1/2 w-12 h-12 bg-gray-900 border-4 border-red-600 rounded-full flex items-center justify-center transform md:-translate-x-1/2 z-20">
                                            <img 
                                                src="/static/AVATAR_1.png" 
                                                alt="Avatar" 
                                                class="w-full h-full object-contain"
                                            />
                                        </div>

                                        // date on opposite side
                                        <div class={format!("hidden md:block absolute top-4 {} md:w-5/12", 
                                            if is_even { "md:ml-8 right-0" } else { "md:mr-8 left-0 text-right" }
                                        )}>
                                            <div class="text-red-600 font-semibold text-sm">{&exp.date}</div>
                                        </div>
                                        
                                        // content
                                        <div class={format!("ml-20 md:ml-0 {} md:w-5/12", 
                                            if is_even { "md:mr-8 md:text-right" } else { "md:ml-8" }
                                        )}>
                                            <div class="bg-gray-900 p-6 rounded-lg shadow-xl border border-gray-700 hover:border-red-600 transition-colors duration-300">
                                                <h3 class="text-xl font-bold text-white mb-2">{&exp.title}</h3>
                                                <div class="text-red-600 font-semibold mb-2">{&exp.company}</div>
                                                // date shown on mobile only
                                                <div class="text-gray-400 text-sm mb-3 md:hidden">{&exp.date}</div>
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
                            src="/static/SKILLS_1.png" 
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