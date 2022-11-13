pub use crate::{function, log};

pub extern crate chrono;

pub use chrono::Local;

use std::{env, io};

pub fn get_current_path() -> io::Result<std::path::PathBuf> {
    env::current_dir()
}

pub fn get_current_path_str() -> String {
    String::from(get_current_path().unwrap().as_os_str().to_str().unwrap())
}

pub fn print_type_of_name<T>(_: &T) {
    log!("{}", std::any::type_name::<T>());
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[macro_export]
macro_rules! function {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        &name[..name.len() - 3] // -3 => remove ::f -_-
    }};
}

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {{
	    print!("{{\"dt\":\"{}\", \"wh\":{}({}), \"ct:\"",
	    $crate::common::Local::now().format("%Y-%m-%dT%H:%M:%S"),$crate::common::function!(),line!());
		print!($($arg)*);
	    println!("}}");
    }}
}

#[macro_export]
macro_rules! get_ref_member {
    ($self:ident, $mem_var:ident) => {
        $self.$mem_var.as_ref().unwrap()
    };
}

// continue macro
#[macro_export]
macro_rules! continue_fail_result {
    ($res:expr) => {
        match $res {
            Ok(val) => val,
            Err(e) => {
                continue;
            }
        }
    };
}

#[macro_export]
macro_rules! continue_fail_option {
    ($res:expr) => {
        match $res {
            Some(val) => val,
            None => {
                continue;
            }
        }
    };
}

#[macro_export]
macro_rules! continue_fail_condition {
    ($res:expr) => {
        match $res {
            false => {
                continue;
            }
            _ => {}
        }
    };
}
