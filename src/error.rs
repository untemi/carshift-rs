use std::ops::Deref;

use axum::{
    extract::rejection::FormRejection,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};

use crate::templ;
use askama::Template;
use thiserror::Error;

pub type ServerResult<T> = Result<T, ServerError>;
pub type AnyError = anyhow::Error;

#[derive(Debug, Error)]
pub enum ServerError {
    #[error("{0}")]
    ValidationError(#[from] validator::ValidationErrors),

    #[error("invalid data")]
    AxumFormRejection(#[from] FormRejection),

    #[error("internal error")]
    InternalError(#[from] AnyError),

    #[error("{0}")]
    Encode(&'static str),
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        let self_box = Box::new(self);

        if let Self::InternalError(e) = self_box.deref() {
            eprintln!("Error: {}", e);
        }

        let template = templ::Alert {
            message: self_box.to_string(),
            level: templ::AlertLevel::Error,
        }
        .render()
        .unwrap_or_else(|_| "I failed you so bad".to_string());

        let html = Html(template);

        let res = (
            StatusCode::UNPROCESSABLE_ENTITY,
            [("HX-Reswap", "beforeend"), ("HX-Retarget", "#hxtoast")],
            html,
        );

        res.into_response()
    }
}
