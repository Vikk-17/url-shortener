use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct UserLongUrl {
    pub longurl: String,
}

