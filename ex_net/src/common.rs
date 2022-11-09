use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::net::IpAddr::V4;
use std::ops::Deref;
use network_interface::NetworkInterface;

pub fn is_available_local_port(ip: &String, port: u16) -> bool {
	let ip = &ip[..];
	std::net::TcpStream::connect((ip, port)).is_err() == true
}

pub fn is_available_port(ipAddr: &IpAddr, port: u16) -> bool {
	std::net::TcpStream::connect(&SocketAddr::new(*ipAddr, port)).is_err() == true
}

pub fn _is_wsl_interface(network_interface: &NetworkInterface) -> bool {
	network_interface.name.find("WSL").is_none() == false
}
