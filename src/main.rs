use std::env;
use extern_config::config;

/*
todo!
 - 이제 진짜 웹 서버 만들어 보자!!
 - 접속 처리 루틴이 좀 완성되면 DB 도 해보자!!
 - 로거
 - Signal 처리
 - 종료 처리
*/

mod app;

fn main() -> anyhow::Result<()> {
	init_singletons();
	
	let config_path = config::parse_config_path(env::args().collect())?;
	app::get_instance().load_config(config_path)
}

pub fn init_singletons()
{
	let _ret = app::get_instance();
}