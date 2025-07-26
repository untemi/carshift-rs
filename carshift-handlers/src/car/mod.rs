use csutils::error::{ServerError, ServerResult};
use db::FillDbRef;

use axum::extract::Path;
use axum::response::{IntoResponse, Response};

mod add;

pub async fn display(Path(id): Path<u64>) -> ServerResult<Response> {
    let Some(mut car) = db::car::fetch_one(id).await? else {
        return Ok("not found".into_response());
    };

    car.owner.fill().await.map_err(ServerError::InternalError)?;
    templ::render(templ::DisplayCar { car: &car })
}
