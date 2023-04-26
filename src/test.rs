pub enum TestResult {
    Pass,
    Fail
}

#[macro_export]
macro_rules! test_all {
    ($($test:expr),+) => {
        use ansi_rgb::{ Background, Foreground, green, magenta_pink, black };

        let mut total = 0;
        let mut passed = 0;

        $({
            let test_name = stringify!($test);
            log::info!("Testing {}", test_name);
            
            match $test {
                $crate::test::TestResult::Pass => {
                    log::info!("{} {}", "[Passed]".fg(green()), test_name);
                    passed += 1;
                },
                $crate::test::TestResult::Fail => log::info!("{} {}", "[Failed]".fg(magenta_pink()), test_name)
            }
            total += 1;
        })+

        if total == passed {
            log::info!("{}", "  All tests passed  ".bg(green()).fg(black()))
        } else {
            log::info!("{}", "  Some tests failed  ".bg(magenta_pink()).fg(black()))
        }
    };
}