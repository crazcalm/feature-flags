use std::env;
use warp::Filter;

use feature_flags::db::get_db_server;

#[tokio::main]
async fn main() {
    if env::var_os("RUST_LOG").is_none() {
        // Setting the logger to info as the default
        env::set_var("RUST_LOG", "todos=info");
    }

    pretty_env_logger::init();

    let db_lite = get_db_server();

    let flags_api = filters::feature_flag_all_routes(db_lite);

    // match any request and return hello world!
    let routes = flags_api.with(warp::log("flags"));

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

mod filters {
    use super::handlers;
    use warp::Filter;

    use feature_flags::db::{DBLite, Flag, FlagValue};

    /// All the Feature Flag filters combined.
    pub fn feature_flag_all_routes(
        db: DBLite,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        feature_flag_create(db.clone())
            .or(flags_list(db.clone()))
            .or(flags_update(db.clone()))
            .or(flags_delete(db))
    }

    /// GET flags
    pub fn flags_list(
        db: DBLite,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("flags")
            .and(warp::get())
            .and(with_db_lite(db))
            .and_then(handlers::list_flags)
    }

    /// POST Feature Flag
    pub fn feature_flag_create(
        db: DBLite,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("flags")
            .and(warp::post())
            .and(json_flag_body())
            .and(with_db_lite(db))
            .and_then(handlers::create_flag)
    }

    pub fn flags_update(
        db: DBLite,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("flags" / u64)
            .and(warp::put())
            .and(json_bool_body())
            .and(with_db_lite(db))
            .and_then(handlers::update_flag)
    }

    /// DELETE
    pub fn flags_delete(
        db: DBLite,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("flags" / u64)
            .and(warp::delete())
            .and(with_db_lite(db))
            .and_then(handlers::delete_flag)
    }

    fn with_db_lite(
        db: DBLite,
    ) -> impl Filter<Extract = (DBLite,), Error = std::convert::Infallible> + Clone {
        warp::any().map(move || db.clone())
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
    use feature_flags::db::{DBLite, Flag, FlagValue, FlagWithID};
    use std::convert::Infallible;
    use warp::http::StatusCode;

    use rusqlite::params;

    pub async fn list_flags(db: DBLite) -> Result<impl warp::Reply, Infallible> {
        let conn = db.lock().await;

        let mut stmt = conn.prepare("SELECT id, name, value FROM flags").unwrap();

        let flag_iter = stmt
            .query_map([], |row| {
                let value = matches!(row.get(2).unwrap(), 1);

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

    pub async fn create_flag(new_flag: Flag, db: DBLite) -> Result<impl warp::Reply, Infallible> {
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

    pub async fn update_flag(
        id: u64,
        flag_value: FlagValue,
        db: DBLite,
    ) -> Result<impl warp::Reply, Infallible> {
        log::debug!("update_flag: id: {:?}, value {:?}", id, flag_value);

        let conn = db.lock().await;

        let mut stmt = conn
            .prepare("SELECT name, value FROM flags WHERE id = ?")
            .unwrap();
        let exist = stmt.exists(params![id]).unwrap();

        // Not Found early exit
        if !exist {
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

    pub async fn delete_flag(id: u64, db: DBLite) -> Result<impl warp::Reply, Infallible> {
        log::debug!("delete flag id <{}>", id);

        let conn = db.lock().await;
        let result = conn.execute("DELETE FROM flags WHERE id = ?", params![id]);

        match result {
            Ok(_) => Ok(StatusCode::NO_CONTENT),
            Err(err) => {
                log::debug!("Error when deleting a flag: {:?}", err);
                Ok(StatusCode::from_u16(500).unwrap())
            }
        }
    }
}

#[cfg(test)]
mod tests {}
