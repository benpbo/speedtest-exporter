use std::{net::SocketAddr, process::Command};

use log::{debug, info};
use prometheus_exporter_base::{
    prelude::{Authorization, ServerOptions},
    render_prometheus,
};
use response::Response;

mod render;
mod response;

fn perform_request() -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    info!("Running speedtest");
    let output = Command::new("speedtest")
        .arg("--format=json")
        .arg("--accept-license")
        .arg("--accept-gdpr")
        .output()
        .expect("Failed to execute 'speedtest'");

    debug!("Parsing response");
    let response: Response = serde_json::from_slice(&output.stdout).unwrap();
    debug!("Got {response:?}");
    let rendered = render::render(response);

    Ok(rendered)
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    env_logger::init();

    let addr: SocketAddr = ([0, 0, 0, 0], 9798).into();
    let server_options = ServerOptions {
        addr,
        authorization: Authorization::None,
    };

    render_prometheus(server_options, (), |_request, _options| async {
        perform_request()
    })
    .await;
}
