mod util;
mod lexer;
mod parser;
mod codegen;

use std::env;
use std::process;

use lexer::*;
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
  let mut tokens: Vec<Token> = Vec::new();
  tokenize(&mut user_input, &original, &mut tokens);

  // println!("{:?}", tokens); // デバッグ用

  let mut pos: usize = 0;
  let node: Node = Node::expr(&tokens, &mut pos, &original);

  // println!("{:#?}", node); // デバッグ用

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
