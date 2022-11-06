#![feature(decl_macro)]

#[macro_use]
extern crate rocket;

use std::borrow::{Borrow, BorrowMut};
use std::cell::{Cell, UnsafeCell};
use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::{AcqRel, Acquire, Release};
use std::sync::mpsc::{channel, Receiver};
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

// use std::net::{Incoming, TcpListener, TcpStream};
// use std::ops::Deref;
// use std::sync::{Arc, mpsc, Mutex};
// use std::sync::atomic::AtomicBool;
// use std::sync::mpsc::{channel, Receiver};
// use std::thread;
//
// use chrono::Local;
// use ex_common::{
// 	log, function
// };
// use futures::Stream;
//
// struct Worker {
// 	id_: usize,
// 	thread_: Option<thread::JoinHandle<()>>,
// 	is_active_: AtomicBool,
// }
//
// impl Worker {
// 	fn new(id: usize) -> Worker {
// 		Worker {
// 			id_: id,
// 			thread_: None,
// 			is_active_: AtomicBool::new(true),
// 		}
// 	}
//
// 	fn start(&mut self) {//, receiver: Arc<Mutex<Receiver<Job>>>) {
// 		let my_id = self.id_;
// 		let thread = thread::spawn(move || {
// 			while self.is_active_.load(std::sync::atomic::Ordering::AcqRel) == true {
// 				thread::sleep(Duration::from_millis(1));
// 			}
//
// 			log!("out of thread!!!({})", my_id);
// 		});
// 		self.thread_ = Some(thread);
// 	}
//
// 	fn stop(&mut self) {
// 		self.is_active_.store(false, std::sync::atomic::Ordering::AcqRel);
// 	}
// }
//
// type Job = Box<dyn FnOnce() + Send + 'static>;
//
// pub struct ThreadPool {
// 	workers_: Vec<Worker>,
// 	sender_: Option<mpsc::Sender<Job>>,
// }
//
// impl ThreadPool {
// 	pub fn new(size: usize) -> ThreadPool {
// 		let (sender, receiver) = mpsc::channel();
// 		let receiver = Arc::new(Mutex::new(receiver));
//
// 		let mut workers = Vec::with_capacity(size);
//
// 		for id in 0..size {
// 			let mut worker = Worker::new(id);
// 			worker.start();//Arc::clone(&receiver));
// 			workers.push(worker);
// 		}
//
// 		ThreadPool {
// 			workers_: workers,
// 			sender_: Some(sender),
// 		}
// 	}
//
// 	pub fn execute<F>(&self, f: F)
// 	                  where F: FnOnce() + Send + 'static
// 	{
// 		let job = Box::new(f);
// 		self.sender_.as_ref().unwrap().send(job).unwrap();
// 	}
//
// 	pub fn stop(&mut self)
// 	{
// 		for x in &mut self.workers_ {
// 			x.stop();
// 		}
// 	}
// }
//
// impl Drop for ThreadPool {
// 	fn drop(&mut self) {
// 		for worker in &mut self.workers_ {
// 			log!("Shutting down worker {}", worker.id_);
//
// 			if let Some(thread) = worker.thread_.take() {
// 				thread.join().unwrap();
// 			}
// 		}
// 	}
// }
//
// fn handle_connection(stream: TcpStream) {}
//
// fn regist_signal_handler() -> Receiver<()> {
// 	let (tx, rx) = channel();
// 	ctrlc::set_handler(move || {
// 		println!("signal handling!!!!!");
// 		tx.send(()).expect("Could not send signal on channel.");
// 	}).expect("Error setting Ctrl-C handler");
//
// 	println!("Waiting for Ctrl-C...");
// 	rx
// }
//
// fn main() -> anyhow::Result<()> {
// 	let rx = regist_signal_handler();
//
// 	let listener = TcpListener::bind("localhost:7878")?;
// 	let mut pool = ThreadPool::new(10);
//
// 	// for stream in listener.incoming() {
// 	// 	log!("loop");
// 	// 	pool.execute(|| {
// 	// 		handle_connection(stream.unwrap());
// 	// 	});
// 	// }
//
// 	rx.recv().expect("Could not receive from channel");
//
// 	pool.stop();
//
// 	println!("Got it! Exiting...");
//
// 	println!("Shutting down");
// 	Ok(())
// }

struct StopHandle {
	stop_flag_: Arc<AtomicBool>
}

impl StopHandle {
	fn new() -> StopHandle {
		StopHandle {
			stop_flag_: Arc::new(AtomicBool::new(false))
		}
	}
	
	fn is_stop(&self) -> bool {
		self.stop_flag_.load(Acquire)
	}
	
	fn stop(&mut self) {
		self.stop_flag_.store(true, Release);
	}
}

fn spawn_thread(stop_handle: Arc<StopHandle>) {
	let handle = thread::spawn(move || {
		let a = stop_handle.clone();
		while a.is_stop() == false {
			thread::sleep(Duration::from_millis(1));
		}
		
		log!("out of thread");
	});
	
	log!("before join");
	handle.join().unwrap();
	log!("after join");
}

fn main() {
	// main thread channel
	let (tx, rx) = channel::<()>();
	
	// thread stop handle
	let mut stop_handle = Arc::new(StopHandle::new());
	
	// register handler
	{
		let mut stop_handle2 = stop_handle.clone();
		ctrlc::set_handler(move || {
			log!("signal handler");
			
			// stop thread
			stop_handle2.stop_flag_.store(true, Release);
			
			// stop main channel
			tx.send(());
		}).expect("Error setting Ctrl-C handler");
		log!("register stop handler");
	}
	
	// thread spawn
	spawn_thread(stop_handle.clone());
	
	// main thread wait
	rx.recv().expect("!!!!!!!");
}