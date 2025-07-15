# daoyi-cloud-rs

环境变量：APP_ROOT=example/daoyi-cloud-demo/resources

```shell
#cargo build --release --target x86_64-pc-windows-gnu  # OKK
#
#cargo build --release --target x86_64-unknown-linux-musl
#
#cargo build --release --target x86_64-unknown-linux-gnu
```

```shell

cargo install cross --git https://github.com/cross-rs/cross

cross build --release --target x86_64-pc-windows-gnu  # OKK

cross build --release --target x86_64-unknown-linux-musl  # OKK
```