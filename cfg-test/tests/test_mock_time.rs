use cfg_test::now;

#[test]
fn test_mock_time() {
    // Enable this block using
    //
    //     RUSTFLAGS='--cfg cfg_test_crate_tests' cargo test -p cfg-test
    //
    // This generates following error:
    //
    // error[E0432]: unresolved import `cfg_test::mock_time`
    //   --> cfg-test/tests/test_mock_time.rs:5:9
    //    |
    // 18 |     use cfg_test::mock_time;
    //    |         ^^^^^^^^^^^^^^^^^^^ no `mock_time` in the root
    #[cfg(cfg_test_crate_tests)]
    {
        use cfg_test::mock_time;
        use std::time::SystemTime;
        mock_time::set_mock_time(Some(SystemTime::UNIX_EPOCH));
    }

    dbg!(now());
}
