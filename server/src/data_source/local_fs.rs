use std::{error::Error, path::{self, Path, PathBuf}};

use serde::{Deserialize, Serialize};

pub const FS_ROOT: &'static str = "/home/stas/dev/temp/root";

#[derive(Debug,Serialize,Deserialize)]
pub struct ListDirResponse {
    breadcrumbs: Vec<Breadcrumb>,
    entries: Vec<DirEntryInfo>,
}

#[derive(Debug,Serialize,Deserialize,PartialEq, PartialOrd)]
pub struct Breadcrumb {
    title: String,
    path: String,   
}

#[derive(Debug,Serialize,Deserialize)]
pub struct DirEntryInfo {
    pub path: String,
    pub is_file: bool,
}

pub async fn list_dir(path: &str) -> anyhow::Result<ListDirResponse> {
    let root_path = PathBuf::from(FS_ROOT);
    let Some(absolute_path) = resolve_nested(path) else {
        anyhow::bail!("Invalid input path: {path}");
    };
    let target_path = root_path.join(&absolute_path);
    if !target_path.starts_with(root_path.as_path()) {
        anyhow::bail!("Forbidden target path: {path}");
    }

    let breadcrumbs = build_breadcrumbs(&absolute_path);

    let mut entries = Vec::new();
    let mut dir_listing = tokio::fs::read_dir(target_path.as_path()).await?;
    while let Some(entry) = dir_listing.next_entry().await? {
        let entry_type = entry.file_type().await?;
        if !entry_type.is_dir() && !entry_type.is_file() {
            continue;
        }
        let entry_path = entry.path();
        let relative_path = entry_path.strip_prefix(root_path.as_path())?.to_str().unwrap().to_string();

        entries.push(DirEntryInfo {
            path: relative_path,
            is_file: entry_type.is_file(),
        });
    }

    Ok(ListDirResponse {breadcrumbs, entries})
}

pub fn build_breadcrumbs(path: impl AsRef<Path>) -> Vec<Breadcrumb> {
    let mut maybe_parent = path.as_ref().parent();

    let mut result = Vec::new();
    while let Some(parent) = maybe_parent {
        if let Some(p) = parent.to_str() {
            let title = parent.file_name()
                .and_then(|os_path| os_path.to_str().map(|s|s.to_string()))
                .unwrap_or("ROOT".to_string());
            result.push(Breadcrumb { title, path: p.to_string() });
        }
        maybe_parent = parent.parent();
    }
    result.reverse();
    result
}

pub fn resolve_nested(p: impl AsRef<Path>) -> Option<PathBuf> {
    let mut result = Vec::new();
    let components = p.as_ref().components();
    for c in components {
        match c {
            path::Component::Prefix(prefix_component) => result.push(c),
            path::Component::RootDir => (),
            path::Component::CurDir => (),
            path::Component::ParentDir => {
                let previous = result.pop()?;
                if let path::Component::RootDir = previous {
                    return None
                }
                if let path::Component::Prefix(_) = previous {
                    return None
                }
            },
            path::Component::Normal(os_str) => result.push(c),
        }
    }
    Some(result.into_iter().collect())
}

#[test]
fn test_build_breadcrumbs() {
    let path = PathBuf::from(format!("a/b/c"));
    assert_eq!(build_breadcrumbs(path), vec![
        Breadcrumb { title: "ROOT".to_string(), path: "".to_string() },
        Breadcrumb { title: "a".to_string(), path: "a".to_string() },
        Breadcrumb { title: "b".to_string(), path: "a/b".to_string() }
    ]);
}

#[test]
fn test_resolve_path() {
    println!("{:?}", resolve_nested("/home/stas/../dev/temp/root"));

    assert_eq!(resolve_nested("/"), Some(PathBuf::from("")));
    assert_eq!(resolve_nested("../home/stas/dev/temp/root"), None);
    assert_eq!(resolve_nested("/../home/stas/dev/temp/root"), None);
    assert_eq!(resolve_nested("/home/stas/../dev/./temp/root"), Some(PathBuf::from("home/dev/temp/root")));
}