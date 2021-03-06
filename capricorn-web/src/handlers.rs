use actix_web::{web, Error, HttpResponse};

use juniper_actix::{
    graphiql_handler as gqli_handler, graphql_handler, playground_handler as play_handler,
};

use crate::{
    database::DatabasePool,
    schemas::root::{DataSource, Schema},
};

pub async fn graphiql_handler() -> Result<HttpResponse, Error> {
    gqli_handler("/", None).await
}
pub async fn playground_handler() -> Result<HttpResponse, Error> {
    play_handler("/", None).await
}
pub async fn graphql(
    req: actix_web::HttpRequest,
    payload: actix_web::web::Payload,
    pool: web::Data<DatabasePool>,
    schema: web::Data<Schema>,
) -> Result<HttpResponse, Error> {
    let ctx = DataSource {
        database: pool.get_ref().to_owned(),
    };
    graphql_handler(&schema, &ctx, req, payload).await
}
