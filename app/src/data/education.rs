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
            icon: "/static/about/education/U_GOTH.png".to_string(),
        },
        ExperienceItem {
            title: "European Law (LLM)".to_string(),
            institution: "Leiden University, NL".to_string(),
            date: "2015 - 2016".to_string(),
            description: None,
            icon: "/static/about/education/U_LEID.png".to_string(),
        },
        ExperienceItem {
            title: "Law (LLB)".to_string(),
            institution: "University of Reading, UK".to_string(),
            date: "2012 - 2015".to_string(),
            description: None,
            icon: "/static/about/education/U_READ.png".to_string(),
        },
    ]
}
