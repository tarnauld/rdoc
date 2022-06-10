pub fn template_search() -> Box<String> {
    Box::new(String::from("
    <div class=\"search\">
        <input type=\"text\" placeholder=\"Filter...\"/>
    </div>
    "))
}