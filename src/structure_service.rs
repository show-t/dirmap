use std::path::Path;
use std::fs;
use serde_yaml::{ Value, Mapping };
use anyhow::Result;

use crate::macros::*;

pub(crate) fn create_from_value(base: &Path, val: &Value) -> Result<()> {
    tracing::debug!("{base:?}, {val:?}");

    if let Value::Mapping(map) = val {
        for (k, v) in map {
            let Some(name) = k.as_str() else {
                tracing::warn!("YAML key is not a string, skipping: {:?}", k);
                continue;
            };
            let path = base.join(name);
            match v {
                Value::Mapping(_) => {
                    try_with_log!(fs::create_dir_all(&path));
                    try_with_log!(create_from_value(&path, v));
                }
                Value::String(content) => {
                    try_with_log!(fs::write(&path, content));
                }
                Value::Null => {
                    try_with_log!(fs::write(&path, ""));
                }
                _ => {}
            }
        }
    }

    Ok(())
}

pub(crate) fn dir_to_yaml(path: &Path) -> Result<Value> {
    let root_name = path
        .file_name()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.display().to_string());

    let root_map = try_with_log!(dir_to_yaml_internal(path));

    let mut root = Mapping::new();
    root.insert(Value::String(root_name), root_map);

    Ok(Value::Mapping(root))
}

fn dir_to_yaml_internal(path: &Path) -> Result<Value> {
    let mut map = Mapping::new();

    for entry in try_with_log!(fs::read_dir(path)) {
        let entry = entry?;
        let name = entry.file_name().to_string_lossy().into_owned();
        let value = if entry.path().is_dir() {
            try_with_log!(dir_to_yaml_internal(&entry.path()))
        } else {
            Value::Null
        };
        map.insert(Value::String(name), value);
    }

    Ok(Value::Mapping(map))
}