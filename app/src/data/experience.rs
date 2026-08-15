// data/experience.rs
// hardcoded work-experience content for the About page's Experience tabs
use crate::data::ExperienceItem;

pub fn experience_data() -> Vec<ExperienceItem> {
    vec![
        ExperienceItem {
            title: "Teaching Assistant".to_string(),
            institution: "University of Gothenburg".to_string(),
            date: "2024 - 2025".to_string(),
            description: Some(vec![
                "TA for Software Architecture, Requirements Engineering and Systems Development.".to_string(),
                "Led TA meetings and workshops with students, provided in-person and remote guidance.".to_string(),
                "Provided support and feedback to professors concerning assignments and course materials.".to_string(),
                "Graded student assignments and exams.".to_string()]),
            icon: "/static/about/education/U_GOTH.png".to_string(),
        },
        ExperienceItem {
            title: "Compliance Officer".to_string(),
            institution: "Huawei Technologies S.A.".to_string(),
            date: "2019 - 2022".to_string(),
            description: Some(vec![
                "Legal compliance assessment and risk analysis for all areas of company operations in Athens and Cyprus offices (specialization in Data Protection (GDPR) and Cybersecurity)".to_string(),
                "Legal support to regional offices in Albania, Northern Macedonia and Bulgaria.".to_string(),
                "Led training sessions for all regional offices on sensitive areas (personal data protection, cybersecurity, anti-bribery).".to_string(),]),
            icon: "/static/about/experience/HUAWEI.png".to_string(),
        },
    ]
}
