use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct UserLongUrl {
    pub longurl: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DbOutput {
    pub id: i64,
    pub longurl: String,
    pub slug: String,
}
