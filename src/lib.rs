pub mod analyzer;
pub mod cli;
pub mod cli_handler;
pub mod collector;
pub mod config;
pub mod git_manager;
pub mod pipeline_detector;
pub mod report;
pub mod task_issues;
pub mod task_types;

pub use analyzer::analyze_pipelines;
pub use cli::Cli;
pub use cli_handler::handle_cli;
pub use collector::{CollectedTask, TaskImplementationCollector};
pub use config::{Config, Credentials, VersionCompare};
pub use git_manager::GitManager;
pub use pipeline_detector::find_pipeline_files;
pub use task_issues::TaskIssues;
pub use task_types::{SupportedTask, TaskImplementation, TaskValidState};
