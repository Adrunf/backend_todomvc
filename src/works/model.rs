// src/user/model.rs
use crate::db;
use crate::error_handler::CustomError;
use crate::schema::works;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

//Modelo de Work
#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "works"]
pub struct Work {
    pub work: String,
    pub active: bool
}

//Modelo de Works
#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "works"]
pub struct Works {
    pub id: i32,
    pub work: String,
    pub active: bool
}

//Funcionalidades en Works
impl Works {
    pub fn find_all() -> Result<Vec<Self>, CustomError> {
        let conn = db::connection()?;
        let works = works::table.load::<Works>(&conn)?;
        Ok(works)
    }

    pub fn find(id: i32) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let work = works::table.filter(works::id.eq(id)).first(&conn)?;
        Ok(work)
    }

    pub fn create(work: Work) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let work = Work::from(work);
        let work = diesel::insert_into(works::table)
            .values(work)
            .get_result(&conn)?;
        Ok(work)
    }

    pub fn update(id: i32, work: Work) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let work = diesel::update(works::table)
            .filter(works::id.eq(id))
            .set(work)
            .get_result(&conn)?;
        Ok(work)
    }

    pub fn delete(id: i32) -> Result<usize, CustomError> {
        let conn = db::connection()?;
        let res = diesel::delete(works::table.filter(works::id.eq(id))).execute(&conn)?;
        Ok(res)
    }
}

//Funcionalidades en Work
impl Work {
    fn from(work: Work) -> Work {
        Work {
            work: work.work,
            active: work.active
        }
    }
}
