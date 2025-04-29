use axum::{
    Router,
    routing::post,
};
use axum::response::IntoResponse;
use axum::http::{StatusCode, Request as HttpRequest};

use std::net::SocketAddr;
use hyper::Server;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use tower_http::trace::TraceLayer;

use chrono::Local;
use axum::extract::DefaultBodyLimit; //for file upload size upgrade

use tracing_subscriber;

mod docs;
mod routes;

async fn handler_404(request: HttpRequest<hyper::Body>) -> impl IntoResponse {
    let path = request.uri().path
    (); 
    println!("🚨 404 fallback: unmatched path => {}", path);

    (StatusCode::NOT_FOUND, format!("404 Not Found: {}", path))
}



#[tokio::main]
async fn main() {
    // tracing init
    tracing_subscriber::fmt::init();

    println!("🚀 Starting server at {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S"));

    // let app = Router::new()
    // // .layer(DefaultBodyLimit::max(10 * 1024 * 1024))
    // .merge(
    //     SwaggerUi::new("/docs")
    //     .url("/api-docs/openapi.json", docs::ApiDoc::openapi()), 
    // )
    // .route("/proportion/{bacteria}", post(routes::excel::upload_file))
    //     .layer(TraceLayer::new_for_http())
    //     .fallback(handler_404); //404 handler middleware.
    
     let app = Router::new()
        .route("/proportion/:bacteria", post(routes::excel::upload_file)) 
        .route("/test", post(routes::excel::test_api))
        .layer(TraceLayer::new_for_http())
        .merge(
            SwaggerUi::new("/docs")                           // ✅ Swagger UI 나중에
                .url("/api-docs/openapi.json", docs::ApiDoc::openapi()),
        )
        .fallback(handler_404);                               // ✅ fallback

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("🚀 Server running at http://{}", addr);
    println!("📜 API Docs available at http://{}/docs", addr);
    
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
