use actix_web::{ web, HttpResponse, Responder };
use chrono::Utc;
use futures::StreamExt;
use mongodb::{ bson::{ doc, oid::ObjectId }, Client };
use serde_json::json;

use crate::models::hobby::{ Hobby, CreateHobby, ListHobby };

pub async fn create_hobby(
    db_client: web::Data<Client>,
    hobby: web::Json<CreateHobby>
) -> impl Responder {
    let db = db_client.database("rust_mongo");
    let collection = db.collection::<Hobby>("hobbies");
    let new_hobby = Hobby {
        id: None,
        hobby_name: hobby.hobby_name.clone(),
        hobby_description: hobby.hobby_description.clone(),
        created_at: Utc::now(),
        person: hobby.person,
    };

    let result = collection.insert_one(new_hobby).await;

    match result {
        Ok(insert_result) =>
            HttpResponse::Ok().json(json!({ "inserted_id": insert_result.inserted_id })),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn get_hobbies(db_client: web::Data<Client>) -> impl Responder {
    let db = db_client.database("rust_mongo");
    let collection = db.collection::<ListHobby>("hobbies");

    let cursor = collection.find(doc! {}).await;
    match cursor {
        Ok(mut results) => {
            let mut hobbies: Vec<ListHobby> = vec![];
            while let Some(result) = results.next().await {
                match result {
                    Ok(hobby) => hobbies.push(hobby),
                    Err(_) => {
                        return HttpResponse::InternalServerError().finish();
                    }
                }
            }
            HttpResponse::Ok().json(hobbies)
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn get_hobby_by_id(
    db_client: web::Data<Client>,
    id: web::Path<String>
) -> impl Responder {
    let db = db_client.database("rust_mongo");
    let collection = db.collection::<Hobby>("hobbies");
    let object_id = match ObjectId::parse_str(&id.into_inner()) {
        Ok(oid) => oid,
        Err(_) => {
            return HttpResponse::BadRequest().body("Invalid ID format");
        }
    };

    match collection.find_one(doc! { "_id": object_id }).await {
        Ok(Some(hobby)) => HttpResponse::Ok().json(hobby),
        Ok(None) => HttpResponse::NotFound().body("Hobby not found"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn update_hobby_by_id(
    db_client: web::Data<Client>,
    id: web::Path<String>,
    hobby: web::Json<CreateHobby>
) -> impl Responder {
    let db = db_client.database("rust_mongo");
    let collection = db.collection::<Hobby>("hobbies");
    let object_id = match ObjectId::parse_str(&id.into_inner()) {
        Ok(oid) => oid,
        Err(_) => {
            return HttpResponse::BadRequest().body("Invalid ID format");
        }
    };

    let update_doc =
        doc! {
        "$set": {
            "hobby_name": &hobby.hobby_name,
            "hobby_description": &hobby.hobby_description,
        }
    };

    match collection.update_one(doc! { "_id": object_id }, update_doc).await {
        Ok(update_result) if update_result.matched_count > 0 =>
            HttpResponse::Ok().body("Hobby updated"),
        Ok(_) => HttpResponse::NotFound().body("Hobby not found"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn delete_hobby_by_id(
    db_client: web::Data<Client>,
    id: web::Path<String>
) -> impl Responder {
    let db = db_client.database("rust_mongo");
    let collection = db.collection::<Hobby>("hobbies");
    let object_id = match ObjectId::parse_str(&id.into_inner()) {
        Ok(oid) => oid,
        Err(_) => {
            return HttpResponse::BadRequest().body("Invalid ID format");
        }
    };

    match collection.delete_one(doc! { "_id": object_id }).await {
        Ok(delete_result) if delete_result.deleted_count > 0 =>
            HttpResponse::Ok().body("Hobby deleted"),
        Ok(_) => HttpResponse::NotFound().body("Hobby not found"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
