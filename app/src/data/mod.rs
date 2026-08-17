// src/data/mod.rs
// hardcoded site content (temporary, can be moved to a db later), kept
// separate from the pages/components that render it.
pub mod projects;
pub mod doom_projects;
pub mod education;
pub mod experience;
pub mod technologies;
pub mod about;
pub mod social_buttons;

// shared by data::education and data::experience
#[derive(Clone, PartialEq)]
pub struct ExperienceItem {
    pub title: String,
    pub institution: String,
    pub date: String,
    pub description: Option<Vec<String>>,
    pub icon: String,
    pub icon_bg: &'static str, // backdrop class for the icon badge, matching each logo's own background
    pub social_button: Option<&'static str>, // title of a SocialButton (see data::social_buttons) overlaid on the detail panel; None hides it
}
