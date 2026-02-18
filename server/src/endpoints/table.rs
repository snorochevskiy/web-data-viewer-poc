use axum::{Json, extract::Path, http::StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::data_source::csv_reader::{ColumnInfo, CsvTable, DataRows, TableInfo, read_csv_table};

#[utoipa::path(get, path = "/csv-table/{*path}", summary = "Provide table metadata")]
pub async fn provide_csv_table_info(Path(path): Path<String>) -> Result<Json<CsvTable>, (StatusCode,String)> {
    match read_csv_table(&path) {
        Ok(csv_table) => Ok(Json(csv_table)),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}

#[utoipa::path(get, path = "/table/info", summary = "Provide table metadata")]
pub async fn provide_table_info() -> Json<TableInfo> {
    let colum_names = vec![
        ColumnInfo::new("id", "ID"),
        ColumnInfo::new("title", "Title"),
        ColumnInfo::new("product_type", "Product type"),
        ColumnInfo::new("price", "Price"),
        ColumnInfo::new("stock", "Stock"),
    ];

    Json(TableInfo {
        columns: colum_names,
    })
}

#[utoipa::path(get, path = "/table/rows", summary = "Provide table rows")]
pub async fn provide_table_rows() -> Json<DataRows> {
    let rows = vec![
        json!({"id": 1, "title": "Chair", "product_type": "furniture", "price": 500.0, "stock": 10}),
        json!({"id": 2, "title": "Table", "product_type": "furniture", "price": 200.0, "stock": 30}),
    ];

    Json(DataRows {
        rows: rows,
    })
}


