use std::net::{IpAddr, Ipv4Addr, };

use anyhow::bail;
use network_interface::{NetworkInterface};
use network_interface::NetworkInterfaceConfig;

pub fn is_available_local_port(ip: &String, port: u16) -> bool {
	let ip = &ip[..];
	match std::net::TcpStream::connect((ip, port)) {
		Ok(_) => false,
		Err(_) => true,
	}
}

pub fn get_my_ip<T>() -> anyhow::Result<Ipv4Addr> {
	let network_interface_list = NetworkInterface::show()?;
	for val in network_interface_list.iter() {
		if _is_wsl_interface(&val) == true {
			continue
		}
		
		match val.addr {
			None => continue,
			Some(addr) => {
				let addr = addr.ip();
				if addr.is_loopback() == true {
					continue
				}
				
				match _ip_selector::<T>(&addr) {
					None => continue,
					Some(e) => { return Ok(e); },
				}
			}
		};
	}
	bail!("!!!")
}

fn _ip_selector<T>(t: &IpAddr) -> Option<Ipv4Addr> {
	if t.is_ipv4() == true {
		let a = match t {
			IpAddr::V4(v4) => { Some(v4.clone()) },
			_ => { None }
		};
		return a;
	} else if t.is_ipv6() == true {
		let a = match t {
			IpAddr::V4(v4) => { Some(v4.clone()) },
			_ => { None }
		};
		return a;
	}
	None
}

fn _is_wsl_interface(network_interface: &NetworkInterface) -> bool {
	network_interface.name.find("WSL").is_none() == false
}