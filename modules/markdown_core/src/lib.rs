//! Markdown → safe HTML with GFM (tables, tasks, strikethrough) and Telegram-rich extensions.

use ammonia::Builder;
use comrak::{markdown_to_html, Options};
use regex::Regex;
use std::sync::LazyLock;

static SPOILER: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\|\|([^|\n]+?)\|\|").expect("spoiler regex"));

pub fn render_markdown(source: &str) -> String {
    let pre = preprocess_telegram_extensions(source);
    let mut opts = Options::default();
    opts.extension.table = true;
    opts.extension.tasklist = true;
    opts.extension.strikethrough = true;
    opts.extension.autolink = true;
    opts.extension.superscript = true;
    opts.extension.footnotes = true;
    opts.render.r#unsafe = true;
    opts.render.github_pre_lang = true;

    let html = markdown_to_html(&pre, &opts);
    sanitize_html(&html)
}

fn preprocess_telegram_extensions(src: &str) -> String {
    let mut s = src.to_string();
    s = SPOILER
        .replace_all(&s, r#"<span class="tg-spoiler">$1</span>"#)
        .into_owned();
    s = replace_telegram_mark(&s);
    s = s.replace("<tg-spoiler>", "<span class=\"tg-spoiler\">");
    s = s.replace("</tg-spoiler>", "</span>");
    s
}

fn replace_telegram_mark(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let bytes = input.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if i + 1 < bytes.len() && bytes[i] == b'=' && bytes[i + 1] == b'=' {
            let triple = i > 0 && bytes[i - 1] == b'='
                || i + 2 < bytes.len() && bytes[i + 2] == b'=';
            if !triple {
                if let Some(end) = input[i + 2..].find("==") {
                    let inner = &input[i + 2..i + 2 + end];
                    if !inner.contains('=') {
                        out.push_str("<mark>");
                        out.push_str(inner);
                        out.push_str("</mark>");
                        i += 2 + end + 2;
                        continue;
                    }
                }
            }
        }
        if let Some(ch) = input[i..].chars().next() {
            out.push(ch);
            i += ch.len_utf8();
        } else {
            break;
        }
    }
    out
}

fn sanitize_html(html: &str) -> String {
    let mut builder = Builder::default();
    let class = &["class"];
    let attrs = &[
        "class", "open", "type", "checked", "disabled", "align", "colspan", "rowspan", "valign",
        "id", "href", "target", "rel", "data-type", "data-url", "emoji-id", "unix", "format",
        "lat", "long", "zoom", "style", "data-type", "data-url",
    ];
    for tag in [
        "div", "span", "button", "table", "th", "td", "tr", "details", "summary", "figure",
        "figcaption", "mark", "ins", "sub", "sup", "u", "tg-spoiler", "tg-collage", "tg-slideshow",
        "tg-button-row", "tg-button", "tg-map", "tg-emoji", "tg-time", "tg-math", "aside", "cite",
        "pre", "code", "a", "img", "video", "audio", "input", "hr", "footer", "blockquote",
    ] {
        builder.add_tag_attributes(tag, class);
    }
    builder.add_tag_attributes("a", &["href", "name", "class"]);
    builder.add_tag_attributes("img", &["src", "alt", "class"]);
    builder.add_tag_attributes("video", &["src", "class"]);
    builder.add_tag_attributes("audio", &["src", "class"]);
    builder.add_tag_attributes("td", attrs);
    builder.add_tag_attributes("th", attrs);
    builder.clean(html).to_string()
}

pub fn split_view_modes(source: &str) -> (String, String) {
    (source.to_string(), render_markdown(source))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_gfm_table() {
        let md = "| a | b |\n|---|---|\n| 1 | 2 |";
        let html = render_markdown(md);
        assert!(html.contains("<table"));
        assert!(html.contains("1"));
    }

    #[test]
    fn renders_task_list() {
        let md = "- [x] done\n- [ ] todo";
        let html = render_markdown(md);
        assert!(html.contains("done"));
    }

    #[test]
    fn telegram_spoiler_and_mark() {
        let md = "||secret==highlight==";
        let html = render_markdown(&md);
        assert!(html.contains("tg-spoiler") || html.contains("secret"));
        assert!(html.contains("mark") || html.contains("highlight"));
    }
}
