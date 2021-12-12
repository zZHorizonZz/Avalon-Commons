mod logging;

#[cfg(test)]
mod tests {
    use crate::logging::logger::Logger;

    #[test]
    fn test_0() {
        let mut logger = Logger::new();
        logger.write_simple("sfbafiwaegfgbweaf".to_string());
        logger.write_line("sfbafiweaf{}".to_string(), &["test"]);
    }
}
