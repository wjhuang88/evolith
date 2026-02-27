//! Skill routes

use crate::handlers::skill_handlers::*;
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.app_data(web::Data::new(SkillState::new()))
        .route("/skills", web::get().to(list_skills))
        .route("/skills", web::post().to(create_skill))
        .route("/skills/{id}", web::get().to(get_skill))
        .route("/skills/{id}", web::put().to(update_skill))
        .route("/skills/{id}", web::delete().to(delete_skill))
        .route("/skills/{id}/load", web::get().to(load_skill))
        .route("/skills/{id}/execute", web::post().to(execute_skill));
}
