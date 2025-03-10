use actix_web::{ web, App, HttServer,Responder, HttpResponse};
use actix_cors :: Cors;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::async:: Mutex;
use chrono::{Utc, DateTime};

#[derive(Serialize, Deserialize,Clone)]
struct TodoItem{
    id: Uuid,
    title: String,
    completed: bool,
    created_at: DateTime<Utc>
}

#[derive( Deserialize)]
struct CreateTodoItem{
    title: String,
    completed: bool,

}

#[derive(Deserialize)]
struct UpdateTodoItem{
    title: Option<String>,
    completed: Option<bool>,
}

struct AppState{
    todo_list: Mutex<Vec<TodoItem>>,
}

async fn get_todos(data: web::Data<AppState>) -> impl Responder{
    let todo_list = data.todo_list.lock().unwrap();
    HttpResponse::Ok().json(&*todo_list)
}

async fn create_todo(
    data: web::Data<AppState>,
    params: web::Json<CreateTodoItem>,
) -> impl Responder{
    let mut todo_list = data.todo_list.lock().unwrap();
    let todo_item = TodoItem{
        id: Uuid::new_v4(),
        title: params.title.clone(),
        completed: params.completed,
        created_at: Utc::now(),
    };
    todo_list.push(todo_item);
    HttpResponse::Ok().json(&*todo_list);

}

async fn update_todo(
    path: web::Path<Uuid>,
    item: web::Json<UpdateTodoItem>,
    data: web::Data<AppState>
)-> impl Responder{
    let mut todos = data.todo_list.lock().unwrap();

    if let Some(todo)=tdoos.iter_mut().find(|todo| todo.id==*path){
        if let Some(title)=&item.title{
            todo.title = title.clone();

        }
        if let Some(completed)=item.completed{
            todo.completed =completed;
        }
        HttpResponse::ok().json(&*todos);
    }
    else{
        HttpResponse::NotFound().body(
            "Todo not found"
        )
    }
}


async fn delete_todo(
    path: web::Path<Uuid>,
    data: web::Data<AppState>
)-> impl Responder{
    let mut todos = data.todo_list.lock().unwrap();
    if todos.iter().any(|todo| todo.id == *path){
        todos.retain(|todo| todo.id != *path);
        HttpResponse::Ok().json(&*todos);
    }
    else{
        HttpResponse::NotFound().body(
            "Todo not found"
        )
    }
}


#[actix_web::main]
async fn main()-> std::io::Result<()>{
    let app_state= web::Data::new(AppState{
        todo_list: Mutex::new(Vec::new()),
    });


     HttpServer::new(move ||{
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);
        App::new()
            .app_data(app_state.clone())
            .wrap(cors)
            .route("/todos", web::get().to(get_todos))
            .route("/todos", web::post().to(create_todo))
            .route("/todos/{id}", web::put().to(update_todo))
            .route("/todos/{id}", web::delete().to(delete_todo))
     })
     .bind("127.0.0.1:8080")?
     .run()
     .await
     
    


}








