use serde::{Deserialize, Serialize};

// use crate::stdafx::config_format::default::{host, naming};

/*
 * 지원하는 Element 를 적는다.
 * 순서는 상관없다.
 * 이름은 정확하게 일치해야한다.  */
#[derive(Clone, Default, Debug, Deserialize, Serialize, PartialEq)]
pub struct Host {
	pub ip: String,
	pub port: u16,
}

#[derive(Clone, Default, Debug, Deserialize, Serialize, PartialEq)]
pub struct Naming {
	pub service_name: String,
}

#[derive(Clone, Default, Debug, Deserialize, Serialize, PartialEq)]
pub struct Customize {
	pub env_type: String,
	pub worker_count: usize,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct ServerConfig {
	pub host: Host,
	pub naming: Naming,
	pub customize: Customize,
}

impl Default for ServerConfig {
	fn default() -> Self {
		Self {
			host: internal::host(),
			naming: internal::naming(),
			customize: internal::customize(),
		}
	}
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct ServerGroup {
	pub server_group: Vec<ServerConfig>,
}

impl Default for ServerGroup {
	fn default() -> Self {
		Self {
			server_group: Vec::new()
		}
	}
}

mod internal {
	use crate::config_format::{Customize, Host, internal, Naming, ServerConfig};
	
	pub fn host() -> Host {
		Host {
			ip: ("127.0.0.1").to_owned(),
			port: 30002,
		}
	}
	
	pub fn naming() -> Naming {
		Naming {
			service_name: ("123123").to_owned(),
		}
	}
	
	pub fn customize() -> Customize {
		Customize {
			env_type: ("debug").to_owned(),
			worker_count: 5,
		}
	}
	
	pub fn _server_config() -> ServerConfig {
		ServerConfig {
			host: internal::host(),
			naming: internal::naming(),
			customize: internal::customize(),
		}
	}
}
