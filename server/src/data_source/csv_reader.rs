use std::{collections::{BTreeMap, HashMap}, fs::File, path::PathBuf};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value, to_string};

use crate::data_source::local_fs::{FS_ROOT, resolve_nested};

#[derive(Debug,Serialize,Deserialize)]
pub struct ColumnInfo {
    key: String,
    name: String,
}

impl ColumnInfo {
    pub fn new(key: &str, name: &str) -> ColumnInfo {
        ColumnInfo {
            key: key.to_string(),
            name: name.to_string()
        }
    }
}

impl From<&str> for ColumnInfo {
    fn from(value: &str) -> Self {
        ColumnInfo {
            key: value.to_string(),
            name: value.to_string(),
        }
    }
}

#[derive(Debug,Serialize,Deserialize)]
pub struct CsvTable {
    columns: Vec<ColumnInfo>,
    rows: Vec<Value>,
}

#[derive(Debug,Serialize,Deserialize)]
pub struct TableInfo {
    pub columns: Vec<ColumnInfo>,
}

#[derive(Debug,Serialize,Deserialize)]
pub struct DataRows {
    pub rows: Vec<Value>
}

pub fn read_csv_table(path: &str) -> anyhow::Result<CsvTable> {
    let root_path = PathBuf::from(FS_ROOT);
    let Some(absolute_path) = resolve_nested(path) else {
        anyhow::bail!("Invalid input path: {path}");
    };
    let target_path = root_path.join(&absolute_path);
    if !target_path.starts_with(root_path.as_path()) {
        anyhow::bail!("Forbidden target path: {path}");
    }
    let file = File::open(target_path)?;

    let mut rdr = csv::Reader::from_reader(file);
    let headers = rdr.headers()?.into_iter()
        .map(|h|h.to_string())
        .collect::<Vec<_>>();
    let mut rows = Vec::new();

    for result in rdr.records() {
        let record = result?;
        let map = headers.iter().zip(record.into_iter())
            .map(|(h,v)| (h.to_string(), v.to_string()))
            .fold(Map::new(), |mut m, (h, v)| {
                m.insert(h, Value::String(v));
                m
            });
        rows.push(Value::Object(map));
    }

    let columns = headers.into_iter()
        .map(|s| ColumnInfo::from(s.as_str()))
        .collect::<Vec<_>>();

    Ok(CsvTable { columns, rows })
}
