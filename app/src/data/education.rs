// data/education.rs
// hardcoded education content for the About page's Education timeline
use crate::data::ExperienceItem;

pub fn education_data() -> Vec<ExperienceItem> {
    vec![
        ExperienceItem {
            title: "Software Engineering & Management (BSc)".to_string(),
            institution: "University of Gothenburg, SE".to_string(),
            date: "2022 - 2025".to_string(),
            description: None,
            icon: "/static/about/education/U_GOTH.svg".to_string(),
            icon_bg: "bg-black",
            social_button: None,
        },
        ExperienceItem {
            title: "European Law (LLM)".to_string(),
            institution: "Leiden University, NL".to_string(),
            date: "2015 - 2016".to_string(),
            description: None,
            icon: "/static/about/education/U_LEID.svg".to_string(),
            icon_bg: "bg-white",
            social_button: None,
        },
        ExperienceItem {
            title: "Law (LLB)".to_string(),
            institution: "University of Reading, UK".to_string(),
            date: "2012 - 2015".to_string(),
            description: None,
            icon: "/static/about/education/U_READ.svg".to_string(),
            icon_bg: "bg-white",
            social_button: None,
        },
    ]
}
