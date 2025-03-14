use axum::{
    routing::post,
    Router,
    extract::Multipart,
    http::StatusCode,
    response::{IntoResponse, Json}
};
use calamine::{open_workbook_auto, Reader, RangeDeserializerBuilder};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use std::path::Path;
use serde::{Serialize, Deserialize};
use futures::stream::StreamExt;

#[tokio::main]
async fn main() {
    // Create the app
    let app = Router::new()
        .route("/api/upload", post(upload_file)); // POST route for file upload

    // Run the server on port 8080
    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// Define a structure to handle the response
#[derive(Serialize, Deserialize)]
struct UploadResponse {
    filename: String,
    message: String,
}

// Handle file upload
async fn upload_file(mut multipart: Multipart) -> impl IntoResponse {
    // Extract file data from multipart form
    while let Some(field) = multipart.next_field().await.unwrap() {
        let content_disposition = field.content_disposition().unwrap();
        let filename = content_disposition.get_filename().unwrap_or("file");

        // Read file into memory
        let bytes = field.bytes().await.unwrap();

        // Optionally, save the file to disk
        let filepath = Path::new("uploads").join(filename);
        let mut file = File::create(&filepath).await.unwrap();
        file.write_all(&bytes).await.unwrap();

        // Process the Excel file (if necessary)
        if let Err(e) = process_excel_file(&bytes).await {
            return (StatusCode::INTERNAL_SERVER_ERROR, format!("Error processing file: {}", e)).into_response();
        }

        // Return success response
        let response = UploadResponse {
            filename: filename.to_string(),
            message: "File uploaded and processed successfully".to_string(),
        };

        return (StatusCode::OK, Json(response)).into_response();
    }

    (StatusCode::BAD_REQUEST, "No file uploaded").into_response()
}

// Function to process the Excel file using calamine
async fn process_excel_file(file_bytes: &[u8]) -> Result<(), String> {
    // Open the Excel file from memory
    let mut workbook = open_workbook_auto(file_bytes).map_err(|e| e.to_string())?;

    // Get the first sheet
    if let Some(Ok(range)) = workbook.worksheet_range_at(0) {
        // Read the first sheet's first row
        let mut iter = RangeDeserializerBuilder::new().from_range(&range).unwrap();
        if let Some(Ok(record)) = iter.next() {
            // Example: log the first value of the first row (or do more complex parsing)
            if let Some(first_value) = record.get(0) {
                println!("First cell value: {}", first_value);
            }
        }
    } else {
        return Err("Failed to read the first sheet".to_string());
    }

    Ok(())
}
