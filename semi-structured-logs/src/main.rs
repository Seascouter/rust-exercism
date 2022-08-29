

enum LogLevel {
    Info,
    Warning,
    Error,
}

fn log(level: LogLevel, message: &str) -> String {
    let level = format!("{:?}", level);
    println!("[{}]: {}", level.to_uppercase(), message);
}

fn info(message: &str) -> String {
    log(LogLevel::Info, message)
}

fn warn(message: &str) -> String {
    log(LogLevel::Warning, message)
}

fn error(message: &str) -> String {
    log(LogLevel::Error, message)
}

