mod recipes;

use actix_web::{middleware, web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Hello, world!");

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(web::scope("/recipes").configure(recipes::config))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
