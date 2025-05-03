use once_cell::sync::Lazy;
use std::sync::Once;

static INIT_LOGGER: Lazy<Once> = Lazy::new(Once::new);

pub fn init_logger() {
    INIT_LOGGER.call_once(|| {
        env_logger::builder()
            .target(env_logger::Target::Stdout)
            .filter_level(log::LevelFilter::Trace)
            .is_test(cfg!(test))
            .init();
    });
}

#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => {{
        $crate::kumpun::utils::logger::init_logger();
        log::debug!($($arg)*);
    }};
}

#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {{
        $crate::kumpun::utils::logger::init_logger();
        log::error!($($arg)*);
    }};
}
