use anyhow::Result;
use std::path::PathBuf;
use walkdir::WalkDir;

pub async fn find_pipeline_files(repo_path: &PathBuf, verbose: bool) -> Result<Vec<PathBuf>> {
    if verbose {
        println!("Searching for pipeline files in {:?}", repo_path);
    }

    let pipeline_files: Vec<PathBuf> = WalkDir::new(repo_path)
        .follow_links(true)
        .into_iter()
        .filter_map(Result::ok)
        .map(|e| e.path().to_path_buf())
        .filter(|path| {
            path.extension()
                .map_or(false, |ext| ext == "yml" || ext == "yaml")
        })
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
