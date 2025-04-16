use askama::Template;
use axum::{
    extract::{Form, FromRequest, Request, rejection::FormRejection},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};
use serde::de::DeserializeOwned;
use thiserror::Error;
use validator::Validate;

use crate::templ;

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedForm<T>(pub T);

impl<T, S> FromRequest<S> for ValidatedForm<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
    Form<T>: FromRequest<S, Rejection = FormRejection>,
{
    type Rejection = ServerError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Form(value) = Form::<T>::from_request(req, state).await?;
        value.validate()?;
        Ok(ValidatedForm(value))
    }
}

#[derive(Debug, Error)]
pub enum ServerError {
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),

    #[error(transparent)]
    AxumFormRejection(#[from] FormRejection),
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        let message = match self {
            Self::ValidationError(_) => self.to_string(),
            Self::AxumFormRejection(_) => "invalid data".to_string(),
        };

        let template = templ::Alert {
            message,
            level: templ::AlertLevel::Error,
        };

        let html = Html(template.render().unwrap());

        let res = (
            StatusCode::BAD_REQUEST,
            [("HX-Reswap", "beforeend"), ("HX-Retarget", "#hxtoast")],
            html,
        );

        res.into_response()
    }
}
