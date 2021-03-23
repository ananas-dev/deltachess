use actix_web::{web, Error, HttpResponse};
use juniper_actix::{graphql_handler, playground_handler};

use crate::graphql::schema::Schema;

async fn playground_route() -> Result<HttpResponse, Error> {
    playground_handler("/graphql", None).await
}
async fn graphql_route(
    req: actix_web::HttpRequest,
    payload: actix_web::web::Payload,
    schema: web::Data<Schema>,
) -> Result<HttpResponse, Error> {
    graphql_handler(&schema, &(), req, payload).await
}

pub fn routes(app: &mut web::ServiceConfig) {
    app.service(
        web::resource("/graphql")
            .route(web::post().to(graphql_route))
            .route(web::get().to(graphql_route)),
    )
    .service(web::resource("/playground").route(web::get().to(playground_route)));
}
