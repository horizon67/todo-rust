use actix_web::{get, post, put, delete, web, HttpResponse, Error};
use diesel::prelude::*;
use crate::{models::{TodoTask, NewTodoTask, UpdateTodoTask}, DbPool};

#[get("/tasks")]
pub async fn list_tasks(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    use crate::schema::todo_tasks::dsl::*;
    
    let mut conn = pool.get().expect("couldn't get db connection from pool");
    let tasks = web::block(move || {
        todo_tasks.load::<TodoTask>(&mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(tasks))
}

#[post("/tasks")]
pub async fn create_task(
    pool: web::Data<DbPool>,
    new_task: web::Json<NewTodoTask>,
) -> Result<HttpResponse, Error> {
    use crate::schema::todo_tasks::dsl::*;

    let mut conn = pool.get().expect("couldn't get db connection from pool");
    let task = web::block(move || {
        diesel::insert_into(todo_tasks)
            .values(&new_task.0)
            .get_result::<TodoTask>(&mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Created().json(task))
}

#[put("/tasks/{id}")]
pub async fn update_task(
    pool: web::Data<DbPool>,
    task_id: web::Path<i32>,
    task_update: web::Json<UpdateTodoTask>,
) -> Result<HttpResponse, Error> {
    use crate::schema::todo_tasks::dsl::*;

    let mut conn = pool.get().expect("couldn't get db connection from pool");
    let task = web::block(move || {
        diesel::update(todo_tasks.find(task_id.into_inner()))
            .set(&task_update.0)
            .get_result::<TodoTask>(&mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(task))
}

#[delete("/tasks/{id}")]
pub async fn delete_task(
    pool: web::Data<DbPool>,
    task_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    use crate::schema::todo_tasks::dsl::*;

    let mut conn = pool.get().expect("couldn't get db connection from pool");
    web::block(move || {
        diesel::delete(todo_tasks.find(task_id.into_inner()))
            .execute(&mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::NoContent().finish())
} 
