pub fn strtol(s: &mut String) -> i64 {
  let mut ans = 0;
  let mut index = 0;

  let mut is_negative = false;

  if &(s.as_str())[..1] == "-" {
    is_negative = true;
    s.remove(0);
  }

  for c in s.chars() {
    match c.to_digit(10) {
      Some(n) => {
        ans = ans * 10 + (n as i64);
      }
      None => {
        break;
      }
    }
    index += 1;
  }

  for _ in 0..index {
    s.remove(0);
  }
  
  if is_negative {
    ans * (-1)
  } else {
    ans
  }
}

#[test]
fn check_strtol() {
  let mut test: String = "12345ABC".to_string();
  assert_eq!(12345, strtol(&mut test));
  let mut test2: String = "-123ABC".to_string();
  assert_eq!(-123, strtol(&mut test2));
}