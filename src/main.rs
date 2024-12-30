use actix_web::{web, App, HttpServer, http::header};
use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;
use dotenv::dotenv;
use std::env;
use todo_rust::handlers;
use actix_cors::Cors;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let cors_origin = env::var("CORS_ALLOWED_ORIGIN")
        .expect("CORS_ALLOWED_ORIGIN must be set");

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    println!("Starting server at http://localhost:8080");
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(&cors_origin)
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            .allowed_header(header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
            .service(handlers::list_tasks)
            .service(handlers::create_task)
            .service(handlers::update_task)
            .service(handlers::delete_task)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
