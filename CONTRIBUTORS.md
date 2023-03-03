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

* Install [pre-commit](https://pre-commit.com/) via package manager or pip
* Install hooks
```shell
pre-commit install
```



# Release

### Before

Currently, use [commitizen](https://github.com/commitizen-tools/commitizen)

install tool:
```shell
sudo pip3 install -U Commitizen
```

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
