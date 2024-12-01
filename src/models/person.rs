use mongodb::bson::oid::ObjectId;
use serde::{ Deserialize, Serialize };
use chrono::Utc;
use crate::models::utils::{
    deserialize_datetime,
    deserialize_object_id,
    serialize_datetime,
    serialize_object_id,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Person {
    #[serde(
        rename = "_id",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_object_id",
        deserialize_with = "deserialize_object_id"
    )]
    pub id: Option<ObjectId>,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "email")]
    pub email: String,

    #[serde(rename = "phone")]
    pub phone: String,

    #[serde(
        rename = "created_at",
        serialize_with = "serialize_datetime",
        deserialize_with = "deserialize_datetime"
    )]
    pub created_at: chrono::DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreatePerson {
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "email")]
    pub email: String,

    #[serde(rename = "phone")]
    pub phone: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListPerson {
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "email")]
    pub email: String,
}
