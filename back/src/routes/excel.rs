use axum::{
    extract::Multipart,
    response::{IntoResponse, Json},
    http::StatusCode,
};
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;
use tokio::fs::{self, File};
use tokio::io::AsyncWriteExt;
use uuid::Uuid;
use std::collections::HashMap;
use calamine::{Reader, open_workbook_auto, Xlsx};
 use axum::extract::Path;
use crate::docs::UploadForm;
use crate::docs::UploadResponse;

#[utoipa::path(
    post,
    path = "/proportion/{bacteria}",
    tag = "Proportion",
    params(
        ("bacteria" = String, Path, description = "Bacteria name")
    ),
    request_body(
        content = UploadForm,
        description = "Excel file to upload",
        content_type = "multipart/form-data"
    ),
    responses(
        (status = 200, description = "File uploaded successfully", body = UploadResponse),
        (status = 400, description = "No file uploaded", body = UploadResponse),
    )
)]
pub async fn upload_file(Path(bacteria): Path<String>, mut multipart: Multipart) -> impl IntoResponse {
    println!("🚀 upload file...");
    // Get the file name
    while let Some(field) = multipart.next_field().await.unwrap() {
        if let Some(name) = field.name() {
            if name == "file" {
                let file_name = field.file_name().unwrap_or("unknown").to_string();
                // Print the file name to the console (for debugging)
                println!("Uploaded file: {}", file_name);

                // Return a simple success response
                let response = UploadResponse {
                    message: format!("File uploaded successfully: {}", file_name),
                };
                return (StatusCode::OK, Json(response));
            }
        }
    }

    // If no file was uploaded
    let response = UploadResponse {
        message: "No file uploaded".to_string(),
    };
    (StatusCode::BAD_REQUEST, Json(response))
}
