mod recipes;
pub mod schema;

use actix_web::{middleware, web, App, HttpServer};

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Hello, world!");

    let conn = &mut establish_connection();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(web::scope("/recipes").configure(recipes::config))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
