pub fn template_authors(authors: &String) -> Box<String> {
    let mut s: String = String::from("");

    authors.split(";").for_each(| author | {
        s.push_str(format!("<div class=\"author\">{}</div>", author).as_str());
    });

    Box::new(s)
}