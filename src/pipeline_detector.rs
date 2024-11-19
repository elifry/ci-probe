use anyhow::Result;
use std::{fs, path::PathBuf};

pub fn find_pipeline_files(repo_path: &PathBuf, verbose: bool) -> Result<Vec<PathBuf>> {
    if verbose {
        println!("Searching for pipeline files in {:?}", repo_path);
    }

    let mut pipeline_files = Vec::new();
    find_yaml_files_recursive(repo_path, &mut pipeline_files)?;

    if verbose {
        for path in &pipeline_files {
            let stripped_path = path
                .strip_prefix(std::path::Path::new("C:\\repos\\ciprobe\\temp_repos"))
                .unwrap_or(path);
            let repo_name = repo_path.file_name().and_then(|n| n.to_str()).unwrap_or("");
            let final_path = stripped_path
                .strip_prefix(repo_name)
                .unwrap_or(stripped_path);
            println!("   └─ Found: {}", final_path.display());
        }
    }

    Ok(pipeline_files)
}

fn find_yaml_files_recursive(dir: &PathBuf, files: &mut Vec<PathBuf>) -> Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            find_yaml_files_recursive(&path, files)?;
        } else if let Some(ext) = path.extension() {
            if ext == "yml" || ext == "yaml" {
                files.push(path);
            }
        }
    }
    Ok(())
}
