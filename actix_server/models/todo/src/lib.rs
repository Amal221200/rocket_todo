use serde::{Deserialize, Serialize};
use validator::Validate;

use wither::{bson::oid::ObjectId, Model};

#[derive(Debug, Deserialize, Serialize, Model, Clone, Validate)]
pub struct Todo {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    #[validate(length(min = 1))]
    pub body: String,
    pub completed: bool,
    // #[serde(default = "default_order")]
    // pub order: i32,
}
