use lazy_static::lazy_static;
use r2d2_redis::r2d2::Pool;
use r2d2_redis::RedisConnectionManager;
use rocket::{build, get, launch, routes};
use std::env::var;

lazy_static! {
    static ref REDIS_CONNECTION_STRING: String =
        var("REDIS_CONNECTION_STRING").unwrap_or("redis://localhost:6379".to_string());
}

fn redis_pool() -> Pool<RedisConnectionManager> {
    let manager = RedisConnectionManager::new(REDIS_CONNECTION_STRING.as_str()).unwrap();
    Pool::new(manager).unwrap()
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    build().mount("/", routes![index]).manage(redis_pool())
}
