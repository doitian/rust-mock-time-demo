use std::time::SystemTime;

#[cfg(not(feature = "mock-time"))]
pub fn now() -> SystemTime {
    eprintln!("SystemTime::now()");
    SystemTime::now()
}

#[cfg(feature = "mock-time")]
pub mod mock_time {
    use super::*;
    use std::cell::RefCell;

    thread_local! {
        static MOCK_TIME: RefCell<Option<SystemTime>> = RefCell::new(None);
    }

    pub fn now() -> SystemTime {
        eprintln!("mock_time::now()");
        MOCK_TIME.with(|cell| {
            cell.borrow()
                .as_ref()
                .cloned()
                .unwrap_or_else(SystemTime::now)
        })
    }

    pub fn set_mock_time(time: Option<SystemTime>) {
        MOCK_TIME.with(|cell| *cell.borrow_mut() = time);
    }
}

#[cfg(feature = "mock-time")]
pub use mock_time::now;
