use actix_web::{App, HttpRequest, HttpServer, Responder, web::{self, Query}};
use scraper::{Html, Selector};
use serde::{Deserialize};

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/embed", web::get().to(embed))
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}

#[derive(Deserialize)]
struct Info {
    url: String,
}

async fn embed(info: Query<Info>) -> impl Responder {
    let url = info.into_inner().url;

    let resp = reqwest::get(url).await.unwrap();
    assert!(resp.status().is_success());

    let body = resp.text().await.unwrap();
    let fragment = Html::parse_document(&body);
    let selector = Selector::parse("meta").unwrap();

    for el in fragment.select(&selector) {
        let node = el.value();
        dbg!(node.attr("property"));
        dbg!(node.attr("content"));
    }

    format!("gaming has been deposited into stdout")
}
