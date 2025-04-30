use axum::{
    extract::{Multipart,Query,Path},
    response::{IntoResponse, Json},
    http::StatusCode,
};

use calamine::{Reader, Xlsx, DataType};
use serde::Serialize;
use std::{collections::HashMap, io::Cursor};
use tokio::io::AsyncReadExt;

use crate::docs::{UploadResponse, YearlySummary, UploadForm};


#[derive(Serialize)]
pub struct SimpleResponse {
    pub message: String,
}

#[utoipa::path(
    post,
    path = "/disk/proportion/{antibiotic}",
    tag = "disk",
    request_body(
        content = UploadForm,
        content_type = "multipart/form-data",
        description = "Form with Excel file upload"
    ),
    params(
        ("antibiotic" = String, Path, description = "antibiotic name")
    ),
    responses(
        (status = 200, description = "Proportion calculated successfully", body = Vec<YearlySummary>),
        (status = 400, description = "Bad Request", body = UploadResponse),
    )
)]
pub async fn proportion(
    Path(antibiotic): Path<String>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, (StatusCode, Json<SimpleResponse>)> {

    let mut bytes = None;
    while let Some(field) = multipart.next_field().await.unwrap() {
        if let Some(name) = field.name() {
            println!("📥 Received field: {}", name);
            if name == "file" {
                bytes = Some(field.bytes().await.unwrap());
            }
        }
    }

    let Some(data) = bytes else {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(SimpleResponse {
                message: "No file uploaded".to_string(),
            }),
        ));
    };

    let mut workbook = Xlsx::new(Cursor::new(data)).unwrap();
    let range = workbook.worksheet_range_at(0).unwrap().unwrap();

    let headers: Vec<String> = range
        .rows()
        .next()
        .unwrap()
        .iter()
        .map(|cell| cell.to_string())
        .collect();

    // Column indices
    let year_idx = headers.iter().position(|h| h.trim() == "Prove_aar").unwrap(); //year
    let rslt_idx = headers.iter().position(|h| h.trim() == antibiotic).unwrap(); //result
    let res_type_idx = headers.iter().position(|h| h.trim() == &format!("{}_ResType", antibiotic)).unwrap(); //type - MIC or zone
    let concl_idx = headers.iter().position(|h| h.trim() == &format!("{}_SIR", antibiotic)).unwrap(); //categorized value - I or R or S
    let mut per_year: HashMap<i32, Vec<(f64, String)>> = HashMap::new();

    // Iterate over the rows and collect data
    // Skip the first row (header) and process the res
    for (i, row) in range.rows().skip(1).enumerate() {
        let res_type = row.get(res_type_idx).and_then(|c| c.get_string()).unwrap_or("");
        if res_type.trim().eq_ignore_ascii_case("MIC") {
            continue;
        }
    
        let year = row.get(year_idx).and_then(|c| match c {
            DataType::Int(i) => Some(*i as i32),
            DataType::Float(f) => Some(*f as i32),
            DataType::String(s) => s.parse::<i32>().ok(),
            _ => None,
        });
        let rslt = row.get(rslt_idx).and_then(|c| match c {
            DataType::Float(f) => Some(*f),
            DataType::Int(i) => Some(*i as f64),
            DataType::String(s) => s.trim().replace(",", ".").parse::<f64>().ok(),
            _ => None,
        });
        
        if let (Some(year), Some(value)) = (year, rslt) {
            let concl = row.get(concl_idx).and_then(|c| c.get_string()).unwrap_or("?").to_string();
            per_year.entry(year).or_default().push((value, concl));
        }
    }
    
    let mut summary = vec![];

    for (year, entries) in per_year.into_iter() {
        let count = entries.len();
        let mean_rslt = entries.iter().map(|(v, _)| *v).sum::<f64>() / count as f64;
        let min_rslt = entries.iter().map(|(v, _)| *v).fold(f64::MAX, f64::min);
        let max_rslt = entries.iter().map(|(v, _)| *v).fold(f64::MIN, f64::max);
        let s_count = entries.iter().filter(|(_, c)| c == "S").count();
        let s_rate = (s_count as f64 / count as f64) * 100.0;
    
        summary.push(YearlySummary {
            year,
            mean_rslt,
            min_rslt,
            max_rslt,
            s_rate,
            count,
        });
    }
    
    summary.sort_by_key(|s| s.year);

    Ok((StatusCode::OK, Json(summary)))

}

