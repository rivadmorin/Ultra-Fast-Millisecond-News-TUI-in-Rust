pub fn cleanse_html(html: &str) -> String {
    // 1. Convert HTML to Markdown (handles most tag stripping)
    let md = html2md::parse_html(html);

    // 2. Further cleaning
    md.lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn cleanse_text(text: &str) -> String {
    text.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cleanse_html() {
        let html = "<div><p>Hello World</p><p></p><span>   </span></div>";
        let cleaned = cleanse_html(html);
        assert_eq!(cleaned, "Hello World");
    }

    #[test]
    fn test_cleanse_text() {
        let text = "  Some messy text  \n\n ";
        assert_eq!(cleanse_text(text), "Some messy text");
    }
}
