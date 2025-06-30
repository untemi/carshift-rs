use axum::extract::{ConnectInfo, Request};
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::IntoResponse;

use chrono::Local;
use colored::*;
use std::net::SocketAddr;
use tokio::time::Instant;

fn colorize_status(status: StatusCode) -> ColoredString {
    let status_str = status.as_str();
    match status.as_u16() {
        100..200 => status_str.blue(),
        200..300 => status_str.green(),
        300..400 => status_str.blue(),
        400..500 => status_str.yellow(),
        500..600 => status_str.bright_red(),
        _ => status_str.normal(),
    }
}

fn clorize_duration(duration: u128) -> ColoredString {
    let duration_str = format!("{duration}µs");
    match duration {
        0..100 => duration_str.green(),
        100..200 => duration_str.blue(),
        200..500 => duration_str.yellow(),
        500..800 => duration_str.bright_yellow(),
        800.. => duration_str.red(),
    }
}

pub async fn log_request(
    ConnectInfo(ip): ConnectInfo<SocketAddr>,
    req: Request,
    next: Next,
) -> impl IntoResponse {
    let method = req.method().clone().as_str().blue();
    let uri = req.uri().clone();
    let start = Instant::now();

    let response = next.run(req).await;

    let duration = start.elapsed().as_micros();
    let status_code = response.status();

    tokio::spawn(async move {
        let fmt_duration = clorize_duration(duration);
        let status = colorize_status(status_code);
        let timestamp = Local::now().format("%Y/%m/%d %H:%M:%S").to_string().magenta();

        println!("{timestamp} {method} {uri} from [{ip}] - {status} in {fmt_duration}",);
    });

    response
}
