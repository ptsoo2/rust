use crate::{
    command_line::CommandLine,
    database::{boot_redis, MapRedisPool, RedisPool},
    server::mount_port1,
    server::mount_port2,
    server_common,
};
use ex_common::get_ref_member;
use ex_config::config::{Config, EConfigLoadType};
use rocket::{Ignite, Rocket};

pub struct App {
    command_line_: Option<CommandLine>,
    config_: Option<Config>,
    map_redis_pool_: Option<MapRedisPool>,
}

pub static mut INSTANCE: App = App {
    command_line_: None,
    config_: None,
    map_redis_pool_: None,
};

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

        // database
        self._boot_third_party()
    }

    #[allow(unused)]
    pub async fn launch(&'static self) -> anyhow::Result<Vec<Rocket<Ignite>>> {
        let server_config_list = &self.get_config().server_group.data;
        let launch_hint_list =
            server_common::make_launch_hint_list(server_config_list, &[mount_port1, mount_port2])?;

        server_common::launch_all(launch_hint_list).await
    }

    pub fn get_config(&'static self) -> &Config {
        get_ref_member!(self, config_)
    }

    pub fn get_redis_pool(&'static self, db_no: i64) -> Option<&RedisPool> {
        let map = get_ref_member!(self, map_redis_pool_);
        map.get(&db_no)
    }

    #[allow(unused)]
    pub fn get_command_line(&'static self) -> &CommandLine {
        get_ref_member!(self, command_line_)
    }

    fn _boot_third_party(&'static mut self) -> anyhow::Result<&'static mut App> {
        // redis
        self.map_redis_pool_ = Some(boot_redis(get_ref_member!(self, config_))?);
        Ok(self)
    }
}

pub fn get_instance() -> &'static mut App {
    unsafe { &mut INSTANCE }
}
