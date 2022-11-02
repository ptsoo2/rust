use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread::sleep;
use std::time::Duration;
use anyhow::bail;
use chrono::Local;
use ex_common::{
	log, function
};
use crate::common::{is_available_local_port};

fn _get_hard_coding_html() -> &'static str {
	"
	<!DOCTYPE html>
<html lang=\"en\">
  <head>
    <meta charset=\"utf-8\">
    <title>Hello!</title>
  </head>
  <body>
    <h1>Hello! Taesoo~!</h1>
    <p>Hi from Rust</p>
  </body>
</html>"
}

fn handle_connection(mut stream: &TcpStream) -> anyhow::Result<usize> {
	let mut buffer = [0; 512];
	
	let ret_size = stream.read(&mut buffer)?;
	let response = format!(
		"HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
		_get_hard_coding_html().len(),
		_get_hard_coding_html()
	);
	
	log!("{}", String::from_utf8_lossy(&buffer[..]));
	
	stream.write(response.as_bytes())?;
	stream.flush()?;
	
	Ok(ret_size)
}

pub fn startup_retry(ip: &String, mut port: u16, retry_count: u8) -> anyhow::Result<()> {
	_pre_bind(ip, &mut port, retry_count)?;
	
	// 오픈
	let host = ip.clone() + ":" + &port.to_string();
	let listener = TcpListener::bind(&host)?;
	log!("listener start({})", &host);
	
	let mut vecStream: Vec<TcpStream> = Vec::new();
	
	for stream in listener.incoming() {
		let stream = stream?;
		log!("connection extablished{:?}", stream);
		handle_connection(&stream)?;
		vecStream.push(stream);
	}
	
	Ok(())
}

pub fn startup(ip: &String, port: u16) -> anyhow::Result<()> {
	startup_retry(ip, port, 5)
}

fn _pre_bind(ip: &String, port: &mut u16, mut retry_count: u8) -> anyhow::Result<()> {
	// 포트 가능한지 확인하고,
	while retry_count > 0 {
		if is_available_local_port(&ip, *port) == true {
			return Ok(());
		}
		if *port == u16::MAX {
			bail!("not enough port");
		}
		retry_count = retry_count - 1;
		*port = *port + 1;
	}
	bail!("not enough port");
}