use std::collections::BTreeMap;

use anyhow::bail;
use ex_config::config;
use ex_database::redis_entry::{self, Stub};

use r2d2::Pool;

pub(crate) type RedisPool = Pool<Stub>;
pub(crate) type MapRedisPool = BTreeMap<i64, RedisPool>;

pub fn boot_redis(config: &config::Config) -> anyhow::Result<MapRedisPool> {
    let mut map_redis_pool = MapRedisPool::new();

    for conf in config.redis_conf.data.iter() {
        let pool = redis_entry::make_pool_default(
            redis_entry::make_connection_info_from_config(conf),
            redis_entry::StubConfig::default(),
            None,
        )?;

        if let Some(_) = map_redis_pool.insert(conf.db_no, pool) {
            bail!("alreay exist db_no({})", conf.db_no);
        }
    }

    Ok(map_redis_pool)
}

fn _connect_test() {}
