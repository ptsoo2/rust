use crate::{
    command_line::CommandLine,
    server::mount_port1,
    server_common,
    third_party::{boot_mq, boot_redis, MapRedisPool, RedisPool},
};
use ex_common::{get_mut_ref_member, get_ref_member, log};
use ex_config::config::{Config, EConfigLoadType};
use ex_rabbitmq::publisher::Publisher;

pub struct App {
    command_line_: Option<CommandLine>,
    config_: Option<Config>,
    map_redis_pool_: Option<MapRedisPool>,
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

        this._cleanup().await;
        log!("success all cleanup");
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

    pub fn get_redis_pool(&'static self, db_no: i64) -> Option<&RedisPool> {
        let map = get_ref_member!(self, map_redis_pool_);
        map.get(&db_no)
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
        self.map_redis_pool_ = Some(boot_redis(get_mut_ref_member!(self, config_))?);
        self.mq_publisher_ = Some(boot_mq(&get_mut_ref_member!(self, config_).mq_conf));
        get_mut_ref_member!(self, mq_publisher_).start().await?;
        Ok(self)
    }

    async fn _cleanup(&'static mut self) -> anyhow::Result<&'static mut App> {
        // rabbitmq
        if let Some(publisher) = &mut self.mq_publisher_ {
            publisher.stop();
        }
        Ok(self)
    }
}

pub static mut INSTANCE: App = App {
    command_line_: None,
    config_: None,
    map_redis_pool_: None,
    mq_publisher_: None,
};

pub fn get_instance() -> &'static mut App {
    unsafe { &mut INSTANCE }
}
