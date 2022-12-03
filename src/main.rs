use actix_web::{get, web, App, HttpServer, Responder};
use serde::Deserialize;
const CONTRACT_ID: &str = "dev-18952.r-64063-70432.ewtd.testnet";
const NEAR_URL: &str = "https://rpc.testnet.near.org";

mod helper;
mod market_routes;
mod traits_routes;
use market_routes::*;
use traits_routes::*;
#[get("/")]
async fn entry_point() -> impl Responder {
    format!("Server Time: {}", chrono::Utc::now())
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    //get env port
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "5001".to_string())
        .parse()
        .unwrap();
    println!("Server running at port {}", port);
    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/v2")
                    .service(market)
                    .service(market_by_id)
                    .service(traits)
                    .service(traits_by_id),
            )
            .service(entry_point)
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}

#[derive(Debug, Deserialize)]
pub struct QueryResult {
    pub result: Vec<u8>,
}
