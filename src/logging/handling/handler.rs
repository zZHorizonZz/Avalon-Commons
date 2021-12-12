use crate::logging::logger::Record;

pub trait Handler {
    fn handle(&self, record: &Record);
}