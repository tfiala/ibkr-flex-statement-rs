use anyhow::Result;
use rstest::fixture;
use std::fs;
use std::path::PathBuf;

#[fixture]
pub fn sample_statement_paths() -> Result<Vec<PathBuf>> {
    let data_dir = std::env::current_dir()
        .unwrap()
        .join("tests")
        .join("fixtures")
        .join("data");

    let paths = data_dir
        .is_dir()
        .then(|| {
            fs::read_dir(data_dir)
                .unwrap()
                .filter_map(|entry| {
                    let path = entry.unwrap().path();
                    if path.extension().and_then(|s| s.to_str()) == Some("xml") {
                        Some(path)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    Ok(paths)
}
