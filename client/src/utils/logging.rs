use std::{fs::File, sync::Arc};
use tracing_subscriber::{filter, prelude::*};

/// app_name **must** match Cargo.toml's package.name, or else app-level logging won't work properly!
pub fn setup(log_level: String, app_name: impl AsRef<str>) {
    let stdout_log = tracing_subscriber::fmt::layer()
        .pretty()
        .event_format(tracing_subscriber::fmt::format::Format::default().compact());

    let log_file_path = format!(
        ".teams_stauts_monitor/{}_{}.log",
        app_name.as_ref(),
        chrono::Utc::now().format("%Y_%m_%d")
    );

    // A layer that logs events to a file.
    let file = File::create(log_file_path);
    let file = match file {
        Ok(file) => file,
        Err(error) => panic!("Error: {:?}", error),
    };
    let debug_log = tracing_subscriber::fmt::layer()
        .with_ansi(false)
        .with_file(true)
        .with_target(true)
        .with_line_number(true)
        .with_writer(Arc::new(file));

    // A layer that collects metrics using specific events.
    //let metrics_layer = /* ... */ filter::LevelFilter::DEBUG;

    // Convert the log level string to the appropriate enum. Default to Info if the conversion fails.
    let log_level_filter = match log_level.to_lowercase().as_str() {
        "error" => filter::LevelFilter::ERROR,
        "warn" => filter::LevelFilter::WARN,
        "info" => filter::LevelFilter::INFO,
        "debug" => filter::LevelFilter::DEBUG,
        "trace" => filter::LevelFilter::TRACE,
        _ => filter::LevelFilter::INFO,
    };

    let metrics_layer = /* ... */ log_level_filter;

    tracing_subscriber::registry()
        .with(
            stdout_log
                // Add an `INFO` filter to the stdout logging layer
                .with_filter(log_level_filter)
                // Combine the filtered `stdout_log` layer with the
                // `debug_log` layer, producing a new `Layered` layer.
                .and_then(debug_log)
                // Add a filter to *both* layers that rejects spans and
                // events whose targets start with `metrics`.
                .with_filter(filter::filter_fn(|metadata| {
                    !metadata.target().starts_with("metrics")
                })),
        )
        .with(
            // Add a filter to the metrics label that *only* enables
            // events whose targets start with `metrics`.
            metrics_layer.with_filter(filter::filter_fn(|metadata| {
                metadata.target().starts_with("metrics")
            })),
        )
        .init();
}
