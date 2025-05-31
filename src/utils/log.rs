#[macro_export]
macro_rules! log {
    ($($txt:tt)*) => {
        println!("[wCloner] [LOG] - {}", format!($($txt)*));
    };
}

#[macro_export]
macro_rules! debug {
    ($($txt:tt)*) => {
        println!("[wCloner] [DEBUG] - {}", format!($($txt)*));
    };
}
