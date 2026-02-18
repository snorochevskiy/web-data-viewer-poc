use axum::{Json, extract::Path, http::StatusCode};

use crate::data_source::local_fs::{ListDirResponse, list_dir};

#[utoipa::path(get, path = "/dir/list{*dir}", summary = "List directory")]
pub async fn list_path(Path(dir): Path<String>) -> Result<Json<ListDirResponse>, (StatusCode,String)> {
    match list_dir(&dir).await {
        Ok(result) => Ok(Json(result)),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string()))
    }
}
