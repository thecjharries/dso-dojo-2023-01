use lazy_static::lazy_static;
use r2d2_redis::r2d2::{Pool, PooledConnection};
use r2d2_redis::redis::Commands;
use r2d2_redis::RedisConnectionManager;
use reqwest;
use rocket::http::Status;
use rocket::outcome::try_outcome;
use rocket::request::{self, FromRequest, Request};
use rocket::serde::json::Json;
use rocket::{async_trait, build, get, launch, routes, State};
use serde::{Deserialize, Serialize};
use std::env::var;
use std::ops::{Deref, DerefMut};

lazy_static! {
    static ref REDIS_CONNECTION_STRING: String =
        var("REDIS_CONNECTION_STRING").unwrap_or("redis://localhost:6379".to_string());
    static ref API_ROOT: String = var("API_ROOT").unwrap_or("http://localhost:8001".to_string());
}

fn redis_pool() -> Pool<RedisConnectionManager> {
    let manager = RedisConnectionManager::new(REDIS_CONNECTION_STRING.as_str()).unwrap();
    Pool::new(manager).unwrap()
}

struct RedisConnection(PooledConnection<RedisConnectionManager>);
type RedisConnectionPool = Pool<RedisConnectionManager>;

#[async_trait]
impl<'r> FromRequest<'r> for RedisConnection {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let pool = try_outcome!(request.guard::<&State<RedisConnectionPool>>().await);
        match pool.get() {
            Ok(conn) => request::Outcome::Success(RedisConnection(conn)),
            Err(_) => request::Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

impl Deref for RedisConnection {
    type Target = PooledConnection<RedisConnectionManager>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for RedisConnection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct ApiResponse {
    id: u64,
    token: String,
}

#[get("/api/<id>")]
async fn api(mut connection: RedisConnection, id: u64) -> Json<ApiResponse> {
    match connection.get(id.to_string()) {
        Ok(token) => Json(ApiResponse { id, token }),
        Err(_) => {
            let response: ApiResponse = reqwest::get(format!("{}/api/{}", API_ROOT.as_str(), id))
                .await
                .unwrap()
                .json::<ApiResponse>()
                .await
                .unwrap();
            let _: () = connection
                .set(id.to_string(), response.token.clone())
                .unwrap();
            Json(response.clone())
        }
    }
}

#[launch]
fn rocket() -> _ {
    build().mount("/", routes![api]).manage(redis_pool())
}

#[cfg(test)]
mod tests {
    use super::*;
    use r2d2_redis::redis;
    use r2d2_redis::redis::Commands;
    use rocket::local::blocking::{Client, LocalResponse};
    use std::time::{Duration, Instant};

    #[test]
    fn test_redis_connection() {
        let pool = redis_pool();
        let mut conn = pool.get().unwrap();
        let _: () = redis::cmd("PING").query(conn.deref_mut()).unwrap();
    }

    #[test]
    fn test_api() {
        let pool = redis_pool();
        let mut conn = pool.get().unwrap();
        match conn.del::<String, ()>(10.to_string()) {
            Ok(_) => (),
            Err(_) => (),
        };
        assert!(conn.get::<String, String>(10.to_string()).is_err());
        let client = Client::tracked(rocket()).unwrap();
        let slow_start = Instant::now();
        let slow_response: LocalResponse = client.get("/api/10").dispatch();
        let slow_end: Duration = slow_start.elapsed();
        assert_eq!(slow_response.status(), Status::Ok);
        assert!(slow_end.as_millis() > 5000);
        assert_eq!(
            slow_response.into_json::<ApiResponse>().unwrap(),
            ApiResponse {
                id: 10,
                token: "hjupifwjnzholhbcehxlmdgaayihhjfbsnkmaecvmumzcmyfqueruzayamxhpflo"
                    .to_string()
            }
        );
        assert_eq!(
            conn.get::<String, String>(10.to_string()).unwrap(),
            "hjupifwjnzholhbcehxlmdgaayihhjfbsnkmaecvmumzcmyfqueruzayamxhpflo".to_string()
        );
        let fast_start = Instant::now();
        let timed_response: LocalResponse = client.get("/api/10").dispatch();
        let end: Duration = fast_start.elapsed();
        assert_eq!(timed_response.status(), Status::Ok);
        assert!(end.as_millis() < 100);
    }
}
