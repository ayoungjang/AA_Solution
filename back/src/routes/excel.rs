use axum::{extract::{Multipart, Path}, response::{IntoResponse, Json}, http::StatusCode};
use crate::models::{UploadResponse, UploadForm}; // ✅ 꼭 UploadForm도 import 해야 함
use utoipa::ToSchema; // ✅ 필요

#[utoipa::path(
    post,
    path = "/proportion/{bacteria}",
    params(
        ("bacteria" = String, Path, description = "Bacteria name")
    ),
    request_body(
        content = UploadForm,
        content_type = "multipart/form-data",
        description = "Excel file to upload"
    ),
    responses(
        (status = 200, description = "File uploaded successfully", body = UploadResponse),
        (status = 400, description = "No file uploaded", body = UploadResponse),
    )
)]
pub async fn upload_file(Path(bacteria): Path<String>, mut multipart: Multipart) -> impl IntoResponse {
    // 여기에 proportion 처리
}
