use csutils::error::*;
use csutils::extractors::UploadForm;
use mw::LogginProps;

use axum::body::Bytes;
use axum::response::Response;
use axum::Extension;
use axum_typed_multipart::{FieldData, TryFromMultipart};

use std::fs;
use uuid::Uuid;

const ALLOWED_TYPES: [&str; 3] = ["image/jpeg", "image/png", "image/gif"];

#[derive(TryFromMultipart)]
pub struct UploadData {
    #[form_data(limit = "3Mib")]
    pub image: FieldData<Bytes>,
}

pub async fn upload_picture(
    Extension(user): LogginProps,
    form: UploadForm<UploadData>,
) -> ServerResult<Response> {
    // ref to content type
    let metadata = &form.image.metadata;
    let content_type = metadata
        .content_type
        .as_ref()
        .ok_or(ServerError::Encode("missing type"))?;

    // validating content type
    let is_allowed = ALLOWED_TYPES.iter().any(|t| content_type == t);
    if !is_allowed {
        return Err(ServerError::Encode("not valid MIME type"));
    };

    // building file name
    let file_name = {
        let uuid = Uuid::new_v4();
        let file_extention = content_type.split("/").nth(1).unwrap_or("png");
        format!("{uuid}.{file_extention}")
    };

    // saving file to disk & db
    let file_path = format!("pictures/profile/{file_name}");
    fs::write(&file_path, &form.image.contents).map_err(AnyError::new)?;
    db::user::update_picture(user.id, file_path).await?;

    // removing old file
    if let Some(pfp_file) = &user.pfp_file {
        fs::remove_file(pfp_file).map_err(AnyError::new)?;
    }

    // horay
    templ::render(templ::Alert {
        level: templ::AlertLevel::Success,
        message: "updated".to_string(),
    })
}
