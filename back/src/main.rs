use axum::{
    extract::{Multipart, Path},
    response::{IntoResponse, Json},
    routing::post,
    Router,
    http::StatusCode,
};
use hyper::Server;
use std::net::SocketAddr;
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;
use serde::{Serialize, Deserialize};
use calamine::{Reader, Xlsx, DataType};
use std::collections::HashMap;
use std::io::Cursor;

#[derive(Serialize, ToSchema)]
pub struct UploadResponse {
    filename: String,
    message: String,
}

#[derive(Deserialize, ToSchema)]
pub struct UploadForm {
    #[schema(format = "binary")]
    file: String
}

#[utoipa::path(
    post,
    path = "/proportion/{bacteria}",
    params(
        ("bacteria" = String, Path, description = "Name of the bacteria")
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
    println!("Received bacteria: {}", bacteria);

    if let Some(field) = multipart.next_field().await.unwrap() {
        println!("Received a file field: {:?}", field.name());

        let data = field.bytes().await.unwrap();
        let cursor = Cursor::new(&data);
        let mut workbook = Xlsx::new(cursor).unwrap();
        let range = workbook.worksheet_range_at(0).unwrap().unwrap();

        let mut year_sum: HashMap<i32, (f64, usize)> = HashMap::new();

        for row in range.rows().skip(1) {
            let year = match row.get(0) {
                Some(DataType::Int(val)) => *val as i32,
                Some(DataType::Float(val)) => *val as i32,
                Some(DataType::String(s)) => s.parse::<i32>().unwrap_or(0),
                _ => continue,
            };

            let value = match row.get(1) {
                Some(DataType::Float(val)) => *val,
                Some(DataType::Int(val)) => *val as f64,
                Some(DataType::String(s)) => s.parse::<f64>().unwrap_or(0.0),
                _ => continue,
            };

            let entry = year_sum.entry(year).or_insert((0.0, 0));
            entry.0 += value;
            entry.1 += 1;
        }

        println!("📊 Proportion Result: {:?}", year_sum);

        return (StatusCode::OK, Json(UploadResponse {
            filename: "".to_string(),
            message: format!("Proportion calculated for bacteria: {}", bacteria),
        }));
    }

    (StatusCode::BAD_REQUEST, Json(UploadResponse {
        filename: "".to_string(),
        message: "No file uploaded".to_string(),
    }))
}

#[derive(OpenApi)]
#[openapi(
    paths(
        upload_file
    ),
    components(
        schemas(UploadResponse, UploadForm)
    ),
    tags(
        (name = "Proportion API", description = "Upload Excel and Calculate proportion")
    )
)]
pub struct ApiDoc;

#[tokio::main]
async fn main() {
    println!("🚀 Starting server...");
    let app = Router::new()
        .route("/proportion/{bacteria}", post(upload_file))
        .merge(SwaggerUi::new("/api/docs").url("/api-docs/openapi.json", ApiDoc::openapi()));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("🚀 Server running at http://{}", addr);
    println!("📄 Swagger API Docs at http://{}/docs", addr);

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
