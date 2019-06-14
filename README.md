# 9cc.rs

C Compiler.  
cf. https://www.sigbus.info/compilerbook

## How to run

```bash
docker build . -t rust-9cc
docker run -it --name rust-9cc -v $PWD:/9cc rust-9cc "/bin/bash"
cd 9cc
cargo run --release -- [...args]
```