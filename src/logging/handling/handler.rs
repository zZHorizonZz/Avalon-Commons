use crate::logging::logger::{Logger, Record};

pub trait Handler {
    fn handle(&self, record: &Record);
}