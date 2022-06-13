use crate::models::commit;
use crate::templates::{fingerprint, authors};
use crate::utils::gitmoji;

pub fn template_link<'link>(commit: &commit::CommitInfo) -> Box<String>{
    Box::new(format!(
        "<a href=\"{}\" class=\"item\">
            {}
            <div class=\"commit-id\">
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
        </a>\n",
        format!("{}.html", commit.id),
        fingerprint::template_fingerprint(),
        &commit.id[..6],
        *authors::template_authors(&commit.authors),
        gitmoji::replace(String::from(commit.message.as_str()))
    ))
}