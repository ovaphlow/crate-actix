use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use sqlx::mysql::MySqlSslMode;
use sqlx::mysql::MySqlConnectOptions;
use sqlx::mysql::MySqlPoolOptions;
use std::env;

mod event;
use event::get_event;

async fn index(req: HttpRequest) -> impl Responder {
    print!("{} {}\n", req.method().to_string(), req.uri().to_string());
    HttpResponse::Ok().body("Hello world")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_username = env::var("DATABASE_USERNAME").expect("DATABASE_USERNAME must be set");
    let database_password = env::var("DATABASE_PASSWORD").expect("DATABASE_PASSWORD must be set");
    let database_host = env::var("DATABASE_HOST").expect("DATABASE_HOST must be set");
    let database_port_str = env::var("DATABASE_PORT").expect("DATABASE_PORT must be set");
    let database_port: u16 = database_port_str.parse().expect("DATABASE_PORT must be a number");
    let database_name = env::var("DATABASE_NAME").expect("DATABASE_NAME must be set");
    let options = MySqlConnectOptions::new()
        .username(&database_username)
        .password(&database_password)
        .host(&database_host)
        .port(database_port)
        .database(&database_name)
        .ssl_mode(MySqlSslMode::Disabled);
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await
        .expect("Failed to create pool.");
    println!("Connected to {}", database_host);

    let server_address = env::var("SERVER_ADDRESS").expect("SERVER_ADDRESS must be set");
    let server_port: u16 = env::var("SERVER_PORT")
        .expect("SERVER_PORT must be set")
        .parse()
        .expect("SERVER_PORT must be a number");

    HttpServer::new(move || {
        App::new()
            .app_data(pool.clone())
            .route("/", web::get().to(index))
            .route("/crate-api/event", web::get().to(get_event))
    })
    .bind((server_address, server_port))?
    .run()
    .await
}
