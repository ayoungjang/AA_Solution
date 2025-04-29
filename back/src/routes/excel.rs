use axum::{
    extract::{Multipart,Query},
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

use crate::docs::UploadResponse;
use crate::docs::UploadForm;

#[utoipa::path(
    post,
    path = "/proportion/{bacteria}",
    tag = "Proportion",
    request_body(
        content = UploadForm,
        content_type = "multipart/form-data",
        description = "Form with Excel file upload"
    ),
    params(
        ("bacteria" = String, Path, description = "Bacteria name")
    ),
    responses(
        (status = 200, description = "Proportion calculated successfully", body = UploadResponse),
        (status = 400, description = "Bad Request", body = UploadResponse),
    )
)]
pub async fn upload_file(Path(bacteria): Path<String>, mut multipart: Multipart) -> impl IntoResponse {
    println!("📥 upload_file called with bacteria: {}", bacteria);

    match multipart.next_field().await {
        Ok(Some(field)) => {
            println!("📦 Got a field: {:?}", field.name());
            // 나머지 로직
            (StatusCode::OK, Json(UploadResponse { message: "ok".to_string() }))
        },
        Ok(None) => {
            println!("⚠️ No field found in multipart.");
            (StatusCode::BAD_REQUEST, Json(UploadResponse { message: "No file uploaded".to_string() }))
        },
        Err(err) => {
            println!("❌ Multipart parse error: {:?}", err);
            (StatusCode::BAD_REQUEST, Json(UploadResponse { message: format!("Multipart error: {}", err) }))
        }
    }
}
#[utoipa::path(
    post,
    path = "/test",
    tag = "Proportion",
    params(
        ("param" = String,Query, description = "Bacteria name")
    ),
    responses(
        (status = 200, description = "Proportion calculated successfully", body = UploadResponse),
        (status = 400, description = "Bad Request", body = UploadResponse),
    )
)]
pub async fn test_api(Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {
    let param = params.get("param").cloned().unwrap_or_else(|| "".to_string());
    
    println!("📥 Received bacteria: {}", param);

    (StatusCode::OK, Json(UploadResponse {
        message: format!("Proportion calculation started for {}", param),
    }))
}

// #[utoipa::path(
//     post,
//     path = "/proportion/{bacteria}",
//     tag = "Proportion",
//     params(
//         ("bacteria" = String, Path, description = "Bacteria name")
//     ),
//     request_body(
//         content = UploadForm,
//         description = "Excel file to upload",
//         content_type = "multipart/form-data"
//     ),
//     responses(
//         (status = 200, description = "File uploaded successfully", body = UploadResponse),
//         (status = 400, description = "No file uploaded", body = UploadResponse),
//     )
// )]
// pub async fn upload_file(Path(bacteria): Path<String>, mut multipart: Multipart) -> impl IntoResponse
//  {
//     println!("📥 upload_file called with bacteria: {}", bacteria);

//     match multipart.next_field().await {
//         Ok(Some(field)) => {
//             println!("📦 Got a field: {:?}", field.name());
//             // 나머지 로직
//             (StatusCode::OK, Json(UploadResponse { message: "ok".to_string() }))
//         },
//         Ok(None) => {
//             println!("⚠️ No field found in multipart.");
//             (StatusCode::BAD_REQUEST, Json(UploadResponse { message: "No file uploaded".to_string() }))
//         },
//         Err(err) => {
//             println!("❌ Multipart parse error: {:?}", err);
//             (StatusCode::BAD_REQUEST, Json(UploadResponse { message: format!("Multipart error: {}", err) }))
//         }
//     }
// }
