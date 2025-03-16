# Find Text in Text File

[![crates.io](https://img.shields.io/crates/v/findtext_textfile?label=latest)](https://crates.io/crates/findtext_textfile)
[![Documentation](https://docs.rs/findtext_textfile/badge.svg?version=latest)](https://docs.rs/findtext_textfile)
[![Dependency Status](https://deps.rs/crate/findtext_textfile/latest/status.svg)](https://deps.rs/crate/findtext_textfile)
[![Releases Workflow](https://github.com/nabbisen/findtext-textfile-rs/actions/workflows/release-executable.yaml/badge.svg)](https://github.com/nabbisen/findtext-textfile-rs/actions/workflows/release-executable.yaml)
[![License](https://img.shields.io/github/license/nabbisen/findtext-textfile-rs)](https://github.com/nabbisen/findtext-textfile-rs/blob/main/LICENSE)

## Summary

Search text in text file

## Development

First, add dependency:

```sh
cargo add findtext_textfile
```

Usage:

```rust
use findtext_textfile::search;

fn awesome_fn(keyword: &str, filepath: &str) {
    let ret = search(keyword, filepath);
}
```

## Executable Usage

Available in [Assets](https://github.com/nabbisen/findtext-textfile-rs/releases/latest) in Releases. Cross-platform supported.

```sh
findtext_textfile <keyword> <filepath>
# will print out like:
# [charset = UTF-8]
# (2, 1): hej
```
