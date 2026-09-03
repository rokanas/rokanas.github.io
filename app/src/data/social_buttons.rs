// data/social_buttons.rs
// hardcoded social/contact link content, shared by every place SocialButtons renders
// (about.rs footer, contact.rs, and the overlay badge in components/experience.rs)

#[derive(Clone, PartialEq)]
pub enum SocialButtonTag {
    General,
    Professional,
    Personal,
    Other,
}

#[derive(Clone, PartialEq)]
pub struct SocialButton {
    pub title: &'static str, // also used as a lookup key, e.g. SocialButtonsProps::only
    pub url: &'static str,
    pub icon: &'static str, // path to an svg asset under /static/social_buttons/
    pub bg: &'static str,
    pub border: &'static str,
    pub hover_bg: &'static str,
    pub hover_border: &'static str,
    pub tags: &'static [SocialButtonTag],
}

pub fn social_buttons_data() -> Vec<SocialButton> {
    vec![
        SocialButton {
            title: "Github",
            url: "https://github.com/rokanas",
            icon: "/static/social_buttons/GITHUB.svg",
            bg: "bg-doom-panel-inner",
            border: "border-black",
            hover_bg: "hover:bg-gray-700",
            hover_border: "hover:border-gray-500",
            tags: &[SocialButtonTag::General],
        },
        SocialButton {
            title: "Github Hype",
            url: "https://github.com/hypekostas",
            icon: "/static/social_buttons/GITHUB_HYPE.svg",
            bg: "bg-black",
            border: "border-gray-800",
            hover_bg: "hover:bg-black",
            hover_border: "hover:border-[#2fb2d9]",
            tags: &[SocialButtonTag::Other],
        },
        SocialButton {
            title: "LinkedIn",
            url: "https://www.linkedin.com/in/konstantinos-rokanas-1ab1a113a/",
            icon: "/static/social_buttons/LINKEDIN.svg",
            bg: "bg-doom-panel-inner",
            border: "border-black",
            hover_bg: "hover:bg-blue-600",
            hover_border: "hover:border-blue-500",
            tags: &[SocialButtonTag::General],
        },
        SocialButton {
            title: "CV",
            url: "/static/about/KR_CV.pdf",
            icon: "/static/social_buttons/CV.svg",
            bg: "bg-doom-panel-inner",
            border: "border-black",
            hover_bg: "hover:bg-doom-red",
            hover_border: "hover:border-red-900",
            tags: &[SocialButtonTag::Professional],
        },
        SocialButton {
            title: "Discord",
            url: "https://discord.gg/TODO",
            icon: "/static/social_buttons/DISCORD.svg",
            bg: "bg-doom-panel-inner",
            border: "border-black",
            hover_bg: "hover:bg-[#5865F2]",
            hover_border: "hover:border-[#5865F2]",
            tags: &[SocialButtonTag::Personal],
        },
        SocialButton {
            title: "Instagram",
            url: "https://www.instagram.com/charybdis.maw/",
            icon: "/static/social_buttons/INSTAGRAM.svg",
            bg: "bg-doom-panel-inner",
            border: "border-black",
            hover_bg: "hover:bg-gradient-to-br hover:from-purple-600 hover:to-pink-500",
            hover_border: "hover:border-pink-500",
            tags: &[SocialButtonTag::Personal],
        },
    ]
}
