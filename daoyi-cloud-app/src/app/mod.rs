mod root_router;

use daoyi_cloud_config::config;
use daoyi_cloud_hoops::hoops;
pub use daoyi_cloud_models::models::error::AppError;
use salvo::catcher::Catcher;
use salvo::conn::rustls::{Keycert, RustlsConfig};
use salvo::prelude::*;
use salvo::server::ServerHandle;
use tokio::signal;
use tracing::info;

pub async fn start(router: Router) {
    let config = config::get();
    let service = Service::new(root_router::root(router))
        .catcher(Catcher::default().hoop(hoops::error_handler::http_error_handler))
        .hoop(hoops::cors_hoop());
    println!("🔄 在以下位置监听 {}", &config.web.listen_addr);
    //Acme 支持，自动从 Let's Encrypt 获取 TLS 证书。例子请看 https://github.com/salvo-rs/salvo/blob/main/examples/acme-http01-quinn/src/main.rs
    if let Some(tls) = &config.tls {
        let listen_addr = &config.web.listen_addr;
        println!(
            "📖 Open API Page: https://{}/scalar",
            listen_addr.replace("0.0.0.0", "127.0.0.1")
        );
        println!(
            "🔑 Open API JSON: https://{}/api-doc/openapi.json",
            listen_addr.replace("0.0.0.0", "127.0.0.1")
        );
        // println!(
        //     "🔑 Login Page: https://{}/login",
        //     listen_addr.replace("0.0.0.0", "127.0.0.1")
        // );
        let config = RustlsConfig::new(Keycert::new().cert(tls.cert.clone()).key(tls.key.clone()));
        let acceptor = TcpListener::new(listen_addr).rustls(config).bind().await;
        let server = Server::new(acceptor);
        tokio::spawn(shutdown_signal(server.handle()));
        server.serve(service).await;
    } else {
        println!(
            "📖 Open API 页面: http://{}/scalar",
            config.web.listen_addr.replace("0.0.0.0", "127.0.0.1")
        );
        println!(
            "🔑 Open API JSON: http://{}/api-doc/openapi.json",
            config.web.listen_addr.replace("0.0.0.0", "127.0.0.1")
        );
        // println!(
        //     "🔑 Login Page: http://{}/login",
        //     config.web.listen_addr.replace("0.0.0.0", "127.0.0.1")
        // );
        let acceptor = TcpListener::new(&config.web.listen_addr).bind().await;
        let server = Server::new(acceptor);
        tokio::spawn(shutdown_signal(server.handle()));
        server.serve(service).await;
    }
}

async fn shutdown_signal(handle: ServerHandle) {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => info!("ctrl_c signal received"),
        _ = terminate => info!("terminate signal received"),
    }
    handle.stop_graceful(std::time::Duration::from_secs(60));
}

#[cfg(test)]
mod tests {
    use salvo::prelude::*;
    use salvo::test::{ResponseExt, TestClient};

    use daoyi_cloud_config::config;

    #[tokio::test]
    async fn test_hello_world() {
        config::init(Some(String::from(env!("CARGO_MANIFEST_DIR")))).await;

        let service = Service::new(Router::new());

        let content = TestClient::get(format!(
            "http://{}",
            config::get()
                .web
                .listen_addr
                .replace("0.0.0.0", "127.0.0.1")
        ))
        .send(&service)
        .await
        .take_string()
        .await
        .unwrap();
        assert_eq!(content, "Hello World from salvo");
    }
}
