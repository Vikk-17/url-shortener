use serde::{Deserialize, Serialize};
use url::Url;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct UserLongUrl {
    #[schema(example="https://doc.rust-lang.org/book/")]
    pub longurl: String,
}

impl UserLongUrl {
    pub fn is_valid_url(&self) -> bool {
        match Url::parse(&self.longurl) {
            Ok(parsed) => parsed.scheme() == "http" || parsed.scheme() == "https",
            Err(_) => false,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct DbOutput {
    #[schema(example=1)]
    pub id: i64,

    #[schema(example="https://doc.rust-lang.org/book/")]
    pub longurl: String,

    #[schema(example="abd2A")]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_urls() {
        let valid = UserLongUrl {
            longurl: "https://example.com/test".to_string(),
        };
        assert!(valid.is_valid_url());

        let valid_http = UserLongUrl {
            longurl: "http://example.com".to_string(),
        };
        assert!(valid_http.is_valid_url());
    }

    #[test]
    fn test_invalid_urls() {
        let invalid_scheme = UserLongUrl {
            longurl: "ftp://example.com".to_string(),
        };
        assert!(!invalid_scheme.is_valid_url());

        let invalid_format = UserLongUrl {
            longurl: "not-a-url".to_string(),
        };
        assert!(!invalid_format.is_valid_url());

        let empty = UserLongUrl {
            longurl: "".to_string(),
        };
        assert!(!empty.is_valid_url());
    }
}

