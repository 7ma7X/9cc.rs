mod util;
mod tokenizer;

use std::env;
use std::process;

use tokenizer::*;

fn main() {
  let argv: Vec<String> = env::args().collect();
  let argc = argv.len();

  if argc != 2 {
    eprintln!("引数の個数が正しくありません");
    process::exit(1);
  }

  let mut user_input = argv[1].to_string();
  let original = user_input.clone();
  let mut tokens = vec![Token::init(); 100];
  tokenize(&mut user_input, &original, &mut tokens);

  // println!("{:?}", tokens); // デバッグ用

  println!(".intel_syntax noprefix");
  println!(".global main");
  println!("main:");

  // 式の最初は数でなければならないので、それをチェックして
  // 最初のmov命令を出力
  if tokens[0].ty != Tk::Num {
    util::error_at(&tokens[0].input, &original, "数ではありません".to_string());
  }
  println!("    mov rax, {}", tokens[0].val);
  
  // `+ <数>`あるいは`- <数>`というトークンの並びを消費しつつ
  // アセンブリを出力
  let mut i = 1;

  while tokens[i].ty != Tk::EOF {
    if tokens[i].ty == Tk::Plus {
      i += 1;
      if tokens[i].ty != Tk::Num {
        util::error_at(&tokens[i].input, &original, "数ではありません".to_string());
      }
      println!("    add rax, {}", tokens[i].val);
      i += 1;
      continue;
    } 

    if tokens[i].ty == Tk::Minus {
      i += 1;
      if tokens[i].ty != Tk::Num {
        util::error_at(&tokens[i].input, &original, "数ではありません".to_string());
      }
      println!("    sub rax, {}", tokens[i].val);
      i += 1;
      continue;
    } 

    util::error_at(&tokens[i].input, &original, "予期しないトークンです".to_string());
  }

  println!("    ret");
}
