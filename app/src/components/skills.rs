// components/skills.rs
use yew::prelude::*;

// struct to hold skill data
#[derive(Clone, PartialEq)]
pub struct Skill {
    pub name: String,
    pub icon: String,
    pub color: String,
}

#[function_component(Skills)]
pub fn skills() -> Html {
    // skills data
    let skills = vec![
        Skill { name: "Python".to_string(), icon: "/static/about/skills/PYTHON.svg".to_string(), color: "bg-orange-500".to_string() },
        Skill { name: "Java".to_string(), icon: "/static/about/skills/JAVA.svg".to_string(), color: "bg-blue-500".to_string() },
        Skill { name: "C++".to_string(), icon: "/static/about/skills/CPP.svg".to_string(), color: "bg-blue-600".to_string() },
        Skill { name: "Cmake".to_string(), icon: "/static/about/skills/CMAKE.svg".to_string(), color: "bg-yellow-600".to_string() },
        Skill { name: "Typescript".to_string(), icon: "/static/about/skills/TS.svg".to_string(), color: "bg-cyan-500".to_string() },
        Skill { name: "SQL".to_string(), icon: "/static/about/skills/SQL.svg".to_string(), color: "bg-purple-500".to_string() },
        Skill { name: "Docker".to_string(), icon: "/static/about/skills/DOCKER.svg".to_string(), color: "bg-yellow-600".to_string() },
        Skill { name: "Yew".to_string(), icon: "/static/about/skills/YEW.svg".to_string(), color: "bg-blue-700".to_string() },
        Skill { name: "Vue".to_string(), icon: "/static/about/skills/VUE.svg".to_string(), color: "bg-yellow-500".to_string() },
        Skill { name: "React".to_string(), icon: "/static/about/skills/REACT.svg".to_string(), color: "bg-yellow-600".to_string() },
        Skill { name: "Svelte".to_string(), icon: "/static/about/skills/SVELTE.svg".to_string(), color: "bg-blue-800".to_string() },
        Skill { name: "Express".to_string(), icon: "/static/about/skills/EXPRESS.svg".to_string(), color: "bg-yellow-600".to_string() },
        Skill { name: "Postman".to_string(), icon: "/static/about/skills/POSTMAN.svg".to_string(), color: "bg-yellow-600".to_string() },
        Skill { name: "TensorFlow".to_string(), icon: "/static/about/skills/TENSORFLOW.svg".to_string(), color: "bg-yellow-600".to_string() },
        Skill { name: "Arduino".to_string(), icon: "/static/about/skills/ARDUINO.svg".to_string(), color: "bg-yellow-600".to_string() },
    ];

    html! {
        // skills grid
        <div class="grid grid-cols-3 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-6 gap-6 max-w-6xl mx-auto">
            { for skills.iter().map(|skill| html! {
                <div 
                    class="relative text-white flex items-center justify-left text-left p-6 hover:scale-105 transition-transform duration-300 aspect-square"
                    style="background-image: url('/static/STBAR6_2.png'); 
                            background-repeat: no-repeat; 
                            background-size: 100% 100%; 
                            image-rendering: pixelated;"
                >
                <div class="w-17 h-17 rounded-lg flex text-center items-center justify-center mx-auto mb-3 group-hover:scale-110 transition-transform duration-300 overflow-hidden">
                        <img 
                            src={skill.icon.clone()} 
                            alt={skill.name.clone()} 
                            title={skill.name.clone()} 
                            class="object-contain w-full h-full drop-shadow-[2px_4px_6px_rgba(0,0,0,0.9)]"
                        />
                    </div>
                </div>
            })}
        </div>
    }
}