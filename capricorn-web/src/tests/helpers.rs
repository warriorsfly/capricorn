#[cfg(test)]
pub mod tests {

    use crate::{
        config::CONFIG,
        constants,
        datasource::{add_pool, add_redis, init_pool, DatabasePool},
        middlewares,
        routes::routes,
        state::{new_state, AppState},
        utils::jwt::{create_jwt, Claims},
    };
    use actix_web::{
        dev::ServiceResponse,
        test,
        web::{Data, Json},
        App,
    };
    use serde::Serialize;

    fn jwt() -> String {
        let claims = Claims::new(1, 1);
        let jwt = create_jwt(claims).unwrap();
        jwt
    }

    /// Helper for HTTP GET integration tests
    pub async fn test_get(route: &str) -> ServiceResponse {
        let mut app = test::init_service(
            App::new()
                // .wrap(middlewares::JwtAuthorization)
                .configure(add_redis)
                .app_data(app_state())
                .configure(add_pool)
                .configure(routes),
        )
        .await;

        // let cookie = response.response().headers().get(constants::AUTHORIZATION);

        test::call_service(&mut app, test::TestRequest::get().uri(route).to_request()).await
    }

    /// Helper for HTTP GET integration tests
    pub async fn test_post<T: Serialize>(route: &str, params: T) -> ServiceResponse {
        let mut app = test::init_service(
            App::new()
                .configure(add_redis)
                .app_data(app_state())
                .configure(add_pool)
                .configure(routes),
        )
        .await;
        test::call_service(
            &mut app,
            test::TestRequest::post()
                .set_json(&params)
                .uri(route)
                .to_request(),
        )
        .await
    }

    pub async fn assert_get(route: &str) -> ServiceResponse {
        let response = test_get(route).await;
        assert!(response.status().is_success());
        response
    }

    pub async fn assert_post<T: Serialize>(route: &str, params: T) -> ServiceResponse {
        let response = test_post(route, params).await;
        assert!(response.status().is_success());
        response
    }

    // Mock applicate sql connection pool
    pub fn get_pool() -> DatabasePool {
        init_pool(CONFIG.clone()).unwrap()
    }

    /// Returns a r2d2 Pooled Connection wrappedn in Actix Application Data
    pub fn get_data_pool() -> Data<DatabasePool> {
        Data::new(get_pool())
    }

    // Mock applicate state
    pub fn app_state() -> AppState<'static, String> {
        new_state::<String>()
    }
}
