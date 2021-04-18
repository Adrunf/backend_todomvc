use crate::works::{Work, Works};
use crate::error_handler::CustomError;
use actix_web::{delete, get, post, put, web, HttpResponse};
use serde_json::json;

#[get("/works")]
async fn find_all() -> Result<HttpResponse, CustomError> {
    let works = Works::find_all()?;
    Ok(HttpResponse::Ok().json(works))
}

#[get("/works/{id}")]
async fn find(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let work = Works::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(work))
}


#[post("/works")]
async fn create(work: web::Json<Work>) -> Result<HttpResponse, CustomError> {
    let work = Works::create(work.into_inner())?;
    Ok(HttpResponse::Ok().json(work))
}

#[put("/works/{id}")]
async fn update(
    id: web::Path<i32>,
    work: web::Json<Work>,
) -> Result<HttpResponse, CustomError> {
    let work = Works::update(id.into_inner(), work.into_inner())?;
    Ok(HttpResponse::Ok().json(work))
}

#[delete("/works/{id}")]
async fn delete(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let deleted_work = Works::delete(id.into_inner())?;
    Ok(HttpResponse::Ok().json(json!({ "deleted": deleted_work })))
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(find_all);
    config.service(find);
    config.service(create);
    config.service(update);
    config.service(delete);
}