# simple-brightness

## Patch

Set valid paths for your system in src/main.rs

## Build

```bash
cargo build --release
sudo chown root:root ./target/release/brightness
sudo mv ./target/release/brightness /usr/local/bin/
```
