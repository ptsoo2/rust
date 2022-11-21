use ex_database::redis_entry;
use redis::{Cmd, ConnectionLike, Pipeline, Value};

#[allow(unused)]
fn test_redis() -> anyhow::Result<()> {
    let connection_info = redis_entry::make_connection_info("localhost", 6379, 1, None, None);

    let pool_config = redis_entry::StubConfig::default();
    let pool = redis_entry::make_pool_default(connection_info, pool_config, None)?;

    let mut conn = pool.get()?;
    {
        let rpy = conn.req_command(Cmd::new().arg("PING"))?;
        if let Value::Status(stat) = rpy {
            println!("{}", stat);
        }
    }
    {
        let a = 1;
    }
    {
        // 와.. 너무 쓰레기같이 써야하네..
        let result: Vec<Value> = Pipeline::with_capacity(3)
            .set("132123", "!@3123")
            .get("132123")
            .zrevrange("test_ranking", 0, -1)
            .query(&mut conn)?;

        let result = result.get(3).unwrap();
        if let Value::Bulk(result) = result {
            println!("{:?}", result);
        }
    }
    Ok(())
}
