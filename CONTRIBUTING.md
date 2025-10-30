# Contributing to int64grep

Thanks for wanting to contribute. These guidelines help keep the project organized and make reviews faster.

1) Reporting a bug or proposing a feature
- Open an issue with a clear description of the problem or the proposal.
- Include steps to reproduce, the expected output, and the actual output.

2) Work on a branch
- Fork the repository and create a descriptive branch name such as `feature/your-feature` or `fix/short-description`.

3) Formatting and linting
- Format your code with `rustfmt` before submitting a PR:

```powershell
cargo fmt
```

- Consider running `cargo clippy` to catch warnings and improve code quality:

```powershell
cargo clippy -- -D warnings
```

4) Tests
- Add or update unit tests when you add new functionality or fix bugs.
- Run tests locally before opening a PR:

```powershell
cargo test
```

5) Commits and PRs
- Keep commits small and focused, with clear messages. Examples: `fix: correct line counting` or `feat: add case-insensitive search`.
- In the PR description, explain what the change does, why it was made, and how to verify it.

6) Review
- PRs are accepted after tests pass and at least one reviewer approves.

Thanks for your contribution — small improvements matter!
