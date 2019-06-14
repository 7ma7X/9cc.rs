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
    exit 1
  fi
}

cargo build --release

try 0 0
try 42 42

echo OK
rm tmp.s tmp