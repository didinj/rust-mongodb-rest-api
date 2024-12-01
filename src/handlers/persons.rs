use actix_web::{ web, HttpResponse, Responder };
use chrono::Utc;
use futures::StreamExt;
use mongodb::{ bson::{ doc, oid::ObjectId }, Client };
use serde_json::json;

use crate::models::person::{ Person, CreatePerson, ListPerson };

pub async fn create_person(
    db_client: web::Data<Client>,
    person: web::Json<CreatePerson>
) -> impl Responder {
    let db = db_client.database("rust_mongo");
    let collection = db.collection::<Person>("persons");
    let new_person = Person {
        id: None,
        name: person.name.clone(),
        email: person.email.clone(),
        phone: person.phone.clone(),
        created_at: Utc::now(),
    };

    let result = collection.insert_one(new_person).await;

    match result {
        Ok(insert_result) =>
            HttpResponse::Ok().json(json!({ "inserted_id": insert_result.inserted_id })),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn get_persons(db_client: web::Data<Client>) -> impl Responder {
    let db = db_client.database("rust_mongo");
    let collection = db.collection::<ListPerson>("persons");

    let cursor = collection.find(doc! {}).await;
    match cursor {
        Ok(mut results) => {
            let mut persons: Vec<ListPerson> = vec![];
            while let Some(result) = results.next().await {
                match result {
                    Ok(person) => persons.push(person),
                    Err(_) => {
                        return HttpResponse::InternalServerError().finish();
                    }
                }
            }
            HttpResponse::Ok().json(persons)
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn get_person_by_id(
    db_client: web::Data<Client>,
    id: web::Path<String>
) -> impl Responder {
    let db = db_client.database("rust_mongo");
    let collection = db.collection::<Person>("persons");
    let object_id = match ObjectId::parse_str(&id.into_inner()) {
        Ok(oid) => oid,
        Err(_) => {
            return HttpResponse::BadRequest().body("Invalid ID format");
        }
    };

    match collection.find_one(doc! { "_id": object_id }).await {
        Ok(Some(person)) => HttpResponse::Ok().json(person),
        Ok(None) => HttpResponse::NotFound().body("Person not found"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn update_person_by_id(
    db_client: web::Data<Client>,
    id: web::Path<String>,
    person: web::Json<CreatePerson>
) -> impl Responder {
    let db = db_client.database("rust_mongo");
    let collection = db.collection::<Person>("persons");
    let object_id = match ObjectId::parse_str(&id.into_inner()) {
        Ok(oid) => oid,
        Err(_) => {
            return HttpResponse::BadRequest().body("Invalid ID format");
        }
    };

    let update_doc =
        doc! {
        "$set": {
            "name": &person.name,
            "email": &person.email,
            "phone": &person.phone,
        }
    };

    match collection.update_one(doc! { "_id": object_id }, update_doc).await {
        Ok(update_result) if update_result.matched_count > 0 =>
            HttpResponse::Ok().body("Person updated"),
        Ok(_) => HttpResponse::NotFound().body("Person not found"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn delete_person_by_id(
    db_client: web::Data<Client>,
    id: web::Path<String>
) -> impl Responder {
    let db = db_client.database("rust_mongo");
    let collection = db.collection::<Person>("persons");
    let object_id = match ObjectId::parse_str(&id.into_inner()) {
        Ok(oid) => oid,
        Err(_) => {
            return HttpResponse::BadRequest().body("Invalid ID format");
        }
    };

    match collection.delete_one(doc! { "_id": object_id }).await {
        Ok(delete_result) if delete_result.deleted_count > 0 =>
            HttpResponse::Ok().body("Person deleted"),
        Ok(_) => HttpResponse::NotFound().body("Person not found"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
