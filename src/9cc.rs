mod util;
mod tokenizer;
mod parser;

use std::env;
use std::process;

use tokenizer::*;
use parser::*;

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

  let mut pos: usize = 0;
  let node: Node = Node::expr(&tokens, &mut pos, &original);

  // println!("{:?}", tokens); // デバッグ用

  println!(".intel_syntax noprefix");
  println!(".global main");
  println!("main:");
  
  // 抽象構文木を下りながらコード生成
  node.gen();

  // スタックトップに式全体の値が残っているはずなので
  // それをRAXにロードして関数からの返り値とする
  println!("    pop rax");
  println!("    ret");
}
