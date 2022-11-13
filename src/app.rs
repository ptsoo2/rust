use crate::{
    command_line::CommandLine,
    database::{boot_redis, RedisPool},
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
    redis_pool_: Option<RedisPool>,
}

pub static mut INSTANCE: App = App {
    command_line_: None,
    config_: None,
    redis_pool_: None,
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
        self._boot_database()
    }

    pub async fn launch(&'static self) -> anyhow::Result<Vec<Rocket<Ignite>>> {
        let server_config_list = &self.config().server_group.data;
        let launch_hint_list =
            server_common::make_launch_hint_list(server_config_list, &[mount_port1, mount_port2])?;

        server_common::launch_all(launch_hint_list).await
    }

    pub fn config(&'static self) -> &Config {
        get_ref_member!(self, config_)
    }

    pub fn redis_pool(&'static self) -> &RedisPool {
        get_ref_member!(self, redis_pool_)
    }

    #[allow(unused)]
    pub fn command_line(&'static self) -> &CommandLine {
        get_ref_member!(self, command_line_)
    }

    fn _boot_database(&'static mut self) -> anyhow::Result<&'static mut App> {
        self.redis_pool_ = Some(boot_redis(get_ref_member!(self, config_))?);
        Ok(self)
    }
}

pub fn get_instance() -> &'static mut App {
    unsafe { &mut INSTANCE }
}
