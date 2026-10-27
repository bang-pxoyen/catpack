# Contributing to catpack

Thank you for your interest in `catpack`! This document provides guidelines for building and releasing the project.

## Building from Source

### Prerequisites
- Rust 1.70+ ([rustup](https://rustup.rs/))
- Linux: `xclip` or `wl-clipboard` (for clipboard support)

### Local Build
```bash
git clone https://github.com/bang-pxoyen/catpack.git
cd catpack
cargo build --release
# Binary will be in target/release/catpack
```

## Cross-Compilation
We use [`cross`](https://github.com/cross-rs/cross) for reliable cross-compilation.

```bash
cargo install cross

# Linux (static binary)
cross build --release --target x86_64-unknown-linux-musl

# macOS (must be run on macOS)
cargo build --release --target x86_64-apple-darwin
cargo build --release --target aarch64-apple-darwin

# Windows
cross build --release --target x86_64-pc-windows-gnu
```

## Making a Release
1. Update the version in `Cargo.toml`.
2. Commit the change: `git commit -am "chore: bump version to v0.X.0"`
3. Create and push a tag: 
   ```bash
   git tag v0.X.0
   git push origin v0.X.0
   ```
4. The GitHub Actions workflow will automatically build binaries for all platforms and attach them to a new GitHub Release.
5. Publish to crates.io: `cargo publish`
