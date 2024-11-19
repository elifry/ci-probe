use anyhow::Result;
use std::path::PathBuf;
use walkdir::WalkDir;

pub fn find_pipeline_files(repo_path: &PathBuf, verbose: bool) -> Result<Vec<PathBuf>> {
    if verbose {
        println!("Searching for pipeline files in {:?}", repo_path);
    }

    // Note: This assumes that the only files in the repo are yml/yaml files already
    let pipeline_files: Vec<PathBuf> = WalkDir::new(repo_path)
        .follow_links(true)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        .map(|e| e.path().to_path_buf())
        .collect();

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
