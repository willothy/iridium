
macro_rules! log {
    ($s: expr) => {
        if std::env::var("VERBOSE").is_ok() {
            println!("{}", $s);
        }
    };
}

pub(crate) use log;