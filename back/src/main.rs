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

use axum::extract::DefaultBodyLimit; //for file upload size upgrade

use tower_http::trace::TraceLayer;
use tracing_subscriber;

mod routes;
mod docs;


async fn handler_404(request: HttpRequest<hyper::Body>) -> impl IntoResponse {
    let path = request.uri().path(); // ✅ 요청 경로 가져오기
    println!("🚨 404 fallback: unmatched path => {}", path);

    (StatusCode::NOT_FOUND, format!("404 Not Found: {}", path))
}
#[tokio::main]
async fn main() {
    // tracing 초기화
    tracing_subscriber::fmt::init();  // 👈 추가

    println!("🚀 Starting server...");

    let app = Router::new()
        .route("/proportion/{bacteria}", post(routes::excel::upload_file))
        .layer(DefaultBodyLimit::max(10 * 1024 * 1024))
        .merge(
            SwaggerUi::new("/docs")
                .url("/api-docs/openapi.json", docs::ApiDoc::openapi()), 
        )
        .layer(TraceLayer::new_for_http())
        .fallback(handler_404);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("🚀 Server running at http://{}", addr);
    println!("📜 API Docs available at http://{}/docs", addr);
    
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
