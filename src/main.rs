// use salvo::catcher::Catcher;
// use salvo::conn::rustls::{Keycert, RustlsConfig};
// use salvo::prelude::*;
// use salvo::server::ServerHandle;
// use serde::Serialize;
// use tokio::signal;
// use tracing::info;

mod app_config;

use crate::app_config::app_init;
use daoyi_cloud_common::{
    config, db, hoops, common_test_routers_example, shutdown_signal, tokio, Catcher, Keycert, Listener, RustlsConfig
    , Server, Service, TcpListener,
};

#[tokio::main]
async fn main() {
    config::common_init(app_init());
    let config = config::get();
    db::init(&config.db).await;

    let service = Service::new(common_test_routers_example::root())
        .catcher(Catcher::default().hoop(hoops::error_404))
        .hoop(hoops::cors_hoop());
    println!("🔄 在以下位置监听 {}", &config.listen_addr);
    //Acme 支持，自动从 Let's Encrypt 获取 TLS 证书。例子请看 https://github.com/salvo-rs/salvo/blob/main/examples/acme-http01-quinn/src/main.rs
    if let Some(tls) = &config.tls {
        let listen_addr = &config.listen_addr;
        println!(
            "📖 Open API Page: https://{}/scalar",
            listen_addr.replace("0.0.0.0", "127.0.0.1")
        );
        println!(
            "🔑 Login Page: https://{}/login",
            listen_addr.replace("0.0.0.0", "127.0.0.1")
        );
        let config = RustlsConfig::new(Keycert::new().cert(tls.cert.clone()).key(tls.key.clone()));
        let acceptor = TcpListener::new(listen_addr).rustls(config).bind().await;
        let server = Server::new(acceptor);
        tokio::spawn(shutdown_signal(server.handle()));
        server.serve(service).await;
    } else {
        println!(
            "📖 Open API 页面: http://{}/scalar",
            config.listen_addr.replace("0.0.0.0", "127.0.0.1")
        );
        println!(
            "🔑 Login Page: http://{}/login",
            config.listen_addr.replace("0.0.0.0", "127.0.0.1")
        );
        let acceptor = TcpListener::new(&config.listen_addr).bind().await;
        let server = Server::new(acceptor);
        tokio::spawn(shutdown_signal(server.handle()));
        server.serve(service).await;
    }
}
