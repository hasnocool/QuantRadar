// QuantRadar machine-readable report writer.
use anyhow::Result;
use serde::Serialize;
use std::path::Path;
pub fn write_json<T:Serialize>(path:impl AsRef<Path>,value:&T)->Result<()> {if let Some(p)=path.as_ref().parent(){std::fs::create_dir_all(p)?;}std::fs::write(path,serde_json::to_vec_pretty(value)?)?;Ok(())}
