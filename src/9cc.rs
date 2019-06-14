mod util;

use std::env;
use std::process;

fn main() {
  let argv: Vec<String> = env::args().collect();
  let argc = argv.len();

  let mut p = argv[1].to_string();

  if argc != 2 {
    eprintln!("引数の個数が正しくありません");
    process::exit(1);
  }

  println!(".intel_syntax noprefix");
  println!(".global main");
  println!("main:");
  println!("    mov rax, {}", util::strtol(&mut p));

  while !(p.is_empty()) {
    if &(p.as_str())[..1] == "+" {
      p.remove(0);
      println!("    add rax, {}", util::strtol(&mut p));
      continue;
    } 

    if &(p.as_str())[..1] == "-" {
      p.remove(0);
      println!("    sub rax, {}", util::strtol(&mut p));
      continue;
    }

    eprintln!("予期しない文字です: {}", &(p.as_str())[..1]);
    process::exit(1);
  }

  println!("    ret");
}
