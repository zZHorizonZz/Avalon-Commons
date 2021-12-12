use std::io;
use std::io::Write;

use crate::logging::handling::handler::Handler;
use crate::logging::logger::Record;

pub struct ConsoleHandler;

impl Handler for ConsoleHandler {

    fn handle(&self, record: &Record) {
        let stdout = io::stdout();
        let mut handle = stdout.lock();

        handle.write_all(record.message().as_ref());
    }
}

impl ConsoleHandler {
    pub fn new() -> Self {
        ConsoleHandler {}
    }
}
