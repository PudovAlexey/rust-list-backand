use std::str::FromStr;
use axum::Json;
use axum::response::IntoResponse;
use axum::{
    routing::get
};

mod std::goods::routes;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Data {
    payload: String
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address = "0.0.0.0";
	let port = 3000;

    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    let router = axum::Router::new()
    .route("/test/world", get(routes::get_goods));

    log::info!("server listening on http://{}:{}", address, port);
    axum::Server::bind(&std::net::SocketAddr::new(
            std::net::IpAddr::from_str(&address).unwrap(),
            port
        )).serve(router.into_make_service()).await?;
    Ok(())
}
