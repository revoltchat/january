use actix_web::{web, App, HttpServer};

pub mod routes;
pub mod structs;
pub mod util;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/embed", web::get().to(routes::embed::get))
            .route("/proxy", web::get().to(routes::proxy::get))
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
