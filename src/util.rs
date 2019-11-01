use std::process;

pub fn strtol(s: &mut String) -> i32 {
    let mut ans = 0;
    let mut index = 0;

    let mut is_negative = false;

    if &(s.as_str())[..1] == "-" {
        is_negative = true;
        remove_times(s, 1);
    }

    for c in s.chars() {
        if let Some(n) = c.to_digit(10) {
            ans = ans * 10 + (n as i32);
        } else {
            break;
        }
        index += 1;
    }

    for _ in 0..index {
        remove_times(s, 1);
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
pub fn error_at(unread_string_length: usize, user_input: &String, msg: &str) {
    let pos = user_input.len() - unread_string_length;
    let mut return_string = "".to_string();
    return_string += user_input;
    return_string += "\n";
    for _ in 0..pos {
        return_string += " ";
    }
    return_string += "^ ";
    return_string += msg;

    eprintln!("{}", return_string);
    process::exit(1);
}

/**
 * 指定された回数だけ先頭文字を削除する
 */
pub fn remove_times(s: &mut String, n: usize) {
    for _ in 0..n {
        if s.is_empty() {
            eprintln!("空文字を削除しようとしています");
            process::exit(1);
        } else {
            s.remove(0);
        }
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
    let unread_string_length = String::from("foo + 5").len();
    let user_input = String::from("1 + foo + 5");
    let msg = "トークナイズできません";

    error_at(unread_string_length, &user_input, msg);
}

#[test]
fn check_remove_times() {
    let mut test: String = "ueueo".to_string();
    remove_times(&mut test, 2);
    assert_eq!("ueo".to_string(), test);
    remove_times(&mut test, 3);
    assert_eq!("".to_string(), test);
}
