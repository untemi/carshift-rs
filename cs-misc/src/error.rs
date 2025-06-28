use axum::extract::rejection::FormRejection;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};
use axum_typed_multipart::TypedMultipartError;

use std::ops::Deref;
use thiserror::Error;

pub type ServerResult<T> = Result<T, ServerError>;
pub type AnyError = anyhow::Error;

#[derive(Debug, Error)]
pub enum ServerError {
    #[error("{0}")]
    ValidationError(#[from] validator::ValidationErrors),

    #[error("invalid data")]
    AxumFormRejection(#[from] FormRejection),

    #[error("{0}")]
    UploadError(#[from] TypedMultipartError),

    #[error("internal error")]
    InternalError(#[from] AnyError),

    #[error("{0}")]
    Encode(&'static str),
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        let self_box = Box::new(self);

        if let Self::InternalError(e) = self_box.deref() {
            eprintln!("Error: {e}");
        }

        // HACK: the goddamn cycle
        let template = format!(
            r#"<div role="alert" remove-me="4s" class="alert alert-error alert-soft"><pre>{}</pre></div>"#,
            self_box,
        );

        let html = Html(template);

        let res = (
            StatusCode::UNPROCESSABLE_ENTITY,
            [("HX-Reswap", "beforeend"), ("HX-Retarget", "#hxtoast")],
            html,
        );

        res.into_response()
    }
}
