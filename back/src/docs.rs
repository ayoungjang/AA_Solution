use utoipa::OpenApi;
use crate::routes::excel::upload_file;
use crate::models::{UploadResponse, UploadForm};

#[derive(OpenApi)]
#[openapi(
    paths(
        upload_file,
    ),
    components(
        schemas(UploadResponse, UploadForm)
    ),
    tags(
        (name = "Proportion API", description = "Upload Excel file and calculate proportion")
    )
)]
pub struct ApiDoc;
