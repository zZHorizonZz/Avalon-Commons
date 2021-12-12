use std::collections::LinkedList;
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

    pub fn write_line(&mut self, log_message: String, arguments: &[&str]) {
        self.write_new_line();
        self.write_simple(self.transform_string(log_message, arguments));
    }

    pub fn write_with_arguments(&mut self, log_message: String, arguments: &[&str]) {
        self.write_simple(self.transform_string(log_message, arguments));
    }

    pub fn write_simple(&mut self, log_message: String) {
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

    pub fn write_new_line(&mut self) {
        self.write_simple(String::from("\n"));
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