use ex_config::config;
use ex_database::redis_entry::{self, Stub};
use r2d2::Pool;

pub(crate) type RedisPool = Pool<Stub>;

pub fn boot_redis(config: &config::Config) -> anyhow::Result<RedisPool> {
    redis_entry::make_pool_default(
        redis_entry::make_connection_info_from_config(&config.redis_conf),
        redis_entry::StubConfig::default(),
        None,
    )
}

fn _connect_test() {}
