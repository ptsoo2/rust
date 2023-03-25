use std::collections::BTreeMap;

use anyhow::bail;

use ex_common::log;
use ex_database::{
    ex_mysql::mysql_entry::{make_pool_default, MySQLPool},
    ex_redis::redis_entry::{self, RedisPool},
};
use ex_rabbitmq::{context::MQContext, publisher::Publisher};
use futures::FutureExt;
use lapin::ExchangeKind;

use crate::app;

pub(crate) type MapRedisPool = BTreeMap<u8 /* ERedisType */, RedisPool>;

// todo! &'static str 써도 문제없을까? 주소로 비교해서 못찾아오거나, 일캐 쓸때 성능이 안좋거나
pub(crate) type MapMySQLPool = BTreeMap<&'static str /* EMySQLType */, MySQLPool>;

#[non_exhaustive]
pub struct ERedisType;
impl ERedisType {
    #[allow(unused)]
    pub const DB_0: u8 = 0;
    #[allow(unused)]
    pub const DB_1: u8 = 1;
    #[allow(unused)]
    pub const DB_2: u8 = 2;
    #[allow(unused)]
    pub const DB_3: u8 = 3;
}

#[non_exhaustive]
pub struct EMySQLType;
impl EMySQLType {
    #[allow(unused)]
    pub const ACCOUNT: &'static str = "account_db";
    #[allow(unused)]
    pub const GAME: &'static str = "game_db";
}

pub(crate) fn boot_redis() -> anyhow::Result<MapRedisPool> {
    let mut map_redis_pool = MapRedisPool::new();

    let redis_conf = &app::get_instance().get_config().redis_conf;
    for conf in redis_conf.data.iter() {
        let pool = redis_entry::make_pool_default(
            redis_entry::make_connection_info_from_config(conf),
            redis_entry::StubConfig::default(),
            None,
        )?;

        if map_redis_pool.insert(conf.db_no as u8, pool).is_some() {
            bail!("alreay exist db_no({})", conf.db_no);
        }
    }

    log!("Success to boot redis");
    Ok(map_redis_pool)
}

#[allow(unused)]
pub(crate) async fn boot_mysql() -> anyhow::Result<MapMySQLPool> {
    let mut map_mysql_pool = MapMySQLPool::new();
    let mysql_conf = &app::get_instance().get_config().mysql_conf;
    for conf in mysql_conf.data.iter() {
        let pool = make_pool_default(conf, None).await?;
        if map_mysql_pool.insert(&conf.schema_name[..], pool).is_some() {
            bail!("alreay exist schema_name({})", conf.schema_name);
        }
    }

    log!("Success to boot mysql");
    Ok(map_mysql_pool)
}

#[allow(unused)]
pub(crate) async fn boot_mq() -> anyhow::Result<Publisher> {
    let mut publisher = Publisher::new(|| {
        async move {
            let mq_conf = &app::get_instance().get_config().mq_conf;
            let mut context = MQContext::new(mq_conf).await?;
            context
                .channel()
                .await?
                .declare_exchange(
                    1,
                    &mq_conf.publish_exchange.direct[..],
                    ExchangeKind::Direct,
                )
                .await?;
            Ok(context)
        }
        .boxed()
    });
    publisher.start().await?;

    log!("Success to boot rabbitmq");
    Ok(publisher)
}
