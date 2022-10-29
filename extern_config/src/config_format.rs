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

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Base_Format {
	pub host: Host,
	pub naming: Naming,
}

impl Default for Base_Format {
	fn default() -> Self {
		Self {
			host: internal::host(),
			naming: internal::naming(),
		}
	}
}
// #[derive(Clone, Default, Debug, Deserialize, Serialize, PartialEq)]
// pub struct Base_Format {
// 	#[serde(default = "default::host")]
// 	host: Host,
// 	#[serde(default = "default::naming")]
// 	naming: Naming,
// }

mod internal {
	use crate::config_format::{Host, Naming};
	
	pub fn host() -> Host {
		Host {
			ip: ("").to_owned(),
			port: 30002,
		}
	}
	
	pub fn naming() -> Naming {
		Naming {
			service_name: ("123123").to_owned(),
		}
	}
}