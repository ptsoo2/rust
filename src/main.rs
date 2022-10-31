use std::{env, thread};
use std::time::Duration;
use extern_config::config;
use ::function_name::named;
use extern_common::{function, log};

mod app;

struct Cacher<T>
	where T: Fn(u32) -> u32
{
	calculation: T,
	value: Option<u32>,
}

#[named]
fn closure_test(arg: Vec<i32>) {
	let lambda = |arg: Vec<i32>| {
		println!("{} | {} | {:?}", function_name!(), line!(), arg.as_ptr());
	};

	println!("{} | {} | {:?}", function_name!(), line!(), arg.as_ptr());

	let mut local_vec: Vec<i32> = Vec::new();
	local_vec.push(1);
	local_vec.push(1);
	local_vec.push(1);
	local_vec.push(1);
	local_vec.push(1);
	local_vec.push(1);

	log!("{:?}", local_vec.as_ptr());
	lambda(local_vec);
	log!("{:?}", arg.as_ptr());
}

#[named]
fn main() -> anyhow::Result<()> {
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

	let closure_as_move = |vec: Vec<i32>| {
		log!("{:?}", vec.as_ptr());
	};

	closure_as_ref(&vec);
	// closure_as_move(vec);

	let mut vec2: Vec<i32> = Vec::new();

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


	// let closure_as_move_capture = move || println!("{:?}", vec.as_ptr());
	// vec.len() == 2 as usize;

	// println!("{}", closure_as_move_capture());

	// println!("{}", vec.len()); // compile-error!!!

	Ok(())


	// init_singletons();
	//
	// let config_path = config::parse_config_path(env::args().collect())?;
	// app::get_instance().load_config(config_path)
}

pub fn init_singletons()
{
	let _ret = app::get_instance();
}