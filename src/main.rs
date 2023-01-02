use lazy_static::lazy_static;
use r2d2_redis::r2d2::{Pool, PooledConnection};
use r2d2_redis::RedisConnectionManager;
use rocket::http::Status;
use rocket::outcome::try_outcome;
use rocket::request::{self, FromRequest, Request};
use rocket::{async_trait, build, get, launch, routes, State};
use std::env::var;

lazy_static! {
    static ref REDIS_CONNECTION_STRING: String =
        var("REDIS_CONNECTION_STRING").unwrap_or("redis://localhost:6379".to_string());
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

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    build().mount("/", routes![index]).manage(redis_pool())
}
