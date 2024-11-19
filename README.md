# CI Probe

Pipeline standardization tool for CI/CD pipelines written in yml.

## Overview

CI Probe helps you maintain consistency across your CI/CD pipelines by analyzing, validating, and reporting on task versions across all your repositories. It performs efficient sparse checkouts to analyze pipeline files and generates comprehensive reports about task usage and version discrepancies.

## Features

- **Smart Repository Handling**
  - Sparse checkout support for efficient pipeline file analysis
  - Automatic detection of default branches (main/master/develop)
  - Support for both new clones and repository updates

- **Flexible Authentication**
  - Command line arguments
  - Environment variables
  - .env file support
  - Secure credential handling

- **Comprehensive Analysis**
  - Task version validation
  - Missing task detection
  - Invalid version identification
  - Cross-repository task usage analysis

- **Detailed Reporting**
  - Markdown report generation
  - Repository analysis summary
  - Task version inconsistencies
  - Implementation locations
  - Skipped repositories tracking

## Installation

Download the latest release from the [releases page](https://github.com/bastionlabs/ciprobe/releases).

## Usage

```bash
ciprobe --repos "repo1,repo2" [--credentials "username:token"] [--config path/to/config.yml] [--verbose]
```

### Configuration

Create a `ciprobeconfig.yml` file to define valid task versions:

```yaml
task_versions:
  'UseNode':
    - '1'
  'gitversion/setup':
    - '3'
    - '2'
  'gitversion/execute':
    - '3'
    - '2'
```

You can add multiple versions to a task to support multiple task versions, if you want.

### Authentication

Credentials can be provided in order of precedence:

1. Command line: `--credentials "username:token"`
2. Environment: `AZURE_USERNAME` and `AZURE_TOKEN`
3. `.env` file: containing above environment variables

### Generated Report

The tool generates a detailed `report.md` containing:

- Total repositories analyzed
- Skipped repositories
- Task version analysis
- Missing tasks
- Invalid versions
- Implementation locations

## Technical Details

- Uses sparse checkout to minimize network traffic and disk usage
- Normalizes task names for consistent matching

## Requirements when Running the Binary

- Git command-line tool
- Access to repository URLs provided with the given credentials

## Contributing

Contributions welcome for:

- Additional CI platform support
- New task version validators
- Performance improvements
- Documentation enhancements
- Bug fixes

## License

MIT License - See LICENSE for details
