use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct UserLongUrl {
    #[schema(example="https://doc.rust-lang.org/book/")]
    pub longurl: String,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct DbOutput {
    #[schema(example=1)]
    pub id: i64,

    #[schema(example="abd2A")]
    pub longurl: String,

    #[schema(example="https://doc.rust-lang.org/book/")]
    pub slug: String,
}

#[derive(Serialize, Debug, ToSchema)]
pub struct ShortenResponse {
    /// Response Message
    #[schema(example="Data Inserted")]
    pub message: String,

    /// URL Data
    pub output: DbOutput,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ErrorResponse {
    #[schema(example="URL not Found")]
    pub error: String,
}
