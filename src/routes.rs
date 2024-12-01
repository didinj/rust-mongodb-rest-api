use actix_web::web;

use crate::handlers::{
    hobbies::{ create_hobby, delete_hobby_by_id, get_hobbies, get_hobby_by_id, update_hobby_by_id },
    persons::{
        create_person,
        delete_person_by_id,
        get_person_by_id,
        get_persons,
        update_person_by_id,
    },
};

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web
            ::scope("/api/v1")
            .route("/persons", web::post().to(create_person))
            .route("/persons", web::get().to(get_persons))
            .route("/persons/{id}", web::get().to(get_person_by_id))
            .route("/persons/{id}", web::put().to(update_person_by_id))
            .route("/persons/{id}", web::delete().to(delete_person_by_id))
            .route("/hobbies", web::post().to(create_hobby))
            .route("/hobbies", web::get().to(get_hobbies))
            .route("/hobbies/{id}", web::get().to(get_hobby_by_id))
            .route("/hobbies/{id}", web::put().to(update_hobby_by_id))
            .route("/hobbies/{id}", web::delete().to(delete_hobby_by_id))
    );
}
