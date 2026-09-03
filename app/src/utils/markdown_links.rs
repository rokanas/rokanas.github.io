// utils/markdown_links.rs
// renders a plain string containing zero or more markdown-style inline
// links ("...text [link text](url) more text...") into Html, converting
// each [text](url) into an <a> and leaving everything else as plain text.
use yew::prelude::*;

// scans forward for the next well-formed [text](url), skipping past any
// `[` that doesn't form one, mirroring \[([^\]]+)\]\(([^)]+)\) semantics
fn find_next_link(text: &str) -> Option<(usize, usize, &str, &str)> {
    let mut search_start = 0;
    while let Some(rel_start) = text[search_start..].find('[') {
        let start = search_start + rel_start;
        search_start = start + 1;

        let Some(close_bracket) = text[start..].find(']').map(|i| start + i) else {
            continue;
        };
        if close_bracket == start + 1 || text.as_bytes().get(close_bracket + 1) != Some(&b'(') {
            continue;
        }
        let Some(close_paren) = text[close_bracket + 2..].find(')').map(|i| close_bracket + 2 + i) else {
            continue;
        };
        if close_paren == close_bracket + 2 {
            continue;
        }

        return Some((
            start,
            close_paren + 1,
            &text[start + 1..close_bracket],
            &text[close_bracket + 2..close_paren],
        ));
    }
    None
}

pub fn render_inline_links(text: &str) -> Html {
    let mut segments: Vec<Html> = Vec::new();
    let mut rest = text;

    while let Some((start, end, link_text, url)) = find_next_link(rest) {
        if start > 0 {
            segments.push(html! { {rest[..start].to_string()} });
        }
        segments.push(html! {
            <a
                href={url.to_string()}
                target="_blank"
                rel="noopener noreferrer"
                class="text-doom-red hover:text-doom-red-light hover:underline transition-colors duration-200"
            >
                {link_text.to_string()}
            </a>
        });
        rest = &rest[end..];
    }

    if !rest.is_empty() {
        segments.push(html! { {rest.to_string()} });
    }

    html! { <>{ for segments.into_iter() }</> }
}
