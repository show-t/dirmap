use std::path::Path;
use std::fs;
use serde_yaml::{ Value, Mapping };
use anyhow::Result;

use logkit_macros::try_with_log;

#[try_with_log]
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
                    fs::create_dir_all(&path)?;
                    create_from_value(&path, v)?;
                }
                Value::String(content) => {
                    fs::write(&path, content)?;
                }
                Value::Null => {
                    fs::write(&path, "")?;
                }
                _ => {}
            }
        }
    }

    Ok(())
}

#[try_with_log]
pub(crate) fn dir_to_yaml(path: &Path) -> Result<Value> {
    let root_name = path
        .file_name()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.display().to_string());

    let root_map = dir_to_yaml_internal(path)?;

    let mut root = Mapping::new();
    root.insert(Value::String(root_name), root_map);

    Ok(Value::Mapping(root))
}

#[try_with_log]
fn dir_to_yaml_internal(path: &Path) -> Result<Value> {
    let mut map = Mapping::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let name = entry.file_name().to_string_lossy().into_owned();
        let value = if entry.path().is_dir() {
            dir_to_yaml_internal(&entry.path())?
        } else {
            Value::Null
        };
        map.insert(Value::String(name), value);
    }

    Ok(Value::Mapping(map))
}