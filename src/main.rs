use r2d2_redis::r2d2::Pool;
use r2d2_redis::RedisConnectionManager;
use rocket::{build, get, launch, routes};

fn redis_pool() -> Pool<RedisConnectionManager> {
    let manager = RedisConnectionManager::new("redis://localhost:6379").unwrap();
    Pool::new(manager).unwrap()
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    build().mount("/", routes![index])
}
