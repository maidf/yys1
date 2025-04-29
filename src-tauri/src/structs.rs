use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct ResType {
    id: Option<String>,
    name: String,
 }

#[derive(Serialize, Deserialize, FromRow)]
pub struct Res {
    id: Option<String>,
    tid: String,
    aid: String,
    num: u32,
}


#[derive(Serialize, Deserialize, FromRow)]
pub struct Activity {
    pub id: Option<String>,
    pub name: String,
    pub num: u32,
    pub consume: u32,
}