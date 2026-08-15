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
            "🎓 I'm a recent software engineering graduate eager to begin a career in tech.".to_string()
        ),
        BioParagraph::Plain(
            "🛠 I want to build intuitive software solutions that make your jobs and lives easier, saving you time and effort. Currently learning Rust, Solidity smart contracts and agentic AI implementations.".to_string()
        ),
        BioParagraph::Plain(
            "⚖️ Formerly a legal professional with a focus on personal data and cybersecurity policy compliance and experience both in international organizations and in the private sector.".to_string()
        ),
        BioParagraph::WithEmphasis {
            before: "⛧ I'm also a lifelong ".to_string(),
            emphasis: "Doom".to_string(),
            after: " enthusiast, mapmaker and content creator.".to_string(),
        },
        BioParagraph::Plain(
            "🤼‍♂️ When away from the computer, I enjoy practicing mixed-martial arts, submission wrestling, rock-climbing and playing electric guitar.".to_string()
        ),
    ]
}
