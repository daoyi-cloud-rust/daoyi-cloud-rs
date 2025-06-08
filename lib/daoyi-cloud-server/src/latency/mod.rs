#![allow(dead_code)]

use axum::http::Response;
use daoyi_cloud_logger::logger;
use std::fmt::{Display, Formatter};
use std::time::Duration;
use tower_http::trace::OnResponse;
use tracing::Span;

#[derive(Debug, Copy, Clone)]
pub struct LatencyOnResponse;

impl<B> OnResponse<B> for LatencyOnResponse {
    fn on_response(self, response: &Response<B>, latency: Duration, _span: &Span) {
        logger::info!(
            latency = %Latency(latency),
            status = response.status().as_u16(),
            "finished processing request"
        );
    }
}

struct Latency(Duration);

impl Display for Latency {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.0.as_millis() > 0 {
            write!(f, "{}ms", self.0.as_millis())
        } else {
            write!(f, "{}µs", self.0.as_micros())
        }
    }
}
