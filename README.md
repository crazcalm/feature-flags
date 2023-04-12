# feature-flags
Feature flag service in Rust

## Code Coverage
See [tarpaulin](https://github.com/xd009642/tarpaulin) for installation instructions.

**NOTE:** This depends on the [openssl crate](https://docs.rs/openssl/0.10.29/openssl/#automatic), which has its own list dependencies that need to be installed.

### How to Run
```
cargo tarpaulin
```

## Pre-Commit Hooks
See [pre-commit website](https://pre-commit.com/) for installation instructions.

### Install hooks locally
```
pre-commit install
```

### Rust Hooks
Documentation for the hooks I am using can be found [here](https://github.com/doublify/pre-commit-rust).
