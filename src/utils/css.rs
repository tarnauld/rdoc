use crate::utils::file::save_file;
use crate::templates::css;

pub fn generate_css_file() {
    match save_file("style.css", css::template_css()) {
        Ok(_) => (),
        Err(_) => (),
    };
}
