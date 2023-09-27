extern crate redis;

use self::redis::{ConnectionInfo, ConnectionLike, IntoConnectionInfo, RedisError};
use crate::counter::Counter;
use crate::limit::Limit;
use crate::storage::{Authorization, CounterStorage, StorageErr};
use r2d2::{ManageConnection, Pool};
use std::collections::HashSet;
use std::time::Duration;


const DEFAULT_REDIS_URL: &str = "redis://127.0.0.1:6379";
const MAX_REDIS_CONNS: u32 = 20; // TODO: make it configurable

pub struct RedisHiLoStorage {
    conn_pool: Pool<RedisConnectionManager>,
    // naive_counter (our batch)
    // naive_counter = 0
}



impl CounterStorage for RedisHiLoStorage {

    fn is_within_limits(&self, counter: &Counter, delta: i64) -> Result<bool, StorageErr> {
        //
        // Used by:
        //
        Ok(true)
    }

    fn add_counter(&self, _limit: &Limit) -> Result<(), StorageErr> {
        //
        // Used by:
        //
        Ok(())
    }

    fn update_counter(&self, counter: &Counter, delta: i64) -> Result<(), StorageErr> {
        //
        // Used by:
        //
        Ok(())  
    }

    fn check_and_update(
        &self,
        counters: &mut Vec<Counter>,
        delta: i64,
        load_counters: bool,
    ) -> Result<Authorization, StorageErr> {
        // 
        // Used by:
        //

        // check counters etc
        // if naive_counter > 0
        // naive_counter--
        // return authorized
        // else - request batch
        // update naive_counter = 10
        // update counter in redis = counter - 10
        // ^^ hi/lo algorithm  
        Ok(Authorization::Ok)
    }

    fn get_counters(&self, limits: &HashSet<Limit>) -> Result<HashSet<Counter>, StorageErr> {
        // Returns all of the counters as a hashset
        // Used by:
        // 
        let mut res = HashSet::new();
        Ok(res) 
    }

    fn delete_counters(&self, limits: HashSet<Limit>) -> Result<(), StorageErr> {
        // Iterate through each counter in turn and delete from Redis
        // Used by:
        //
        Ok(())  
    }

    fn clear(&self) -> Result<(), StorageErr> {
        // Call FLUSHDB
        // Delete all the keys of the currently selected DB. This command never fails.
        // Used by:
        //
        Ok(())
    }
}   

impl RedisHiLoStorage {
    pub fn new(redis_url: &str) -> Result<Self, String> {
        let conn_manager = match RedisConnectionManager::new(redis_url) {
            Ok(conn_manager) => conn_manager,
            Err(err) => {
                return Err(err.to_string());
            }
        };
        match Pool::builder()
            .connection_timeout(Duration::from_secs(3))
            .max_size(MAX_REDIS_CONNS)
            .build(conn_manager)
        {
            Ok(conn_pool) => Ok(Self { conn_pool }),
            Err(err) => Err(err.to_string()),
        }
    }
}

#[derive(Debug)]
pub struct RedisConnectionManager {
    connection_info: ConnectionInfo,
}

impl RedisConnectionManager {
    pub fn new<T: IntoConnectionInfo>(params: T) -> Result<Self, RedisError> {
        Ok(Self {
            connection_info: params.into_connection_info()?,
        })
    }
}

impl ManageConnection for RedisConnectionManager {
    type Connection = redis::Connection;
    type Error = RedisError;

    fn connect(&self) -> Result<Self::Connection, Self::Error> {
        match redis::Client::open(self.connection_info.clone()) {
            Ok(client) => client.get_connection(),
            Err(err) => Err(err),
        }
    }

    fn is_valid(&self, conn: &mut Self::Connection) -> Result<(), Self::Error> {
        redis::cmd("PING").query(conn)
    }

    fn has_broken(&self, conn: &mut Self::Connection) -> bool {
        !conn.is_open()
    }
}

impl Default for RedisHiLoStorage {
    fn default() -> Self {
        Self::new(DEFAULT_REDIS_URL).unwrap()
    }
}