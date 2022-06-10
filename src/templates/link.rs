use crate::models::commit;
use crate::templates::{fingerprint, authors};

pub fn template_link<'link>(commit: &commit::CommitInfo) -> Box<String>{
    Box::new(format!(
        "<div class=\"item\">
            {}
            <div class=\"commit-id\">
                <div class=\"title\">
                    <a href=\"{}.html\">{}</a>
                </div>
                <div class=\"metadatas\">
                    <div class=\"authors\">
                        {}
                    </div>
                </div>
                <div class=\"content\">
                    {}
                </div>
            </div>
        </div>\n",
        fingerprint::template_fingerprint(),
        commit.id,
        &commit.id[..6],
        *authors::template_authors(&commit.authors),
        commit.message
    ))
}