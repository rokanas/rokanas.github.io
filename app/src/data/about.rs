// data/about.rs
// hardcoded bio paragraphs for the About page's introduction.
// WithEmphasis exists to bold a single word ("Doom")
pub enum BioParagraph {
    Plain(String),
    WithEmphasis { before: String, emphasis: String, after: String },
}

pub fn bio_paragraphs() -> Vec<BioParagraph> {
    vec![
        BioParagraph::Plain(
            "💻 I'm software engineer working in Web3 / DeFi development and passionate about decentralized solutions.".to_string()
        ),
        BioParagraph::Plain(
            "⚖️ Formerly a legal professional with a focus on personal data and cybersecurity compliance.".to_string()
        ),
        BioParagraph::WithEmphasis {
            before: "⛧ I'm also a lifelong ".to_string(),
            emphasis: "DOOM".to_string(),
            after: " enthusiast, mapmaker and content creator.".to_string(),
        },
        BioParagraph::Plain(
            "🤼‍♂️ When AFK, I enjoy practicing submission wrestling, mixed-martial arts, rock-climbing and electric guitar.".to_string()
        ),
    ]
}
