use axum::{
    routing::post,
    extract::Multipart,
    Router,
    http::StatusCode,
    response::{IntoResponse, Json},
};
use std::net::SocketAddr;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use std::path::Path;
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;
use serde::Serialize;

#[derive(OpenApi)]
#[openapi(
    paths(upload_file),
    components(schemas(UploadResponse)),
    tags((name = "File Upload", description = "Endpoints for file upload"))
)]
struct ApiDoc;

#[derive(Serialize, ToSchema)]
struct UploadResponse {
    filename: String,
    message: String,
}


#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/upload", post(upload_file))
        .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", ApiDoc::openapi()));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("🚀 Server running at http://{}", addr);
    println!("📜 API Docs available at http://{}/docs", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// 파일 업로드 핸들러
#[utoipa::path(
    post,
    path = "/upload",
    request_body(
        content = String,
        description = "File to upload",
        content_type = "multipart/form-data"
    ),
    responses(
        (status = 200, description = "File uploaded successfully", body = UploadResponse),
        (status = 400, description = "No file uploaded", body = UploadResponse),
    )
)]

async fn upload_file(mut multipart: Multipart) -> impl IntoResponse {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let filename = field.file_name().unwrap_or("unknown_file").to_string();
        let data = field.bytes().await.unwrap();

        println!("📂 Received file: {} ({} bytes)", filename, data.len());

        // 파일 저장
        let filepath = Path::new("uploads").join(&filename);
        let mut file = File::create(&filepath).await.unwrap();
        file.write_all(&data).await.unwrap();

        return (
            StatusCode::OK,
            Json(UploadResponse {
                filename,
                message: "File uploaded successfully!".to_string(),
            }),
        );
    }

    (
        StatusCode::BAD_REQUEST,
        Json(UploadResponse {
            filename: "".to_string(),
            message: "No file uploaded".to_string(),
        }),
    )
}
