# stresstestcpu
Simple CPU stress-test in Rust

## To get started:
* **Download the latest revision**
```
git clone https://github.com/VHSgunzo/stresstestcpu.git && cd stresstestcpu
```
* **Compile a binary**
```
rustup default nightly
rustup target add x86_64-unknown-linux-musl
rustup component add rust-src --toolchain nightly
cargo build --release
```
* Or take an already precompiled binary file from the [releases](https://github.com/VHSgunzo/stresstestcpu/releases)

* **Usage**
```
./stresstestcpu --duration <DURATION> --threads <THREADS>

FLAGS:
-h, --help       Prints help information
-V, --version    Prints version information

OPTIONS:
-d, --duration <DURATION>    Duration of the test in seconds
-t, --threads <THREADS>      Number of threads to create
```
