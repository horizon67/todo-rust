use actix_web::{test, web, App};
use actix_http::Request;
use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;
use dotenv::dotenv;
use todo_rust::{
    handlers::{list_tasks, create_task, update_task, delete_task},
    models::{NewTodoTask, UpdateTodoTask},
};
use serde_json::Value;

// テストヘルパー関数
async fn setup_test_app() -> impl actix_web::dev::Service<
    Request,
    Response = actix_web::dev::ServiceResponse,
    Error = actix_web::Error,
> {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    test::init_service(
        App::new()
            .app_data(web::Data::new(pool))
            .service(list_tasks)
            .service(create_task)
            .service(update_task)
            .service(delete_task)
    ).await
}

// レスポンスを処理するヘルパー関数
async fn handle_response<T>(resp: actix_web::dev::ServiceResponse) -> T 
where 
    T: serde::de::DeserializeOwned,
{
    assert!(resp.status().is_success(), 
        "Request failed with status: {}, body: {:?}", 
        resp.status(),
        test::read_body(resp).await
    );
    test::read_body_json(resp).await
}

// タスクを作成するヘルパー関数
async fn create_test_task(
    app: &impl actix_web::dev::Service<Request, Response = actix_web::dev::ServiceResponse, Error = actix_web::Error>,
    content: &str,
) -> Value {
    let task = NewTodoTask {
        content: content.to_string(),
        state: None,
    };

    let req = test::TestRequest::post()
        .uri("/tasks")
        .set_json(&task)
        .to_request();

    let resp = test::call_service(app, req).await;
    handle_response(resp).await
}

#[actix_web::test]
async fn test_create_task() {
    let app = setup_test_app().await;
    let result = create_test_task(&app, "テストタスク").await;
    assert_eq!(result["content"], "テストタスク");
}

#[actix_web::test]
async fn test_list_tasks() {
    let app = setup_test_app().await;
    
    let req = test::TestRequest::get()
        .uri("/tasks")
        .to_request();

    let resp = test::call_service(&app, req).await;
    let result: Vec<Value> = handle_response(resp).await;
    assert!(!result.is_empty());
}

#[actix_web::test]
async fn test_update_task() {
    let app = setup_test_app().await;
    
    // タスクを作成
    let created = create_test_task(&app, "更新するタスク").await;
    let task_id = created["id"].as_i64().unwrap();

    // タスクを更新
    let update = UpdateTodoTask {
        content: Some("更新後のタスク".to_string()),
        state: Some(1),
    };

    let req = test::TestRequest::put()
        .uri(&format!("/tasks/{}", task_id))
        .set_json(&update)
        .to_request();

    let resp = test::call_service(&app, req).await;
    let result: Value = handle_response(resp).await;
    
    assert_eq!(result["content"], "更新後のタスク");
    assert_eq!(result["state"], 1);
}

#[actix_web::test]
async fn test_delete_task() {
    let app = setup_test_app().await;
    
    // タスクを作成
    let created = create_test_task(&app, "削除するタスク").await;
    let task_id = created["id"].as_i64().unwrap();

    let req = test::TestRequest::delete()
        .uri(&format!("/tasks/{}", task_id))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 204);
}
