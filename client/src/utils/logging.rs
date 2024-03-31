use log::LevelFilter;
use log4rs::{
    append::console::ConsoleAppender,
    append::file::FileAppender,
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
};
use std::path::PathBuf;

pub fn setup(log_level: String) {
    // Define the pattern for the log messages.
    let pattern = "{d} {l} - {m}\n";

    // Create a console appender.
    let console_appender = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(pattern)))
        .build();

    // Define the path for the log file. Make sure the directory exists.
    let mut log_file_path = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    log_file_path.push(".teams_stauts_monitor");
    let date = chrono::Local::now().format("%Y-%m-%d");
    log_file_path.push(format!("app_{}.log", date));
    let file_appender = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(pattern)))
        .build(log_file_path)
        .expect("Failed to create file appender");

    // Convert the log level string to the appropriate enum. Default to Info if the conversion fails.
    let log_level_filter = match log_level.to_lowercase().as_str() {
        "error" => LevelFilter::Error,
        "warn" => LevelFilter::Warn,
        "info" => LevelFilter::Info,
        "debug" => LevelFilter::Debug,
        "trace" => LevelFilter::Trace,
        _ => LevelFilter::Info,
    };

    // Construct the log4rs config.
    let config = Config::builder()
        .appender(Appender::builder().build("console", Box::new(console_appender)))
        .appender(Appender::builder().build("file", Box::new(file_appender)))
        .build(
            Root::builder()
                .appender("console")
                .appender("file")
                .build(log_level_filter),
        )
        .expect("Failed to build config");

    // Initialize log4rs with the config.
    log4rs::init_config(config).expect("Failed to initialize log4rs");
}
