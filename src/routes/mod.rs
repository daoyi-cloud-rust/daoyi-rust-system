use daoyi_cloud_common::rust_embed;
use daoyi_cloud_common::rust_embed::RustEmbed;
use daoyi_cloud_common::salvo::serve_static::{EmbeddedFileExt, static_embed};
use daoyi_cloud_common::{Logger, OpenApi, Router, Scalar};
mod demo;

#[derive(RustEmbed)]
#[folder = "assets"]
struct Assets;

pub fn root() -> Router {
    let favicon = Assets::get("favicon.ico")
        .expect("favicon not found")
        .into_handler();
    let router = Router::new()
        .hoop(Logger::new())
        .get(demo::index)
        .push(Router::with_path("favicon.ico").get(favicon))
        .push(Router::with_path("assets/{**rest}").get(static_embed::<Assets>()));
    let doc = OpenApi::new("道一开源 Rust Admin web api", "0.0.1").merge_router(&router);
    router
        .unshift(doc.into_router("/api-doc/openapi.json"))
        .unshift(Scalar::new("/api-doc/openapi.json").into_router("scalar"))
}
