use std::io::ErrorKind::WouldBlock;
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::{Acquire, Release};

use std::thread;
use std::thread::spawn;
use ex_common::{function, log};

use chrono::{Duration, Local};

pub fn _test_closure_and_lambda() {
	let mut vec: Vec<i32> = Vec::new();
	vec.push(1);
	vec.push(1);
	
	log!("{:?}", vec.as_ptr());
	
	fn lambda_as_ref(vec: &Vec<i32>) {
		log!("{:?}", vec.as_ptr());
	}
	
	fn lambda_as_move(vec: Vec<i32>) {
		log!("{:?}", vec.as_ptr());
	}
	
	lambda_as_ref(&vec);
	// lambda_as_move(vec);
	
	let closure_as_ref = |vec: &Vec<i32>| {
		log!("{:?}", vec.as_ptr());
	};
	
	let _closure_as_move = |vec: Vec<i32>| {
		log!("{:?}", vec.as_ptr());
	};
	
	closure_as_ref(&vec);
	// closure_as_move(vec);
	
	let vec2: Vec<i32> = Vec::new();
	
	let closure_as_all_ref_capture = || {
		log!("{:?}", vec.as_ptr());
		log!("{:?}", vec2.as_ptr());
	};
	closure_as_all_ref_capture();
	
	let closure_as_all_move_capture = move || {
		log!("{:?}", vec.as_ptr());
		log!("{:?}", vec2.as_ptr());
	};
	closure_as_all_move_capture();
}

// attempt to add with overflow
pub fn _test_lambda_performance() {
	fn lambda() {
		let mut sum: u32 = 0;
		for idx in 0..10000000 {
			if u32::MAX - sum >= idx {
				sum = 0
			}
			sum += idx;
		}
	}
	
	lambda();
}

pub fn _test_closure_performance() {
	let closure = || {
		let mut sum: u32 = 0;
		for idx in 0..10000000 {
			if u32::MAX - sum >= idx {
				sum = 0
			}
			sum += idx;
		}
	};
	
	closure();
}

fn _handle_connection(stream: TcpStream) {
	log!("{:?}", stream);
}

pub fn _test_acceptor() {
	let stop_handle = Arc::new(AtomicBool::new(false));
	let stop_handle_clone = stop_handle.clone();
	
	ctrlc::set_handler(move || {
		log!("Signal detected!!!!!(request stop)");
		stop_handle_clone.store(true, Release);
	}).expect("Error setting Ctrl-C handler");
	
	let listener = TcpListener::bind("localhost:7878").unwrap();
	while listener.set_nonblocking(true).is_ok() == false {}
	
	log!("Waiting for Ctrl-C...");
	for stream in listener.incoming() {
		match stream {
			Ok(stream) => { _handle_connection(stream) },
			Err(err) => {
				if err.kind() != WouldBlock {
					log!("leaving loop. error: {}", err);
					break;
				}
			}
		}
		
		if stop_handle.load(Acquire) == true {
			log!("stop!!");
			break;
		}
	}
	
	log!("Exit!!!");
}