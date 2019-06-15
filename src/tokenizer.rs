use crate::util::*;

// トークンの型を表す
#[derive(Clone, PartialEq, Debug)]
pub enum Tk {
  Plus, // '+'
  Minus, // '-'
  Num, //数値
  EOF
}

#[derive(Clone, Debug)]
pub struct Token {
  pub ty: Tk,
  pub val: i32,
  pub input: String,
}

impl Token {
  pub fn init() -> Token {
    Token { ty: Tk::EOF, val: 0, input: "".to_string() }
  }
}

/**
 * user_inputが指している文字列をトークンに分割してtokensに保存する
 */
pub fn tokenize(user_input: &mut String, original: &String, tokens: &mut Vec<Token>) {
  let p = user_input;

  let mut i = 0;
  while !(p.is_empty()) {
    let top_char = &(p.as_str())[..1];

    if top_char == " " {
      removes(p);
      continue;
    }

    if top_char == "+" || top_char == "-" {
      if top_char == "+" {
        tokens[i].ty = Tk::Plus;
      } else {
        tokens[i].ty = Tk::Minus;
      }
      tokens[i].input = p.to_string();
      i += 1;
      removes(p);
      continue;
    }

    if p.chars().next().unwrap().is_digit(10) {
      tokens[i].ty = Tk::Num;
      tokens[i].input = p.to_string();
      tokens[i].val = strtol(p);
      i += 1;
      continue;
    }

    error_at(p, original, "トークナイズできません".to_string());
  }

  tokens[i].ty = Tk::EOF;
  tokens[i].input = p.to_string();
}