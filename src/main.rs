#![feature(decl_macro)]

#[macro_use]
extern crate rocket;

use std::borrow::{Borrow, BorrowMut};
use std::cell::{Cell, UnsafeCell};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::ops::Deref;
use std::os::windows::io::AsRawSocket;
use std::sync::{Arc, mpsc, Mutex};
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::{AcqRel, Acquire, Relaxed, Release};
use std::sync::mpsc::{channel, Receiver, RecvError, Sender};
use std::{io, thread};
use std::pin::Pin;
use std::thread::{JoinHandle, sleep, spawn};
use std::time::Duration;
use anyhow::bail;
use chrono::Local;
use ex_common::{
	log, function
};
use ex_util::stop_handle::{
	StopToken, StopHandle
};

use ex_config::config::{CConfig, EConfigLoadType};

use rocket::{Build, Rocket};

use crate::server::mount;
use crate::server_common::{_make_rocket, launch_all, LaunchHint, make_launch_hint};

mod tests;
mod server_common;
mod server;
mod command_line;

fn regist_signal_handler() -> Receiver<()> {
	let (tx, rx) = channel();
	ctrlc::set_handler(move || {
		log!("Signal detected!!!!!");
		tx.send(()).expect("Could not send signal on channel.");
	}).expect("Error setting Ctrl-C handler");
	
	log!("Waiting for Ctrl-C...");
	rx
}

#[get("/")]
fn test() {
	println!("test!!!!!!");
}

#[rocket::main]
async fn main() -> anyhow::Result<()> {
	let launch_hint = LaunchHint::default();
	
	let rocket = _make_rocket(&launch_hint).await?;
	let rocket = rocket.mount("/", routes![test]);
	rocket.launch().await?;
	
	// rx.recv().unwrap();
	Ok(())
	// let rx = regist_signal_handler();
	//
	// // parse commandLine
	// let command_line = command_line::CommandLine::default()
	// 	.load()?;
	//
	// // load config
	// let config = CConfig::default()
	// 	.load(command_line.config_file_path_, EConfigLoadType::YAML)?;
	//
	// let launch_hint = make_launch_hint(
	// 	&config.server_group_.server_group,
	// 	&[mount, mount]
	// )?;
	//
	// // launch
	// launch_all(launch_hint)?;
	//
	// println!("run out spawn rocket");
	// match rx.recv()
	// {
	// 	Ok(_) => { Ok(()) }
	// 	Err(e) => { bail!(e); }
	// }
}
