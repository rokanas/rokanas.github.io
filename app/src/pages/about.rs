// pages/portfolio.rs
use yew::prelude::*;
use web_sys::window;

// struct to hold education and experience data
#[derive(Clone, PartialEq)]
pub struct Experience {
    pub title: String,
    pub company: String,
    pub date: String,
    pub description: Option<Vec<String>>,
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
    let selected_job = use_state(|| 0usize);    // track which job is selected

    use_effect_with((), |_| {
        // scroll to top when component mounts
        if let Some(window) = window() {
            window.scroll_to_with_x_and_y(0.0, 0.0);
        }
        || {}
    });

    // cv download handler
    let download_cv = Callback::from(|_| {
        if let Some(window) = web_sys::window() {
            // direct link to cv file
            let _ = window.open_with_url_and_target("/static/KR_CV.pdf", "_blank");
        }
    });

    // education data
    let education: Vec<Experience> = vec![
        Experience {
            title: "Software Engineering & Management (BSc)".to_string(),
            company: "University of Gothenburg, SE".to_string(),
            date: "2022 - 2025".to_string(),
            description: None,
            icon: "/static/U_GOTH.png".to_string(),
        },
        Experience {
            title: "European Law (LLM)".to_string(),
            company: "Leiden University, NL".to_string(),
            date: "2015 - 2016".to_string(),
            description: None,
            icon: "/static/U_LEID.png".to_string(),
        },
        Experience {
            title: "Law (LLB)".to_string(),
            company: "University of Reading, UK".to_string(),
            date: "2012 - 2015".to_string(),
            description: None,
            icon: "/static/U_READ.png".to_string(),
        },
    ];

    // experience data
    let experiences = vec![
        Experience {
            title: "Teaching Assistant".to_string(),
            company: "University of Gothenburg".to_string(),
            date: "2024 - 2025".to_string(),
            description: Some(vec![
                "TA for Software Architecture, Requirements Engineering and Systems Development.".to_string(),
                "Led TA meetings and workshops with students, provided in-person and remote guidance.".to_string(),
                "Provided support and feedback to professors concerning assignments and course materials.".to_string(),
                "Graded student assignments and exams.".to_string()]),
            icon: "/static/U_GOTH.png".to_string(),
        },
        Experience {
            title: "Compliance Officer".to_string(),
            company: "Huawei Technologies S.A.".to_string(),
            date: "2019 - 2022".to_string(),
            description: Some(vec![
                "Legal compliance assessment and risk analysis for all areas of company operations in Athens and Cyprus offices (specialization in Data Protection (GDPR) and Cybersecurity)".to_string(),
                "Legal support to regional offices in Albania, Northern Macedonia and Bulgaria.".to_string(),
                "Led training sessions for all regional offices on sensitive areas (personal data protection, cybersecurity, anti-bribery).".to_string(),]),
            icon: "/static/HUAWEI.png".to_string(),
        },
        Experience {
            title: "Intern at Academy of European Public Law".to_string(),
            company: "European Public Law Organization (EPLO)".to_string(),
            date: "2019".to_string(),
            description: Some(vec![
                "Assisted in the design, organization and implementation of European and Public Law university programs and summer exchange programs".to_string(),
                "Assisted in the administration of the Department of Education.".to_string(),
                "Drafted reports on EPLO activities and conducted research for various other departments.".to_string(),
                ]),
            icon: "/static/EPLO.png".to_string(),
        },
        Experience {
            title: "Intern at Greek National Desk".to_string(),
            company: "Eurojust".to_string(),
            date: "2019 - 2020".to_string(),
            description: Some(vec![
                "Assisted in administrative functions of the Greek national desk.".to_string(),
                "Composed external and internal communications towards national authorities and national desks.".to_string(),
                "Legal case file and database management.".to_string(),
                    ]),
            icon: "/static/EUROJUST.png".to_string(),
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
                        <div class="flex-shrink-0 w-full lg:w-auto flex flex-col items-center">
                            <div class="w-64 h-64 lg:w-80 lg:h-80 rounded-full bg-[#1a1a1a] flex items-center justify-center text-6xl lg:text-8xl shadow-2xl border-8 border-[#0b0b0a] hover:scale-105 transition-transform duration-300">
                                <img 
                                    src="/static/AVATAR_1.png" 
                                    alt="Avatar" 
                                    class="w-full h-full object-contain"
                                />
                            </div>
                            
                            // social media icon links
                            <div class="flex justify-center items-center gap-4 mt-8 w-full">

                                // gitHub
                                <a 
                                    href="https://github.com/rokanas" 
                                    target="_blank" 
                                    class="w-12 h-12 bg-[#1a1a1a] hover:bg-gray-700 rounded-lg flex items-center justify-center transition-all duration-300 hover:scale-110 group border-2 border-black hover:border-gray-500"
                                    title="GitHub"
                                >
                                    <svg class="w-8  h-8 fill-current text-gray-300 group-hover:text-white transition-colors duration-300" viewBox="0 0 16 16">
                                        <path d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.013 8.013 0 0016 8c0-4.42-3.58-8-8-8z"/>
                                    </svg>
                                </a>

                                // linkedIn
                                <a 
                                    href="https://www.linkedin.com/in/konstantinos-rokanas-1ab1a113a/" 
                                    target="_blank" 
                                    class="w-12 h-12 bg-[#1a1a1a] hover:bg-blue-600 rounded-lg flex items-center justify-center transition-all duration-300 hover:scale-110 group border-2 border-black hover:border-blue-500"
                                    title="LinkedIn"
                                >
                                    <svg class="w-8 h-8 fill-current text-gray-300 group-hover:text-white transition-colors duration-300" viewBox="0 0 24 24">
                                        <path d="M20.5 2h-17A1.5 1.5 0 002 3.5v17A1.5 1.5 0 003.5 22h17a1.5 1.5 0 001.5-1.5v-17A1.5 1.5 0 0020.5 2zM8 19H5v-9h3zM6.5 8.25A1.75 1.75 0 118.3 6.5a1.78 1.78 0 01-1.8 1.75zM19 19h-3v-4.74c0-1.42-.6-1.93-1.38-1.93A1.74 1.74 0 0013 14.19a.66.66 0 000 .14V19h-3v-9h2.9v1.3a3.11 3.11 0 012.7-1.4c1.55 0 3.36.86 3.36 3.66z"/>
                                    </svg>
                                </a>

                                // instagram
                                <a 
                                    href="https://www.instagram.com/charybdis.maw/" 
                                    target="_blank" 
                                    class="w-12 h-12 bg-[#1a1a1a] hover:bg-gradient-to-br hover:from-purple-600 hover:to-pink-500 rounded-lg flex items-center justify-center transition-all duration-300 hover:scale-110 group border-2 border-black hover:border-pink-500"
                                    title="Instagram"
                                >
                                    <svg class="w-8 h-8 fill-current text-gray-300 group-hover:text-white transition-colors duration-300" viewBox="0 0 1000 1000">
                                        <path d="M295.42,6c-53.2,2.51-89.53,11-121.29,23.48-32.87,12.81-60.73,30-88.45,57.82S40.89,143,28.17,175.92c-12.31,31.83-20.65,68.19-23,121.42S2.3,367.68,2.56,503.46,3.42,656.26,6,709.6c2.54,53.19,11,89.51,23.48,121.28,12.83,32.87,30,60.72,57.83,88.45S143,964.09,176,976.83c31.8,12.29,68.17,20.67,121.39,23s70.35,2.87,206.09,2.61,152.83-.86,206.16-3.39S799.1,988,830.88,975.58c32.87-12.86,60.74-30,88.45-57.84S964.1,862,976.81,829.06c12.32-31.8,20.69-68.17,23-121.35,2.33-53.37,2.88-70.41,2.62-206.17s-.87-152.78-3.4-206.1-11-89.53-23.47-121.32c-12.85-32.87-30-60.7-57.82-88.45S862,40.87,829.07,28.19c-31.82-12.31-68.17-20.7-121.39-23S637.33,2.3,501.54,2.56,348.75,3.4,295.42,6m5.84,903.88c-48.75-2.12-75.22-10.22-92.86-17-23.36-9-40-19.88-57.58-37.29s-28.38-34.11-37.5-57.42c-6.85-17.64-15.1-44.08-17.38-92.83-2.48-52.69-3-68.51-3.29-202s.22-149.29,2.53-202c2.08-48.71,10.23-75.21,17-92.84,9-23.39,19.84-40,37.29-57.57s34.1-28.39,57.43-37.51c17.62-6.88,44.06-15.06,92.79-17.38,52.73-2.5,68.53-3,202-3.29s149.31.21,202.06,2.53c48.71,2.12,75.22,10.19,92.83,17,23.37,9,40,19.81,57.57,37.29s28.4,34.07,37.52,57.45c6.89,17.57,15.07,44,17.37,92.76,2.51,52.73,3.08,68.54,3.32,202s-.23,149.31-2.54,202c-2.13,48.75-10.21,75.23-17,92.89-9,23.35-19.85,40-37.31,57.56s-34.09,28.38-57.43,37.5c-17.6,6.87-44.07,15.07-92.76,17.39-52.73,2.48-68.53,3-202.05,3.29s-149.27-.25-202-2.53m407.6-674.61a60,60,0,1,0,59.88-60.1,60,60,0,0,0-59.88,60.1M245.77,503c.28,141.8,115.44,256.49,257.21,256.22S759.52,643.8,759.25,502,643.79,245.48,502,245.76,245.5,361.22,245.77,503m90.06-.18a166.67,166.67,0,1,1,167,166.34,166.65,166.65,0,0,1-167-166.34"/>
                                    </svg>
                                </a>

                                // cv download button
                                <button 
                                    onclick={download_cv}
                                    class="w-12 h-12 bg-[#1a1a1a] hover:bg-red-600 rounded-lg flex items-center justify-center transition-all duration-300 hover:scale-110 group border-2 border-black hover:border-red-900 cursor-pointer"
                                    title="CV"
                                >
                                    <svg
                                        class="w-8 h-8 fill-current text-gray-300 group-hover:text-white transition-colors duration-300"
                                        viewBox="0 0 24 24"
                                        xmlns="http://www.w3.org/2000/svg"
                                    >
                                        // document shape
                                        <path d="M14,2H6A2,2 0 0,0 4,4V20A2,2 0 0,0 6,22H18A2,2 0 0,0 20,20V8L14,2M18,20H6V4H13V9H18V20Z" />

                                        // cv text inside doc
                                        <text
                                            x="8"
                                            y="16"
                                            font-size="6"
                                            font-family="Arial, Helvetica, sans-serif"
                                            fill="currentColor"
                                            font-weight="bold"
                                        >
                                            {"CV"}
                                        </text>
                                    </svg>
                                </button>
                            </div>
                        </div>

                // description
                <div class="flex-1 text-center lg:text-left">
                    <div class="text-center mb-10 mt-10">
                        <img 
                            src="/static/KONSTANTINOS_ROKANAS_1.png" 
                            alt="Konstantinos Rokanas"
                            class="w-auto h-auto mx-auto"
                        />
                    </div>
                            <p class="text-gray-300 text-lg lg:text-lg leading-relaxed">
                                {"🎓 I'm a recent software engineering graduate eager to begin a career in tech."}
                            </p>
                            <p class="text-gray-300 text-lg lg:text-lg leading-relaxed mt-4">
                                {"🛠 I want to build software solutions that make your life easier, saving you time and effort. I'm also a big believer in process automation and am interested in the increasing adoption of AI Agents."}
                            </p>
                            <p class="text-gray-300 text-lg lg:text-lg leading-relaxed mt-4">
                                {"⚖️ Formerly a legal professional with a specialization European Law. I have experience both in international organizations and in the private sector, with a focus on personal data and cybersecurity policy compliance."}
                            </p>
                            <p class="text-gray-300 text-lg lg:text-lg leading-relaxed mt-4">
                                {"⛧ I'm also a lifelong "} <strong>{"Doom"}</strong> {" enthusiast, mapmaker and content creator."}
                            </p>
                            <p class="text-gray-300 text-lg lg:text-lg leading-relaxed mt-4">
                                {"🤼‍♂️ When away from the computer, I enjoy practicing mixed-martial arts, submission wrestling, rock-climbing and playing electric guitar."}
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
                            <div class="absolute left-4 md:left-1/2 w-1 bg-red-600 transform md:-translate-x-1/2" style="top: 50px; bottom: 50px;"></div>
                            
                            { for education.iter().enumerate().map(|(index, exp)| {
                                let is_even = index % 2 == 0;
                                html! {
                                    <div class={format!("relative flex items-center mb-12 {}", 
                                        if is_even { "md:flex-row" } else { "md:flex-row-reverse" }
                                    )}>
                                        // timeline node
                                        <div class="absolute left-4 md:left-1/2 w-16 h-16 bg-white border-4 border-red-600 rounded-full flex items-center justify-center transform md:-translate-x-1/2 z-20">
                                            <img 
                                                src={exp.icon.clone()} 
                                                alt="Avatar" 
                                                class="w-12 h-12 object-contain rounded-full"
                                            />
                                        </div>

                                            // date on opposite side
                                            <div class={format!("hidden md:block absolute top-1/2 transform -translate-y-1/2 {} md:w-5/12", 
                                                if is_even { "md:ml-8 right-10 pl-5" } else { "md:mr-8 left-10 pr-5 text-right" }
                                            )}>
                                                <div class="text-red-600 font-semibold text-l">{&exp.date}</div>
                                        </div>
                                        
                                        // content with HUD-style background
                                        <div class={format!("ml-20 md:ml-0 {} md:w-5/12", 
                                            if is_even { "md:mr-8 md:text-right" } else { "md:ml-8" }
                                        )}>
                                            <div 
                                                class="relative text-white flex items-center justify-left text-left p-6 hover:scale-105 transition-transform duration-300"
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
                                                    <h3 class="text-xl font-bold text-white mb-2 pl-3">{&exp.title}</h3>
                                                    <div class="text-red-600 font-semibold mb-2 pl-3">{&exp.company}</div>
                                                    // date shown on mobile only
                                                    <div class="text-gray-400 text-sm mb-3 md:hidden pl-3">{&exp.date}</div>
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

                    <div class="max-w-6xl mx-auto">
                        <div class="flex flex-col lg:flex-row gap-8">
                    
                            // left sidebar (job titles)
                            <div class="lg:w-1/3 space-y-4">
                                { for experiences.iter().enumerate().map(|(index, exp)| {
                                    let is_selected = *selected_job == index; // first item selected by default
                                    let selected_job_clone = selected_job.clone();
                                    let onclick = Callback::from(move |_| {
                                        selected_job_clone.set(index);
                                    });
                                    html! {
                                        <div class={format!("flex items-center p-4 rounded-lg cursor-pointer transition-all duration-300 {}", 
                                            if is_selected { 
                                                "bg-red-600/30 border border-red-600/40" 
                                            } else { 
                                                "bg-gray-900/50 border border-gray-700 hover:border-gray-600" 
                                            }
                                        )}
                                        onclick={onclick}>
                                            <div class="flex-shrink-0 mr-4">
                                                <div class="w-12 h-12 bg-gray-800 rounded-full flex items-center justify-center">
                                                    <img 
                                                        src={exp.icon.clone()} 
                                                        alt="Company logo" 
                                                        class="w-10 h-10 object-contain rounded-full bg-white"
                                                    />
                                                </div>
                                            </div>
                                            <div class="flex-1">
                                                <h3 class="text-white font-semibold text-lg mb-1">{&exp.title}</h3>
                                                <p class="text-gray-400 text-sm">{&exp.company}</p>
                                            </div>
                                        </div>
                                    }
                                })}
                            </div>

                            // right content (job details))
                            <div class="lg:w-2/3">
                                <div 
                                    class="relative p-8"                                                 
                                    style="background-image: url('/static/STBAR7.png'); 
                                           background-repeat: no-repeat; 
                                           background-size: 100% 100%; 
                                           image-rendering: pixelated;
                                           min-height: 120px;
                                           transparency: 0.9;"
                                >
                                    // inner box
                                    <div 
                                        class="absolute inset-0 m-4 z-5 bg-[#1a1a1a] bg-opacity-60 border-4 border-[#0b0b0a]"
                                    ></div>
                                    
                                    <div class="relative z-10">
                                        <div class="flex items-center mb-6">
                                            <div class="flex-shrink-0 mr-4">
                                                <div class="w-16 h-16 bg-gray-800 rounded-full flex items-center justify-center">
                                                    <img 
                                                        src={experiences[*selected_job].icon.clone()}
                                                        alt="Company logo" 
                                                        class="w-13 h-13 object-contain rounded-full bg-white"
                                                    />
                                                </div>
                                            </div>
                                            <div>
                                                <h2 class="text-2xl font-bold text-white mb-2">{&experiences[*selected_job].title}</h2>
                                                <p class="text-red-600 text-lg mb-1">{&experiences[*selected_job].company}</p>
                                                <p class="text-gray-400 text-sm">{&experiences[*selected_job].date}</p>
                                            </div>
                                        </div>

                                        <div class="space-y-4">                                        
                                            if let Some(description) = &experiences[*selected_job].description {
                                                { for description.iter().map(|point| {
                                                    html! {
                                                        <li class="flex items-start">
                                                            <div class="flex-shrink-0 w-2 h-2 bg-red-500 rounded-full mt-2 mr-3"></div>
                                                            <p class="text-gray-300 leading-relaxed">{point}</p>
                                                        </li>
                                                    }
                                                })}
                                            }
                                        </div>
                                    </div>
                                </div>
                            </div>
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