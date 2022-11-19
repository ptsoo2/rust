use std::collections::BTreeMap;

use anyhow::bail;
use ex_config::config;
use ex_database::redis_entry::{self, Stub};

use ex_rabbitmq::{context::MQContext, runner::MQRunnerBase};
use futures::FutureExt;
use lapin::ExchangeKind;
use r2d2::Pool;

use crate::app;

pub(crate) type RedisPool = Pool<Stub>;
pub(crate) type MapRedisPool = BTreeMap<i64, RedisPool>;

pub(crate) fn boot_redis(config: &config::Config) -> anyhow::Result<MapRedisPool> {
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

#[allow(unused)]
pub(crate) fn boot_mq(config: &config::Config) -> MQRunnerBase {
    let runner = MQRunnerBase::new(|| {
        async move {
            let mq_conf = &app::get_instance().get_config().mq_conf;
            let mut context = MQContext::new(mq_conf).await?;
            context
                .channel()
                .await?
                .declare_exchange(1, "game_server.direct", ExchangeKind::Direct)
                .await?;
            Ok(context)
        }
        .boxed()
    });
    runner
}

fn _connect_test() {}
