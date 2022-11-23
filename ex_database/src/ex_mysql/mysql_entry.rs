use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};

type Config = ex_config::config_format::MySQLSchemaConf;
pub type MySQLPool = Pool<MySql>;

type FnStubBuildHook = Option<fn(&mut MySqlPoolOptions)>;

pub async fn make_pool_default(
    sql_conf:&Config,
    fn_build_hook: FnStubBuildHook,
) -> anyhow::Result<MySQLPool> {
	let mut pool_options = MySqlPoolOptions::new().max_connections(5);

	// hooking
	if fn_build_hook.is_none() == false {
		fn_build_hook.unwrap()(&mut pool_options);
	}

    Ok(pool_options.connect(&_into_uri(&sql_conf)[..]).await?)
}

pub(crate) fn _into_uri(sql_conf: &Config) -> String {
    ("mysql://").to_owned()
        + &sql_conf.auth.user
        + ":"
        + &sql_conf.auth.password
        + "@"
        + &sql_conf.host.ip
        + ":"
        + &sql_conf.host.port.to_string() 
		+ "/" + &sql_conf.schema_name
}
