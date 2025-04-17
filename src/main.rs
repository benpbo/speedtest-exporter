use core::str;
use std::{net::SocketAddr, process::Command};

use log::{debug, error, info};
use prometheus_exporter_base::{
    prelude::{Authorization, ServerOptions},
    render_prometheus,
};
use response::Response;
use tokio_util::sync::CancellationToken;

mod render;
mod response;

fn perform_request() -> Result<String, RequestError> {
    info!("Running speedtest");
    let stdout = Command::new("speedtest")
        .arg("--format=json")
        .arg("--accept-license")
        .arg("--accept-gdpr")
        .output()
        .map_err(RequestError::Command)?
        .stdout;

    debug!("Parsing response");
    let response = serde_json::from_slice::<Response>(&stdout)
        .map_err(|error| RequestError::Parse(error, stdout))?;
    debug!("Got {response:?}");
    let rendred = render::render(response);
    Ok(rendred)
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    env_logger::init();

    let token = CancellationToken::new();
    let shutdown_token = token.clone();
    tokio::spawn(async move {
        if let Err(error) = tokio::signal::ctrl_c().await {
            error!("Failed to listen for shutdown signal {error}");
            shutdown_token.cancel();
            return;
        }

        info!("Received shutdown signal. Shutting down...");
        shutdown_token.cancel();
    });

    let addr: SocketAddr = ([0, 0, 0, 0], 9798).into();
    let server_options = ServerOptions {
        addr,
        authorization: Authorization::None,
    };

    tokio::select! {
        _ = token.cancelled() => {}
        _ = render_prometheus(server_options, (), |_request, _options| async {
            match perform_request() {
                Ok(result) => Ok(result),
                Err(error) => {
                    error!("{error}");
                    Err(Box::new(error).into())
                }
            }
        }) => {}
    }
}

#[derive(Debug)]
enum RequestError {
    Command(std::io::Error),
    Parse(serde_json::error::Error, Vec<u8>),
}

impl std::fmt::Display for RequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Command(inner) => write!(f, "Failed to execute speedtest command due to {inner}"),
            Self::Parse(inner, bytes) => match str::from_utf8(&bytes) {
                Ok(s) => write!(f, "Failed to parse command output: {s} {inner}"),
                Err(_) => write!(f, "Failed to parse command output: {bytes:#?} {inner}"),
            },
        }
    }
}

impl std::error::Error for RequestError {
    fn description(&self) -> &str {
        match self {
            Self::Command(_) => "Failed to execute speedtest command",
            Self::Parse(_, _) => "Failed to parse command output",
        }
    }
}
