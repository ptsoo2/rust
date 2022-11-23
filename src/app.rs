use crate::{
    command_line::CommandLine,
    server::mount_port1,
    server_common,
    third_party::{boot_mq, boot_mysql, boot_redis, MapMySQLPool, MapRedisPool},
};
use ex_common::{get_mut_ref_member, get_ref_member, log};
use ex_config::config::{Config, EConfigLoadType};
use ex_database::{ex_mysql::mysql_entry::MySQLPool, ex_redis::redis_entry::RedisPool};
use ex_rabbitmq::publisher::Publisher;

pub struct App {
    command_line_: Option<CommandLine>,
    config_: Option<Config>,
    map_redis_pool_: Option<MapRedisPool>,
    mysql_pool_: Option<MapMySQLPool>,
    mq_publisher_: Option<Publisher>,
}

impl App {
    pub fn init(&'static mut self) -> anyhow::Result<&'static mut App> {
        // parse commandLine
        self.command_line_ = Some(CommandLine::default().load()?);

        // load config
        let config_file_path = &get_ref_member!(self, command_line_).config_file_path_;
        self.config_ = Some(Config::create_and_load(
            config_file_path.clone(),
            EConfigLoadType::YAML,
        )?);
        Ok(self)
    }

    #[allow(unused)]
    pub async fn launch(&'static mut self) -> anyhow::Result<()> {
        let this = self._boot_third_party().await?._launch().await?;

        //this._cleanup().await;
        Ok(())
    }

    pub async fn _launch(&'static mut self) -> anyhow::Result<&'static mut App> /*anyhow::Result<(&'static mut App, Vec<Rocket<Ignite>>)>*/
    {
        let server_config_list = &mut get_mut_ref_member!(self, config_).server_group.data;
        let launch_hint_list = server_common::make_launch_hint_list(
            server_config_list,
            &[mount_port1 /*, mount_port2 */],
        )?;

        server_common::launch_all(launch_hint_list).await?;
        Ok(self)
    }

    pub fn get_config(&'static self) -> &Config {
        get_ref_member!(self, config_)
    }

    pub fn get_redis_pool(&'static self, db_no: u8) -> Option<&RedisPool> {
        get_ref_member!(self, map_redis_pool_).get(&db_no)
    }

    #[allow(unused)]
    pub fn get_mysql_pool(&'static self, schema_name: &'static str) -> Option<&MySQLPool> {
        get_ref_member!(self, mysql_pool_).get(&schema_name)
    }

    #[allow(unused)]
    pub fn get_mq_publisher(&'static mut self) -> &mut Publisher {
        get_mut_ref_member!(self, mq_publisher_)
    }

    #[allow(unused)]
    pub fn get_command_line(&'static self) -> &CommandLine {
        get_ref_member!(self, command_line_)
    }

    async fn _boot_third_party(&'static mut self) -> anyhow::Result<&'static mut App> {
        // database
        self.map_redis_pool_ = Some(boot_redis()?);
        self.mysql_pool_ = Some(boot_mysql().await?);

        // middleware
        self.mq_publisher_ = Some(boot_mq().await);
        // todo! start 를 boot_mq 내부에서 하니까 바로 스레드가 종료된다. 이유가 뭐지? 알아봐야함.
        get_mut_ref_member!(self, mq_publisher_).start().await?;

        Ok(self)
    }

    async fn _cleanup(&'static mut self) -> anyhow::Result<&'static mut App> {
        // rabbitmq
        if let Some(publisher) = &mut self.mq_publisher_ {
            publisher.stop();
        }

        log!("success all cleanup");
        Ok(self)
    }
}

pub static mut INSTANCE: App = App {
    command_line_: None,
    config_: None,
    map_redis_pool_: None,
    mysql_pool_: None,
    mq_publisher_: None,
};

pub fn get_instance() -> &'static mut App {
    unsafe { &mut INSTANCE }
}
