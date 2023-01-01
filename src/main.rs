use redis::{Client, Commands, Connection, RedisResult};

fn main() {
    test_redis().unwrap();
}

fn test_redis() -> RedisResult<()> {
    let client = Client::open("redis://127.0.0.1/")?;
    let mut connection: Connection = client.get_connection()?;
    connection.set("foo", "bar")?;
    let value: String = connection.get("foo")?;
    println!("value: {}", value);
    Ok(())
}
