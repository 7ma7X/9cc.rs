use crate::util::*;

// トークンの型を表す
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Tk {
    Plus,     // '+'
    Minus,    // '-'
    Multi,    // '*'
    Div,      // '/'
    LParen,   // '('
    RParen,   // ')'
    Equ,      // '=='
    Ne,       // '!='
    Lt,       // '<'
    Le,       // '<='
    Gt,       // '>'
    Ge,       // '>='
    Num(i32), // 数値
    EOF,
}

impl Default for Tk {
    fn default() -> Self {
        Tk::EOF
    }
}

#[derive(Copy, Clone, Default, Debug)]
pub struct Token {
    pub ty: Tk,
    pub unread_string_length: usize,
}

/**
 * user_inputが指している文字列をトークンに分割してtokensに保存する
 */
pub fn tokenize(user_input: &mut String, original: &String, tokens: &mut Vec<Token>) {
    let p = user_input;

    while !(p.is_empty()) {
        let top_char = &(p.as_str())[..1];

        match top_char {
            " " => {
                remove_times(p, 1);
            }
            op @ "+" | op @ "-" | op @ "*" | op @ "/" | op @ "(" | op @ ")" => {
                let mut tk: Token = Default::default();
                if op == "+" {
                    tk.ty = Tk::Plus;
                } else if op == "-" {
                    tk.ty = Tk::Minus;
                } else if op == "*" {
                    tk.ty = Tk::Multi;
                } else if op == "/" {
                    tk.ty = Tk::Div;
                } else if op == "(" {
                    tk.ty = Tk::LParen;
                } else {
                    tk.ty = Tk::RParen;
                }
                tk.unread_string_length = p.to_string().len();
                tokens.push(tk);
                remove_times(p, 1);
            }
            op @ "=" | op @ "!" => {
                let mut tk: Token = Default::default();
                if op == "=" {
                    if p.starts_with("==") {
                        tk.ty = Tk::Equ;
                        tokens.push(tk);
                        remove_times(p, 2);
                    } else {
                        error_at(p.len(), original, "トークナイズできません");
                    }
                } else {
                    if p.starts_with("!=") {
                        tk.ty = Tk::Ne;
                        tokens.push(tk);
                        remove_times(p, 2);
                    } else {
                        error_at(p.len(), original, "トークナイズできません");
                    }
                }
            }
            op @ "<" | op @ ">" => {
                let mut tk: Token = Default::default();
                if op == "<" {
                    if p.starts_with("<=") {
                        tk.ty = Tk::Le;
                        remove_times(p, 2);
                    } else {
                        tk.ty = Tk::Lt;
                        remove_times(p, 1);
                    }
                } else {
                    if p.starts_with(">=") {
                        tk.ty = Tk::Ge;
                        remove_times(p, 2);
                    } else {
                        tk.ty = Tk::Gt;
                        remove_times(p, 1);
                    }
                }
                tokens.push(tk);
            }
            _ => {
                if p.chars().next().unwrap().is_digit(10) {
                    let mut tk: Token = Default::default();
                    tk.unread_string_length = p.to_string().len();
                    tk.ty = Tk::Num(strtol(p));
                    tokens.push(tk);
                } else {
                    error_at(p.len(), original, "トークナイズできません");
                }
            }
        }
    }

    let tk: Token = Default::default();
    tokens.push(tk);
}
