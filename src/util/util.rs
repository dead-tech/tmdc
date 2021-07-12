pub fn replace_html_tag_n(mut s: String, from: &str, to: &str) -> String {
    for _ in 0..s.matches(from).count() / 2 {
        s = s.replacen(from, format!("<{}>", to).as_str(), 1);
        s = s.replacen(from, format!("</{}>", to).as_str(), 1);
    }

    s
}
