// #![feature(decl_macro)]
//
// #[macro_use]
// extern crate rocket;
//
// use std::time::Duration;
// use ex_config::config::{CConfig, EConfigLoadType};
//
// use crate::server::mount;
// use crate::server_common::{launch_all, make_launch_hint};
//
// mod tests;
// mod server_common;
// mod server;
// mod command_line;
//
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

use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::channel;
use std::thread;

use chrono::Local;
use ex_common::{
	log, function
};
use futures::Stream;
use rocket::config::Environment::Production;

struct Worker {
	id_: usize,
	thread_: Option<thread::JoinHandle<()>>,
}

impl Worker {
	fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
		let thread = thread::spawn(move || {
			loop {
				let message = receiver.lock().unwrap().recv();
				match message {
					Ok(job) => {
						log!("Worker {id} got a job, executing.");
						job();
					},
					Err(_) => {
						log!("Worker {id} disconnected; shutting down.");
						break;
					}
				}
			}
		});
		
		Worker {
			id_: id,
			thread_: Some(thread)
		}
	}
}

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
	workers_: Vec<Worker>,
	sender_: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
	pub fn new(size: usize) -> ThreadPool {
		let (sender, receiver) = mpsc::channel();
		let receiver = Arc::new(Mutex::new(receiver));
		
		let mut workers = Vec::with_capacity(size);
		
		for id in 0..size {
			workers.push(Worker::new(id, Arc::clone(&receiver)));
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
}

impl Drop for ThreadPool {
	fn drop(&mut self) {
		for worker in &mut self.workers_ {
			log!("Shutting down worker {}", worker.id_);
			
			if let Some(thread) = worker.thread_.take() {
				thread.join().unwrap();
			}
		}
	}
}

fn handle_connection(stream: TcpStream) {}

fn regist_signal_handler() {
	let (tx, rx) = channel();
	ctrlc::set_handler(move || {
		println!("signal handling!!!!!");
		tx.send(()).expect("Could not send signal on channel.");
	}).expect("Error setting Ctrl-C handler");
	
	println!("Waiting for Ctrl-C...");
	rx.recv().expect("Could not receive from channel");
	println!("Got it! Exiting...");
}

fn main() -> anyhow::Result<()> {
	regist_signal_handler();
	
	let listener = TcpListener::bind("localhost:7878")?;
	let pool = ThreadPool::new(4);
	
	for stream in listener.incoming().take(2) {
		pool.execute(|| {
			handle_connection(stream.unwrap());
		});
	}
	
	println!("Shutting down");
	Ok(())
}