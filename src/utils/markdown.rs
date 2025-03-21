pub fn markdown_to_html(content: &str) -> String {
    let mut html = String::new();
    for line in content.lines() {
        if line.starts_with("# ") {
            html.push_str(&format!("<h1>{}</h1>", &line[2..]));
        } else if line.starts_with("## ") {
            html.push_str(&format!("<h2>{}</h2>", &line[3..]));
        } else if line.starts_with("- ") {
            html.push_str(&format!("<li>{}</li>", &line[2..]));
        } else if line.is_empty() {
            html.push_str("<br />");
        } else {
            html.push_str(&format!("<p>{}</p>", line));
        }
    }
    html
}
