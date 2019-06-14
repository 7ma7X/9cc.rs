use std::env;
use std::process;

fn main() {
  let argv: Vec<String> = env::args().collect();
  let argc = argv.len();

  if argc != 2 {
    eprintln!("引数の個数が正しくありません");
    process::exit(1);
  }

  println!(".intel_syntax noprefix");
  println!(".global main");
  println!("main:");
  println!("    mov rax, {}", argv[1]);
  println!("    ret");
}
