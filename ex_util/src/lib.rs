use std::sync::mpsc::{channel, Receiver};
use ex_common::{log};

pub mod stop_handle;

pub fn regist_signal_handler() -> Receiver<()> {
	let (tx, rx) = channel();
	ctrlc::set_handler(move || {
		log!("Signal detected!!!!!");
		tx.send(()).expect("Could not send signal on channel.");
	}).expect("Error setting Ctrl-C handler");
	
	log!("Waiting for Ctrl-C...");
	rx
}