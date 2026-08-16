// utils/markdown_links.rs
// renders a plain string containing zero or more markdown-style inline
// links ("...text [link text](url) more text...") into Html, converting
// each [text](url) into an <a> and leaving everything else as plain text.
use std::sync::OnceLock;
use regex::Regex;
use yew::prelude::*;

fn link_pattern() -> &'static Regex {
    static PATTERN: OnceLock<Regex> = OnceLock::new();
    PATTERN.get_or_init(|| Regex::new(r"\[([^\]]+)\]\(([^)]+)\)").unwrap())
}

pub fn render_inline_links(text: &str) -> Html {
    let re = link_pattern();
    let mut segments: Vec<Html> = Vec::new();
    let mut last_end = 0;

    for capture in re.captures_iter(text) {
        let whole_match = capture.get(0).unwrap();
        if whole_match.start() > last_end {
            segments.push(html! { {text[last_end..whole_match.start()].to_string()} });
        }

        let link_text = capture.get(1).unwrap().as_str().to_string();
        let url = capture.get(2).unwrap().as_str().to_string();
        segments.push(html! {
            <a
                href={url}
                target="_blank"
                rel="noopener noreferrer"
                class="text-red-600 hover:text-red-400 hover:underline transition-colors duration-200"
            >
                {link_text}
            </a>
        });

        last_end = whole_match.end();
    }

    if last_end < text.len() {
        segments.push(html! { {text[last_end..].to_string()} });
    }

    html! { <>{ for segments.into_iter() }</> }
}
