use crate::logging::handling::handler::Handler;
use crate::logging::logger::Record;

pub struct FileHandler;

impl Handler for FileHandler {
    fn handle(&self, record: &Record) {
        todo!()
    }
}

impl FileHandler {
    pub fn new() -> Self {
        FileHandler {}
    }
}
