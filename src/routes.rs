// JSONを返すのに必要
use rocket_contrib::json::Json;

use crate::models::ToDo;

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

#[get("/api")]
pub fn api() -> &'static str {
    "Hello, world!"
}

/// TODOリストを返す。
/// Jsonの型がResponderをimplしているので、JSON文字列を返すことができる
#[get("/todos")]
pub fn todos() -> Json<Vec<ToDo>> {
    Json(vec![ToDo {
        id: 1,
        title: "Read Rocket tutorial".into(),
        description: "Read https://rocket.rs/guide/quickstart/".into(),
        done: false,
    }])
}

/// 新しいTODOを作成する
/// POSTの時はこうする
#[post("/todos", data = "<todo>")]
pub fn new_todo(todo: Json<ToDo>) -> String {
    format!("Accepted post request! {:?}", todo.0)
}

/// TODOを取得する
#[get("/todos/<todoid>")]
pub fn todo_by_id(todoid: u32) -> String {
    let todo = ToDo {
        id: 1,
        title: "Read Rocket tutorial".into(),
        description: "Read https://rocket.rs/guide/quickstart/".into(),
        done: false,
    };
    format!("{:?}", todo)
}