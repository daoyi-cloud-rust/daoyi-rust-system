use daoyi_cloud_common::{
    Catcher, Keycert, Listener, RustlsConfig, Server, Service, TcpListener, config, common_hoops, info,
    shutdown_signal, tokio,
};
use daoyi_rust_system::{app_config, routes};

#[tokio::main]
async fn main() {
    app_config::app_init().await;
    let config = config::get();

    let service = Service::new(routes::root())
        .catcher(Catcher::default().hoop(common_hoops::error_404))
        .hoop(common_hoops::cors_hoop());
    info!("🔄 在以下位置监听 {}", &config.listen_addr);
    //Acme 支持，自动从 Let's Encrypt 获取 TLS 证书。例子请看 https://github.com/salvo-rs/salvo/blob/main/examples/acme-http01-quinn/src/main.rs
    if let Some(tls) = &config.tls {
        let listen_addr = &config.listen_addr;
        info!(
            "📖 Open API Page: https://{}/scalar",
            listen_addr.replace("0.0.0.0", "127.0.0.1")
        );
        let config = RustlsConfig::new(Keycert::new().cert(tls.cert.clone()).key(tls.key.clone()));
        let acceptor = TcpListener::new(listen_addr).rustls(config).bind().await;
        let server = Server::new(acceptor);
        tokio::spawn(shutdown_signal(server.handle()));
        server.serve(service).await;
    } else {
        info!(
            "📖 Open API 页面: http://{}/scalar",
            config.listen_addr.replace("0.0.0.0", "127.0.0.1")
        );
        let acceptor = TcpListener::new(&config.listen_addr).bind().await;
        let server = Server::new(acceptor);
        tokio::spawn(shutdown_signal(server.handle()));
        server.serve(service).await;
    }
}
