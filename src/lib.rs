mod logging;

#[cfg(test)]
mod tests {
    use crate::logging::logger::Logger;

    #[test]
    fn test_0() {
        let mut logger = Logger::new();
        logger.log_simple("sfbafiwaegfgbweaf".to_string());
        logger.log_arguments("sfbafiweaf{}".to_string(), &["test"]);
    }
}
