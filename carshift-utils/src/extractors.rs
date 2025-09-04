use axum::extract::rejection::FormRejection;
use axum::extract::{Form, FromRequest, Request};
use axum_typed_multipart::BaseMultipart;

use crate::error::ServerError;
use serde::de::DeserializeOwned;
use validator::Validate;

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedForm<T>(pub T);

impl<T, S> FromRequest<S> for ValidatedForm<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
    Form<T>: FromRequest<S, Rejection = FormRejection>,
{
    type Rejection = crate::error::ServerError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Form(value) = Form::<T>::from_request(req, state).await?;
        value.validate()?;
        Ok(ValidatedForm(value))
    }
}

#[derive(Debug)]
pub struct ValidatedUploadForm<T>(pub T);

impl<T, S> FromRequest<S> for ValidatedUploadForm<T>
where
    T: Validate,
    S: Send + Sync,
    BaseMultipart<T, ServerError>: FromRequest<S, Rejection = ServerError>,
{
    type Rejection = ServerError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let upload_form = BaseMultipart::<T, ServerError>::from_request(req, state).await?;
        upload_form.data.validate()?;
        Ok(ValidatedUploadForm(upload_form.data))
    }
}

pub type UploadForm<T> = BaseMultipart<T, ServerError>;
