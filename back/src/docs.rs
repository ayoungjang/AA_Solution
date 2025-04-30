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


#[derive(Serialize, ToSchema)]
pub struct YearlySummary {
    pub year: i32,
    pub mean_rslt: f64,
    pub min_rslt: f64,
    pub max_rslt: f64,
    pub s_rate: f64,
    pub count: usize,
}


#[derive(OpenApi)]
#[openapi(
    paths(
        crate::routes::excel::proportion
    ),
    components(
        schemas(YearlySummary,UploadForm,UploadResponse)
    ),
    tags(
        (name = "disk", description = "Calculate proportion based on bacteria name"),
    )
)]
pub struct ApiDoc;
