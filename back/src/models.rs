use serde::{Serialize, Deserialize};
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct UploadResponse {
    pub filename: String,
    pub message: String,
}

#[derive(Deserialize, ToSchema)]
pub struct UploadForm {
    #[schema(format = "binary")]
    pub file: String,
}
