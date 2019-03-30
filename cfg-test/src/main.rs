mod lib;
use lib::now;

use std::thread;
use std::time::{Duration, SystemTime, SystemTimeError};

/// Just a function to demonstrate a feature that depends on time.
fn elapsed(since: SystemTime) -> Result<Duration, SystemTimeError> {
    now().duration_since(since)
}

fn main() {
    let start_time = now();
    thread::sleep(Duration::from_millis(10));
    println!(
        "Elapsed milliseconds: {}",
        elapsed(start_time)
            .expect("time never goes back")
            .as_millis()
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use lib::mock_time;

    #[test]
    fn test_elapsed() {
        let since = now();
        mock_time::set_mock_time(Some(since + Duration::from_millis(10)));
        let elapsed = elapsed(since).expect("time nerver goes back");
        assert_eq!(10, elapsed.as_millis());
    }
}
