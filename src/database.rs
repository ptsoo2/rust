use ex_config::config;
use ex_database::redis_entry;

pub fn boot_redis(config: &config::Config) -> anyhow::Result<()> {
    let _pool = redis_entry::make_pool_default(
        redis_entry::make_connection_info_from_config(&config.redis_conf),
        redis_entry::StubConfig::default(),
        None,
    )?;
    Ok(())
}

fn _connect_test() {}
