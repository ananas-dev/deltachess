use std::env;

use actix_cors::Cors;
use actix_session::CookieSession;
use actix_web::{http::header, middleware, App, HttpServer};

mod graphql;

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let server = HttpServer::new(move || {
        App::new()
            .data(graphql::schema::schema())
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["POST", "GET"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600),
            )
            .wrap(CookieSession::signed(&[0; 32]).secure(false))
            .configure(graphql::routes::routes)
    });
    server.bind(("0.0.0.0", 8080)).unwrap().run().await
}
