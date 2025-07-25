use axum::extract::Path;
use axum::response::{IntoResponse, Response};
use csutils::error::{ServerError, ServerResult};
use db::db_ref::FillDbRef;

mod add;

pub async fn display(Path(id): Path<i64>) -> ServerResult<Response> {
    let Some(mut car) = db::car::fetch_one(id).await? else {
        return Ok("not found".into_response());
    };

    car.owner.fill().await.map_err(ServerError::InternalError)?;
    templ::render(templ::DisplayCar { car: &car })
}
