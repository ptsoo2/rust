#![feature(decl_macro)]

#[macro_use]
extern crate rocket;

use std::borrow::{Borrow, BorrowMut};
use std::cell::{Cell, UnsafeCell};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::{AcqRel, Acquire, Release};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::thread::spawn;
use std::time::Duration;
use chrono::Local;
use ex_common::{
	log, function
};
use ex_config::config::{CConfig, EConfigLoadType};
use rocket::config::Environment::Production;

use crate::server::mount;
use crate::server_common::{launch_all, make_launch_hint};

mod tests;
mod server_common;
mod server;
mod command_line;

// fn main() -> anyhow::Result<()> {
//
// 	// parse commandLine
// 	let command_line = command_line::CommandLine::default()
// 		.load()?;
//
// 	// load config
// 	let config = CConfig::default()
// 		.load(command_line.config_file_path_, EConfigLoadType::YAML)?;
//
// 	let launch_hint = make_launch_hint(
// 		&config.server_group_.server_group,
// 		&[mount, mount]
// 	)?;
//
// 	// launch
// 	launch_all(launch_hint)?;
//
// 	println!("run out spawn rocket");
// 	loop {
// 		std::thread::sleep(Duration::from_millis(1))
// 	}
// }

/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct Worker {
	id_: usize,
	thread_: Option<thread::JoinHandle<()>>,
	stop_handle_: Arc<AtomicBool>,
}

impl Worker {
	fn new(id: usize) -> Worker {
		Worker {
			id_: id,
			thread_: None,
			stop_handle_: Arc::new(AtomicBool::new(false))
		}
	}
	
	fn start(&mut self, _receiver: Arc<Mutex<Receiver<Job>>>) {
		let my_id = self.id_;
		let handle = self.stop_handle_.clone();
		let thread = spawn(move || {
			log!("Spawn up worker({})", my_id);
			while handle.load(Acquire) == false {
				thread::sleep(Duration::from_millis(1));
			}
		});
		self.thread_ = Some(thread);
	}
	
	fn stop(&mut self) {
		self.stop_handle_.store(true, Release);
	}
}

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
	workers_: Vec<Worker>,
	sender_: Option<Sender<Job>>,
}

impl ThreadPool {
	pub fn new(size: usize) -> ThreadPool {
		let (sender, receiver) = channel();
		let receiver = Arc::new(Mutex::new(receiver));
		
		let mut workers = Vec::with_capacity(size);
		
		for id in 0..size {
			let mut worker = Worker::new(id);
			worker.start(Arc::clone(&receiver));
			workers.push(worker);
		}
		
		ThreadPool {
			workers_: workers,
			sender_: Some(sender),
		}
	}
	
	pub fn execute<F>(&self, f: F)
	                  where F: FnOnce() + Send + 'static
	{
		let job = Box::new(f);
		self.sender_.as_ref().unwrap().send(job).unwrap();
	}
	
	pub fn stop(&mut self)
	{
		for x in &mut self.workers_ {
			x.stop();
		}
	}
}

impl Drop for ThreadPool {
	fn drop(&mut self) {
		for worker in &mut self.workers_ {
			if let Some(thread) = worker.thread_.take() {
				thread.join().unwrap();
				log!("Shutting down worker({})", worker.id_);
			}
		}
	}
}

fn handle_connection(_stream: TcpStream) {}

fn regist_signal_handler() -> Receiver<()> {
	let (tx, rx) = channel();
	ctrlc::set_handler(move || {
		println!("Signal detected!!!!!");
		tx.send(()).expect("Could not send signal on channel.");
	}).expect("Error setting Ctrl-C handler");
	
	println!("Waiting for Ctrl-C...");
	rx
}

fn main() -> anyhow::Result<()> {
	let rx = regist_signal_handler();
	
	let _listener = TcpListener::bind("localhost:7878")?;
	let mut pool = ThreadPool::new(10);
	
	// for stream in listener.incoming() {
	// 	log!("loop");
	// 	pool.execute(|| {
	// 		handle_connection(stream.unwrap());
	// 	});
	// }
	
	pool.stop();
	
	println!("Got it! Exiting...");
	rx.recv().expect("Could not receive from channel");
	
	println!("Shutting down");
	Ok(())
}
