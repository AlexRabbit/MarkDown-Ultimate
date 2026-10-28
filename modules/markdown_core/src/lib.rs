//! Markdown → safe HTML with GFM (tables, tasks, strikethrough) and Telegram-rich extensions.

use ammonia::Builder;
use comrak::{markdown_to_html, ComrakOptions};

pub fn render_markdown(source: &str) -> String {
    let pre = preprocess_telegram_extensions(source);
    let mut opts = ComrakOptions::default();
    opts.extension.table = true;
    opts.extension.tasklist = true;
    opts.extension.strikethrough = true;
    opts.extension.autolink = true;
    opts.extension.tagfilter = false;
    opts.extension.superscript = true;
    opts.extension.footnotes = true;
    opts.render.unsafe_ = true;
    opts.render.github_pre_lang = true;

    let html = markdown_to_html(&pre, &opts);
    sanitize_html(&html)
}

fn preprocess_telegram_extensions(src: &str) -> String {
    let mut s = src.to_string();
    // Telegram markdown aliases → CommonMark / HTML passthrough
    s = s.replace("==", "<mark>").replace("==", "</mark>"); // naive; prefer HTML mode for ==
    s = s.replace("||", "<tg-spoiler>").replace("||", "</tg-spoiler>");
    // Preserve raw HTML blocks for Telegram tags comrak would strip
    s = s
        .replace("<tg-spoiler>", "<span class=\"tg-spoiler\">")
        .replace("</tg-spoiler>", "</span>");
    s = s.replace("<tg-collage>", "<div class=\"tg-collage\">");
    s = s.replace("</tg-collage>", "</div>");
    s = s.replace("<tg-slideshow>", "<div class=\"tg-slideshow\">");
    s = s.replace("</tg-slideshow>", "</div>");
    s = s.replace("<tg-button-row", "<div class=\"tg-button-row\"");
    s = s.replace("</tg-button-row>", "</div>");
    s = s.replace("<tg-button ", "<button class=\"tg-button\" ");
    s = s.replace("<tg-map ", "<div class=\"tg-map\" ");
    s
}

fn sanitize_html(html: &str) -> String {
    let mut builder = Builder::default();
    builder.add_tag_attributes("div", &["class"]);
    builder.add_tag_attributes("span", &["class"]);
    builder.add_tag_attributes("button", &["class", "type", "data-type", "data-url"]);
    builder.add_tag_attributes("table", &["class"]);
    builder.add_tag_attributes("th", &["align"]);
    builder.add_tag_attributes("td", &["align", "colspan", "rowspan"]);
    builder.add_tag_attributes("details", &["open"]);
    builder.add_tag_attributes("summary", &["class"]);
    builder.add_tag_attributes("input", &["type", "checked", "disabled"]);
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
}
