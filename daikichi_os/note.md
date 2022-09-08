## thumbv7em-none-eabihfは組み込みシステムを表す
    rustup target add thumbv7em-none-eabihf 

## リンカエラーを回避
    cargo build --target thumbv7em-none-eabihf

## コンパイル
[Rust nightly使った](https://doc.rust-jp.rs/book-ja/appendix-07-nightly-rust.html)
実験的機能が利用できるようになるけど、不安定だから将来のRustバージョンでは警告なしに削除されるかもしれん

    cargo build --target x86_64-daikichi_os.json # add json of setting

## ブートイメージを作る
    # in Cargo.toml
    
    [dependencies]
    bootloader = "0.9.8"

    cargo install bootimage
    rustup component add llvm-tools-preview
    cargo bootimage

## 実行
[qemu使った](https://www.qemu.org/)
    `brew install qemu`
    `qemu-system-x86_64 -drive format=raw,file=target/x86_64-daikichi_os/debug/bootimage-daikichi_os.bin`