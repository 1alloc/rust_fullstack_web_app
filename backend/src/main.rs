use actix_web::{App, HttpServer};
use tokio_postgres::NoTls;
mod config;
mod handlers;
mod models;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = crate::config::Config::from_file("Config.toml").unwrap();
    let pool = config.pg.create_pool(NoTls).unwrap();

    let server = HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .configure(routes::init::initialize)
    })
        .bind(format!("{}:{}", config.server.host, config.server.port))?
        .run();

    println!("Server running at {}:{}", config.server.host, config.server.port);

    server.await
}
