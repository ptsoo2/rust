use std::fmt;
use redis::Value;
use crate::redis_value::ERedisValueType::{ARRAY, CLIENT_ERR, INTEGER, NIL, STRING};

#[derive(Debug, Clone)]
enum ERedisValueType {
	CLIENT_ERR = 0,
	NIL = 1,
	INTEGER,
	STRING,
	ARRAY,
}

pub struct RedisValue {
	value_: redis::Value,
	value_type_: ERedisValueType,
	integer_: i64,
	string_: String,
}

impl fmt::Debug for RedisValue {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("RedisValue")
			.field("value", &self.value_)
			.field("value_type", &self.value_type_)
			.finish()
	}
}

impl RedisValue {
	pub fn new(value: redis::Value) -> Self {
		let mut ret = Self {
			value_: value,
			value_type_: CLIENT_ERR,
			integer_: 0,
			string_: String::new()
		};
		ret._analyze();
		ret
	}
	
	fn is_type(&self, value_type: ERedisValueType) -> bool { return self.value_type_.clone() as u8 == value_type as u8; }
	pub fn is_nil(&self) -> bool { return self.is_type(NIL) == true; }
	pub fn is_integer(&self) -> bool { return self.is_type(INTEGER) == true; }
	pub fn is_string(&self) -> bool { return self.is_type(STRING) == true; }
	pub fn is_array(&self) -> bool { return self.is_type(ARRAY) == true; }
	
	fn _analyze(&mut self) {
		match &self.value_ {
			Value::Nil => {
				self.value_type_ = NIL;
			},
			Value::Int(value) => {
				self.value_type_ = INTEGER;
				self.integer_ = *value;
			},
			Value::Status(value) => {
				self.value_type_ = STRING;
				self.string_ = value.clone();
			},
			Value::Okay => {
				self.value_type_ = STRING;
				self.string_ = ("OK").to_owned();
			}
			_ => {}
		};
	}
}