[![CircleCI](https://circleci.com/gh/7ma7X/9cc.rs.svg?style=svg&circle-token=c0c8c4a5b71f9e0a44872393939759895bd98310)](https://circleci.com/gh/7ma7X/9cc.rs)

# 9cc.rs

C Compiler.  
cf. https://www.sigbus.info/compilerbook

## How to run

```bash
git clone https://github.com/7ma7X/9cc.rs.git 9cc
cd 9cc
docker build . -t rust-9cc
docker run -it --name rust-9cc -v $PWD:/9cc rust-9cc "/bin/bash"
cd 9cc
cargo run --release -- [args]
```