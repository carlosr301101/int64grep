# int64grep

int64grep is a small Rust utility and library that searches for strings inside text (similar to `grep`). The crate exposes search helper functions used by the package binary.

Key features
- Plain (case-sensitive) search
- Case-insensitive search helper
- Helpers to number/count search results

Status
- Example/learning project. Good for practicing unit testing in Rust and building small CLI utilities.

Repository layout
- `src/main.rs` — CLI binary that uses the crate functions
- `src/lib.rs` — search logic and unit tests

Requirements
- Rust toolchain (rustc and cargo). Use a recent stable toolchain.

How to build

In PowerShell (Windows):

```powershell
cargo build --release
```

How to run (examples)

From the project directory you can run the binary with arguments:

```powershell
# Run via cargo (pass program args after --)
cargo run  -- "query" "path\to\file.txt" 

# Or use the compiled release executable
.\target\release\int64grep "query" "path\\to\\file.txt"
```

Tests

Run the unit tests with:

```powershell
cargo test
```

Contributing

See `CONTRIBUTING.md` for contribution guidelines: how to open issues, submit PRs, formatting, and running tests.

License

This project is distributed under the MIT license — see the `LICENSE` file.

Contact

If you'd like to help improve the project, please open an issue or a pull request with a clear description and tests when appropriate.
