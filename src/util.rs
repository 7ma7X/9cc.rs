use std::process;

pub fn strtol(s: &mut String) -> i32 {
  let mut ans = 0;
  let mut index = 0;

  let mut is_negative = false;

  if &(s.as_str())[..1] == "-" {
    is_negative = true;
    removes(s);
  }

  for c in s.chars() {
    match c.to_digit(10) {
      Some(n) => {
        ans = ans * 10 + (n as i32);
      }
      None => {
        break;
      }
    }
    index += 1;
  }

  for _ in 0..index {
    removes(s);
  }
  
  if is_negative {
    ans * (-1)
  } else {
    ans
  }
}

/**
 * エラー箇所を報告するための関数
 */
pub fn error_at(loc: &String, user_input: &String, msg: String) {
  let pos = user_input.len() - loc.len();
  let mut return_string = "".to_string();
  return_string += user_input;
  return_string += "\n";
  for _ in 0..pos {
    return_string += " ";
  }
  return_string += "^ ";
  return_string += msg.as_str();

  eprintln!("{}", return_string);
  process::exit(1);
}

/**
 * 空文字の先頭文字を削除する際にエラーを出す
 */
pub fn removes(s: &mut String) {
  if s.is_empty() {
    eprintln!("空文字を削除しようとしています");
    process::exit(1);
  } else {
    s.remove(0);
  }
}

#[test]
fn check_strtol() {
  let mut test: String = "12345ABC".to_string();
  assert_eq!(12345, strtol(&mut test));
  let mut test2: String = "-123ABC".to_string();
  assert_eq!(-123, strtol(&mut test2));
}

#[test]
fn check_error_at() {
  let loc = String::from("foo + 5");
  let user_input = String::from("1 + foo + 5");
  let msg = String::from("トークナイズできません");

  error_at(&loc, &user_input, msg);
}

#[test]
fn check_removes() {
  let mut test: String = "ue".to_string();
  removes(&mut test);
  assert_eq!("e".to_string(), test);
  removes(&mut test);
  assert_eq!("".to_string(), test);
}