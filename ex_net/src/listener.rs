use std::io::Read;
use std::net::{TcpListener, TcpStream};
use anyhow::bail;
use chrono::Local;
use ex_common::{
	log, function
};
use crate::common::{is_available_local_port};

fn handle_connection(mut stream: &TcpStream) -> anyhow::Result<usize> {
	let mut buffer = [0; 64];
	
	let ret_size = stream.read(&mut buffer)?;
	log!("{:?}", buffer);
	Ok(ret_size)
}

pub fn startup_retry(ip: &String, mut port: u16, mut retry_count: u8) -> anyhow::Result<()> {
	// 포트 가능한지 확인하고,
	while retry_count > 0 {
		match is_available_local_port(&ip, port) {
			true => break,
			false => {
				if port == u16::max_value() {
					bail!("not enough port");
				}
				
				retry_count = retry_count - 1;
				port = port + 1;
			}
		}
	}
	
	// 오픈
	let host = ip.clone() + ":" + &port.to_string();
	let listener = TcpListener::bind(&host)?;
	log!("listener start({})", &host);
	
	for stream in listener.incoming() {
		let stream = stream?;
		handle_connection(&stream)?;
		log!("connection extablished{:?}", stream);
	}
	
	Ok(())
}

pub fn startup(ip: &String, port: u16) -> anyhow::Result<()> {
	startup_retry(ip, port, 5)
}