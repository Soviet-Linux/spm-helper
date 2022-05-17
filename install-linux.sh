#curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo build --release
sudo mv target/release/spm-helper /usr/bin/spm-helper