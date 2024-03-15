use serde::{Deserialize, Serialize};
use wither::{bson::oid::ObjectId, Model};

#[derive(Debug, Deserialize, Serialize, Model, Clone)]
pub struct Todo {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub body: String,
    pub completed: bool,
}
