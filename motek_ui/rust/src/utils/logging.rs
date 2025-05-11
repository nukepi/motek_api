use std::sync::Mutex;
use once_cell::sync::Lazy;

// Zmiana typu LogCallback na Box<dyn Fn(&str, &str) + Send + 'static>
pub type LogCallback = Box<dyn Fn(&str, &str) + Send + 'static>;

static LOG_CALLBACK: Lazy<Mutex<Option<LogCallback>>> = Lazy::new(|| Mutex::new(None));

pub fn set_log_callback<F>(callback: F) 
where 
    F: Fn(&str, &str) + Send + 'static 
{
    let mut cb = LOG_CALLBACK.lock().unwrap();
    *cb = Some(Box::new(callback));
}

pub fn log(level: &str, message: &str) {
    if let Ok(cb) = LOG_CALLBACK.lock() {
        if let Some(callback) = &*cb {
            callback(level, message);
        }
    }
}

#[macro_export]
macro_rules! rust_log {
    ($level:expr, $($arg:tt)*) => {
        $crate::utils::logging::log($level, &format!($($arg)*));
    };
}

#[macro_export]
macro_rules! rust_trace {
    ($($arg:tt)*) => {
        $crate::rust_log!("TRACE", $($arg)*);
    };
}

#[macro_export]
macro_rules! rust_debug {
    ($($arg:tt)*) => {
        $crate::rust_log!("DEBUG", $($arg)*);
    };
}

#[macro_export]
macro_rules! rust_info {
    ($($arg:tt)*) => {
        $crate::rust_log!("INFO", $($arg)*);
    };
}

#[macro_export]
macro_rules! rust_warn {
    ($($arg:tt)*) => {
        $crate::rust_log!("WARN", $($arg)*);
    };
}

#[macro_export]
macro_rules! rust_error {
    ($($arg:tt)*) => {
        $crate::rust_log!("ERROR", $($arg)*);
    };
}
