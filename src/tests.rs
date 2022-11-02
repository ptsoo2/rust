use ex_common::{
	log, function
};

use chrono::Local;

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
