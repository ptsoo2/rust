use std::{env, io};

pub fn get_current_path() -> io::Result<std::path::PathBuf> {
	env::current_dir()
}

pub fn get_current_path_str() -> String {
	String::from(get_current_path().unwrap().as_os_str().to_str().unwrap())
}

pub fn print_type_of_name<T>(_: &T) {
	println!("{}", std::any::type_name::<T>())
}

pub fn is_available_local_port(port: u16) -> bool {
	match std::net::TcpStream::connect(("127.0.0.1", port)) {
		Ok(_) => false,
		Err(_) => true,
	}
}
