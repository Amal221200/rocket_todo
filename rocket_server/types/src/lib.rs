use serde::{Deserialize, Serialize};
use wither::{bson::oid::ObjectId, Model};

#[derive(Debug, Deserialize, Serialize, Model, Clone)]
pub struct Todo {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub body: String,
    pub completed: bool,
    #[serde(default = "default_order")]
    pub order: i32,
}

#[derive(Debug, Serialize, Deserialize)]
 pub struct Replacer {
    pub replace_id: String
}

fn default_order()-> i32 {
    0
}