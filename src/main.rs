use std::env;
use warp::Filter;

#[tokio::main]
async fn main() {
    if env::var_os("RUST_LOG").is_none() {
        // Setting the logger to info as the default
        env::set_var("RUST_LOG", "todos=info");
    }

    pretty_env_logger::init();

    let db = models::blank_db();
    let db_lite = models::get_db();

    let api = filters::todos(db);
    let flags_api = filters::feature_flag_all_routes(db_lite);

    // match any request and return hello world!
    let routes = api.with(warp::log("todos"));

    let more_routes = routes.or(flags_api);

    warp::serve(more_routes).run(([127, 0, 0, 1], 3030)).await;
}

mod filters {

    use crate::handlers::{create_flag, list_flags, update_flag};

    use super::handlers;
    use super::models::{Db, ListOptions, Todo};
    use super::models::{DbLite, Flag, FlagValue};
    use serde::de::value::Error;
    use warp::Filter;

    /// All the TODOs filters combined.
    pub fn todos(
        db: Db,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        todos_list(db.clone())
            .or(todos_create(db.clone()))
            .or(todos_update(db.clone()))
            .or(todos_delete(db))
    }

    /// All the Feature Flag filters combined.
    pub fn feature_flag_all_routes(
        db: DbLite,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        feature_flag_create(db.clone())
            .or(flags_list(db.clone()))
            .or(flags_update(db.clone()))
    }

    /// GET
    pub fn todos_list(
        db: Db,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("todos")
            .and(warp::get())
            .and(warp::query::<ListOptions>())
            .and(with_db(db))
            .and_then(handlers::list_todos)
    }

    /// GET flags
    pub fn flags_list(
        db: DbLite,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("flags")
            .and(warp::get())
            .and(with_db_lite(db))
            .and_then(handlers::list_flags)
    }

    /// POST
    pub fn todos_create(
        db: Db,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("todos")
            .and(warp::post())
            .and(json_body())
            .and(with_db(db))
            .and_then(handlers::create_todo)
    }

    /// POST Feature Flag
    pub fn feature_flag_create(
        db: DbLite,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("flags")
            .and(warp::post())
            .and(json_flag_body())
            .and(with_db_lite(db))
            .and_then(handlers::create_flag)
    }

    /// PUT
    pub fn todos_update(
        db: Db,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("todos" / u64)
            .and(warp::put())
            .and(json_body())
            .and(with_db(db))
            .and_then(handlers::update_todo)
    }

    pub fn flags_update(
        db: DbLite,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("flags" / u64)
            .and(warp::put())
            .and(json_bool_body())
            .and(with_db_lite(db))
            .and_then(handlers::update_flag)
    }

    /// DELETE
    pub fn todos_delete(
        db: Db,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        // We'll make one of our endpoints admin-only to show how authentication filters are used
        let admin_only = warp::header::exact("authorization", "Bearer admin");

        warp::path!("todos" / u64)
            // It is important to put the auth check _after_ the path filters
            // If we put the auth check before, the request `PUT /todos/invalid-string`
            // would try this filter and reject because the authorization header doesn't match,
            // rather because the param is wrong for that other path
            .and(admin_only)
            .and(warp::delete())
            .and(with_db(db))
            .and_then(handlers::delete_todo)
    }

    fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = std::convert::Infallible> + Clone {
        warp::any().map(move || db.clone())
    }

    fn with_db_lite(
        db: DbLite,
    ) -> impl Filter<Extract = (DbLite,), Error = std::convert::Infallible> + Clone {
        warp::any().map(move || db.clone())
    }

    fn json_body() -> impl Filter<Extract = (Todo,), Error = warp::Rejection> + Clone {
        //When accepting a body, we want a JSON body
        // (and to reject huge payloads)
        warp::body::content_length_limit(1024 * 16).and(warp::body::json())
    }

    fn json_flag_body() -> impl Filter<Extract = (Flag,), Error = warp::Rejection> + Clone {
        //When accepting a body, we want a JSON body
        // (and to reject huge payloads)
        warp::body::content_length_limit(1024 * 16).and(warp::body::json())
    }

    fn json_bool_body() -> impl Filter<Extract = (FlagValue,), Error = warp::Rejection> + Clone {
        warp::body::content_length_limit(1024 * 16).and(warp::body::json())
    }
}

mod handlers {
    use super::models::{Db, ListOptions, Todo};
    use super::models::{DbLite, Flag, FlagValue, FlagWithID};
    use std::convert::Infallible;
    use warp::http::StatusCode;

    use rusqlite::params;

    pub async fn list_flags(db: DbLite) -> Result<impl warp::Reply, Infallible> {
        let conn = db.lock().await;

        let mut stmt = conn.prepare("SELECT id, name, value FROM flags").unwrap();

        let flag_iter = stmt
            .query_map([], |row| {
                let value = match row.get(2).unwrap() {
                    1 => true,
                    _ => false,
                };

                Ok(FlagWithID {
                    id: row.get(0).unwrap(),
                    name: row.get(1).unwrap(),
                    value,
                })
            })
            .unwrap();

        let flags_list: Vec<FlagWithID> = flag_iter.into_iter().map(|item| item.unwrap()).collect();

        Ok(warp::reply::json(&flags_list))
    }

    pub async fn list_todos(opts: ListOptions, db: Db) -> Result<impl warp::Reply, Infallible> {
        // Just return a JSON array of todos, applying the limit and offset.
        let todos = db.lock().await;
        let todos: Vec<Todo> = todos
            .clone()
            .into_iter()
            .skip(opts.offset.unwrap_or(0))
            .take(opts.limit.unwrap_or(std::usize::MAX))
            .collect();

        Ok(warp::reply::json(&todos))
    }

    pub async fn create_flag(new_flag: Flag, db: DbLite) -> Result<impl warp::Reply, Infallible> {
        log::debug!("create_flag: {:?}", new_flag);

        let conn = db.lock().await;
        let result = conn.execute(
            "INSERT INTO flags (name, value) Values (?1, ?2)",
            params![new_flag.name, new_flag.value],
        );

        match result {
            Err(err) => {
                log::debug!("Failed to create_new flag: {:?}", err);
                Ok(StatusCode::BAD_REQUEST)
            }
            Ok(_) => Ok(StatusCode::CREATED),
        }
    }

    pub async fn create_todo(create: Todo, db: Db) -> Result<impl warp::Reply, Infallible> {
        log::debug!("create_todo: {:?}", create);

        let mut vec = db.lock().await;

        for todo in vec.iter() {
            if todo.id == create.id {
                log::debug!("    -> id already exists: {}", create.id);
                // Return a 400 bad request
                return Ok(StatusCode::BAD_REQUEST);
            }
        }

        // No existing Todo with id, so insert and return `201 Created`
        vec.push(create);

        Ok(StatusCode::CREATED)
    }

    pub async fn update_flag(
        id: u64,
        flag_value: FlagValue,
        db: DbLite,
    ) -> Result<impl warp::Reply, Infallible> {
        log::debug!("update_flag: id: {:?}, value {:?}", id, flag_value);

        let conn = db.lock().await;

        let mut stmt = conn
            .prepare("SELECT name, value FROM flags WHERE id = ?")
            .unwrap();
        let exist = stmt.exists(params![id]).unwrap();

        // Not Found early exit
        if exist == false {
            return Ok(StatusCode::NOT_FOUND);
        }

        let value_as_int = match flag_value.value {
            true => 1,
            false => 0,
        };

        let result = conn.execute(
            "UPDATE flags SET value = ? WHERE id = ?",
            params![value_as_int, id],
        );
        match result {
            Ok(_) => Ok(StatusCode::OK),
            Err(_) => {
                log::debug!("Unble to update flag");
                Ok(StatusCode::from_u16(500).unwrap())
            }
        }
    }

    pub async fn update_todo(
        id: u64,
        update: Todo,
        db: Db,
    ) -> Result<impl warp::Reply, Infallible> {
        log::debug!("update_todo: id={}, todo={:?}", id, update);
        let mut vec = db.lock().await;

        // Look for the specified Todo...
        for todo in vec.iter_mut() {
            if todo.id == id {
                *todo = update;
                return Ok(StatusCode::OK);
            }
        }

        log::debug!("     -> todo id not found!");

        //If the for loop didn't return Ok, then the ID doesn't exist...
        Ok(StatusCode::NOT_FOUND)
    }

    pub async fn delete_todo(id: u64, db: Db) -> Result<impl warp::Reply, Infallible> {
        log::debug!("delete_todo: id={}", id);

        let mut vec = db.lock().await;

        let len = vec.len();
        vec.retain(|todo| {
            // Retain all Todos that aren't this id...
            // In other words, remove all that *are* this id...
            todo.id != id
        });

        let deleted = vec.len() != len;

        if deleted {
            // respond with a `204 No Content`, which means successful,
            // yet no body expected...
            Ok(StatusCode::NO_CONTENT)
        } else {
            log::debug!("    -> todo id not found!");
            Ok(StatusCode::NOT_FOUND)
        }
    }
}

mod models {
    use serde::{Deserialize, Serialize};
    use std::sync::Arc;
    use tokio::sync::Mutex;

    use rusqlite::Connection;
    use std::path::Path;

    /// So we don't have to tackle how different dfatabase works, we'll just use
    /// a simple in=memory DB, a vector synchronized by a mutex.
    pub type Db = Arc<Mutex<Vec<Todo>>>;
    pub type DbLite = Arc<Mutex<Connection>>;

    pub fn blank_db() -> Db {
        Arc::new(Mutex::new(Vec::new()))
    }

    pub fn get_db() -> DbLite {
        let path = Path::new("instance").join("flag.db");

        let conn = Connection::open(path).expect("Unable to find the db");

        Arc::new(Mutex::new(conn))
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct Todo {
        pub id: u64,
        pub text: String,
        pub completed: bool,
    }

    // The query parameters for list_todos.
    #[derive(Debug, Deserialize)]
    pub struct ListOptions {
        pub offset: Option<usize>,
        pub limit: Option<usize>,
    }

    #[derive(Debug, Serialize)]
    pub struct FlagWithID {
        pub id: i32,
        pub name: String,
        pub value: bool,
    }

    #[derive(Debug, Deserialize)]
    pub struct Flag {
        pub name: String,
        pub value: bool,
    }

    #[derive(Debug, Deserialize)]
    pub struct FlagValue {
        pub value: bool,
    }
}

#[cfg(test)]
mod tests {
    use warp::http::StatusCode;
    use warp::test::request;

    use super::{
        filters,
        models::{self, Todo},
    };

    #[tokio::test]
    async fn test_post() {
        let db = models::blank_db();
        let api = filters::todos(db);

        let resp = request()
            .method("POST")
            .path("/todos")
            .json(&todo1())
            .reply(&api)
            .await;

        assert_eq!(resp.status(), StatusCode::CREATED)
    }

    #[tokio::test]
    async fn test_post_confict() {
        let db = models::blank_db();

        db.lock().await.push(todo1());
        let api = filters::todos(db);

        let resp = request()
            .method("POST")
            .path("/todos")
            .json(&todo1())
            .reply(&api)
            .await;

        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_put_unknown() {
        let _ = pretty_env_logger::try_init();
        let db = models::blank_db();
        let api = filters::todos(db);

        let resp = request()
            .method("PUT")
            .path("/todos/1")
            .header("authorization", "Bearer admin")
            .json(&todo1())
            .reply(&api)
            .await;

        assert_eq!(resp.status(), StatusCode::NOT_FOUND)
    }

    fn todo1() -> Todo {
        Todo {
            id: 1,
            text: "test 1".to_string(),
            completed: false,
        }
    }
}
