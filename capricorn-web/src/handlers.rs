use actix_web::{web, Error, HttpResponse};
use juniper::{graphiql::graphiql_source, http::GraphQLRequest};

use crate::{
    database::Pool,
    schemas::root::{create_schema, Context, Schema},
};

pub async fn graphql(
    pool: web::Data<Pool>,
    schema: web::Data<Schema>,
    request: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    let ctx = Context {
        pool: pool.get_ref().to_owned(),
    };
    let res = web::block(move || {
        let res = request.execute(&schema, &ctx);
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .await
    .map_err(Error::from)?;

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(res))
}

pub async fn graphql_playground() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(graphiql_source("/graphql"))
}

pub fn add_graphql(config: &mut web::ServiceConfig) {
    config
        .data(create_schema())
        .route("/graphql", web::post().to(graphql))
        .route("/graphiql", web::get().to(graphql_playground));
}
