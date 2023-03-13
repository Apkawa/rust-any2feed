If you want to learn the rust programming language,
and you are interested in the project, then you can join.

# Build from source

## Requirements

already installed `cargo` via `rustup`

## Build
```
git clone https://github.com/Apkawa/rust-any2feed.git
cd rust-any2feed
cargo run --release --bin any2feed -- ./any2feed_config.toml
```

### Crosscompile build

* Install mingw
```shell
sudo apt install mingw-w64
```
* add target
```shell
rustup target add x86_64-pc-windows-gnu
```
* Build
```shell
cargo build --release --target x86_64-pc-windows-gnu --bin any2feed
```
* Locate file in `./target/x86_64-pc-windows-gnu/release/any2feed.exe`

# Commit

* Install [pre-commit](https://pre-commit.com/) via package manager or `pip`
* Install hooks
```shell
pre-commit install
```

# Checks

## pre

install clippy:
```shell
rustup component add clippy
```

## run

* `cargo fmt --fix` - fix formatting
* `cargo check`
* `cargo clippy --fix` - lint and fix lint issues
* `cargo test --all` - run all tests

Also, this checks run in git hoots

# Release

### Before

Currently, use [commitizen](https://github.com/commitizen-tools/commitizen)

Install tool:
```shell
sudo pip install -U git+https://github.com/Apkawa/commitizen@dev#egg=commitizen
```
We are using a fork because the original is not compatible with semver.

After my [patch](https://github.com/commitizen-tools/commitizen/pull/686) is accepted, it will be possible to use =)

### Release
```shell
cz bump --check-consistency --no-verify
```

### Prelease

```shell
cz bump --check-consistency --no-verify --prerelease alpha
```

## Update changelog

```shell
cz changelog
```
