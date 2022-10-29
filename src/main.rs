mod app;

use std::env;
use crate::app::CApplication;

extern crate extern_config;

use extern_config::config;
use extern_common::common;

fn main() {
	println!("start main =================================================================");
	
	let config_path = config::parse_config_path(env::args().collect())
		.unwrap_or((common::get_current_path_str() + "/cfg/config.yaml").to_owned());
	
	let mut app = CApplication::new();
	app.load_config(config_path);
	
	/*
	// config2::CConfig::new().load_from_yaml(config_path);

	//
	//
	// {
	// 	Ok(yaml) => yaml,
	// 	Err(e) => { panic!("Failed load config2({})", e); }
	// };
	// println!("Load Config Elem: {}", ret.len());

	// YamlLoader::load_from_str()

	// println!("{}", String::from()));

	// println!("{:?}", getCurrentPath());

	//
	// println!("{}", common2::isAvailableLocalPort(6379));
	//
	// let ret = match TcpListener::bind("127.0.0.1:6379")
	// {
	// 	Ok(e) => {
	// 		println!("OK");
	// 		e
	// 	}
	// 	Err(e) => {
	// 		println!("{}", e);
	// 		panic!("WTF")
	// 	}
	// };
	//
	// ret.accept();
	//
	//
	// let ret = match TcpListener::bind(":::6379")
	// {
	// 	Ok(w) => w,
	// 	Err(e) => {
	// 		println!("{}", e);
	// 		return Err(e);
	// 	}
	// };
	// for stream in ret.incoming()
	// {
	// 	println!("ho!!!")
	// }
	//
	// Ok(())
	*/
}
