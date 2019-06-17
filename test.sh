#!/bin/bash
try() {
  expected="$1"
  input="$2"

  ./target/release/9cc "$input" > tmp.s
  gcc -o tmp tmp.s
  ./tmp
  actual="$?"

  if [ "$actual" = "$expected" ]; then
    echo "$input => $actual"
  else
    echo "$expected expected, but got $actual"
    rm tmp.s tmp
    exit 1
  fi
}

cargo build --release

try 0 0
try 42 42
try 21 '5+20-4'
try 41 " 12 + 34 - 5 "
try 47 "5+6*7"
try 15 "5*(9-6)"
try 4 "(3+5)/2"
try 10 "-10+20"
try 75 "5*(9-(-6))"
try 58 "+10*6-(+2)"

try 1 " 1 + 2 == 3 "
try 0 "9==7"
try 1 "9!=7"
try 0 " 2  + 24 / 3  != 6 + 4"

echo OK
rm tmp.s tmp