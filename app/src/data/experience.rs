// data/experience.rs
// hardcoded work-experience content for the About page's Experience tabs
use crate::data::ExperienceItem;

pub fn experience_data() -> Vec<ExperienceItem> {
    vec![
        ExperienceItem {
            title: "Software Engineer".to_string(),
            institution: "Hypotenuse Labs".to_string(),
            date: "2025 - Now".to_string(),
            description: Some(vec![
                "Full-stack and smart-contract development on Web3 projects.".to_string(),
                "Maintenance and integration support for the [Stellar Disbursement Platform](https://developers.stellar.org/docs/platforms/stellar-disbursement-platform).".to_string(),
                "Technical deep-dives, guidance and direct coordination with clients and stakeholders.".to_string(),
                "CVE / bug bounty triage and remediation.".to_string(),
            ]),
            icon: "/static/about/experience/HYPE.svg".to_string(),
            icon_bg: "bg-black",
            social_button: Some("Github Hype"),
        },
        ExperienceItem {
            title: "Teaching Assistant".to_string(),
            institution: "University of Gothenburg".to_string(),
            date: "2024 - 2025".to_string(),
            description: Some(vec![
                "TA for Software Architecture, Requirements Engineering and Systems Development.".to_string(),
                "Guidance to students, both in-person (meetings, workshops) and remote.".to_string(),
                "Support to professors concerning assignments and course materials.".to_string(),
            ]),
            icon: "/static/about/education/U_GOTH.svg".to_string(),
            icon_bg: "bg-black",
            social_button: None,
        },
        ExperienceItem {
            title: "Compliance Officer".to_string(),
            institution: "Huawei Technologies S.A.".to_string(),
            date: "2019 - 2022".to_string(),
            description: Some(vec![
                "Legal compliance assessment and risk analysis for all areas of company operations in Athens and Cyprus offices (specialization in Data Protection (GDPR) and Cybersecurity)".to_string(),
                "Legal support to regional offices in Albania, Northern Macedonia and Bulgaria.".to_string(),
                "Led training sessions for all regional offices on sensitive areas (personal data protection, cybersecurity, AMLY & anti-bribery).".to_string(),
            ]),
            icon: "/static/about/experience/HUAWEI.svg".to_string(),
            icon_bg: "bg-doom-white",
            social_button: None,
        },
    ]
}
