use crate::models::commit;
use crate::templates::authors;
use crate::utils::gitmoji;

pub fn template_link<'link>(commit: &commit::CommitInfo) -> Box<String> {
    Box::new(format!(
        "<div class=\"item\">
            <div class=\"commit\">
                <div class=\"title\">
                    <div>{}</div>
                </div>
                <div class=\"metadatas\">
                    <div class=\"authors\">
                        {}
                    </div>
                </div>
                <div class=\"content\">
                    Message: {}
                </div>
            </div>
            <div class=\"go\">
                <a href=\"{}\">GO</a>
            </div>
        </div>\n",
        &commit.id[..6],
        *authors::template_authors(&commit.authors),
        gitmoji::replace(String::from(commit.message.as_str())),
        format!("{}.html", &commit.id)
    ))
}
