use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, ToSchema)]
pub struct UploadResponse {
    pub message : String,
}
#[derive(serde::Deserialize, ToSchema)]
pub struct UploadForm {
    #[schema(format = "binary")]
    pub file: String
}
// OpenAPI docs
#[derive(OpenApi)]
#[openapi(
    paths(crate::routes::excel::upload_file),  
    components(schemas(UploadResponse, UploadForm)),
    tags(
        (name = "Proportion API", description = "Upload Excel and Calculate proportion")
    )
)]



pub struct ApiDoc;
