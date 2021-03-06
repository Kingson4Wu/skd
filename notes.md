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
`$ cargo install --path .`


---

### Blocking waiting for file lock on package cache
+ `rm ~/.cargo/.package-cache`

---


### linux
+ rustup target add x86_64-unknown-linux-musl
+ brew install filosottile/musl-cross/musl-cross
### windows
+ rustup target add x86_64-pc-windows-gnu
+ brew install mingw-w64  
+ vi ~/.cargo/config
<pre>
[target.x86_64-unknown-linux-musl]
linker = "x86_64-linux-musl-gcc"
[target.x86_64-pc-windows-gnu]
linker = "x86_64-w64-mingw32-gcc"
ar = "x86_64-w64-mingw32-gcc-ar"
</pre>

+ cargo build --release --target x86_64-unknown-linux-musl
+ cargo build --release --target x86_64-pc-windows-gnu
+ cargo build --release --target x86_64-apple-darwin


+ https://www.toolsqa.com/git/github-releases/

+ Rust交叉编译Mac编译Linux/Windows平台 :<https://www.cnblogs.com/007sx/p/15191400.html>

+ brew install hub
+ https://llever.com/hub-zh/hub-release.1.zh.html


+ cd ./target/release
+ zip -q -o skd-mac-bin.zip skd
+ cd ./target/x86_64-pc-windows-gnu/release
+ zip -q -o skd-windows-bin.zip skd.exe
+ cd ./target/x86_64-unknown-linux-musl/release
+ tar -zcvf skd-linux-bin.tar.gz skd

+ hub release create -a ./target/release/skd-mac-bin.zip -a ./target/x86_64-unknown-linux-musl/release/skd-linux-bin.tar.gz -a ./target/x86_64-pc-windows-gnu/release/skd-windows-bin.zip -m "release my first program" v0.0.1 (没成功，自己在界面上操作吧。。。)


+ cp ./target/x86_64-pc-windows-gnu/release/skd-windows-bin.zip ./target/release
+ cp ./target/x86_64-unknown-linux-musl/release/skd-linux-bin.tar.gz ./target/release

+ git tag v0.0.1
+ git push origin v0.0.1

+ https://github.com/Kingson4Wu/skd/releases/new