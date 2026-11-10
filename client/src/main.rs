mod utils;

use std::time::Duration;

use log::{debug, error, info};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
#[cfg(not(target_os = "macos"))]
use {futures::stream::StreamExt, log::trace, serde_json::Value, url::Url};

const APP_NAME: &str = "Teams Meeting Monitor";
const APP_VERSION: &str = "1.0.0";
const AUTHOR: &str = "FawenYo";

struct Config {
    user_name: String,
    user_icon: String,
    server_url: String,
    log_level: String,
}

#[tokio::main]
async fn main() {
    let config = parse_args();
    // Argument parsing - username
    let user_name = config.user_name;
    // Argument parsing - user icon
    let user_icon = config.user_icon;
    // Argument parsing - server URL
    let server_url = config.server_url;
    // Argument parsing - log level
    let log_level = config.log_level;
    let first_open = true;

    // Set up the logger
    let _ = utils::logging::setup(log_level, APP_NAME);

    // Initialize the previous meeting status
    let mut previous_meeting_status = false;
    let meeting_in_progress_flag = Arc::new(AtomicBool::new(false));
    let mut notify_task: Option<tokio::task::JoinHandle<()>> = None;
    let (tx, mut rx) = tokio::sync::mpsc::channel(32);

    // Teams on macOS no longer starts the local 8124 API reliably, so read its power assertion instead
    #[cfg(target_os = "macos")]
    tokio::spawn(async move {
        pmset_monitoring(tx).await;
    });

    #[cfg(not(target_os = "macos"))]
    {
        let ws_url = format!("ws://localhost:8124?protocol-version=2.0.0&manufacturer={}&device=teams_meeting_monitor&app={}&app-version={}", AUTHOR, APP_NAME, APP_VERSION);
        tokio::spawn(async move {
            meeting_monitoring(ws_url, tx).await;
        });
    }

    // Handle received messages (indications of meeting status)
    while let Some(meeting_in_progress) = rx.recv().await {
        if meeting_in_progress {
            meeting_in_progress_flag.store(true, Ordering::SeqCst);

            // If is the first time the meeting is in progress
            if !previous_meeting_status {
                info!("Meeting started");
                // If the server URL is provided, notify the server
                if server_url != "" {
                    // Clone the variables to pass to the async block
                    let server_url = server_url.clone();
                    let user_name = user_name.clone();
                    let user_icon = user_icon.clone();

                    // Notify the server every minute
                    let flag_clone = meeting_in_progress_flag.clone();
                    notify_task = Some(tokio::spawn(async move {
                        loop {
                            if !flag_clone.load(Ordering::SeqCst) {
                                // If the flag is false, break the loop
                                break;
                            }

                            if let Err(e) =
                                notify_server(&user_name, &user_icon, &server_url, true).await
                            {
                                error!("Failed to notify server: {}", e);
                            }
                            tokio::time::sleep(Duration::from_secs(60)).await; // Wait for one minute
                        }
                    }));
                }
            }
            // Update the previous meeting status
            previous_meeting_status = true;
        } else {
            meeting_in_progress_flag.store(false, Ordering::SeqCst);
            // The flag alone is only seen after the 60s sleep; abort so a quick end/start cannot leave two loops
            if let Some(task) = notify_task.take() {
                task.abort();
            }

            let can_notify = previous_meeting_status || first_open;
            if previous_meeting_status {
                info!("Meeting ended");
            } else if first_open {
                info!("No meeting in progress");
            }
            // If the server URL is provided, notify the server
            if server_url != "" && can_notify {
                // Call the function to notify the server
                if let Err(e) = notify_server(&user_name, &user_icon, &server_url, false).await {
                    error!("Failed to notify server: {}", e);
                }
            }

            // Update the previous meeting status
            previous_meeting_status = false;
        }
    }
}

fn parse_args() -> Config {
    let matches = clap::Command::new(APP_NAME)
        .version(APP_VERSION)
        .author(AUTHOR)
        .about("Monitors meeting status and notifies a server")
        .arg(
            clap::Arg::new("username")
                .short('u')
                .long("username")
                .value_name("USERNAME")
                .help("Sets a username. If not provided, the username will call 'user'.")
                .default_value("user")
                .value_parser(clap::value_parser!(String)),
        )
        .arg(
            clap::Arg::new("usericon")
                .short('i')
                .long("usericon")
                .value_name("USERICON")
                .help("Sets a user icon. If not provided, the user icon will call 'user'.")
                .default_value("https://cdn.iconscout.com/icon/free/png-256/avatar-370-456322.png")
                .value_parser(clap::value_parser!(String)),
        )
        .arg(
            clap::Arg::new("server")
                .short('s')
                .long("server")
                .value_name("SERVER")
                .help("Sets a custom server to notify. If not provided, the server will not be notified.")
                .default_value("")
                .value_parser(clap::value_parser!(String)),
        )
        .arg(
            clap::Arg::new("log-level")
                .short('l')
                .long("log-level")
                .value_name("LOG_LEVEL")
                .help("Sets a log level")
                .default_value("INFO")
                .value_parser(clap::builder::PossibleValuesParser::new(&[
                    "TRACE", "DEBUG", "INFO", "WARN", "ERROR",
                ])),
        )
        .get_matches();

    Config {
        user_name: matches.get_one::<String>("username").unwrap().clone(),
        user_icon: matches.get_one::<String>("usericon").unwrap().clone(),
        server_url: matches.get_one::<String>("server").unwrap().clone(),
        log_level: matches.get_one::<String>("log-level").unwrap().clone(),
    }
}

#[cfg(target_os = "macos")]
const PMSET_POLL_INTERVAL: Duration = Duration::from_secs(10);

#[cfg(target_os = "macos")]
async fn pmset_monitoring(tx: tokio::sync::mpsc::Sender<bool>) {
    let mut last_status: Option<bool> = None;
    loop {
        match tokio::process::Command::new("/usr/bin/pmset")
            .args(["-g", "assertions"])
            .output()
            .await
        {
            Ok(output) if output.status.success() => {
                let Some(in_call) = utils::assertions::teams_call_in_progress(
                    &String::from_utf8_lossy(&output.stdout),
                ) else {
                    error!("Unexpected pmset output, keeping previous status");
                    tokio::time::sleep(PMSET_POLL_INTERVAL).await;
                    continue;
                };
                debug!("Teams call in progress: {}", in_call);
                // Send only on change so the main loop does not re-notify on every poll
                if last_status != Some(in_call) {
                    if tx.send(in_call).await.is_err() {
                        error!("Failed to send meeting status update.");
                        return;
                    }
                    last_status = Some(in_call);
                }
            }
            Ok(output) => error!("pmset exited with {}", output.status),
            Err(e) => error!("Failed to run pmset: {}", e),
        }
        tokio::time::sleep(PMSET_POLL_INTERVAL).await;
    }
}

#[cfg(not(target_os = "macos"))]
async fn meeting_monitoring(ws_url: String, tx: tokio::sync::mpsc::Sender<bool>) {
    loop {
        let url = Url::parse(&ws_url).expect("Failed to parse URL");

        match tokio_tungstenite::connect_async(url).await {
            Ok((ws_stream, _)) => {
                debug!("WebSocket connection established");
                let (_write, mut read) = ws_stream.split();

                while let Some(message) = read.next().await {
                    match message {
                        Ok(msg) => {
                            if let tokio_tungstenite::tungstenite::protocol::Message::Text(text) =
                                msg
                            {
                                trace!("Received: {}", text);
                                let json: Value =
                                    serde_json::from_str(&text).unwrap_or_else(|_| Value::Null);
                                if let Some(can_pair) =
                                    json["meetingUpdate"]["meetingPermissions"]["canPair"].as_bool()
                                {
                                    // Send the meeting status to the main task
                                    if tx.send(can_pair).await.is_err() {
                                        error!("Failed to send meeting status update.");
                                        break;
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            error!("Error: {}", e);
                            break; // Exit the reading loop to attempt reconnection
                        }
                    }
                }
            }
            Err(e) => {
                error!(
                    "Failed to connect: {}. Attempting to reconnect in 1 minute...",
                    e
                );
            }
        }

        tokio::time::sleep(Duration::from_secs(60)).await;
    }
}

async fn notify_server(
    user_name: &String,
    user_icon: &String,
    server_url: &str,
    in_meeting: bool,
) -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();
    let res = client
        .post(server_url)
        .json(&serde_json::json!({"user": user_name, "user_icon_url": user_icon, "meeting_status": in_meeting}))
        .send()
        .await?;

    if res.status().is_success() {
        info!("Successfully notified the server.");
    } else {
        error!("Failed to notify the server.");
    }

    Ok(())
}
