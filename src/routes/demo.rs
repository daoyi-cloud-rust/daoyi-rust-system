use askama::Template;
use daoyi_cloud_common::{AppResult, Request, Text, handler, salvo};

#[handler]
pub async fn index(req: &mut Request) -> AppResult<Text<String>> {
    #[derive(Template)]
    #[template(path = "index.html")]
    struct IndexTemplate<'a> {
        name: &'a str,
    }
    let index_tmpl = IndexTemplate {
        name: req.query::<&str>("name").unwrap_or("World"),
    };
    Ok(Text::Html(index_tmpl.render().unwrap()))
}
