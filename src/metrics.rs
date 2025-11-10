use lazy_static::lazy_static;
use prometheus::{IntCounterVec, register_int_counter_vec};

lazy_static! {
    pub static ref HTTP_REQUEST_TOTAL: IntCounterVec = register_int_counter_vec!(
        "http_requests_total",
        "Total number of HTTP Request",
        &["method", "endpoint", "status"] // <- dimensons or tag or labels
        // these allow to slice and dice the metrics in different ways
        // HTTP_REQUESTS_TOTAL
        // .with_label_values(&["GET", "/api/v1/{slug}", "200"])
        // .inc();
    )
    .unwrap();
}
