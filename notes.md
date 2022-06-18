+ cargo new skd
+ cargo build 
+ cargo run 

+ 处理参数的解析过程的 Rust 工具 Clap。

+ cargo run -- -h
+ cargo run -- -V
+ cargo run --
+ cargo run -- readme.md


+ cargo build --release
./target/release

+ 将这个文件复制到你的 PATH 环境变量中，或者使用一个 cargo 命令来自动安装。应用程序将安装在 ~/.cargo/bin/ 目录中（确保该目录在 ~/.bashrc 或 ~/.zshrc 的 PATH 环境变量中）。
$ cargo install --path .
