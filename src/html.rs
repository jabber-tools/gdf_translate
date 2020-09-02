use lazy_static::lazy_static;

const HTML_TAG_NAMES: [&str; 118] = [
    "a",
    "abbr",
    "address",
    "area",
    "article",
    "aside",
    "audio",
    "b",
    "base",
    "bdi",
    "bdo",
    "blockquote",
    "body",
    "br",
    "button",
    "canvas",
    "caption",
    "cite",
    "code",
    "col",
    "colgroup",
    "data",
    "datalist",
    "dd",
    "del",
    "details",
    "dfn",
    "dialog",
    "div",
    "dl",
    "dt",
    "em",
    "embed",
    "fieldset",
    "figcaption",
    "figure",
    "footer",
    "form",
    "h1",
    "h2",
    "h3",
    "h4",
    "h5",
    "h6",
    "head",
    "header",
    "hgroup",
    "hr",
    "html",
    "i",
    "iframe",
    "img",
    "input",
    "ins",
    "kbd",
    "keygen",
    "label",
    "legend",
    "li",
    "link",
    "main",
    "map",
    "mark",
    "math",
    "menu",
    "menuitem",
    "meta",
    "meter",
    "nav",
    "noscript",
    "object",
    "ol",
    "optgroup",
    "option",
    "output",
    "p",
    "param",
    "picture",
    "pre",
    "progress",
    "q",
    "rb",
    "rp",
    "rt",
    "rtc",
    "ruby",
    "s",
    "samp",
    "script",
    "section",
    "select",
    "slot",
    "small",
    "source",
    "span",
    "strong",
    "style",
    "sub",
    "summary",
    "sup",
    "svg",
    "table",
    "tbody",
    "td",
    "template",
    "textarea",
    "tfoot",
    "th",
    "thead",
    "time",
    "title",
    "tr",
    "track",
    "u",
    "ul",
    "var",
    "video",
    "wbr",
];

lazy_static! {
    pub static ref HTML_TAGS: Vec<String> = {
        let mut tags = vec![];
        for tag in HTML_TAG_NAMES.iter() {
            let starting_tag = format!("<{}>", tag);
            let closing_tag = format!("</{}>", tag);
            let standalone_tag_1 = format!("<{}/>", tag);
            let standalone_tag_2 = format!("<{} />", tag);
            tags.push(starting_tag);
            tags.push(closing_tag);
            tags.push(standalone_tag_1);
            tags.push(standalone_tag_2);
        }
        tags
    };
}

pub fn is_html(str_val: &str) -> bool {
    for tag in HTML_TAGS.iter() {
        if str_val.to_lowercase().contains(tag) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;
    // cargo test -- --show-output test_is_html
    #[test]
    fn test_is_html() {
        assert_eq!(is_html("<tr>"), true);
        assert_eq!(is_html("<tr2 />"), false);
        assert_eq!(is_html("this is html <br/>"), true);
        assert_eq!(is_html("this is html <br>"), true);
        assert_eq!(is_html("and  this is not <brrr/>!!"), false);
    }
}
