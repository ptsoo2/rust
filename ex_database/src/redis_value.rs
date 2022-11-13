use crate::redis_value::ERedisValueType::{ARRAY, ERROR, INTEGER, NIL, STRING};
use redis::Value;
use std::fmt;

#[derive(Debug, Clone)]
enum ERedisValueType {
    ERROR = 0,
    NIL,
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
            value_type_: ERROR,
            integer_: 0,
            string_: String::new(),
        };
        ret._analyze();
        ret
    }

    fn is_type(&self, value_type: ERedisValueType) -> bool {
        return self.value_type_.clone() as u8 == value_type as u8;
    }
    pub fn is_nil(&self) -> bool {
        return self.is_type(NIL) == true;
    }
    pub fn is_integer(&self) -> bool {
        return self.is_type(INTEGER) == true;
    }
    pub fn is_string(&self) -> bool {
        return self.is_type(STRING) == true;
    }
    pub fn is_array(&self) -> bool {
        return self.is_type(ARRAY) == true;
    }

    pub fn get_integer(&self) -> i64 {
        assert_eq!(self.is_string(), true);
        self.integer_
    }

    pub fn get_string(&self) -> &String {
        assert_eq!(self.is_string(), true);
        &self.string_
    }

    fn _analyze(&mut self) {
        match &self.value_ {
            Value::Nil => {
                self.value_type_ = NIL;
            }
            Value::Int(value) => {
                self.value_type_ = INTEGER;
                self.integer_ = *value;
            }
            Value::Data(value) => {
                self.value_type_ = STRING;
                self.string_ = String::from_utf8_lossy(value.as_slice()).to_string();
            }
            Value::Status(value) => {
                self.value_type_ = STRING;
                self.string_ = value.clone();
            }
            Value::Okay => {
                self.value_type_ = STRING;
                self.string_ = ("OK").to_owned();
            }
            _ => {}
        };
    }
}
