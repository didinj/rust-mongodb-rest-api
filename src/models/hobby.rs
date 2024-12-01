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
pub struct Hobby {
    #[serde(
        rename = "_id",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_object_id",
        deserialize_with = "deserialize_object_id"
    )]
    pub id: Option<ObjectId>,

    #[serde(rename = "hobby_name")]
    pub hobby_name: String,

    #[serde(rename = "hobby_description")]
    pub hobby_description: String,

    #[serde(
        rename = "created_at",
        serialize_with = "serialize_datetime",
        deserialize_with = "deserialize_datetime"
    )]
    pub created_at: chrono::DateTime<Utc>,

    #[serde(
        rename = "person",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_object_id",
        deserialize_with = "deserialize_object_id"
    )]
    pub person: Option<ObjectId>,
}

#[derive(Debug, Deserialize)]
pub struct CreateHobby {
    #[serde(rename = "hobby_name")]
    pub hobby_name: String,

    #[serde(rename = "hobby_description")]
    pub hobby_description: String,

    #[serde(
        rename = "person",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_object_id",
        deserialize_with = "deserialize_object_id"
    )]
    pub person: Option<ObjectId>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListHobby {
    #[serde(rename = "hobby_name")]
    pub hobby_name: String,
}
