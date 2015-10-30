use redis::{self, Commands, PipelineCommands, FromRedisValue, ToRedisArgs};


pub trait SessionStore: Clone {
    fn get<K: AsRef<str>, T: FromRedisValue>(&self, key: K, default: Option<T>) -> Option<T>;
    fn set<K: AsRef<str>, T: ToRedisArgs>(&self, key: K, value: T, ttl: Option<usize>);
    fn del<K: AsRef<str>>(&self, key: K);
}

#[derive(Debug, Clone)]
pub struct RedisStorage {
    client: redis::Client,
}

impl RedisStorage {
    pub fn new(client: redis::Client) -> RedisStorage {
        RedisStorage {
            client: client,
        }
    }

    pub fn from_url<U: AsRef<str>>(url: U) -> RedisStorage {
        let client = redis::Client::open(url.as_ref()).unwrap();
        RedisStorage {
            client: client,
        }
    }
}

impl SessionStore for RedisStorage {
    fn get<K: AsRef<str>, T: FromRedisValue>(&self, key: K, default: Option<T>) -> Option<T> {
        let conn = self.client.get_connection();
        if conn.is_err() {
            return default;
        }
        let conn = conn.unwrap();
        let data = conn.get(key.as_ref());
        if data.is_err() {
            return default;
        }
        if let Ok(value) = data {
            Some(value)
        } else {
            default
        }
    }

    fn set<K: AsRef<str>, T: ToRedisArgs>(&self, key: K, value: T, ttl: Option<usize>) {
        let key = key.as_ref();
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

    fn del<K: AsRef<str>>(&self, key: K) {
        let conn = self.client.get_connection();
        if conn.is_err() {
            return;
        }
        let conn = conn.unwrap();
        let _: () = redis::pipe().del(key.as_ref()).ignore().query(&conn).unwrap_or(());
    }
}
