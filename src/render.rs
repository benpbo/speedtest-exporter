use num::Num;
use prometheus_exporter_base::{MetricType, MissingValue, PrometheusInstance, PrometheusMetric};

use crate::response::Response;

macro_rules! create_metric {
    ($instance:expr, $name:literal, $help:expr, $value:expr) => {
        PrometheusMetric::build()
            .with_name(concat!("speedtest_", $name))
            .with_metric_type(MetricType::Gauge)
            .with_help($help)
            .build()
            .render_and_append_instance(&$instance.clone().with_value($value))
            .render()
    };
}

pub fn render(response: Response) -> String {
    let mut f64instance: PrometheusInstance<'_, f64, MissingValue> = PrometheusInstance::new();
    let mut i64instance: PrometheusInstance<'_, i64, MissingValue> = PrometheusInstance::new();
    match response {
        Response::Result {
            timestamp,
            ping,
            download,
            upload,
            isp,
            interface,
            server,
            result,
        } => {
            let timestamp = timestamp.timestamp_millis().try_into().unwrap();
            let server_id = server.id.to_string();
            let labels = vec![
                ("isp", isp.as_str()),
                ("interface_external_ip", interface.external_ip.as_str()),
                ("server_id", server_id.as_str()),
                ("server_name", server.name.as_str()),
                ("result_id", result.id.as_str()),
                ("result_url", result.url.as_str()),
            ];

            f64instance = setup_instance(f64instance, timestamp, labels.clone());
            i64instance = setup_instance(i64instance, timestamp, labels.clone());

            let metrics = vec![
                create_metric!(
                    f64instance,
                    "ping_jitter_milliseconds",
                    "Ping jitter in milliseconds",
                    ping.jitter
                ),
                create_metric!(
                    f64instance,
                    "ping_latency_milliseconds",
                    "Ping latency in milliseconds",
                    ping.latency
                ),
                create_metric!(
                    i64instance,
                    "download_speed_bytes_per_second",
                    "Download speed in bytes per second",
                    download.bytes
                ),
                create_metric!(
                    i64instance,
                    "upload_speed_bytes_per_second",
                    "Upload speed in bytes per second",
                    upload.bytes
                ),
            ];

            metrics.join("\n")
        }
    }
}

fn setup_instance<
    'a,
    N: Num + std::fmt::Display + std::fmt::Debug,
    L: Into<&'a str>,
    V: Into<&'a str>,
>(
    mut instance: PrometheusInstance<'a, N, MissingValue>,
    timestamp: u128,
    labels: Vec<(L, V)>,
) -> PrometheusInstance<'a, N, MissingValue> {
    instance = instance.with_timestamp(timestamp);
    for (key, value) in labels {
        instance = instance.with_label(key, value);
    }

    instance
}
