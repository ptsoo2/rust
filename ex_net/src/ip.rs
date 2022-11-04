use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use anyhow::bail;
use ex_common::{
	continue_fail_option
};

use network_interface::NetworkInterface;
use network_interface::NetworkInterfaceConfig;
use crate::common::_is_wsl_interface;

pub trait ConvertIpAddr<T> {
	fn convert(_: &IpAddr) -> Option<T>;
}

impl ConvertIpAddr<Ipv4Addr> for Ipv4Addr {
	fn convert(a: &IpAddr) -> Option<Ipv4Addr> {
		if a.is_ipv4() == false { return None; }
		if let IpAddr::V4(a) = a {
			return Some(a.clone());
		}
		None
	}
}

impl ConvertIpAddr<Ipv6Addr> for Ipv6Addr {
	fn convert(a: &IpAddr) -> Option<Ipv6Addr> {
		if a.is_ipv6() == false { return None; }
		if let IpAddr::V6(a) = a {
			return Some(a.clone());
		}
		None
	}
}

pub fn get_my_ip<T>() -> anyhow::Result<T> where T: ConvertIpAddr<T> + Copy {
	let network_interface_list = NetworkInterface::show()?;
	
	for network_interface in network_interface_list.iter() {
		if _is_wsl_interface(network_interface) == true {
			continue;
		}
		let addr = network_interface.addr;
		continue_fail_option!(addr);
		
		let addr = addr.unwrap().ip();
		if addr.is_loopback() == true {
			continue;
		}
		
		let addr = <T as ConvertIpAddr::<T>>::convert(&addr);
		continue_fail_option!(addr);
		
		return Ok(addr.unwrap());
	}
	bail!("")
}