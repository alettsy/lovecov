pub fn escape_html(html: &str) -> String {
    html.replace("&", "&amp;")
        .replace(" ", "&nbsp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
        .replace("\"", "&quot;")
        .replace("'", "&#39;")
}
