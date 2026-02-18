use axum::Router;
use utoipa::OpenApi;
use utoipa_axum::{router::OpenApiRouter, routes};
use utoipa_swagger_ui::SwaggerUi;

pub async fn run_server() {
    let app = create_router();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(OpenApi)]
#[openapi(info(description = "My Api description", license(name = "MIT")))]
struct MyApiDoc;

pub fn create_router() -> Router {
    let endpoints_router = OpenApiRouter::new()
        .routes(routes!(crate::endpoints::table::provide_csv_table_info))
        .routes(routes!(crate::endpoints::table::provide_table_info))
        .routes(routes!(crate::endpoints::table::provide_table_rows))
        .routes(routes!(crate::endpoints::dir::list_path));

    let open_api = OpenApiRouter::with_openapi(MyApiDoc::openapi())
        .nest("/api", endpoints_router);

    let (router, api) = open_api.split_for_parts();

    let app = router
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api.clone()))
        .layer(tower_http::cors::CorsLayer::very_permissive());
    app
}
