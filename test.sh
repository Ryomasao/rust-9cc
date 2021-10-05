#!/bin/bash
assert() {
  expected="$1"
  input="$2"

  cargo run "$input"
  cc -o tmp tmp.s
  ./tmp
  actual="$?"

  if [ "$actual" = "$expected" ]; then
    echo "$input => $actual"
  else
    echo "$input => $expected expected, but got $actual"
    exit 1
  fi
}

assert 7 src/tests/expr.c
assert 1 src/tests/comp.c
assert 2 src/tests/base.c

echo OK