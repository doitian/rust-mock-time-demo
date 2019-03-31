fn main() {
    println!("Hello, world!");
}

#[cfg(all(test, feature = "mock-time"))]
mod tests {
    use cfg_feature_lib::{mock_time::set_mock_time, now};
    use std::time::{Duration, SystemTime, SystemTimeError};

    /// Just a function to demonstrate a feature that depends on time.
    fn elapsed(since: SystemTime) -> Result<Duration, SystemTimeError> {
        now().duration_since(since)
    }

    #[test]
    fn test_elapsed() {
        let since = now();
        set_mock_time(Some(since + Duration::from_millis(10)));
        let elapsed = elapsed(since).expect("time nerver goes back");
        assert_eq!(10, elapsed.as_millis());
    }
}
