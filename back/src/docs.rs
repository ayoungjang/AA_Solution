use axum::{
    extract::Path,
    response::{IntoResponse, Json},
    http::StatusCode,
};
use serde::{Serialize, Deserialize};
use utoipa::{OpenApi, ToSchema};

#[derive(Serialize, Deserialize, ToSchema)]
pub struct UploadResponse {
    pub message: String,
}
#[derive(serde::Deserialize, ToSchema)]
pub struct UploadForm {
    #[schema(format = "binary")]
    file: String,
}

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::routes::excel::upload_file,
        crate::routes::excel::test_api
    ),
    components(
        schemas(UploadForm,UploadResponse)
    ),
    tags(
        (name = "Proportion", description = "Calculate proportion based on bacteria name"),
        (name = "Proportion", description = "Test API for proportion calculation")
    )
)]
pub struct ApiDoc;
