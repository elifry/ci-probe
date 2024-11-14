# Bastion Labs CI Probe

Pipeline standardization tool for CI/CD pipelines written in yml.

If you have many repositories, it is often difficult to ensure that all the pipeline tasks are using the same version. `ciprobe` helps you inspect, validate, and report on the versions of all pipeline tasks globally across your repositories.

Currently only compatible with Azure but more support will come soon.

## Features

- Performs sparse checkouts of pipeline files to minimize network traffic and disk usage
- Supports authentication via environment variables, .env file, or command-line arguments
- Generates detailed Markdown reports of task usage and version discrepancies
- Special handling for GitVersion tasks, including setup, execute, and versionSpec configurations
- Parallel processing using tokio for improved performance
- Configurable valid states for tasks via `ciprobeconfig.yml`

## Usage

```bash
ciprobe --repos "repo1,repo2" [--credentials "username:token"] [--config path/to/config.yml] [--verbose]
```

### Authentication

Credentials can be provided in three ways (in order of precedence):

1. Command line argument: `--credentials "username:token"`
2. Environment variables: `AZURE_USERNAME` and `AZURE_TOKEN`
3. `.env` file containing `AZURE_USERNAME` and `AZURE_TOKEN`

### Configuration

Task validation rules are defined in `ciprobeconfig.yml`. This file specifies which task versions are considered valid for your organization.

## How it works

1. Authenticates with provided credentials
2. Performs sparse checkout of pipeline files (_.yml,_.yaml) from specified repositories
3. Analyzes pipeline files for task usage and versions
4. Generates a detailed report including:
   - Missing tasks
   - Invalid versions
   - Tasks without defined valid states
   - Detailed implementation locations

## Future work

- Expand support to any git provider with URL authentication
- Fix issue with repositories containing spaces in names

## Contributing

ci-probe is designed to be extensible. We welcome contributions for:

- Additional CI system support
- New task version validators
- Performance improvements
- Documentation enhancements

## License

MIT License - See LICENSE for details
