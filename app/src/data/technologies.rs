// data/technologies.rs
// hardcoded technology/skill content for the About page's Technologies grid

#[derive(Clone, PartialEq)]
pub struct Skill {
    pub name: String,
    pub icon: String,
    pub color: String,
}

pub fn technologies_data() -> Vec<Skill> {
    vec![
        Skill { name: "Go".to_string(), icon: "/static/about/technologies/GO.svg".to_string(), color: "bg-orange-500".to_string() },
        Skill { name: "Typescript".to_string(), icon: "/static/about/technologies/TS.svg".to_string(), color: "bg-cyan-500".to_string() },
        Skill { name: "Python".to_string(), icon: "/static/about/technologies/PYTHON.svg".to_string(), color: "bg-orange-500".to_string() },
        Skill { name: "Java".to_string(), icon: "/static/about/technologies/JAVA.svg".to_string(), color: "bg-blue-500".to_string() },
        Skill { name: "C++".to_string(), icon: "/static/about/technologies/CPP.svg".to_string(), color: "bg-blue-600".to_string() },
        Skill { name: "Cmake".to_string(), icon: "/static/about/technologies/CMAKE.svg".to_string(), color: "bg-yellow-600".to_string() },
        Skill { name: "SQL".to_string(), icon: "/static/about/technologies/SQL.svg".to_string(), color: "bg-purple-500".to_string() },
        Skill { name: "Docker".to_string(), icon: "/static/about/technologies/DOCKER.svg".to_string(), color: "bg-yellow-600".to_string() },
        Skill { name: "Kubernetes".to_string(), icon: "/static/about/technologies/KUBERNETES.svg".to_string(), color: "bg-yellow-600".to_string() },
        Skill { name: "Helm".to_string(), icon: "/static/about/technologies/HELM.svg".to_string(), color: "bg-yellow-600".to_string() },
        Skill { name: "Yew".to_string(), icon: "/static/about/technologies/YEW.svg".to_string(), color: "bg-blue-700".to_string() },
        Skill { name: "Vue".to_string(), icon: "/static/about/technologies/VUE.svg".to_string(), color: "bg-yellow-500".to_string() },
        Skill { name: "React".to_string(), icon: "/static/about/technologies/REACT.svg".to_string(), color: "bg-yellow-600".to_string() },
        Skill { name: "Svelte".to_string(), icon: "/static/about/technologies/SVELTE.svg".to_string(), color: "bg-blue-800".to_string() },
        Skill { name: "Postman".to_string(), icon: "/static/about/technologies/POSTMAN.svg".to_string(), color: "bg-yellow-600".to_string() },
        Skill { name: "TensorFlow".to_string(), icon: "/static/about/technologies/TENSORFLOW.svg".to_string(), color: "bg-yellow-600".to_string() },
        Skill { name: "Arduino".to_string(), icon: "/static/about/technologies/ARDUINO.svg".to_string(), color: "bg-yellow-600".to_string() },
        Skill { name: "Stellar".to_string(), icon: "/static/about/technologies/STELLAR.svg".to_string(), color: "bg-yellow-600".to_string() },
    ]
}
