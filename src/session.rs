use redis::{self, Commands, PipelineCommands, FromRedisValue, ToRedisArgs};


pub trait SessionStore {
    fn get<T: FromRedisValue>(&self, key: &str, default: Option<T>) -> Option<T>;
    fn set<T: ToRedisArgs>(&self, key: &str, value: T, ttl: Option<usize>);
    fn del(&self, key: &str);
}

//#[derive(Debug, Clone)]
pub struct RedisStorage {
    client: redis::Client,
}

impl RedisStorage {
    pub fn new(client: redis::Client) -> RedisStorage {
        RedisStorage {
            client: client,
        }
    }
}

impl SessionStore for RedisStorage {
    fn get<T: FromRedisValue>(&self, key: &str, default: Option<T>) -> Option<T> {
        let conn = self.client.get_connection();
        if conn.is_err() {
            return default;
        }
        let conn = conn.unwrap();
        let data = conn.get(key);
        if data.is_err() {
            return default;
        }
        if let Ok(value) = data {
            Some(value)
        } else {
            default
        }
    }

    fn set<T: ToRedisArgs>(&self, key: &str, value: T, ttl: Option<usize>) {
        let conn = self.client.get_connection();
        if conn.is_err() {
            return;
        }
        let conn = conn.unwrap();
        if let Some(seconds) = ttl {
            let _: () = redis::pipe().set_ex(key, value, seconds).ignore().query(&conn).unwrap_or(());
        } else {
            let _: () = redis::pipe().set(key, value).ignore().query(&conn).unwrap_or(());
        }
    }

    fn del(&self, key: &str) {
        let conn = self.client.get_connection();
        if conn.is_err() {
            return;
        }
        let conn = conn.unwrap();
        let _: () = redis::pipe().del(key).ignore().query(&conn).unwrap_or(());
    }
}
