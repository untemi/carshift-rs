use axum::{
    extract::{ConnectInfo, Request},
    http::StatusCode,
    middleware::Next,
    response::IntoResponse,
};
use chrono::Local;
use colored::*;
use std::net::SocketAddr;
use tokio::time::Instant;

fn colorize_status(status: StatusCode) -> String {
    let status_str = status.as_str();
    match status.as_u16() {
        100..=199 => status_str.blue(),
        200..=299 => status_str.green(),
        300..=399 => status_str.blue(),
        400..=499 => status_str.yellow(),
        500..=599 => status_str.bright_red(),
        _ => status_str.normal(),
    }
    .to_string()
}

pub async fn log_request(
    ConnectInfo(ip): ConnectInfo<SocketAddr>,
    req: Request,
    next: Next,
) -> impl IntoResponse {
    let method = req.method().clone();
    let uri = req.uri().clone();

    let start = Instant::now();
    let response = next.run(req).await;
    let duration = start.elapsed().as_micros();

    let status_code = response.status();
    tokio::spawn(async move {
        let status = colorize_status(status_code);
        let timestamp = Local::now()
            .format("%Y/%m/%d %H:%M:%S")
            .to_string()
            .magenta();
        println!(
            "{} {} {} from [{}] - {} in {}µs",
            timestamp, method, uri, ip, status, duration
        );
    });

    response
}
