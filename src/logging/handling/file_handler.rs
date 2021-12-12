use std::fs::File;
use crate::logging::handling::handler::Handler;
use crate::logging::logger::{Logger, Record};

pub struct FileHandler;

impl Handler for FileHandler {
    fn handle(&self, record: &Record) {
        todo!()
    }
}

impl FileHandler {
    pub fn new() -> FileHandler {
        FileHandler
    }
}
