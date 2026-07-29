# Contributing to Flux

First off, thanks for taking the time to contribute! 🎉

Flux is a community-driven project, and we love your input. Whether it's a bug report, feature request, or code contribution, we value your help.

## How Can I Contribute?

### Reporting Bugs
- Use the [GitHub Issue Tracker](https://github.com/NotZenith/flux/issues).
- Provide a clear description and steps to reproduce.

### Suggesting Enhancements
- Check if the feature has already been suggested.
- Explain why the enhancement would be useful to most users.

### Pull Requests
1. Fork the repo and create your branch from `main`.
2. If you've added code that should be tested, add tests.
3. If you've changed APIs, update the documentation.
4. Ensure the test suite passes (`cargo test` and `./gradlew test`).
5. Make sure your code lints.

## Development Setup

### Rust Core
1. Install Rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
2. Build: `cargo build`

### Desktop UI
1. Install JDK 21.
2. Build: `./gradlew build`
3. Run: `./gradlew run`

## Code of Conduct
Please be kind and respectful to all contributors.

## License
By contributing, you agree that your contributions will be licensed under the MIT License.
