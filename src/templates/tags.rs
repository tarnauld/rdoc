pub fn template_tags(tags: &String) -> Box<String> {
    let mut s: String = String::from("");

    tags.split(";").for_each(|tag| {
        s.push_str(format!("<div class=\"tag\">{}</div>", tag).as_str());
    });

    Box::new(s)
}
