use actix_web::{ web, App, HttpServer };

pub mod util;
pub mod routes;
pub mod structs;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/embed", web::get().to(routes::embed::get))
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
