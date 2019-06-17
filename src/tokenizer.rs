use crate::util::*;

// トークンの型を表す
#[derive(Clone, PartialEq, Debug)]
pub enum Tk {
  Plus, // '+'
  Minus, // '-'
  Multi, // '*'
  Div, // '/'
  LParen, // '('
  RParen, // ')'
  Equ, // '=='
  Ne, // '!='
  Num(i32), // 数値
  EOF
}

#[derive(Clone, Debug)]
pub struct Token {
  pub ty: Tk,
  pub input: String,
}

impl Token {
  pub fn init() -> Token {
    Token { ty: Tk::EOF, input: "".to_string() }
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

    match top_char {
      " " => {
        remove_times(p, 1);
      }
      op @ "+" | op @ "-" | op @ "*" | op @ "/" | op @ "(" | op @ ")" => {
        if op == "+" {
          tokens[i].ty = Tk::Plus;
        } else if op == "-" {
          tokens[i].ty = Tk::Minus;
        } else if op == "*" {
          tokens[i].ty = Tk::Multi;
        } else if op == "/" {
          tokens[i].ty = Tk::Div;
        } else if op == "(" {
          tokens[i].ty = Tk::LParen;
        } else {
          tokens[i].ty = Tk::RParen;
        }
        tokens[i].input = p.to_string();
        i += 1;
        remove_times(p, 1);
      }
      op @ "=" | op @ "!" => {
        if op == "=" {
          if p.starts_with("==") {
            tokens[i].ty = Tk::Equ;
            i += 1;
            remove_times(p, 2);
          } else {
            error_at(p, original, "トークナイズできません");
          }
        } else {
          if p.starts_with("!=") {
            tokens[i].ty = Tk::Ne;
            i += 1;
            remove_times(p, 2);
          } else {
            error_at(p, original, "トークナイズできません");
          }
        }
      }
      _ => {
        if p.chars().next().unwrap().is_digit(10) {
          tokens[i].input = p.to_string();
          tokens[i].ty = Tk::Num(strtol(p));
          i += 1;
        } else {
          error_at(p, original, "トークナイズできません");
        }
      }
    }

  }

  tokens[i].ty = Tk::EOF;
  tokens[i].input = p.to_string();
}