use actix_web::{web, HttpRequest, HttpResponse, Result};
use actix_web_actors::ws;
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    Schema,
};
use async_graphql_actix_web::{Request, Response, WSSubscription};

use crate::schemas::IndexSchema;

pub async fn index(schema: web::Data<IndexSchema>, request: Request) -> Response {
    schema.execute(request.into_inner()).await.into()
}

pub async fn index_playground() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            GraphQLPlaygroundConfig::new("/").subscription_endpoint("/"),
        )))
}

pub async fn index_ws(
    schema: web::Data<IndexSchema>,
    req: HttpRequest,
    payload: web::Payload,
) -> Result<HttpResponse> {
    ws::start_with_protocols(
        WSSubscription::new(Schema::clone(&*schema)),
        &["graphql-ws"],
        &req,
        payload,
    )
}
