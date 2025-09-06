use crate::{hx_redirect, ALLOWED_PICTURE_TYPES};
use axum::{body::Bytes, response::Response, Extension};
use axum_typed_multipart::{FieldData, TryFromMultipart};
use chrono::NaiveDate;
use mw::LogginProps;
use std::fs;
use uuid::Uuid;
use validator::Validate;

use csutils::{
    error::{AnyError, ServerError, ServerResult},
    extractors::ValidatedUploadForm,
};

pub async fn page() -> ServerResult<Response> {
    templ::render(templ::AddCar {})
}

#[derive(TryFromMultipart, Validate)]
pub struct UploadData {
    #[validate(length(min = 10, max = 80, message = "should be between 10-80 characters"))]
    name: String,

    price: f64,
    start_date: NaiveDate,
    end_date: NaiveDate,
    district: i64,

    #[validate(length(min = 50, max = 300, message = "should be between 50-300 characters"))]
    description: Option<String>,

    #[form_data(limit = "3Mib")]
    image: FieldData<Bytes>,
}

#[axum::debug_handler]
pub async fn post(
    Extension(user): LogginProps,
    ValidatedUploadForm(form): ValidatedUploadForm<UploadData>,
) -> ServerResult<Response> {
    if form.start_date > form.end_date {
        return Err(ServerError::Encode("start date is greated than end date"));
    }

    // Picture
    // ref to content type
    let metadata = &form.image.metadata;
    let content_type = metadata
        .content_type
        .as_ref()
        .ok_or(ServerError::Encode("missing type"))?;

    // validating content type
    let is_allowed = ALLOWED_PICTURE_TYPES.iter().any(|t| content_type == t);
    if !is_allowed {
        return Err(ServerError::Encode("not valid MIME type"));
    };

    // building file name
    let file_name = {
        let uuid = Uuid::new_v4();
        let file_extention = content_type.split("/").nth(1).unwrap_or("png");
        format!("{uuid}.{file_extention}")
    };

    // saving file to disk
    let file_path = format!("pictures/car/{file_name}");
    fs::write(&file_path, &form.image.contents).map_err(AnyError::new)?;

    let id = db::car::add(
        form.name,
        form.price,
        form.start_date,
        form.end_date,
        user.id,
        form.district,
        form.description,
        file_path,
    )
    .await?;

    Ok(hx_redirect(&format!("/car/{id}")))
}
