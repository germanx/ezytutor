use crate::dbaccess::tutor::*;
use crate::errors::EzyTutorError;
use crate::models::tutor::{NewTutor, UpdateTutor};
use crate::state::AppState;
use actix_web::{web, HttpResponse};

pub async fn get_all_tutors(app_state: web::Data<AppState>) 
    -> Result<HttpResponse, EzyTutorError> 
{
    println!(">>> get_all_tutors");

    get_all_tutors_db(&app_state.db)
      .await
      .map(|records| HttpResponse::Ok().json(records))
}

pub async fn get_tutor_details(
    app_state: web::Data<AppState>,
    path: web::Path<i32>,
) -> Result<HttpResponse, EzyTutorError> 
{
    println!(">>> get_tutor_details");

    let tutor_id: i32 = path.into_inner();
    get_tutor_details_db(&app_state.db, tutor_id)
      .await
      .map(|record| HttpResponse::Ok().json(record))
}

pub async fn post_new_tutor(
    new_tutor: web::Json<NewTutor>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, EzyTutorError> 
{
    println!(">>> post_new_tutor");

    post_new_tutor_db(&app_state.db, NewTutor::from(new_tutor))
        .await
        .map(|tutor| HttpResponse::Ok().json(tutor))
}

pub async fn update_tutor_details(
    app_state: web::Data<AppState>,
    path: web::Path<i32>,
    update_record: web::Json<UpdateTutor>,
) -> Result<HttpResponse, EzyTutorError> 
{
    println!(">>> update_tutor_details");

    let tutor_id: i32 = path.into_inner();
    update_tutor_details_db(&app_state.db, tutor_id, UpdateTutor::from(update_record))
      .await
      .map(|record| HttpResponse::Ok().json(record))
}

pub async fn delete_tutor(
    app_state: web::Data<AppState>,
    path: web::Path<i32>,
) -> Result<HttpResponse, EzyTutorError> 
{
    println!(">>> delete_tutor");

    let tutor_id: i32 = path.into_inner();
    delete_tutor_db(&app_state.db, tutor_id)
      .await
      .map(|record| HttpResponse::Ok().json(record))
}
