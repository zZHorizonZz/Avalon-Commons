use std::collections::{HashMap, LinkedList};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::logging::handling::console_handler::ConsoleHandler;
use crate::logging::handling::handler::Handler;

pub struct Logger {
    current_identifier: i64,
    record_list: LinkedList<Record>,
    handler_list: Vec<Box<dyn Handler>>,
}

impl Logger {
    pub fn new() -> Logger {
        Logger {
            current_identifier: 0,
            record_list: Default::default(),
            handler_list: Default::default(),
        }
    }

    pub fn debug() {

    }

    pub fn info() {

    }

    pub fn warn() {

    }

    pub fn error() {

    }

    pub fn unrecoverable() {

    }

    pub fn log_arguments(&mut self, log_message: String, arguments: &[&str]) {
        self.log_simple(self.transform_string(log_message, arguments));
    }

    pub fn log_simple(&mut self, log_message: String) {
        if self.handler_list.is_empty() {
            self.handler_list.push(Box::new(ConsoleHandler::new()));
        }

        let unique_identifier = self.current_identifier;
        let record_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();

        let record = Record {
            identifier: unique_identifier,
            timestamp: record_time,
            message: log_message,
        };

        let iterator = self.handler_list.iter();
        for handler in iterator {
            handler.handle(&record);
        }

        self.record_list.push_back(record);
    }

    pub fn transform_string(&self, mut string: String, arguments: &[&str]) -> String {
        for argument in arguments {
            string = string.replacen("{}", argument, 1);
        }

        string.clone()
    }
}

pub struct Record {
    identifier: i64,
    timestamp: u128,
    message: String,
}

impl Record {
    pub fn new(new_identifier: i64, new_timestamp: u128, new_message: String) -> Record {
        Record {
            identifier: new_identifier,
            timestamp: new_timestamp,
            message: new_message,
        }
    }

    pub fn identifier(&self) -> i64 {
        self.identifier
    }

    pub fn timestamp(&self) -> u128 {
        self.timestamp
    }

    pub fn message(&self) -> String {
        self.message.clone()
    }
}

pub struct RecordType {
    weight: i32,
    prefix: String,
    //filter: LinkedList<?>
}